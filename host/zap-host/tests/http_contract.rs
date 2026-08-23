use async_trait::async_trait;
use axum::{
    body::Body,
    http::{HeaderMap, Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

use zap_host::{
    build_router, AppConfig, AppState, AuthFailure, Authenticator, ContractGateway, DatabaseError,
    DbUser, GatewayError, Identity, NormalizedCreateUser, ReadinessError, ReadinessProbe,
    UserRepository, WebGateway,
};

#[derive(Clone)]
struct TestAuth {
    mode: AuthMode,
}

#[derive(Clone, Copy)]
enum AuthMode {
    Valid,
    Missing,
    MissingScope,
    Internal,
    Unavailable,
}

#[async_trait]
impl Authenticator for TestAuth {
    async fn authenticate(&self, _headers: &HeaderMap) -> Result<Identity, AuthFailure> {
        match self.mode {
            AuthMode::Valid => Ok(Identity::new("test-subject", ["users:read", "users:write"])),
            AuthMode::Missing => Err(AuthFailure::Missing),
            AuthMode::MissingScope => {
                Ok(Identity::new("test-subject", std::iter::empty::<String>()))
            }
            AuthMode::Internal => Err(AuthFailure::Internal),
            AuthMode::Unavailable => Err(AuthFailure::Unavailable),
        }
    }
}

struct FailingReadiness;

#[async_trait]
impl ReadinessProbe for FailingReadiness {
    async fn check(&self) -> Result<(), ReadinessError> {
        Err(ReadinessError::Unavailable)
    }
}

#[derive(Default)]
struct CountingRepository {
    calls: Mutex<u64>,
}

#[async_trait]
impl UserRepository for CountingRepository {
    async fn get_user(&self, user_id: u64) -> Result<Option<DbUser>, DatabaseError> {
        *self.calls.lock().expect("calls lock poisoned") += 1;
        Ok((user_id == 1).then_some(DbUser {
            id: 1,
            name: "Ada".to_string(),
            email: "ada@example.com".to_string(),
        }))
    }

    async fn create_user(&self, _input: NormalizedCreateUser) -> Result<DbUser, DatabaseError> {
        Err(DatabaseError::Unavailable)
    }

    async fn list_users(&self) -> Result<Vec<DbUser>, DatabaseError> {
        Ok(Vec::new())
    }
}

struct ErrorGateway {
    error: GatewayError,
}

#[async_trait]
impl WebGateway for ErrorGateway {
    async fn get_user(&self, _user_id: u64) -> Result<Option<zap_host::PublicUser>, GatewayError> {
        Err(self.error)
    }

    async fn create_user(
        &self,
        _input: NormalizedCreateUser,
    ) -> Result<zap_host::PublicUser, GatewayError> {
        Err(self.error)
    }

    async fn list_users(&self) -> Result<Vec<zap_host::PublicUser>, GatewayError> {
        Err(self.error)
    }
}

fn config() -> AppConfig {
    AppConfig {
        rate_limit: 60,
        ..AppConfig::default()
    }
}

fn request(method: Method, uri: &str, request_id: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("x-request-id", request_id)
        .body(body)
        .expect("valid test request")
}

fn request_without_content_type(
    method: Method,
    uri: &str,
    request_id: &str,
    body: Body,
) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("x-request-id", request_id)
        .body(body)
        .expect("valid test request")
}

async fn json_body(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body must be readable")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body must be JSON")
}

#[tokio::test]
async fn health_is_public_and_security_headers_are_present() {
    let state = AppState::new(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Missing,
        }),
    )
    .expect("valid state");
    let response = build_router(state)
        .oneshot(request(Method::GET, "/health", "health-1", Body::empty()))
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    assert_eq!(response.headers()["cache-control"], "no-store");
    assert_eq!(response.headers()["x-request-id"], "health-1");
    assert_eq!(json_body(response).await, json!({"status": "ok"}));
}

