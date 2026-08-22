#![forbid(unsafe_code)]

use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    extract::{Extension, Path, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    fmt::{Display, Formatter},
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use tower_http::{
    limit::RequestBodyLimitLayer, sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};

pub const CONTRACT_MAX_BODY_BYTES: usize = 65_536;
pub const CONTRACT_MAX_PATH_BYTES: usize = 2_048;
pub const CONTRACT_MAX_REQUEST_ID_BYTES: usize = 128;
pub const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 10_000;
pub const DEFAULT_RATE_LIMIT: u64 = 60;
pub const DEFAULT_RATE_WINDOW_MS: u64 = 60_000;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub max_body_bytes: usize,
    pub max_path_bytes: usize,
    pub max_request_id_bytes: usize,
    pub request_timeout: Duration,
    pub rate_limit: u64,
    pub rate_window: Duration,
    pub rate_limit_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bind_addr: SocketAddr::from(([127, 0, 0, 1], 3_000)),
            max_body_bytes: CONTRACT_MAX_BODY_BYTES,
            max_path_bytes: CONTRACT_MAX_PATH_BYTES,
            max_request_id_bytes: CONTRACT_MAX_REQUEST_ID_BYTES,
            request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS),
            rate_limit: DEFAULT_RATE_LIMIT,
            rate_window: Duration::from_millis(DEFAULT_RATE_WINDOW_MS),
            rate_limit_key: "demo-host".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let defaults = Self::default();
        let bind_addr = env::var("ZAP_HOST_ADDR")
            .ok()
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| ConfigError::invalid("ZAP_HOST_ADDR"))
            })
            .transpose()?
            .unwrap_or(defaults.bind_addr);
        let max_body_bytes = env_u64("ZAP_HOST_MAX_BODY_BYTES")?
            .map(|value| value as usize)
            .unwrap_or(defaults.max_body_bytes);
        let request_timeout_ms =
            env_u64("ZAP_HOST_REQUEST_TIMEOUT_MS")?.unwrap_or(DEFAULT_REQUEST_TIMEOUT_MS);
        let rate_limit = env_u64("ZAP_HOST_RATE_LIMIT")?.unwrap_or(DEFAULT_RATE_LIMIT);
        let rate_window_ms = env_u64("ZAP_HOST_RATE_WINDOW_MS")?.unwrap_or(DEFAULT_RATE_WINDOW_MS);
        let rate_limit_key = env::var("ZAP_HOST_RATE_KEY").unwrap_or(defaults.rate_limit_key);
        let config = Self {
            bind_addr,
            max_body_bytes,
            max_path_bytes: CONTRACT_MAX_PATH_BYTES,
            max_request_id_bytes: CONTRACT_MAX_REQUEST_ID_BYTES,
            request_timeout: Duration::from_millis(request_timeout_ms),
            rate_limit,
            rate_window: Duration::from_millis(rate_window_ms),
            rate_limit_key,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_body_bytes == 0 || self.max_body_bytes > CONTRACT_MAX_BODY_BYTES {
            return Err(ConfigError::message(
                "max_body_bytes must be between 1 and 65536",
            ));
        }
        if self.max_path_bytes == 0 || self.max_path_bytes > CONTRACT_MAX_PATH_BYTES {
            return Err(ConfigError::message(
                "max_path_bytes must be between 1 and 2048",
            ));
        }
        if self.max_request_id_bytes == 0
            || self.max_request_id_bytes > CONTRACT_MAX_REQUEST_ID_BYTES
        {
            return Err(ConfigError::message(
                "max_request_id_bytes must be between 1 and 128",
            ));
        }
        if self.request_timeout.is_zero() {
            return Err(ConfigError::message(
                "request_timeout must be greater than zero",
            ));
        }
        if self.rate_limit == 0 || self.rate_window.is_zero() {
            return Err(ConfigError::message(
                "rate_limit and rate_window must be greater than zero",
            ));
        }
        if self.rate_limit_key.is_empty() || self.rate_limit_key.len() > 256 {
            return Err(ConfigError::message(
                "rate_limit_key must contain 1 to 256 bytes",
            ));
        }
        Ok(())
    }
}

fn env_u64(name: &str) -> Result<Option<u64>, ConfigError> {
    env::var(name)
        .ok()
        .map(|value| value.parse().map_err(|_| ConfigError::invalid(name)))
        .transpose()
}

#[derive(Debug, Clone)]
pub struct ConfigError(String);

impl ConfigError {
    fn invalid(name: &str) -> Self {
        Self(format!(
            "{name} must be an unsigned integer or valid socket address"
        ))
    }

