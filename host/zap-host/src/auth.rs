use crate::{AuthFailure, Authenticator, Identity};
use async_trait::async_trait;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    fmt::{Display, Formatter},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, RwLock};

pub const DEFAULT_JWKS_CACHE_SECONDS: u64 = 300;
pub const DEFAULT_AUTH_CLOCK_SKEW_SECONDS: u64 = 30;
pub const DEFAULT_AUTH_MAX_TOKEN_BYTES: usize = 16 * 1024;
pub const MAX_AUTH_MAX_TOKEN_BYTES: usize = 64 * 1024;
pub const MAX_JWKS_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone)]
pub struct JwtAuthConfig {
    pub issuer: String,
    pub audience: String,
    pub jwks_url: String,
    pub algorithms: Vec<Algorithm>,
    pub clock_skew: Duration,
    pub jwks_cache_ttl: Duration,
    pub max_token_bytes: usize,
}

impl JwtAuthConfig {
    pub fn from_env() -> Result<Option<Self>, AuthConfigError> {
        let mode = env::var("ZAP_AUTH_MODE").unwrap_or_else(|_| "demo".to_string());
        if mode == "demo" {
            return Ok(None);
        }
        if mode != "jwt" {
            return Err(AuthConfigError::message(
                "ZAP_AUTH_MODE must be `jwt` or `demo`",
            ));
        }
        let required = |name: &str| {
            env::var(name).map_err(|_| AuthConfigError::message(&format!("{name} is required")))
        };
        let algorithms = parse_algorithms(
            &env::var("ZAP_AUTH_ALLOWED_ALGORITHMS").unwrap_or_else(|_| "RS256".to_string()),
        )?;
        let config = Self {
            issuer: required("ZAP_AUTH_ISSUER")?,
            audience: required("ZAP_AUTH_AUDIENCE")?,
            jwks_url: required("ZAP_AUTH_JWKS_URL")?,
            algorithms,
            clock_skew: Duration::from_secs(parse_u64(
                "ZAP_AUTH_CLOCK_SKEW_SECONDS",
                DEFAULT_AUTH_CLOCK_SKEW_SECONDS,
            )?),
            jwks_cache_ttl: Duration::from_secs(parse_u64(
                "ZAP_AUTH_JWKS_CACHE_SECONDS",
                DEFAULT_JWKS_CACHE_SECONDS,
            )?),
            max_token_bytes: parse_usize("ZAP_AUTH_MAX_TOKEN_BYTES", DEFAULT_AUTH_MAX_TOKEN_BYTES)?,
        };
        config.validate()?;
        Ok(Some(config))
    }

