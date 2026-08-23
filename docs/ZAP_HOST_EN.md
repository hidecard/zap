# zap-host Axum/Tower Adapter

**Verified baseline:** Zap v2.2.7
**Branch:** `Framework`
**Status:** adapter foundation v0.1, implemented under `host/zap-host` on the `Framework` branch.

This guide defines the first host-side HTTP adapter for the dependency-free Web contracts in [`frameworks/web`](../frameworks/web). It uses Axum for HTTP routing and Tower/Tower-HTTP for bounded request handling, timeout control, sensitive-header marking, and request tracing. The adapter is intentionally separate from the Zap language core and does not claim that the native runtime is already an embeddable Rust library.

> **Boundary:** The Zap Web modules define deterministic request, DTO, authorization, rate-limit, repository, and response contracts. `zap-host` owns sockets, HTTP extraction, middleware ordering, process lifecycle, and translation between HTTP values and those contracts.

## Scope of this foundation

The first crate is a runnable adapter skeleton rather than a complete production deployment. It provides a real Axum router and a Tokio TCP lifecycle, but its default gateway is an in-memory demonstration repository and its default authenticator accepts a fixed demo identity. These defaults make the contract executable and testable; they must be replaced before deployment.

| Area | Implemented in `zap-host` | Production replacement required |
|---|---|---|
| HTTP listener | Tokio `TcpListener` and `axum::serve` | TLS termination, proxy policy, deployment health, and socket hardening |
| Routing | `/`, `/health`, `/api/users`, `/api/users/:id` | Versioning, API compatibility policy, and the complete application route set |
| Request bounds | 2,048-byte path, 65,536-byte body, 128-byte request ID | Edge/proxy limits, multipart policy, decompression limits, and per-route budgets |
| Timeout | Tower `TimeoutLayer`, default 10 seconds | Operation-specific deadlines, cancellation propagation, and downstream timeout budgets |
| Authentication | `Authenticator` trait and identity extension | JWT/OIDC/API-key verification, key rotation, issuer/audience checks, and revocation policy |
| Authorization | `users:read` and `users:write` scope checks | Resource ownership, tenant isolation, policy evaluation, and audit events |
| Database | `UserRepository` trait plus memory demo | Parameterized SQL/driver adapter, pool/transaction policy, migrations, and retry taxonomy |
| Rate limiting | Atomic in-process fixed-window state | Shared atomic store, trusted key selection, proxy-aware identity, and fail-open/closed policy |
| Shutdown | Ctrl-C/SIGTERM graceful shutdown | Deployment drain coordination, readiness transitions, and bounded shutdown timeout |

## Request pipeline

The adapter applies boundaries before the domain gateway is called. The order is significant because it prevents oversized, unsupported, unauthenticated, or rate-exhausted requests from reaching repository code.

| Stage | Responsibility | Failure examples |
|---|---|---|
| 1. Request policy | Validate path shape, traversal markers, method, and request ID; attach or generate a correlation ID | `400 invalid_request`, `405 method_not_allowed` |
| 2. Tower bounds | Enforce body size and whole-request timeout | `400 invalid_request`, `408 request_timeout` |
| 3. Rate limiter | Atomically consume the configured key/window before the gateway | `429 rate_limited`, `500 rate_limit_unavailable` |
| 4. Authenticator | Verify external credentials outside Zap and return a typed identity | `401 unauthenticated`, `500 authentication_unavailable` |
| 5. Route handler | Check required scope, parse path/body, and invoke the gateway | `400`, `403`, `404`, `409`, `503` |
| 6. DTO mapper | Normalize input and expose only public output fields | `400 invalid_name`, `400 invalid_email` |
| 7. Response boundary | Serialize JSON, set security headers, and propagate `x-request-id` | Redacted stable response body |

The last-added middleware is the outermost Tower layer. The code keeps rate limiting ahead of authentication and both ahead of the gateway. This is a deliberate policy choice, not an incidental implementation detail. If a deployment requires a different policy, it must add a test that proves the new side-effect and failure ordering.

## Crate layout

