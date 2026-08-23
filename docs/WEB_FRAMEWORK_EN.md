# Zap Web Framework Foundation

**Verified baseline:** Zap v2.2.7
**Branch:** `Framework`
**Status:** Web Foundation v0.2 — runnable contract package plus initial `zap-host` adapter prototype; production integrations remain separate

## Purpose

The Web package defines a stable boundary between a Zap application and a host HTTP implementation. It is intentionally contract-first: the Zap side normalizes a bounded request, applies route and method policy, and returns a bounded response map. It does not open a listener, own TLS, or perform blocking socket work.

The package is designed to let a host adapter reuse an established HTTP stack rather than forcing the Zap runtime to become a second network reactor. The initial Rust adapter is available under [`host/zap-host`](../host/zap-host) and uses Axum/Tower while translating only the documented DTO boundary.[1]

## Current package layout

| File | Responsibility | Status |
|---|---|---|
| `zap.toml` | Dependency-free contract package manifest | Implemented |
| `zap.lock` | Canonical empty-dependency lockfile | Implemented |
| `web_contract.zp` | Exported request, response, security-header, and router functions | Implemented |
| `main.zp` | Deterministic end-to-end contract demonstration | Implemented |
| `web_contract_test.zp` | Negative and positive contract regression suite | Implemented |
| `frontend_contract.zp` | Browser asset manifest, HTML/CSS/JS routes, and JSON API integration contract | Implemented |
| `frontend_contract_test.zp` | Asset content-type, traversal, and Node-free runtime regression suite | Implemented |
| `public/index.html` | Reference browser entrypoint | Implemented |
| `public/assets/app.css` | Reference stylesheet | Implemented |
| `public/assets/app.js` | Reference browser ES module consuming the JSON API | Implemented |
| `README.md` | Quick-start and package boundary | Implemented |

Run the package from `frameworks/web`:

```bash
zap lock
zap check
zap build
zap run main.zp
zap test .
```

The package has no external dependencies. This is deliberate: the first Web milestone must prove semantics and safety without hiding registry, network, or host-runtime behavior behind an unverified dependency.

## Request contract

The exported `normalize_request(method, path, body, request_id)` function accepts four text values and returns a map. The host adapter must construct this map only after enforcing its own transport-level limits.

| Field | Rule | Rejection behavior |
|---|---|---|
| `method` | Trimmed and upper-cased; `GET` and `POST` are currently supported | Unknown methods produce HTTP-style `405` from the contract |
| `path` | Must start with `/`, must not contain `..`, and must be at most 2,048 bytes according to the current text-length contract | Invalid path produces `400` |
| `body` | Must be at most 65,536 bytes according to the current text-length contract | Oversized body produces `400` |
| `request_id` | Must be non-empty and at most 128 bytes according to the current text-length contract | Missing or oversized ID produces `400` |
| `valid` | True only when path, body, and request ID checks pass | Router refuses invalid requests before route dispatch |

The current contract deliberately keeps query parsing, content negotiation, cookie parsing, multipart uploads, and automatic JSON decoding outside the core. Those concerns belong in an explicit adapter or package with its own limits and tests.

## Response contract

`response(status, body, headers)` returns a map with four stable fields:

```text
{
  "status": number,
  "content_type": "application/json",
  "headers": map<text, text>,
  "body": text
}
```

The starter always attaches `x-content-type-options: nosniff` and `cache-control: no-store`. These are conservative defaults for the contract demonstration, not a complete production security policy. A production adapter must add an explicit policy for TLS, CORS, CSP, HSTS, cookies, compression, access logging, and cache behavior rather than silently inheriting defaults.

## Frontend asset and JavaScript interoperability

`frontend_contract.zp` defines the Zap-owned browser boundary. `frontend_asset_manifest()` declares a `public` root and records that the browser runtime does not require Node. `web_static(asset_path, root_dir)` is a filesystem-capability-gated builtin that returns a normal response map for UTF-8 HTML, CSS, JavaScript/ES modules, JSON, SVG, or text files. It confines the root and resolved file to the Zap workspace, rejects absolute paths, traversal components, encoded traversal, backslashes, unsupported extensions, and files larger than 2 MiB. Missing or unsupported assets return a deterministic `404`; unsafe or unreadable paths fail closed.