#[tokio::test]
async fn metrics_is_public_and_emits_bounded_counters() {
    let state = AppState::new(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Missing,
        }),
    )
    .expect("valid state");
    let response = build_router(state)
        .oneshot(request(Method::GET, "/metrics", "metrics-1", Body::empty()))
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()["content-type"],
        "text/plain; version=0.0.4"
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("metrics body must be readable")
        .to_bytes();
    let body = String::from_utf8(body.to_vec()).expect("metrics body must be UTF-8");
    assert!(body.contains("zap_requests_total 1"));
    assert!(body.contains("zap_in_flight_requests 1"));
    assert!(!body.contains("metrics-1"));
}

#[tokio::test]
async fn readiness_is_public_and_reports_dependency_state() {
    let ready_state = AppState::new(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Missing,
        }),
    )
    .expect("valid state");
    let ready = build_router(ready_state)
        .oneshot(request(Method::GET, "/ready", "ready-1", Body::empty()))
        .await
        .expect("router response");
    assert_eq!(ready.status(), StatusCode::OK);
    assert_eq!(ready.headers()["x-request-id"], "ready-1");
    assert_eq!(json_body(ready).await, json!({"status": "ready"}));

    let unavailable_state = AppState::with_readiness(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Missing,
        }),
        Arc::new(FailingReadiness),
    )
    .expect("valid state");
    let unavailable = build_router(unavailable_state)
        .oneshot(request(Method::GET, "/ready", "ready-2", Body::empty()))
        .await
        .expect("router response");
    assert_eq!(unavailable.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(unavailable.headers()["cache-control"], "no-store");
    assert_eq!(
        json_body(unavailable).await,
        json!({
            "error": "not_ready",
            "message": "host dependencies are not ready",
            "request_id": "ready-2"
        })
    );
}