    fn message(message: &str) -> Self {
        Self(message.to_string())
    }
}

impl Display for ConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone)]
pub struct RequestId(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct Identity {
    pub subject: String,
    pub scopes: HashSet<String>,
}

impl Identity {
    pub fn new(
        subject: impl Into<String>,
        scopes: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            scopes: scopes.into_iter().map(Into::into).collect(),
        }
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.contains(scope)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AuthFailure {
    Missing,
    Forbidden,
    Invalid,
    Internal,
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, request: &Request<Body>) -> Result<Identity, AuthFailure>;
}

#[derive(Debug, Default, Clone)]
pub struct DemoAuthenticator;

#[async_trait]
impl Authenticator for DemoAuthenticator {
    async fn authenticate(&self, _request: &Request<Body>) -> Result<Identity, AuthFailure> {
        Ok(Identity::new("demo-subject", ["users:read", "users:write"]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimitError {
    InvalidPolicy,
    EmptyKey,
    ClockReversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitDecision {
    pub allowed: bool,
    pub remaining: u64,
    pub retry_after_secs: u64,
}

#[derive(Debug, Clone, Copy)]
struct WindowState {
    window_start: Instant,
    used: u64,
}

#[derive(Debug, Default, Clone)]
pub struct FixedWindowStore {
    states: Arc<Mutex<HashMap<String, WindowState>>>,
}

impl FixedWindowStore {
    pub fn check_now(
        &self,
        key: &str,
        limit: u64,
        window: Duration,
    ) -> Result<RateLimitDecision, RateLimitError> {
        self.check_at(key, Instant::now(), limit, window)
    }

    pub fn check_at(
        &self,
        key: &str,
        now: Instant,
        limit: u64,
        window: Duration,
    ) -> Result<RateLimitDecision, RateLimitError> {
        if key.is_empty() {
            return Err(RateLimitError::EmptyKey);
        }
        if limit == 0 || window.is_zero() {
            return Err(RateLimitError::InvalidPolicy);
        }
        let mut states = self
            .states
            .lock()
            .map_err(|_| RateLimitError::InvalidPolicy)?;
        let entry = states.entry(key.to_string()).or_insert(WindowState {
            window_start: now,
            used: 0,
        });
        if now < entry.window_start {
            return Err(RateLimitError::ClockReversal);
        }
        let elapsed = now.duration_since(entry.window_start);
        if elapsed >= window {
            entry.window_start = now;
            entry.used = 0;
        }
        if entry.used >= limit {
            let remaining = window.saturating_sub(now.duration_since(entry.window_start));
            return Ok(RateLimitDecision {
                allowed: false,
                remaining: 0,
                retry_after_secs: ceil_seconds(remaining),
            });
        }
        entry.used += 1;
        Ok(RateLimitDecision {
            allowed: true,
            remaining: limit - entry.used,
            retry_after_secs: 0,
        })
    }
}

fn ceil_seconds(duration: Duration) -> u64 {
    let seconds = duration.as_secs();
    if duration.subsec_nanos() > 0 {
        seconds.saturating_add(1).max(1)
    } else {
        seconds.max(1)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct NormalizedCreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    InvalidShape,
    InvalidName,
    InvalidEmail,
}

pub fn normalize_create_user(input: CreateUserRequest) -> Result<NormalizedCreateUser, InputError> {
    let name = input.name.trim().to_string();
    let email = input.email.trim().to_lowercase();
    if name.is_empty() || name.chars().count() > 120 {
        return Err(InputError::InvalidName);
    }
    if email.is_empty() || email.chars().count() > 254 || !email.contains('@') {
        return Err(InputError::InvalidEmail);
    }
    Ok(NormalizedCreateUser { name, email })
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicUser {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct DbUser {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseError {
    Unavailable,
    Duplicate,
    Internal,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user(&self, user_id: u64) -> Result<Option<DbUser>, DatabaseError>;
    async fn create_user(&self, input: NormalizedCreateUser) -> Result<DbUser, DatabaseError>;
    async fn list_users(&self) -> Result<Vec<DbUser>, DatabaseError>;
}

#[derive(Debug, Clone, Copy)]
pub enum GatewayError {
    Unavailable,
    Duplicate,
    Internal,
}

pub struct ContractGateway<R> {
    repository: Arc<R>,
}

impl<R> ContractGateway<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait WebGateway: Send + Sync {
    async fn get_user(&self, user_id: u64) -> Result<Option<PublicUser>, GatewayError>;
    async fn create_user(&self, input: NormalizedCreateUser) -> Result<PublicUser, GatewayError>;
    async fn list_users(&self) -> Result<Vec<PublicUser>, GatewayError>;
}

#[async_trait]
impl<R> WebGateway for ContractGateway<R>
where
    R: UserRepository + 'static,
{
    async fn get_user(&self, user_id: u64) -> Result<Option<PublicUser>, GatewayError> {
        self.repository
            .get_user(user_id)
            .await
            .map_err(map_database_error)
            .map(|row| row.map(public_user))
    }

    async fn create_user(&self, input: NormalizedCreateUser) -> Result<PublicUser, GatewayError> {
        self.repository
            .create_user(input)
            .await
            .map_err(map_database_error)
            .map(public_user)
    }

    async fn list_users(&self) -> Result<Vec<PublicUser>, GatewayError> {
        self.repository
            .list_users()
            .await
            .map_err(map_database_error)
            .map(|rows| rows.into_iter().map(public_user).collect())
    }
}

fn map_database_error(error: DatabaseError) -> GatewayError {
    match error {
        DatabaseError::Unavailable => GatewayError::Unavailable,
        DatabaseError::Duplicate => GatewayError::Duplicate,
        DatabaseError::Internal => GatewayError::Internal,
    }
}

fn public_user(row: DbUser) -> PublicUser {
    PublicUser {
        id: row.id,
        name: row.name,
        email: row.email,
    }
}

#[derive(Debug, Default)]
pub struct MemoryRepository {
    rows: Mutex<BTreeMap<u64, DbUser>>,
    next_id: AtomicU64,
}

impl MemoryRepository {
    pub fn demo() -> Self {
        let repository = Self::default();
        repository.next_id.store(1, Ordering::Relaxed);
        let mut rows = repository
            .rows
            .lock()
            .expect("demo repository lock poisoned");
        rows.insert(
            1,
            DbUser {
                id: 1,
                name: "Ada".to_string(),
                email: "ada@example.com".to_string(),
            },
        );
        drop(rows);
        repository
    }
}

#[async_trait]
impl UserRepository for MemoryRepository {
    async fn get_user(&self, user_id: u64) -> Result<Option<DbUser>, DatabaseError> {
        let rows = self.rows.lock().map_err(|_| DatabaseError::Internal)?;
        Ok(rows.get(&user_id).cloned())
    }

    async fn create_user(&self, input: NormalizedCreateUser) -> Result<DbUser, DatabaseError> {
        let mut rows = self.rows.lock().map_err(|_| DatabaseError::Internal)?;
        if rows.values().any(|row| row.email == input.email) {
            return Err(DatabaseError::Duplicate);
        }
        let id = self
            .next_id
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        let row = DbUser {
            id,
            name: input.name,
            email: input.email,
        };
        rows.insert(id, row.clone());
        Ok(row)
    }

    async fn list_users(&self) -> Result<Vec<DbUser>, DatabaseError> {
        let rows = self.rows.lock().map_err(|_| DatabaseError::Internal)?;
        Ok(rows.values().cloned().collect())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub gateway: Arc<dyn WebGateway>,
    pub authenticator: Arc<dyn Authenticator>,
    pub rate_limiter: FixedWindowStore,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        gateway: Arc<dyn WebGateway>,
        authenticator: Arc<dyn Authenticator>,
    ) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            gateway,
            authenticator,
            rate_limiter: FixedWindowStore::default(),
        })
    }

    pub fn demo(config: AppConfig) -> Result<Self, ConfigError> {
        let repository = Arc::new(MemoryRepository::demo());
        let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
        let authenticator: Arc<dyn Authenticator> = Arc::new(DemoAuthenticator);
        Self::new(config, gateway, authenticator)
    }
}

pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user))
        // The last-added middleware is the outermost layer. Keep rate limiting
        // before authentication so rejected requests do not reach the gateway.
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ));

    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .merge(api)
        .layer(SetSensitiveHeadersLayer::new([
            header::AUTHORIZATION,
            header::COOKIE,
        ]))
        .layer(RequestBodyLimitLayer::new(state.config.max_body_bytes))
        .layer(TimeoutLayer::new(state.config.request_timeout))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            request_policy_middleware,
        ))
        .with_state(state)
}

