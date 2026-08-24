#![forbid(unsafe_code)]

pub mod auth;

use async_trait::async_trait;
use axum::{
    body::{Body, Bytes},
    extract::{Extension, Path, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, Request, StatusCode, Uri},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    fmt::{Display, Formatter},
    net::SocketAddr,
    ops::Bound,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
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
pub const DEFAULT_SHUTDOWN_TIMEOUT_MS: u64 = 30_000;
pub const DEFAULT_USER_PAGE_SIZE: usize = 50;
pub const MAX_USER_PAGE_SIZE: usize = 100;
pub const DEFAULT_DB_MAX_CONNECTIONS: usize = 16;
pub const DEFAULT_DB_ACQUIRE_TIMEOUT_MS: u64 = 1_000;
pub const DEFAULT_DB_QUERY_TIMEOUT_MS: u64 = 5_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatabasePoolConfig {
    pub max_connections: usize,
    pub acquire_timeout: Duration,
    pub query_timeout: Duration,
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            max_connections: DEFAULT_DB_MAX_CONNECTIONS,
            acquire_timeout: Duration::from_millis(DEFAULT_DB_ACQUIRE_TIMEOUT_MS),
            query_timeout: Duration::from_millis(DEFAULT_DB_QUERY_TIMEOUT_MS),
        }
    }
}

impl DatabasePoolConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_connections == 0 || self.max_connections > 256 {
            return Err(ConfigError::message(
                "database max_connections must be between 1 and 256",
            ));
        }
        if self.acquire_timeout.is_zero() || self.acquire_timeout > Duration::from_secs(30) {
            return Err(ConfigError::message(
                "database acquire_timeout must be between 1ms and 30s",
            ));
        }
        if self.query_timeout.is_zero() || self.query_timeout > Duration::from_secs(120) {
            return Err(ConfigError::message(
                "database query_timeout must be between 1ms and 120s",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolAcquireError {
    Closed,
    Timeout,
}

#[derive(Clone)]
pub struct DatabasePoolGate {
    semaphore: Arc<Semaphore>,
    config: DatabasePoolConfig,
}

impl DatabasePoolGate {
    pub fn new(config: DatabasePoolConfig) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            config,
        })
    }

    pub async fn acquire(&self) -> Result<DatabasePoolPermit, PoolAcquireError> {
        let permit = tokio::time::timeout(
            self.config.acquire_timeout,
            self.semaphore.clone().acquire_owned(),
        )
        .await
        .map_err(|_| PoolAcquireError::Timeout)?
        .map_err(|_| PoolAcquireError::Closed)?;
        Ok(DatabasePoolPermit { permit })
    }

    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub fn close(&self) {
        self.semaphore.close();
    }
}

pub struct DatabasePoolPermit {
    permit: OwnedSemaphorePermit,
}