    pub fn validate(&self) -> Result<(), AuthConfigError> {
        if self.issuer.is_empty() || self.issuer.len() > 2048 {
            return Err(AuthConfigError::message(
                "JWT issuer must contain 1 to 2048 bytes",
            ));
        }
        if self.audience.is_empty() || self.audience.len() > 512 {
            return Err(AuthConfigError::message(
                "JWT audience must contain 1 to 512 bytes",
            ));
        }
        if !self.jwks_url.starts_with("https://") || self.jwks_url.len() > 4096 {
            return Err(AuthConfigError::message(
                "JWT JWKS URL must be an https URL no longer than 4096 bytes",
            ));
        }
        if self.algorithms.is_empty()
            || self.algorithms.iter().any(|algorithm| {
                !matches!(
                    algorithm,
                    Algorithm::RS256
                        | Algorithm::RS384
                        | Algorithm::RS512
                        | Algorithm::ES256
                        | Algorithm::ES384
                )
            })
        {
            return Err(AuthConfigError::message(
                "JWT allowed algorithms must be an explicit RS256/RS384/RS512/ES256/ES384 allowlist",
            ));
        }
        if self.clock_skew > Duration::from_secs(300) {
            return Err(AuthConfigError::message(
                "JWT clock skew must not exceed 300 seconds",
            ));
        }
        if self.jwks_cache_ttl.is_zero() || self.jwks_cache_ttl > Duration::from_secs(86_400) {
            return Err(AuthConfigError::message(
                "JWKS cache TTL must be between 1 second and 86400 seconds",
            ));
        }
        if self.max_token_bytes == 0 || self.max_token_bytes > MAX_AUTH_MAX_TOKEN_BYTES {
            return Err(AuthConfigError::message(
                "JWT max token size must be between 1 and 65536 bytes",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AuthConfigError(String);

impl AuthConfigError {
    fn message(message: &str) -> Self {
        Self(message.to_string())
    }
}

impl Display for AuthConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for AuthConfigError {}

#[derive(Debug, Clone, Deserialize)]
struct JwtClaims {
    sub: Option<String>,
    scope: Option<String>,
    scp: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct JwksDocument {
    keys: Vec<Jwk>,
}

#[derive(Clone)]
struct CachedJwks {
    expires_at: Instant,
    keys: Arc<HashMap<String, Jwk>>,
}

#[derive(Clone)]
pub struct JwtAuthenticator {
    config: JwtAuthConfig,
    client: Client,
    cache: Arc<RwLock<Option<CachedJwks>>>,
    refresh_lock: Arc<Mutex<()>>,
    last_forced_refresh: Arc<RwLock<Option<Instant>>>,
}

impl JwtAuthenticator {
    pub fn new(config: JwtAuthConfig) -> Result<Self, AuthConfigError> {
        config.validate()?;
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(2))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|error| {
                AuthConfigError::message(&format!("JWT HTTP client failed: {error}"))
            })?;
        Ok(Self {
            config,
            client,
            cache: Arc::new(RwLock::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
            last_forced_refresh: Arc::new(RwLock::new(None)),
        })
    }

    async fn fetch_jwks(&self) -> Result<Arc<HashMap<String, Jwk>>, AuthFailure> {
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.expires_at > Instant::now() {
                    return Ok(cached.keys.clone());
                }
            }
        }
        let response = self
            .client
            .get(&self.config.jwks_url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await
            .map_err(|_| AuthFailure::Unavailable)?;
        if !response.status().is_success() {
            return Err(AuthFailure::Unavailable);
        }
        let body = response
            .bytes()
            .await
            .map_err(|_| AuthFailure::Unavailable)?;
        if body.len() > MAX_JWKS_BYTES {
            return Err(AuthFailure::Internal);
        }
        let document =
            serde_json::from_slice::<JwksDocument>(&body).map_err(|_| AuthFailure::Internal)?;
        if document.keys.is_empty() || document.keys.len() > 128 {
            return Err(AuthFailure::Internal);
        }
        let mut keys = HashMap::new();
        for key in document.keys {
            let Some(kid) = key.common.key_id.clone() else {
                continue;
            };
            if kid.len() <= 256 {
                keys.insert(kid, key);
            }
        }
        if keys.is_empty() {
            return Err(AuthFailure::Internal);
        }
        Ok(Arc::new(keys))
    }

    async fn jwks_for_kid(&self, kid: &str) -> Result<Arc<HashMap<String, Jwk>>, AuthFailure> {
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.expires_at > Instant::now() && cached.keys.contains_key(kid) {
                    return Ok(cached.keys.clone());
                }
            }
        }
        let _refresh_guard = self.refresh_lock.lock().await;
        {
            let cache = self.cache.read().await;
            if let Some(cached) = cache.as_ref() {
                if cached.expires_at > Instant::now() && cached.keys.contains_key(kid) {
                    return Ok(cached.keys.clone());
                }
            }
        }
        {
            let last_refresh = self.last_forced_refresh.read().await;
            if let Some(last_refresh) = *last_refresh {
                if last_refresh.elapsed() < Duration::from_secs(5) {
                    return Err(AuthFailure::Unavailable);
                }
            }
        }
        let keys = self.fetch_jwks().await?;
        let mut cache = self.cache.write().await;
        *cache = Some(CachedJwks {
            expires_at: Instant::now() + self.config.jwks_cache_ttl,
            keys: keys.clone(),
        });
        drop(cache);
        let mut last_refresh = self.last_forced_refresh.write().await;
        *last_refresh = Some(Instant::now());
        Ok(keys)
    }

    async fn decode_token(&self, token: &str) -> Result<Identity, AuthFailure> {
        let header = decode_header(token).map_err(|_| AuthFailure::Invalid)?;
        if !self.config.algorithms.contains(&header.alg) {
            return Err(AuthFailure::Invalid);
        }
        let kid = header.kid.ok_or(AuthFailure::Invalid)?;
        let keys = self.jwks_for_kid(&kid).await?;
        let jwk = keys.get(&kid).ok_or(AuthFailure::Invalid)?;
        if jwk
            .common
            .algorithm
            .as_ref()
            .is_some_and(|algorithm| *algorithm != header.alg)
        {
            return Err(AuthFailure::Invalid);
        }
        let decoding_key = DecodingKey::from_jwk(jwk).map_err(|_| AuthFailure::Invalid)?;
        let mut validation = Validation::new(header.alg);
        validation.set_issuer(std::slice::from_ref(&self.config.issuer));
        validation.set_audience(std::slice::from_ref(&self.config.audience));
        validation.leeway = self.config.clock_skew.as_secs();
        validation.validate_exp = true;
        validation.validate_nbf = true;
        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
            .map_err(|_| AuthFailure::Invalid)?;
        let claims = token_data.claims;
        let subject = claims.sub.ok_or(AuthFailure::Invalid)?;
        if subject.is_empty() || subject.len() > 512 {
            return Err(AuthFailure::Invalid);
        }
        let mut scopes = claims
            .scope
            .unwrap_or_default()
            .split_whitespace()
            .filter(|scope| !scope.is_empty() && scope.len() <= 128)
            .map(str::to_string)
            .collect::<Vec<_>>();
        if let Some(scp) = claims.scp {
            scopes.extend(
                scp.into_iter()
                    .filter(|scope| !scope.is_empty() && scope.len() <= 128),
            );
        }
        scopes.sort();
        scopes.dedup();
        Ok(Identity::new(subject, scopes))
    }
}

#[async_trait]
impl Authenticator for JwtAuthenticator {
    async fn authenticate(&self, headers: &axum::http::HeaderMap) -> Result<Identity, AuthFailure> {
        let value = headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AuthFailure::Missing)?;
        let value = value.to_str().map_err(|_| AuthFailure::Invalid)?;
        let token = value
            .strip_prefix("Bearer ")
            .ok_or(AuthFailure::Invalid)?
            .trim()
            .to_owned();
        if token.is_empty() || token.len() > self.config.max_token_bytes {
            return Err(AuthFailure::Invalid);
        }
        self.decode_token(&token).await
    }
}

fn parse_algorithms(value: &str) -> Result<Vec<Algorithm>, AuthConfigError> {
    let mut algorithms = Vec::new();
    for item in value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        let algorithm = match item {
            "RS256" => Algorithm::RS256,
            "RS384" => Algorithm::RS384,
            "RS512" => Algorithm::RS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            _ => return Err(AuthConfigError::message("unsupported JWT algorithm")),
        };
        if !algorithms.contains(&algorithm) {
            algorithms.push(algorithm);
        }
    }
    Ok(algorithms)
}