async fn request_policy_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = match request.headers().get(request_id_header()) {
        Some(value) => match value.to_str() {
            Ok(value)
                if !value.is_empty()
                    && value.as_bytes().len() <= state.config.max_request_id_bytes =>
            {
                value.to_string()
            }
            _ => {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    "invalid_request",
                    "request ID is invalid",
                    "unassigned",
                )
            }
        },
        None => next_request_id(),
    };
    let path = request.uri().path();
    if !path.starts_with('/')
        || path.as_bytes().len() > state.config.max_path_bytes
        || path.contains("..")
    {
        return policy_error_response(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "request path is invalid",
            &request_id,
        );
    }
    if request.method() != axum::http::Method::GET && request.method() != axum::http::Method::POST {
        return policy_error_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
            "HTTP method is not supported",
            &request_id,
        );
    }
    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));
    let mut response = next.run(request).await;
    if response.status() == StatusCode::PAYLOAD_TOO_LARGE {
        response = error_response(
            StatusCode::BAD_REQUEST,
            "invalid_request",
            "request body is too large",
            &request_id,
        );
    } else if response.status() == StatusCode::REQUEST_TIMEOUT {
        response = error_response(
            StatusCode::REQUEST_TIMEOUT,
            "request_timeout",
            "request exceeded the host timeout",
            &request_id,
        );
    }
    apply_security_headers(&mut response, &request_id);
    response
}