The route matcher supports a final `*name` wildcard, so `/assets/*path` can serve nested bundle paths such as `/assets/chunks/app.js`. The wildcard is only a route parameter; the static builtin still performs canonicalization and root confinement. It does not expose a raw file handle or arbitrary SQL/process capability, and it does not stream binary images or fonts in this slice.

HTML, CSS, and JavaScript may be handwritten or produced by an optional frontend toolchain. React, Vue, Svelte, Alpine, or another JavaScript framework can compile its browser output into `public/assets/` and call a Zap JSON route such as `/api/tasks`; the deployed Zap process needs only the Zap runtime and the emitted browser files. Node is therefore an optional **build-time** tool, not a **run-time** prerequisite. Zap does not currently run an npm install, bundle JavaScript, hydrate a component tree, or provide a framework-specific adapter. The contract deliberately separates browser build choices from server execution.

## Route table

The current router is intentionally small and deterministic:

| Method | Path | Status | Body meaning |
|---|---|---:|---|
| `GET` | `/` | 200 | Greeting plus request ID |
| `GET` | `/health` | 200 | Health status |
| `POST` | `/echo` | 200 | Echoed bounded body plus request ID |
| Any supported method | Unknown path | 404 | `not_found` error |
| Unsupported method | Any valid path | 405 | `method_not_allowed` error |
| Invalid path/body/request ID | Any | 400 | `invalid_request` error |

The route function never executes a handler for an invalid request. This ordering is important: validation and capability policy happen before application dispatch. The current `zap-host` adapter attaches authentication, rate limits, tracing, and bounded extraction around this stable boundary.

## Host-adapter contract

The current `zap-host` prototype implements the following pipeline with Axum/Tower; a production deployment must complete each policy explicitly:

```text
HTTP bytes
  -> transport parser and maximum-size checks
  -> method/path/header normalization
  -> identity and capability checks
  -> bounded Zap request DTO
  -> route(request)
  -> response schema validation
  -> HTTP status/header/body encoding
  -> access log with redaction
```

The adapter owns the following responsibilities:

| Boundary | Required behavior |
|---|---|
| Listener | Bind explicitly to an approved address; never expose a development listener accidentally |
| TLS | Terminate TLS in the chosen host stack or trusted proxy and document certificate rotation |
| Headers | Normalize names, cap count and size, and redact authorization/cookie values from logs |
| Body | Enforce the 64 KiB contract limit before creating a Zap value; use streaming only through a separately specified API |
| Timeout | Apply connect, header, body, handler, and shutdown deadlines; propagate cancellation to the handler boundary |
| Identity | Convert authenticated identity into a bounded DTO; do not expose raw socket or credential handles to Zap code |
| Response | Validate status range, header names, content type, and body size before writing bytes |
| Observability | Emit request ID, route, status, duration, and outcome while excluding secrets and unbounded body content |
| Shutdown | Stop accepting new work, drain bounded in-flight work, and report forced termination explicitly |

The adapter must not pass live Rust `Rc<RefCell>` values, socket objects, OS handles, or thread-affine state into a worker thread. Use serializable DTOs or a redesigned ownership boundary when cross-thread execution is required.

## Security model

The Web starter is safe only within its declared boundary. It rejects traversal-shaped paths, unsupported methods, empty request IDs, and oversized body values. It does not itself provide authentication, authorization, TLS, CSRF protection, rate limiting, request signing, or a process sandbox.

Before enabling a real listener, the adapter must satisfy these controls:

| Threat | Minimum control | Required evidence |
|---|---|---|
| Request-body memory exhaustion | Reject before Zap value construction; cap total body bytes | Oversize-body negative test and RSS/budget evidence |
| Path traversal or ambiguous routing | Canonicalize once, reject traversal and invalid encodings, route only normalized paths | Encoded traversal corpus and route differential test |
| Header injection | Reject control characters and duplicate-sensitive headers according to adapter policy | Header fuzz corpus |
| Authentication confusion | Separate authenticated identity from user-supplied request fields | Forged-ID and missing-identity tests |
| Secret leakage | Redact authorization, cookie, and provider credentials in logs | Golden redaction fixtures |
| Slow client/server | Header/body/handler/shutdown deadlines with cancellation | Timeout and cancellation test |
| Replay or duplicate command | Idempotency policy for mutating routes | Duplicate-request test with stable outcome |
| Response splitting | Validate status, names, values, and body encoding before write | Malformed-response corpus |
| Denial by route explosion | Bounded route table and deterministic dispatch cost | Route-count and worst-case dispatch benchmark |