#[tokio::test]
async fn graceful_drain_rejects_new_api_requests_but_keeps_liveness() {
    let state = AppState::new(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Valid,
        }),
    )
    .expect("valid state");
    let lifecycle = state.lifecycle.clone();
    let app = build_router(state);
    lifecycle.begin_draining();

    let api_response = app
        .clone()
        .oneshot(request(
            Method::GET,
            "/api/users/1",
            "drain-api",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(api_response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(api_response.headers()["x-request-id"], "drain-api");
    assert_eq!(json_body(api_response).await["error"], "draining");

    let liveness_response = app
        .oneshot(request(
            Method::GET,
            "/health",
            "drain-health",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(liveness_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn authenticated_get_maps_public_user_and_request_id() {
    let state = AppState::new(
        config(),
        Arc::new(ContractGateway::new(
            Arc::new(CountingRepository::default()),
        )),
        Arc::new(TestAuth {
            mode: AuthMode::Valid,
        }),
    )
    .expect("valid state");
    let response = build_router(state)
        .oneshot(request(Method::GET, "/api/users/1", "get-1", Body::empty()))
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-request-id"], "get-1");
    assert_eq!(
        json_body(response).await,
        json!({"data": {"id": 1, "name": "Ada", "email": "ada@example.com"}, "request_id": "get-1"})
    );
}

#[tokio::test]
async fn authentication_and_scope_failures_are_distinct() {
    for (mode, expected_status, expected_error) in [
        (
            AuthMode::Missing,
            StatusCode::UNAUTHORIZED,
            "unauthenticated",
        ),
        (AuthMode::MissingScope, StatusCode::FORBIDDEN, "forbidden"),
        (
            AuthMode::Internal,
            StatusCode::INTERNAL_SERVER_ERROR,
            "authentication_unavailable",
        ),
        (
            AuthMode::Unavailable,
            StatusCode::SERVICE_UNAVAILABLE,
            "authentication_unavailable",
        ),
    ] {
        let state = AppState::new(
            config(),
            Arc::new(ContractGateway::new(
                Arc::new(CountingRepository::default()),
            )),
            Arc::new(TestAuth { mode }),
        )
        .expect("valid state");
        let response = build_router(state)
            .oneshot(request(
                Method::GET,
                "/api/users/1",
                "auth-1",
                Body::empty(),
            ))
            .await
            .expect("router response");
        assert_eq!(response.status(), expected_status);
        let body = json_body(response).await;
        assert_eq!(body["error"], expected_error);
    }
}

#[tokio::test]
async fn invalid_routes_and_payloads_are_rejected_before_gateway() {
    let repository = Arc::new(CountingRepository::default());
    let state = AppState::new(
        AppConfig {
            max_body_bytes: 32,
            ..config()
        },
        Arc::new(ContractGateway::new(repository.clone())),
        Arc::new(TestAuth {
            mode: AuthMode::Valid,
        }),
    )
    .expect("valid state");
    let app = build_router(state);

    let invalid_request_id = app
        .clone()
        .oneshot(request(Method::GET, "/health", "", Body::empty()))
        .await
        .expect("router response");
    assert_eq!(invalid_request_id.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        invalid_request_id.headers()["x-content-type-options"],
        "nosniff"
    );
    assert_eq!(invalid_request_id.headers()["x-request-id"], "unassigned");

    let traversal = app
        .clone()
        .oneshot(request(
            Method::GET,
            "/../api/users/1",
            "bad-path",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(traversal.status(), StatusCode::BAD_REQUEST);
    assert_eq!(traversal.headers()["x-content-type-options"], "nosniff");
    assert_eq!(traversal.headers()["x-request-id"], "bad-path");

    let unsupported = app
        .clone()
        .oneshot(request(
            Method::DELETE,
            "/health",
            "bad-method",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(unsupported.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(unsupported.headers()["cache-control"], "no-store");
    assert_eq!(unsupported.headers()["x-request-id"], "bad-method");

    let unsupported_media = app
        .clone()
        .oneshot(request_without_content_type(
            Method::POST,
            "/api/users",
            "bad-content-type",
            Body::from(r#"{"name":"A","email":"a@b"}"#),
        ))
        .await
        .expect("router response");
    assert_eq!(
        unsupported_media.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE
    );

    let oversized = app
        .oneshot(request(
            Method::POST,
            "/api/users",
            "large-body",
            Body::from(r#"{"name":"Ada","email":"ada@example.com","padding":"too-large"}"#),
        ))
        .await
        .expect("router response");
    assert_eq!(oversized.status(), StatusCode::BAD_REQUEST);
    assert_eq!(*repository.calls.lock().expect("calls lock poisoned"), 0);
}

#[tokio::test]
async fn rate_limit_rejects_before_gateway_and_returns_retry_after() {
    let repository = Arc::new(CountingRepository::default());
    let state = AppState::new(
        AppConfig {
            rate_limit: 1,
            rate_window: std::time::Duration::from_secs(60),
            ..config()
        },
        Arc::new(ContractGateway::new(repository.clone())),
        Arc::new(TestAuth {
            mode: AuthMode::Valid,
        }),
    )
    .expect("valid state");
    let app = build_router(state);
    let first = app
        .clone()
        .oneshot(request(
            Method::GET,
            "/api/users/1",
            "rate-1",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(first.status(), StatusCode::OK);

    let second = app
        .oneshot(request(
            Method::GET,
            "/api/users/1",
            "rate-2",
            Body::empty(),
        ))
        .await
        .expect("router response");
    assert_eq!(second.status(), StatusCode::TOO_MANY_REQUESTS);
    assert!(second.headers().contains_key("retry-after"));
    assert_eq!(*repository.calls.lock().expect("calls lock poisoned"), 1);
}

#[tokio::test]
async fn database_failures_map_to_stable_http_statuses() {
    for (error, status) in [
        (GatewayError::Unavailable, StatusCode::SERVICE_UNAVAILABLE),
        (GatewayError::Duplicate, StatusCode::CONFLICT),
        (GatewayError::Internal, StatusCode::INTERNAL_SERVER_ERROR),
    ] {
        let state = AppState::new(
            config(),
            Arc::new(ErrorGateway { error }),
            Arc::new(TestAuth {
                mode: AuthMode::Valid,
            }),
        )
        .expect("valid state");
        let response = build_router(state)
            .oneshot(request(Method::GET, "/api/users/1", "db-1", Body::empty()))
            .await
            .expect("router response");
        assert_eq!(response.status(), status);
    }
}