fn next_request_id() -> String {
    static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);
    format!("host-{}", REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed))
}

async fn rate_limit_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = request_id(&request);
    let decision = match state.rate_limiter.check_now(
        &state.config.rate_limit_key,
        state.config.rate_limit,
        state.config.rate_window,
    ) {
        Ok(decision) => decision,
        Err(_) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "rate_limit_unavailable",
                "rate-limit policy is unavailable",
                &request_id,
            )
        }
    };
    if !decision.allowed {
        let mut response = error_response(
            StatusCode::TOO_MANY_REQUESTS,
            "rate_limited",
            "request quota exceeded",
            &request_id,
        );
        if let Ok(value) = HeaderValue::from_str(&decision.retry_after_secs.to_string()) {
            response.headers_mut().insert(header::RETRY_AFTER, value);
        }
        return response;
    }
    next.run(request).await
}

async fn auth_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = request_id(&request);
    match state.authenticator.authenticate(&request).await {
        Ok(identity) => {
            let mut request = request;
            request.extensions_mut().insert(identity);
            next.run(request).await
        }
        Err(AuthFailure::Missing | AuthFailure::Invalid) => error_response(
            StatusCode::UNAUTHORIZED,
            "unauthenticated",
            "authentication is required",
            &request_id,
        ),
        Err(AuthFailure::Forbidden) => error_response(
            StatusCode::FORBIDDEN,
            "forbidden",
            "authenticated identity is not permitted",
            &request_id,
        ),
        Err(AuthFailure::Internal) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "authentication_unavailable",
            "authentication service is unavailable",
            &request_id,
        ),
    }
}

async fn root(Extension(RequestId(request_id)): Extension<RequestId>) -> Response {
    json_response(
        StatusCode::OK,
        json!({"message": "Hello from Zap Web", "request_id": request_id}),
    )
}

async fn health() -> Response {
    json_response(StatusCode::OK, json!({"status": "ok"}))
}

async fn get_user(
    State(state): State<AppState>,
    Extension(RequestId(request_id)): Extension<RequestId>,
    Extension(identity): Extension<Identity>,
    Path(raw_id): Path<String>,
) -> Response {
    if !identity.has_scope("users:read") {
        return error_response(
            StatusCode::FORBIDDEN,
            "forbidden",
            "required scope is missing",
            &request_id,
        );
    }
    let user_id = match raw_id.parse::<u64>() {
        Ok(user_id) if user_id > 0 => user_id,
        _ => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_path",
                "user ID is invalid",
                &request_id,
            )
        }
    };
    match state.gateway.get_user(user_id).await {
        Ok(Some(user)) => json_response(
            StatusCode::OK,
            json!({"data": user, "request_id": request_id}),
        ),
        Ok(None) => error_response(
            StatusCode::NOT_FOUND,
            "not_found",
            "user was not found",
            &request_id,
        ),
        Err(error) => gateway_error_response(error, &request_id),
    }
}