## Testing contract

The Web package must keep four test layers separate:

1. **Contract tests** verify normalization, route status, response schema, headers, and negative cases without network access.
2. **Adapter tests** use a fake host request/response DTO and verify that the adapter maps transport failures into typed Zap-facing failures.
3. **Integration tests** run a loopback server only after the host adapter exists; they must use bounded payloads, fixed ports or injected listeners, and explicit cleanup.
4. **Security and reliability tests** inject malformed paths, oversized headers/bodies, timeouts, cancellation, duplicate requests, log-redaction cases, and shutdown races.

The current `web_contract_test.zp` covers positive routes, 400/405 rejection, request-ID validation, method normalization, JSON content type, and no-store policy. The `api_contract_test.zp` additionally covers DTO mapping, repository success/not-found behavior, 401/403 authorization, 429 quota exhaustion, window reset, clock reversal, and invalid policy. The `frontend_contract_test.zp` covers the asset manifest, HTML/CSS/JavaScript response types, browser API wiring, and the no-Node runtime declaration. Native evaluator tests cover missing/unsupported assets, encoded traversal rejection, workspace confinement, and final-segment wildcard matching. These tests are not evidence that TLS, production concurrency, or external network behavior is complete.

## API and DTO contract

The Web API layer is an orchestration contract, not a server router. `api_contract.zp` exports `get_user_api`, `create_user_api`, and `list_users_api`. A host adapter may map routes such as `GET /users/{id}`, `POST /users`, and `GET /users` to these functions, but variable-path matching remains an adapter responsibility.

| API function | Input DTO | Success | Important failures |
|---|---|---:|---|
| `get_user_api` | `request_id`, numeric `user_id`, auth context, rate state, timestamp | 200 | 401, 403, 404, 429, 500 |
| `create_user_api` | `request_id`, body DTO `{name, email}`, auth context, rate state, timestamp | 201 | 400, 401, 403, 429, 500, 503 |
| `list_users_api` | `request_id`, auth context, rate state, timestamp | 200 | 401, 403, 429, 500, 503 |

The API returns a wrapper containing `response` and the updated `rate_state`. The response is the previously documented map with JSON body and security headers. The wrapper prevents a host adapter from accidentally discarding quota state after a successful request.

The request DTO validator accepts only text `name` and `email`, trims the name, lower-cases the email, bounds lengths, and requires a simple `@` marker. This is a deliberately small contract, not a complete email-verification policy. A real API may add a stricter schema package, but it must preserve explicit size limits and deterministic error mapping.

## Database integration boundary

`database_contract.zp` defines the repository boundary with `repository_info`, `find_user`, `insert_user`, and `list_users`. The current implementation is a deterministic fake repository so the API can be tested without credentials, network access, a database process, or mutable global state. The companion `database_adapter.zp` defines provider-neutral parameterized query descriptors for user lookup and insert plus an explicit `user_row_dto` mapping that exposes only public fields. It describes adapter intent; it does not open a connection or execute request-time queries.

| Database boundary | Contract requirement |
|---|---|
| Driver selection | The host adapter selects PostgreSQL, SQLite, MySQL, or another driver; Zap code must not assume one driver |
| Query arguments | Pass validated DTO fields as bound parameters; never construct SQL by concatenating user text; use `database_adapter.zp` query descriptors |
| Transactions | The adapter owns transaction begin/commit/rollback and exposes only typed success/failure DTOs |
| Connection pool | The adapter owns pool size, acquisition timeout, idle timeout, and shutdown |
| Failure mapping | Not-found is a domain result; connection, timeout, and pool failures map to an explicit repository-unavailable result |
| Returned row | Map only public fields through `user_response`; do not expose password hashes, tokens, internal notes, or driver handles |
| Observability | Log operation name, duration, outcome, and request ID; redact query values and secrets |

The API maps repository not-found to `404` and repository availability failures to `503`. A database adapter must add retry and idempotency rules only for operations where duplicate execution is safe. It must not retry an insert blindly.

## Authentication and authorization

`auth_contract.zp` assumes the host has already validated the raw credential. The host passes only `authenticated`, a bounded `subject`, and a bounded list of scopes to `auth_context`. The Zap contract never reads an `Authorization` header, cookie, private key, or token secret.