| Path | Responsibility |
|---|---|
| [`host/zap-host/Cargo.toml`](../host/zap-host/Cargo.toml) | Rust package and deliberately narrow Axum/Tower feature selection |
| [`host/zap-host/src/lib.rs`](../host/zap-host/src/lib.rs) | Configuration, state, middleware, routes, DTOs, gateway traits, and demo repository |
| [`host/zap-host/src/main.rs`](../host/zap-host/src/main.rs) | Environment loading, logging, listener binding, and graceful shutdown |
| [`host/zap-host/tests/http_contract.rs`](../host/zap-host/tests/http_contract.rs) | Loopback-free Axum service tests for security and reliability behavior |
| [`host/zap-host/Cargo.lock`](../host/zap-host/Cargo.lock) | Reproducible dependency resolution for the standalone adapter crate |

## Integration seams

The adapter is designed around replaceable `WebGateway`, `UserRepository`, `Authenticator`, and `ReadinessProbe` seams. `WebGateway` is the application-facing seam used by Axum handlers. `ContractGateway<R>` maps a `UserRepository` row into the public DTO and translates database failures into stable gateway errors. `Authenticator` is intentionally given the HTTP request but returns only a verified `Identity`; raw credentials must not enter the Zap contract or application logs. `ReadinessProbe` supplies dependency-aware readiness without forcing a particular database or provider into the host crate.

A real application should provide an `AppState` similar to the following shape:

```rust
let repository = Arc::new(MySqlUserRepository::connect(pool));
let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
let authenticator: Arc<dyn Authenticator> = Arc::new(OidcAuthenticator::new(issuer_config));
let state = AppState::new(config, gateway, authenticator)?;
let app = build_router(state);
```

The sample `MemoryRepository` exists only to make the adapter runnable. It must not be interpreted as a database integration, durability guarantee, concurrency design, or migration strategy. The current native runtime is a binary crate, so a future `ZapGateway` should be introduced only after a reviewed library/embedding seam exists. Calling the CLI as an unbounded subprocess for every request is not an acceptable production bridge.

## Configuration

The executable reads the following environment variables. Invalid numeric values or unsafe bounds fail during startup instead of being silently accepted.

| Variable | Default | Rule |
|---|---:|---|
| `ZAP_HOST_ADDR` | `127.0.0.1:3000` | Must parse as a socket address |
| `ZAP_HOST_MAX_BODY_BYTES` | `65536` | Must be between 1 and 65,536 |
| `ZAP_HOST_REQUEST_TIMEOUT_MS` | `10000` | Must be greater than zero |
| `ZAP_HOST_SHUTDOWN_TIMEOUT_MS` | `30000` | Maximum post-signal drain duration; must be greater than zero |
| `ZAP_HOST_RATE_LIMIT` | `60` | Requests per fixed window; must be greater than zero |
| `ZAP_HOST_RATE_WINDOW_MS` | `60000` | Fixed-window duration; must be greater than zero |
| `ZAP_HOST_RATE_KEY` | `demo-host` | Must contain 1–256 bytes; replace with a trusted user/tenant key policy |

The default bind address is loopback to avoid accidentally exposing the demo adapter. A deployment that binds publicly must explicitly configure network policy, TLS termination, proxy trust, access logging, and readiness behavior.

## HTTP contract

| Method | Path | Required scope | Success | Notes |
|---|---|---|---:|---|
| `GET` | `/` | None | `200` | Small root response and correlation ID |
| `GET` | `/health` | None | `200` | Liveness-style response only; it does not prove database readiness |
| `GET` | `/ready` | None | `200`/`503` | Readiness probe result; public and dependency-aware |
| `GET` | `/api/users` | `users:read` | `200` | Public DTO list |
| `GET` | `/api/users/:id` | `users:read` | `200` | `404` for an absent user |
| `POST` | `/api/users` | `users:write` | `201` | JSON body with string `name` and `email` |