async fn list_users(
    State(state): State<AppState>,
    Extension(RequestId(request_id)): Extension<RequestId>,
    Extension(identity): Extension<Identity>,
) -> Response {
    if !identity.has_scope("users:read") {
        return error_response(
            StatusCode::FORBIDDEN,
            "forbidden",
            "required scope is missing",
            &request_id,
        );
    }
    match state.gateway.list_users().await {
        Ok(users) => json_response(
            StatusCode::OK,
            json!({"count": users.len(), "data": users, "request_id": request_id}),
        ),
        Err(error) => gateway_error_response(error, &request_id),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Extension(RequestId(request_id)): Extension<RequestId>,
    Extension(identity): Extension<Identity>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !identity.has_scope("users:write") {
        return error_response(
            StatusCode::FORBIDDEN,
            "forbidden",
            "required scope is missing",
            &request_id,
        );
    }
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    let media_type = content_type.split(';').next().map(str::trim).unwrap_or("");
    if !media_type.eq_ignore_ascii_case("application/json") {
        return error_response(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "unsupported_media_type",
            "content type must be application/json",
            &request_id,
        );
    }
    let input = match serde_json::from_slice::<CreateUserRequest>(&body) {
        Ok(input) => input,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_body",
                "request body must contain string name and email",
                &request_id,
            )
        }
    };
    let normalized = match normalize_create_user(input) {
        Ok(value) => value,
        Err(InputError::InvalidName) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_name",
                "name must contain 1 to 120 characters",
                &request_id,
            )
        }
        Err(InputError::InvalidEmail) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_email",
                "email must contain 1 to 254 characters and an @ marker",
                &request_id,
            )
        }
        Err(InputError::InvalidShape) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_body",
                "request body shape is invalid",
                &request_id,
            )
        }
    };
    match state.gateway.create_user(normalized).await {
        Ok(user) => json_response(
            StatusCode::CREATED,
            json!({"data": user, "request_id": request_id}),
        ),
        Err(error) => gateway_error_response(error, &request_id),
    }
}

fn gateway_error_response(error: GatewayError, request_id: &str) -> Response {
    match error {
        GatewayError::Unavailable => error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "repository_unavailable",
            "user repository is unavailable",
            request_id,
        ),
        GatewayError::Duplicate => error_response(
            StatusCode::CONFLICT,
            "duplicate_user",
            "a user with this email already exists",
            request_id,
        ),
        GatewayError::Internal => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "the request could not be completed",
            request_id,
        ),
    }
}

fn request_id(request: &Request<Body>) -> String {
    request
        .extensions()
        .get::<RequestId>()
        .map(|value| value.0.clone())
        .unwrap_or_else(|| "unassigned".to_string())
}

fn request_id_header() -> HeaderName {
    HeaderName::from_static("x-request-id")
}

fn json_response(status: StatusCode, payload: Value) -> Response {
    (status, axum::Json(payload)).into_response()
}

fn error_response(status: StatusCode, code: &str, message: &str, request_id: &str) -> Response {
    json_response(
        status,
        json!({"error": code, "message": message, "request_id": request_id}),
    )
}

fn policy_error_response(
    status: StatusCode,
    code: &str,
    message: &str,
    request_id: &str,
) -> Response {
    let mut response = error_response(status, code, message, request_id);
    apply_security_headers(&mut response, request_id);
    response
}

fn apply_security_headers(response: &mut Response, request_id: &str) {
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    if let Ok(value) = HeaderValue::from_str(request_id) {
        response.headers_mut().insert(request_id_header(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_window_resets_and_rejects_clock_reversal() {
        let store = FixedWindowStore::default();
        let start = Instant::now();
        let first = store
            .check_at("subject:test", start, 1, Duration::from_secs(60))
            .expect("first request should be allowed");
        assert!(first.allowed);
        assert_eq!(first.remaining, 0);

        let limited = store
            .check_at(
                "subject:test",
                start + Duration::from_secs(1),
                1,
                Duration::from_secs(60),
            )
            .expect("quota exhaustion should be a decision");
        assert!(!limited.allowed);
        assert_eq!(limited.remaining, 0);
        assert!(limited.retry_after_secs >= 59);

        assert_eq!(
            store.check_at(
                "subject:test",
                start
                    .checked_sub(Duration::from_secs(1))
                    .expect("valid instant"),
                1,
                Duration::from_secs(60),
            ),
            Err(RateLimitError::ClockReversal)
        );

        let reset = store
            .check_at(
                "subject:test",
                start + Duration::from_secs(60),
                1,
                Duration::from_secs(60),
            )
            .expect("expired window should reset");
        assert!(reset.allowed);
        assert_eq!(reset.remaining, 0);
    }

    #[test]
    fn configuration_rejects_unsafe_bounds() {
        let invalid = AppConfig {
            max_body_bytes: CONTRACT_MAX_BODY_BYTES + 1,
            ..AppConfig::default()
        };
        assert!(invalid.validate().is_err());

        let invalid_timeout = AppConfig {
            request_timeout: Duration::ZERO,
            ..AppConfig::default()
        };
        assert!(invalid_timeout.validate().is_err());
    }
}
