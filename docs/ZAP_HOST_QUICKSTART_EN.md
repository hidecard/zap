# zap-host Quickstart and Integration Guide

**Verified baseline:** Zap v2.2.7
**Branch:** `Framework`
**Adapter:** `host/zap-host`

This guide is the practical entry point for the first Axum/Tower host adapter. It explains how to run the dependency-free Web contract demo, how to test the HTTP boundary, how to replace the demo repository and authenticator, and what must be completed before deployment. The companion Burmese guide is [`ZAP_HOST_QUICKSTART_MM.md`](ZAP_HOST_QUICKSTART_MM.md).

> **Important:** The default executable is a contract demonstration. It contains an in-memory repository and a fixed demo authenticator. It is useful for local development and adapter tests, but it is not a production database, credential verifier, distributed rate limiter, or complete deployment platform.

## 1. What you need

Install a Rust toolchain compatible with the crate's pinned compatibility line. The current crate is tested with Rust 1.75 and uses Axum 0.7/Tower-HTTP 0.5. A later compatible stable toolchain may work, but CI should remain the source of truth for the supported version.

| Requirement | Why it is needed | Check |
|---|---|---|
| Rust/Cargo | Build and run the host adapter | `rustc --version`, `cargo --version` |
| A free local TCP port | Run the demo listener | Default is `127.0.0.1:3000` |
| The Zap repository | Access `frameworks/web` contracts and host code | `git status` from repository root |
| No database or identity provider | The demo uses deterministic local doubles | Replace them before deployment |

Clone the repository and select the Framework branch if it is not already available locally:

```bash
git clone https://github.com/hidecard/zap.git
cd zap
git switch Framework
```

If you already have the repository, use `git pull --ff-only origin Framework` and verify the branch with `git branch --show-current`.

## 2. Run the demo in five minutes

From the repository root, prepare a local environment file. The example contains only non-secret demo values. Do not put passwords, signing keys, database URLs, or bearer tokens in the example or in a committed file.

```bash
cd host/zap-host
cp .env.example .env.local
set -a
. ./.env.local
set +a
cargo run
```

The process binds to `127.0.0.1:3000` and logs that it is listening. Keep this terminal open. Use another terminal for requests:

```bash
curl -i http://127.0.0.1:3000/
curl -i http://127.0.0.1:3000/health
curl -i http://127.0.0.1:3000/ready
curl -i -H 'x-request-id: quickstart-get' \
  http://127.0.0.1:3000/api/users/1
curl -i -H 'x-request-id: quickstart-list' \
  http://127.0.0.1:3000/api/users
curl -i -H 'content-type: application/json' \
  -H 'x-request-id: quickstart-create' \
  -d '{"name":"Bob","email":"bob@example.com"}' \
  http://127.0.0.1:3000/api/users
```

The root, health, and readiness routes are public. Health is a lightweight liveness response, while readiness calls the injected dependency probe. The demo authenticator supplies a fixed identity so the user routes can be exercised locally. A successful create request returns `201 Created`; a successful read/list request returns `200 OK`. The response contains a public user DTO and the request ID. Stop the process with `Ctrl-C`; the executable listens for Ctrl-C and SIGTERM and completes Axum graceful shutdown.

To use another port or a smaller local limit, export configuration before `cargo run`:

```bash
ZAP_HOST_ADDR=127.0.0.1:3100 \
ZAP_HOST_MAX_BODY_BYTES=32768 \
ZAP_HOST_REQUEST_TIMEOUT_MS=5000 \
ZAP_HOST_SHUTDOWN_TIMEOUT_MS=30000 \
RUST_LOG=zap_host=debug,tower_http=debug \
cargo run
```

If the configured port is already in use, choose another loopback port. Do not solve a local port conflict by exposing the demo on a public interface.

## 3. Understand the first request flow

Every request is processed through bounded host policy before it reaches the gateway. The sequence matters because it keeps invalid and rejected requests away from repository side effects.

| Order | Boundary | What you should observe |
|---:|---|---|
| 1 | Request ID and path policy | A supplied ID is preserved; a missing ID is generated; traversal and oversized IDs are rejected |
| 2 | Method policy | Methods other than the supported GET/POST set return `405` |
| 3 | Body and timeout layers | Oversized bodies map to bounded client errors; a hanging request cannot wait forever |
| 4 | Fixed-window rate gate | Exhausted quota returns `429` with `Retry-After` before gateway access |
| 5 | Authentication | The demo identity is supplied locally; production credentials must be verified outside Zap |
| 6 | Scope authorization | Missing `users:read` or `users:write` permission returns `403` |
| 7 | DTO validation | JSON media type, name, email, length, trim, and normalization rules are applied |
| 8 | Repository/gateway | The typed operation returns a public DTO or a stable error |
| 9 | Response boundary | JSON, security headers, and `x-request-id` are returned without internal fields |