All JSON responses include `x-content-type-options: nosniff`, `cache-control: no-store`, and the validated or generated `x-request-id`. Authorization and cookie headers are marked sensitive for Tower diagnostics. Error responses contain stable error codes and do not expose driver messages, SQL, credentials, tokens, or internal row fields.

The `GET /health` route is public and intentionally lightweight. `GET /ready` is public and calls the injected readiness probe; it returns `503` when required dependencies are not ready. Readiness must remain distinct from liveness and should be wired to real dependency checks before deployment.

## Database adapter checklist

A real `UserRepository` implementation must use parameterized statements and typed input binding. It must own connection-pool sizing, acquisition timeout, query timeout, transaction boundaries, cancellation behavior, duplicate-key classification, unavailable-service classification, and graceful pool shutdown. It must map an unavailable dependency to `503` and a duplicate create to `409` without returning provider-specific text to clients.

The repository must return only the fields required by `PublicUser`. Secret columns, password material, access tokens, internal status fields, and diagnostic metadata must never be serialized by the DTO mapper. Subject or tenant binding must be enforced in repository queries rather than trusted only from a request body.

## Authentication and authorization checklist

The real authenticator must validate the credential at the host boundary using an approved issuer, audience, algorithm, key-rotation, expiry, and revocation policy. The handler should receive a verified identity and scopes through request extensions. It must not parse a bearer token in the Zap module, log the raw `Authorization` header, or use an untrusted forwarded identity without an explicit proxy-trust configuration.

Authorization is a separate decision from authentication. The current example checks scopes, but production code must also define resource ownership, tenant boundaries, administrative exceptions, audit events, and a default-deny behavior. `401` means no valid identity was established; `403` means an identity exists but is not permitted.

## Rate-limit checklist

The sample fixed-window store locks its state update so a single process cannot oversubscribe the counter. A production deployment with more than one process must use a shared store operation that atomically checks and increments the same key. The key must be derived from a trusted policy, such as a verified subject plus tenant and route class; it must not blindly trust an arbitrary client header.

The production policy must document whether store failures fail open or fail closed, how `Retry-After` is calculated, how monotonicity is guaranteed, how limits differ by route/identity, and how a rolling deploy shares state. A local mutex is not a distributed quota solution.

## Lifecycle and shutdown

`main.rs` binds a Tokio TCP listener and uses Axum's graceful-shutdown future for Ctrl-C and SIGTERM. The lifecycle state marks the host draining, `/ready` can be connected to deployment readiness, and the configured shutdown timeout bounds the post-signal drain. The Tower request timeout ensures individual requests do not wait forever. A production supervisor still needs readiness removal before termination, connection-drain limits, downstream cancellation, bounded database-pool close, and an explicit exit status policy.

TLS is deliberately absent from this first crate. It should normally terminate at a controlled edge or be added through a reviewed host deployment configuration. HTTP/2, HTTP/3, WebSockets, compression, CORS, proxy headers, and tracing exporters each require an explicit policy and regression tests rather than being enabled by default.

For a step-by-step local workflow, see [`ZAP_HOST_QUICKSTART_EN.md`](ZAP_HOST_QUICKSTART_EN.md) or [`ZAP_HOST_QUICKSTART_MM.md`](ZAP_HOST_QUICKSTART_MM.md).

## Development and validation

From the repository root:

```bash
cd host/zap-host
cargo check --all-targets
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo run
```

The integration suite verifies public health, request-ID propagation, DTO mapping, authentication and scope failures, path/method/body rejection, rate-limit short-circuiting, and database error mapping. It uses Axum's in-process service interface and does not require a live socket, database, credential provider, or external service.

## Remaining milestone before production

This crate is the first adapter prototype. Before production, add a reviewed runtime bridge, real authentication provider, real database adapter, shared rate-limit store, TLS/proxy policy, observability/redaction review, readiness checks, integration tests with injected dependencies, and deployment-specific load/chaos evidence. Do not promote the demo memory repository or fixed authenticator to a production default.

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation"
[2]: https://docs.rs/tower-http/latest/tower_http/ "Tower-HTTP middleware documentation"
[3]: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs "Axum graceful-shutdown example"