fn parse_u64(name: &str, default: u64) -> Result<u64, AuthConfigError> {
    env::var(name)
        .map(|value| {
            value.parse().map_err(|_| {
                AuthConfigError::message(&format!("{name} must be an unsigned integer"))
            })
        })
        .unwrap_or(Ok(default))
}

fn parse_usize(name: &str, default: usize) -> Result<usize, AuthConfigError> {
    env::var(name)
        .map(|value| {
            value.parse().map_err(|_| {
                AuthConfigError::message(&format!("{name} must be an unsigned integer"))
            })
        })
        .unwrap_or(Ok(default))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_http_jwks_and_unbounded_limits() {
        let config = JwtAuthConfig {
            issuer: "https://issuer.example".into(),
            audience: "zap-api".into(),
            jwks_url: "http://issuer.example/jwks".into(),
            algorithms: vec![Algorithm::RS256],
            clock_skew: Duration::from_secs(30),
            jwks_cache_ttl: Duration::from_secs(300),
            max_token_bytes: DEFAULT_AUTH_MAX_TOKEN_BYTES,
        };
        assert!(config.validate().is_err());
        let mut bounded = config;
        bounded.jwks_url = "https://issuer.example/jwks".into();
        bounded.max_token_bytes = MAX_AUTH_MAX_TOKEN_BYTES + 1;
        assert!(bounded.validate().is_err());
    }

    #[tokio::test]
    async fn missing_or_malformed_bearer_tokens_fail_closed() {
        let config = JwtAuthConfig {
            issuer: "https://issuer.example".into(),
            audience: "zap-api".into(),
            jwks_url: "https://issuer.example/.well-known/jwks.json".into(),
            algorithms: vec![Algorithm::RS256],
            clock_skew: Duration::from_secs(30),
            jwks_cache_ttl: Duration::from_secs(300),
            max_token_bytes: DEFAULT_AUTH_MAX_TOKEN_BYTES,
        };
        let authenticator = JwtAuthenticator::new(config).expect("valid JWT config");
        let request = axum::http::Request::builder()
            .uri("/api/users")
            .body(axum::body::Body::empty())
            .expect("request should build");
        assert!(matches!(
            authenticator.authenticate(request.headers()).await,
            Err(AuthFailure::Missing)
        ));
        let request = axum::http::Request::builder()
            .uri("/api/users")
            .header(axum::http::header::AUTHORIZATION, "Bearer not-a-jwt")
            .body(axum::body::Body::empty())
            .expect("request should build");
        assert!(matches!(
            authenticator.authenticate(request.headers()).await,
            Err(AuthFailure::Invalid)
        ));
    }
}