## 4. Exercise failure and security paths

Use these requests to confirm that the adapter does not silently accept unsafe input:

```bash
# Traversal marker: expect 400 invalid_request.
curl -i -H 'x-request-id: traversal-check' \
  http://127.0.0.1:3000/api/../users

# Unsupported method: expect 405 method_not_allowed.
curl -i -X DELETE -H 'x-request-id: method-check' \
  http://127.0.0.1:3000/health

# Unsupported media type: expect 415 unsupported_media_type.
curl -i -H 'content-type: text/plain' \
  -H 'x-request-id: media-check' \
  --data '{"name":"A","email":"a@b"}' \
  http://127.0.0.1:3000/api/users

# Invalid DTO: expect 400 with a stable validation code.
curl -i -H 'content-type: application/json' \
  -H 'x-request-id: dto-check' \
  --data '{"name":"","email":"not-an-email"}' \
  http://127.0.0.1:3000/api/users
```

The exact body code is the contract; clients should branch on the stable status and error code rather than on provider-specific text. The adapter marks authorization and cookie headers as sensitive for diagnostics and never returns raw credentials in an error body.

## 5. Run the quality gates

Run the host checks from `host/zap-host`:

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --all-targets
```

The current test suite contains unit tests for configuration and fixed-window behavior plus in-process Axum integration tests for health, request IDs, DTO mapping, authentication, scope failures, invalid routes, body/media limits, rate-limit short-circuiting, and database error mapping. The tests do not require a live database or external identity provider.

Run the Zap-side contract separately from the repository root:

```bash
cd frameworks/web
zap lock
zap check
zap build
zap run main.zp
zap test .
```

The Zap-side package remains dependency-free. The host crate is intentionally a separate Rust package; do not add Axum/Tower dependencies to `frameworks/web/zap.toml`.

## 6. Replace the demo repository

The application-facing database seam is `UserRepository`. A production implementation should own the driver, pool, parameterized statements, transaction boundaries, deadlines, cancellation, duplicate-key classification, unavailable-service classification, and graceful shutdown.

The expected integration shape is:

```rust
let repository = Arc::new(MySqlUserRepository::connect(pool));
let gateway: Arc<dyn WebGateway> = Arc::new(ContractGateway::new(repository));
let authenticator: Arc<dyn Authenticator> = Arc::new(VerifiedIdentityProvider::new(config));
let state = AppState::new(app_config, gateway, authenticator)?;
let router = build_router(state);
```

The actual production types will be application-specific. The important rule is that the repository returns typed rows, while `ContractGateway` or an equivalent mapper exposes only `PublicUser { id, name, email }`. Password material, tokens, secret columns, internal status fields, and database diagnostics must never cross the DTO boundary.

For a database outage, map the failure to `503` and log a redacted structured event with a correlation ID. Do not return SQL, connection strings, driver messages, or query parameters to the client. For a duplicate create, use the stable `409` conflict policy. Subject or tenant ownership must be enforced in the database query and authorization policy, not trusted only from a request body.

## 7. Replace the demo authenticator

The `Authenticator` trait is the host boundary for identity verification. A production implementation should validate the credential using a documented issuer, audience, algorithm, expiry, key rotation, and revocation policy. It should return a verified identity and scopes through request extensions.

The raw `Authorization` header, cookie, API key, password, or bearer token must not be passed into the Zap contract, placed in a DTO, or written to logs. Do not trust an identity forwarded by a proxy unless the proxy trust boundary and header-stripping policy are explicit. Keep authentication and authorization separate:

| Result | Meaning | HTTP status |
|---|---|---:|
| No valid identity | Authentication did not establish a principal | `401` |
| Identity exists but lacks scope/ownership | Authorization denied | `403` |
| Provider cannot be reached or policy cannot run | Dependency or policy failure | `500` or a documented fail-closed status |

The demo authenticator is intentionally permissive for local requests. It must be replaced before a public bind address is used.

## 8. Replace the local rate limiter

The current fixed-window store protects one process by synchronizing its in-memory state. It is not a distributed quota service. A multi-process or multi-instance deployment must use a shared operation that atomically checks and increments the same key.

Before deployment, decide and document the following policy:

| Decision | Required definition |
|---|---|
| Key | Verified subject, tenant, client class, and route class; never an arbitrary untrusted header |
| Store failure | Explicit fail-open or fail-closed behavior with alerting |
| Window | Fixed, sliding, or token-bucket semantics and reset behavior |
| Retry | `Retry-After` calculation and clock source |
| Scope | Different quotas for anonymous, authenticated, administrative, and expensive routes |
| Rollout | How state is shared during rolling deployment and failover |

The rate gate must remain before repository access. A local mutex prevents only local oversubscription; it does not provide cross-process atomicity.

## 9. Deployment preparation

For local development, keep `ZAP_HOST_ADDR=127.0.0.1:3000`. A public deployment must not simply change the address and call the demo complete. The repository includes reference artifacts for the host-side operational boundary: [`deploy/zap-host.service`](../deploy/zap-host.service), [`deploy/zap-host.nginx.conf`](../deploy/zap-host.nginx.conf), [`deploy/zap-host-deployment-policy.toml`](../deploy/zap-host-deployment-policy.toml), [`deploy/zap-host.env.example`](../deploy/zap-host.env.example), and [`scripts/validate_zap_host_deployment.sh`](../scripts/validate_zap_host_deployment.sh). They are templates and validation evidence, not a substitute for deployment-specific review. Before binding to `0.0.0.0`, complete the following boundary work:

| Area | Minimum production work |
|---|---|
| TLS | Terminate TLS at a controlled edge or add a reviewed host TLS configuration |
| Proxy | Define trusted proxy headers, forwarded identity handling, and header stripping |
| Identity | Replace `DemoAuthenticator` with verified credentials and key rotation |
| Database | Replace `MemoryRepository` with a real driver/pool/transaction adapter |
| Rate limit | Replace local state with a shared atomic store |
| Readiness | Add dependency-aware readiness distinct from lightweight liveness |
| Shutdown | Remove readiness before termination, drain connections, cancel downstream work, close pools, and enforce a maximum drain time |
| Observability | Redact credentials and sensitive fields; preserve request IDs and stable error categories |
| Resource policy | Set route-specific body, timeout, concurrency, connection, and decompression limits |
| Evidence | Run integration, load, failure-injection, and deployment smoke tests |

HTTP/2, HTTP/3, WebSockets, compression, CORS, multipart uploads, background jobs, and static files are not enabled by this foundation. Each one needs an explicit host policy and tests.

## 10. Troubleshooting

| Symptom | Likely cause | Action |
|---|---|---|
| `Address already in use` | Another process owns the configured port | Set `ZAP_HOST_ADDR=127.0.0.1:3100` and retry |
| Startup rejects a variable | Invalid number, zero timeout, or unsafe bound | Compare the value with `.env.example` and the configuration table |
| `401 unauthenticated` in a custom build | Authenticator did not return a verified identity | Inspect the host provider boundary without logging raw credentials |
| `403 forbidden` | Identity lacks the route scope | Check the host-issued scope policy and tenant/ownership rules |
| `415 unsupported_media_type` | POST body is not `application/json` | Send the correct content type; do not disable the check |
| `429 rate_limited` | Fixed-window quota is exhausted | Respect `Retry-After`; replace local state before scaling out |
| `503 database_unavailable` | Repository adapter cannot reach its dependency | Check pool health and redacted dependency telemetry; do not expose driver text |
| Health is green but users fail | `/health` is liveness-style only | Add and monitor a separate dependency-aware readiness endpoint |
| Tests pass but production is unsafe | Demo doubles are still active | Confirm repository, authenticator, shared limiter, TLS, and readiness replacements |

## 11. Definition of done for the next production milestone

The adapter can be considered ready for a controlled integration environment only when a real gateway seam exists, the repository and authenticator are injected rather than defaulted, the rate policy is shared and atomic, the edge policy is documented, and deployment tests prove error ordering and shutdown behavior. Passing `cargo test` alone is not evidence of database durability, credential correctness, distributed quota safety, or production capacity.

## Related documentation

- [`ZAP_HOST_EN.md`](ZAP_HOST_EN.md): architecture, middleware, lifecycle, security, and production-boundary details.
- [`ZAP_HOST_MM.md`](ZAP_HOST_MM.md): Burmese architecture and production checklist.
- [`WEB_FRAMEWORK_EN.md`](WEB_FRAMEWORK_EN.md): dependency-free Web contract and DTO/database/auth/rate-limit boundaries.
- [`../host/zap-host/README.md`](../host/zap-host/README.md): concise crate-level commands.

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation"
[2]: https://docs.rs/tower-http/latest/tower_http/ "Tower-HTTP middleware documentation"
[3]: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs "Axum graceful-shutdown example"