impl DatabasePoolPermit {
    pub fn is_acquired(&self) -> bool {
        let _permit = &self.permit;
        true
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub max_body_bytes: usize,
    pub max_path_bytes: usize,
    pub max_request_id_bytes: usize,
    pub request_timeout: Duration,
    pub shutdown_timeout: Duration,
    pub rate_limit: u64,
    pub rate_window: Duration,
    pub rate_limit_key: String,
    pub database_pool: DatabasePoolConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            bind_addr: SocketAddr::from(([127, 0, 0, 1], 3_000)),
            max_body_bytes: CONTRACT_MAX_BODY_BYTES,
            max_path_bytes: CONTRACT_MAX_PATH_BYTES,
            max_request_id_bytes: CONTRACT_MAX_REQUEST_ID_BYTES,
            request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS),
            shutdown_timeout: Duration::from_millis(DEFAULT_SHUTDOWN_TIMEOUT_MS),
            rate_limit: DEFAULT_RATE_LIMIT,
            rate_window: Duration::from_millis(DEFAULT_RATE_WINDOW_MS),
            rate_limit_key: "demo-host".to_string(),
            database_pool: DatabasePoolConfig::default(),
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
        let shutdown_timeout_ms =
            env_u64("ZAP_HOST_SHUTDOWN_TIMEOUT_MS")?.unwrap_or(DEFAULT_SHUTDOWN_TIMEOUT_MS);
        let rate_limit = env_u64("ZAP_HOST_RATE_LIMIT")?.unwrap_or(DEFAULT_RATE_LIMIT);
        let rate_window_ms = env_u64("ZAP_HOST_RATE_WINDOW_MS")?.unwrap_or(DEFAULT_RATE_WINDOW_MS);
        let rate_limit_key = env::var("ZAP_HOST_RATE_KEY").unwrap_or(defaults.rate_limit_key);
        let database_pool = DatabasePoolConfig {
            max_connections: env_u64("ZAP_DB_MAX_CONNECTIONS")?
                .map(|value| value as usize)
                .unwrap_or(DEFAULT_DB_MAX_CONNECTIONS),
            acquire_timeout: Duration::from_millis(
                env_u64("ZAP_DB_ACQUIRE_TIMEOUT_MS")?.unwrap_or(DEFAULT_DB_ACQUIRE_TIMEOUT_MS),
            ),
            query_timeout: Duration::from_millis(
                env_u64("ZAP_DB_QUERY_TIMEOUT_MS")?.unwrap_or(DEFAULT_DB_QUERY_TIMEOUT_MS),
            ),
        };
        let config = Self {
            bind_addr,
            max_body_bytes,
            max_path_bytes: CONTRACT_MAX_PATH_BYTES,
            max_request_id_bytes: CONTRACT_MAX_REQUEST_ID_BYTES,
            request_timeout: Duration::from_millis(request_timeout_ms),
            shutdown_timeout: Duration::from_millis(shutdown_timeout_ms),
            rate_limit,
            rate_window: Duration::from_millis(rate_window_ms),
            rate_limit_key,
            database_pool,
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
        if self.shutdown_timeout.is_zero() {
            return Err(ConfigError::message(
                "shutdown_timeout must be greater than zero",
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
        self.database_pool.validate()?;
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
    Unavailable,
    Internal,
}

#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, headers: &HeaderMap) -> Result<Identity, AuthFailure>;
}

#[derive(Debug, Default, Clone)]
pub struct DemoAuthenticator;

#[async_trait]
impl Authenticator for DemoAuthenticator {
    async fn authenticate(&self, _headers: &HeaderMap) -> Result<Identity, AuthFailure> {
        Ok(Identity::new("demo-subject", ["users:read", "users:write"]))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReadinessError {
    Unavailable,
    Internal,
}

#[async_trait]
pub trait ReadinessProbe: Send + Sync {
    async fn check(&self) -> Result<(), ReadinessError>;
}

#[derive(Debug, Default, Clone)]
pub struct DemoReadiness;

#[async_trait]
impl ReadinessProbe for DemoReadiness {
    async fn check(&self) -> Result<(), ReadinessError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct LifecycleState {
    draining: AtomicBool,
}

impl LifecycleState {
    pub fn begin_draining(&self) {
        self.draining.store(true, Ordering::Release);
    }

    pub fn is_draining(&self) -> bool {
        self.draining.load(Ordering::Acquire)
    }
}

/// Low-cardinality process metrics for operator health checks.
///
/// The counters intentionally do not include paths, identities, request IDs, or
/// user-controlled labels. This keeps the endpoint bounded and avoids turning
/// observability into an unbounded memory or secret-leak surface.
#[derive(Debug, Default)]
pub struct Metrics {
    requests_total: AtomicU64,
    responses_5xx_total: AtomicU64,
    in_flight_requests: AtomicU64,
}

impl Metrics {
    fn request_started(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
        self.in_flight_requests.fetch_add(1, Ordering::Relaxed);
    }

    fn response_finished(&self, status: StatusCode) {
        if status.is_server_error() {
            self.responses_5xx_total.fetch_add(1, Ordering::Relaxed);
        }
        self.in_flight_requests.fetch_sub(1, Ordering::Relaxed);
    }

    fn render(&self) -> String {
        format!(
            "# HELP zap_requests_total Total HTTP requests observed by the host adapter.\n# TYPE zap_requests_total counter\nzap_requests_total {}\n# HELP zap_responses_5xx_total Total HTTP responses with a 5xx status.\n# TYPE zap_responses_5xx_total counter\nzap_responses_5xx_total {}\n# HELP zap_in_flight_requests Current HTTP requests being processed.\n# TYPE zap_in_flight_requests gauge\nzap_in_flight_requests {}\n",
            self.requests_total.load(Ordering::Relaxed),
            self.responses_5xx_total.load(Ordering::Relaxed),
            self.in_flight_requests.load(Ordering::Relaxed),
        )
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
    async fn list_users(
        &self,
        limit: usize,
        after_id: Option<u64>,
    ) -> Result<Vec<DbUser>, DatabaseError>;
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
    async fn list_users(
        &self,
        limit: usize,
        after_id: Option<u64>,
    ) -> Result<Vec<PublicUser>, GatewayError>;
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

    async fn list_users(
        &self,
        limit: usize,
        after_id: Option<u64>,
    ) -> Result<Vec<PublicUser>, GatewayError> {
        self.repository
            .list_users(limit, after_id)
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

pub struct SqliteRepository {
    connection: Arc<Mutex<rusqlite::Connection>>,
    pool_gate: DatabasePoolGate,
}

impl SqliteRepository {
    pub fn connect(
        database_url: &str,
        pool_config: DatabasePoolConfig,
    ) -> Result<Self, ConfigError> {
        if database_url.is_empty() || database_url.len() > 4096 {
            return Err(ConfigError::message(
                "DATABASE_URL must contain 1 to 4096 bytes",
            ));
        }
        pool_config.validate()?;
        let connection = rusqlite::Connection::open(database_url)
            .map_err(|_| ConfigError::message("DATABASE_URL could not be opened"))?;
        connection
            .busy_timeout(pool_config.query_timeout)
            .map_err(|_| ConfigError::message("database busy timeout could not be configured"))?;
        connection
            .execute_batch(
                "PRAGMA foreign_keys = ON;
                 CREATE TABLE IF NOT EXISTS users (
                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                     name TEXT NOT NULL,
                     email TEXT NOT NULL UNIQUE
                 );",
            )
            .map_err(|_| ConfigError::message("database schema could not be initialized"))?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            pool_gate: DatabasePoolGate::new(pool_config)?,
        })
    }
}

pub struct SqliteReadiness {
    repository: Arc<SqliteRepository>,
}

#[async_trait]
impl ReadinessProbe for SqliteReadiness {
    async fn check(&self) -> Result<(), ReadinessError> {
        let _permit = self
            .repository
            .pool_gate
            .acquire()
            .await
            .map_err(|_| ReadinessError::Unavailable)?;
        let connection = self.repository.connection.clone();
        tokio::task::spawn_blocking(move || {
            let connection = connection.lock().map_err(|_| ReadinessError::Internal)?;
            connection
                .query_row("SELECT 1", [], |_| Ok(()))
                .map_err(|_| ReadinessError::Unavailable)
        })
        .await
        .map_err(|_| ReadinessError::Internal)?
    }
}

fn map_sqlite_error(error: rusqlite::Error) -> DatabaseError {
    match error {
        rusqlite::Error::SqliteFailure(error, _)
            if matches!(
                error.extended_code,
                rusqlite::ffi::SQLITE_CONSTRAINT
                    | rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY
                    | rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE
            ) =>
        {
            DatabaseError::Duplicate
        }
        _ => DatabaseError::Internal,
    }
}

#[async_trait]
impl UserRepository for SqliteRepository {
    async fn get_user(&self, user_id: u64) -> Result<Option<DbUser>, DatabaseError> {
        let _permit = self
            .pool_gate
            .acquire()
            .await
            .map_err(|_| DatabaseError::Unavailable)?;
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            let connection = connection.lock().map_err(|_| DatabaseError::Internal)?;
            let mut statement = connection
                .prepare("SELECT id, name, email FROM users WHERE id = ?1")
                .map_err(map_sqlite_error)?;
            statement
                .query_row([user_id], |row| {
                    Ok(DbUser {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                    })
                })
                .optional()
                .map_err(map_sqlite_error)
        })
        .await
        .map_err(|_| DatabaseError::Internal)?
    }

    async fn create_user(&self, input: NormalizedCreateUser) -> Result<DbUser, DatabaseError> {
        let _permit = self
            .pool_gate
            .acquire()
            .await
            .map_err(|_| DatabaseError::Unavailable)?;
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            let connection = connection.lock().map_err(|_| DatabaseError::Internal)?;
            connection
                .execute(
                    "INSERT INTO users (name, email) VALUES (?1, ?2)",
                    (&input.name, &input.email),
                )
                .map_err(map_sqlite_error)?;
            let id = connection.last_insert_rowid() as u64;
            Ok(DbUser {
                id,
                name: input.name,
                email: input.email,
            })
        })
        .await
        .map_err(|_| DatabaseError::Internal)?
    }

    async fn list_users(
        &self,
        limit: usize,
        after_id: Option<u64>,
    ) -> Result<Vec<DbUser>, DatabaseError> {
        let _permit = self
            .pool_gate
            .acquire()
            .await
            .map_err(|_| DatabaseError::Unavailable)?;
        let connection = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            let connection = connection.lock().map_err(|_| DatabaseError::Internal)?;
            let mut statement = connection
                .prepare("SELECT id, name, email FROM users WHERE id > ?1 ORDER BY id ASC LIMIT ?2")
                .map_err(map_sqlite_error)?;
            let rows = statement
                .query_map((after_id.unwrap_or(0), limit as u64), |row| {
                    Ok(DbUser {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                    })
                })
                .map_err(map_sqlite_error)?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(map_sqlite_error)
        })
        .await
        .map_err(|_| DatabaseError::Internal)?
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

    async fn list_users(
        &self,
        limit: usize,
        after_id: Option<u64>,
    ) -> Result<Vec<DbUser>, DatabaseError> {
        let rows = self.rows.lock().map_err(|_| DatabaseError::Internal)?;
        let start = after_id.map_or(Bound::Unbounded, Bound::Excluded);
        Ok(rows
            .range((start, Bound::Unbounded))
            .take(limit)
            .map(|(_, row)| row.clone())
            .collect())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub gateway: Arc<dyn WebGateway>,
    pub authenticator: Arc<dyn Authenticator>,
    pub readiness: Arc<dyn ReadinessProbe>,
    pub lifecycle: Arc<LifecycleState>,
    pub rate_limiter: FixedWindowStore,
    pub metrics: Arc<Metrics>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        gateway: Arc<dyn WebGateway>,
        authenticator: Arc<dyn Authenticator>,
    ) -> Result<Self, ConfigError> {
        Self::with_readiness(config, gateway, authenticator, Arc::new(DemoReadiness))
    }

    pub fn with_readiness(
        config: AppConfig,
        gateway: Arc<dyn WebGateway>,
        authenticator: Arc<dyn Authenticator>,
        readiness: Arc<dyn ReadinessProbe>,
    ) -> Result<Self, ConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            gateway,
            authenticator,
            readiness,
            lifecycle: Arc::new(LifecycleState::default()),
            rate_limiter: FixedWindowStore::default(),
            metrics: Arc::new(Metrics::default()),
        })
    }

    pub fn demo(config: AppConfig) -> Result<Self, ConfigError> {
        let repository = Arc::new(MemoryRepository::demo());
        let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
        let authenticator: Arc<dyn Authenticator> = Arc::new(DemoAuthenticator);
        Self::new(config, gateway, authenticator)
    }

    pub fn from_env(config: AppConfig) -> Result<Self, ConfigError> {
        let mode = env::var("ZAP_HOST_MODE").unwrap_or_else(|_| "production".to_string());
        match mode.as_str() {
            "demo" => Self::demo(config),
            "production" => {
                let auth_mode = env::var("ZAP_AUTH_MODE").unwrap_or_default();
                if auth_mode != "jwt" {
                    return Err(ConfigError::message(
                        "production host requires ZAP_AUTH_MODE=jwt; set ZAP_HOST_MODE=demo only for local development",
                    ));
                }
                let auth_config = auth::JwtAuthConfig::from_env()
                    .map_err(|error| ConfigError::message(&error.to_string()))?
                    .ok_or_else(|| {
                        ConfigError::message("production JWT configuration is required")
                    })?;
                let database_url = env::var("DATABASE_URL")
                    .map_err(|_| ConfigError::message("production host requires DATABASE_URL"))?;
                let repository = Arc::new(SqliteRepository::connect(
                    &database_url,
                    config.database_pool,
                )?);
                let gateway: Arc<dyn WebGateway> =
                    Arc::new(ContractGateway::new(repository.clone()));
                let authenticator: Arc<dyn Authenticator> = Arc::new(
                    auth::JwtAuthenticator::new(auth_config)
                        .map_err(|error| ConfigError::message(&error.to_string()))?,
                );
                Self::with_readiness(
                    config,
                    gateway,
                    authenticator,
                    Arc::new(SqliteReadiness { repository }),
                )
            }
            _ => Err(ConfigError::message(
                "ZAP_HOST_MODE must be `production` or explicit local-only `demo`",
            )),
        }
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
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            drain_middleware,
        ));

    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
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
        .layer(middleware::from_fn_with_state(
            state.clone(),
            metrics_middleware,
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
            Ok(value) if !value.is_empty() && value.len() <= state.config.max_request_id_bytes => {
                value.to_string()
            }
            _ => {
                return policy_error_response(
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
    if !path.starts_with('/') || path.len() > state.config.max_path_bytes || path.contains("..") {
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

fn parse_user_page(uri: &Uri) -> Result<(usize, Option<u64>), &'static str> {
    let mut limit = DEFAULT_USER_PAGE_SIZE;
    let mut after_id = None;
    for pair in uri.query().unwrap_or_default().split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair
            .split_once('=')
            .ok_or("pagination parameters must use key=value")?;
        match key {
            "limit" => {
                limit = value
                    .parse::<usize>()
                    .map_err(|_| "limit must be an unsigned integer")?;
                if limit == 0 || limit > MAX_USER_PAGE_SIZE {
                    return Err("limit must be between 1 and 100");
                }
            }
            "cursor" => {
                let cursor = value
                    .parse::<u64>()
                    .map_err(|_| "cursor must be an unsigned integer")?;
                if cursor == 0 {
                    return Err("cursor must be greater than zero");
                }
                after_id = Some(cursor);
            }
            _ => return Err("unsupported pagination parameter"),
        }
    }
    Ok((limit, after_id))
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
    match state.authenticator.authenticate(request.headers()).await {
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
        Err(AuthFailure::Unavailable) => error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "authentication_unavailable",
            "authentication service is temporarily unavailable",
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

async fn metrics(State(state): State<AppState>) -> Response {
    match Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain; version=0.0.4")
        .body(Body::from(state.metrics.render()))
    {
        Ok(response) => response,
        Err(_) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "metrics_unavailable",
            "metrics response could not be rendered",
            "unassigned",
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

async fn ready(
    State(state): State<AppState>,
    Extension(RequestId(request_id)): Extension<RequestId>,
) -> Response {
    match state.readiness.check().await {
        Ok(()) => json_response(StatusCode::OK, json!({"status": "ready"})),
        Err(ReadinessError::Unavailable | ReadinessError::Internal) => error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "not_ready",
            "host dependencies are not ready",
            &request_id,
        ),
    }
}

async fn drain_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if state.lifecycle.is_draining() {
        return error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "draining",
            "host is draining and is not accepting new API requests",
            &request_id(&request),
        );
    }
    next.run(request).await
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
    uri: Uri,
) -> Response {
    if !identity.has_scope("users:read") {
        return error_response(
            StatusCode::FORBIDDEN,
            "forbidden",
            "required scope is missing",
            &request_id,
        );
    }
    let (limit, after_id) = match parse_user_page(&uri) {
        Ok(page) => page,
        Err(message) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "invalid_pagination",
                message,
                &request_id,
            )
        }
    };
    match state.gateway.list_users(limit, after_id).await {
        Ok(users) => {
            let next_cursor = (users.len() == limit)
                .then(|| users.last().map(|user| user.id))
                .flatten();
            json_response(
                StatusCode::OK,
                json!({
                    "count": users.len(),
                    "data": users,
                    "next_cursor": next_cursor,
                    "request_id": request_id
                }),
            )
        }
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

async fn metrics_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    state.metrics.request_started();
    let response = next.run(request).await;
    state.metrics.response_finished(response.status());
    response
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
    fn metrics_render_is_stable_and_low_cardinality() {
        let metrics = Metrics::default();
        metrics.request_started();
        metrics.response_finished(StatusCode::INTERNAL_SERVER_ERROR);
        let rendered = metrics.render();
        assert!(rendered.contains("zap_requests_total 1"));
        assert!(rendered.contains("zap_responses_5xx_total 1"));
        assert!(rendered.contains("zap_in_flight_requests 0"));
        assert!(!rendered.contains("/api"));
    }

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

    #[test]
    fn database_pool_configuration_has_bounded_defaults() {
        let config = AppConfig::default();
        assert_eq!(
            config.database_pool.max_connections,
            DEFAULT_DB_MAX_CONNECTIONS
        );
        assert_eq!(
            config.database_pool.acquire_timeout,
            Duration::from_millis(DEFAULT_DB_ACQUIRE_TIMEOUT_MS)
        );
        assert!(config.database_pool.validate().is_ok());

        let invalid_connections = DatabasePoolConfig {
            max_connections: 0,
            ..DatabasePoolConfig::default()
        };
        assert!(invalid_connections.validate().is_err());

        let invalid_query_timeout = DatabasePoolConfig {
            query_timeout: Duration::from_secs(121),
            ..DatabasePoolConfig::default()
        };
        assert!(invalid_query_timeout.validate().is_err());
    }

    #[tokio::test]
    async fn database_pool_gate_bounds_acquisition_and_closes_cleanly() {
        let config = DatabasePoolConfig {
            max_connections: 1,
            acquire_timeout: Duration::from_millis(5),
            ..DatabasePoolConfig::default()
        };
        let pool = DatabasePoolGate::new(config).expect("valid pool config");
        let permit = pool
            .acquire()
            .await
            .expect("first permit should be available");
        assert!(permit.is_acquired());
        assert_eq!(pool.available_permits(), 0);
        assert!(matches!(
            pool.acquire().await,
            Err(PoolAcquireError::Timeout)
        ));
        drop(permit);
        assert_eq!(pool.available_permits(), 1);
        pool.close();
        assert!(matches!(
            pool.acquire().await,
            Err(PoolAcquireError::Closed)
        ));
    }

    #[test]
    fn user_page_parser_rejects_unbounded_or_unknown_queries() {
        assert_eq!(
            parse_user_page(&Uri::from_static("/api/users?limit=2&cursor=9")).expect("valid page"),
            (2, Some(9))
        );
        assert!(parse_user_page(&Uri::from_static("/api/users?limit=0")).is_err());
        assert!(parse_user_page(&Uri::from_static("/api/users?limit=101")).is_err());
        assert!(parse_user_page(&Uri::from_static("/api/users?offset=1")).is_err());
    }

    #[tokio::test]
    async fn sqlite_repository_persists_and_paginates() {
        let repository = SqliteRepository::connect(":memory:", DatabasePoolConfig::default())
            .expect("sqlite repository");
        let first = repository
            .create_user(NormalizedCreateUser {
                name: "Ada".to_string(),
                email: "ada@example.com".to_string(),
            })
            .await
            .expect("first user");
        let second = repository
            .create_user(NormalizedCreateUser {
                name: "Grace".to_string(),
                email: "grace@example.com".to_string(),
            })
            .await
            .expect("second user");
        let first_page = repository.list_users(1, None).await.expect("first page");
        assert_eq!(first_page.len(), 1);
        assert_eq!(first_page[0].id, first.id);
        let second_page = repository
            .list_users(1, Some(first.id))
            .await
            .expect("second page");
        assert_eq!(second_page.len(), 1);
        assert_eq!(second_page[0].id, second.id);
        assert_eq!(
            repository
                .get_user(second.id)
                .await
                .expect("lookup")
                .unwrap()
                .email,
            "grace@example.com"
        );
    }
}