`authorize(context, "users:read")` or `authorize(context, "users:write")` returns a deterministic decision. Missing identity is `401`; an authenticated identity without the required scope is `403`; an invalid internal policy is `500`. The host adapter must ensure that a user-supplied request field cannot replace the authenticated subject.

The minimum production adapter policy is to validate issuer, audience, expiry, signature, token type, and key rotation in the host identity layer; bound subject and scope counts before creating a Zap value; redact raw credentials from every log path; and define how revocation and clock skew are handled. These are adapter responsibilities and are not implemented by the contract starter.

## Rate-limiting contract

`rate_limit_contract.zp` implements a deterministic fixed-window decision function. The state contains `key`, `limit`, `window_ms`, `window_start`, and `used`. `allow_request` returns `allowed`, `remaining` or `retry_after_ms`, and the next state.

| Decision | Status | Required host behavior |
|---|---:|---|
| Valid request under quota | 200 | Atomically persist returned state before accepting the request result |
| Quota exhausted | 429 | Return `retry_after_ms`; do not invoke the protected repository operation |
| Window expired | 200 | Reset usage at the supplied timestamp and persist the new window start |
| Clock reversal | 500 | Reject the decision and use a monotonic clock source in the adapter |
| Invalid key or policy | 500 | Fail closed and alert the configuration owner |

The adapter chooses the keying strategy, such as authenticated subject for user quotas or a normalized network identity for anonymous quotas. It must not trust an arbitrary client header as the identity key. Because the Zap function returns a new state rather than mutating shared state, the host must use an atomic store or a single-owner event loop; otherwise concurrent requests can oversubscribe the quota.

The fixed-window algorithm is a foundation, not a universal abuse-control solution. Production deployments may add separate burst, endpoint, organization, and global limits, but each added limiter needs its own key, clock, storage, failure, and observability contract.

## API security and reliability test matrix

| Test group | Required cases | Pass evidence |
|---|---|---|
| DTO | Missing fields, wrong types, empty/oversized name, invalid email, lower-case normalization | `api_contract_test.zp` and boundary corpus |
| Repository | Found row, not-found row, invalid ID, insert success, unavailable/timeout mapping | Fake repository contract plus adapter failure tests |
| Authorization | Unauthenticated, missing scope, valid read, valid write, forged subject mismatch | 401/403 matrix and identity-binding fixture |
| Rate limit | First request, last allowed request, 429, reset, duplicate state, clock reversal, invalid policy | State transition table and atomic-store test |
| API composition | Auth before repository, rate limit before repository, DTO before insert, request ID in every response | Call-order or fake-adapter trace |
| Reliability | Repository timeout, cancellation, pool exhaustion, retry/insert idempotency, shutdown | Fault-injection report with bounded deadlines |
| Security | Header redaction, body cap, path normalization, response validation, no raw credential in logs | Golden logs and malformed-input corpus |

The reusable contract test layer runs without a database or network. The native Zap Web slice now also has a loopback integration test for its bounded, single-threaded development server. Before production promotion, add a fake-host adapter suite and then a production-oriented loopback suite with injected listeners, bounded payloads, deterministic clocks, cancellation, and explicit cleanup.

## Extension procedure

To add a route, update the route table in `web_contract.zp`, add a positive test, add at least one negative or authorization test where applicable, and update this guide in both languages. A route change is not complete until the starter validator, `zap check`, `zap build`, `zap run`, and `zap test` pass.

A future adapter package should be introduced only after its capability list, DTO schema, error taxonomy, timeout policy, body limits, cancellation behavior, log-redaction policy, and shutdown contract are documented. The adapter should depend on the Web contract; the Web contract must not depend on a particular server, database, cloud, or UI stack.

## Explicit non-goals

The current Web Foundation does not claim to be a production HTTP server. The Zap-native slice provides a bounded, loopback-only, single-threaded development/reference server; it is not a concurrent production reactor. The API, database, authentication, and rate-limit files remain contract prototypes and deterministic test doubles; the repository still does not provide TLS, HTTP/2 or HTTP/3 policy, WebSocket, real database connectivity, credential verification, distributed quota storage, server-side templates, unrestricted/binary static-file streaming, background jobs, cloud deployment, or automatic code generation. Each production feature requires a separate host adapter contract and evidence.

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation — routing, extractors, responses, and Tower integration"
