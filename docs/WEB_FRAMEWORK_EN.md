# Zap Web Framework Foundation

**Verified baseline:** Zap v2.2.3
**Branch:** `Framework`
**Status:** Web Foundation v0.2 — runnable contract package; production HTTP adapter remains separate

## Purpose

The Web package defines a stable boundary between a Zap application and a host HTTP implementation. It is intentionally contract-first: the Zap side normalizes a bounded request, applies route and method policy, and returns a bounded response map. It does not open a listener, own TLS, or perform blocking socket work.

The package is designed to let a future host adapter reuse an established HTTP stack rather than forcing the Zap runtime to become a second network reactor. A Rust adapter may use an existing routing and middleware ecosystem such as Axum/Tower while translating only the documented DTO boundary.[1]

## Current package layout

| File | Responsibility | Status |
|---|---|---|
| `zap.toml` | Dependency-free contract package manifest | Implemented |
| `zap.lock` | Canonical empty-dependency lockfile | Implemented |
| `web_contract.zp` | Exported request, response, security-header, and router functions | Implemented |
| `main.zp` | Deterministic end-to-end contract demonstration | Implemented |
| `web_contract_test.zp` | Negative and positive contract regression suite | Implemented |
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

The route function never executes a handler for an invalid request. This ordering is important: validation and capability policy happen before application dispatch, so a future adapter can attach authentication, rate limits, and tracing around a stable boundary.

## Host-adapter contract

A production Web adapter should implement the following pipeline:

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

The current `web_contract_test.zp` covers positive routes, 400/405 rejection, request-ID validation, method normalization, JSON content type, and no-store policy. It is not evidence that TLS, authentication, production concurrency, or external network behavior is complete.

## Extension procedure

To add a route, update the route table in `web_contract.zp`, add a positive test, add at least one negative or authorization test where applicable, and update this guide in both languages. A route change is not complete until the starter validator, `zap check`, `zap build`, `zap run`, and `zap test` pass.

A future adapter package should be introduced only after its capability list, DTO schema, error taxonomy, timeout policy, body limits, cancellation behavior, log-redaction policy, and shutdown contract are documented. The adapter should depend on the Web contract; the Web contract must not depend on a particular server, database, cloud, or UI stack.

## Explicit non-goals

The current Web Foundation does not claim to be a production HTTP server. It does not implement a multi-request reactor, TLS, HTTP/2 or HTTP/3 policy, WebSocket, database access, templates, static-file serving, authentication, authorization, rate limiting, background jobs, cloud deployment, or automatic code generation. These features require separate contracts and evidence.

## References

[1]: https://docs.rs/axum/latest/axum/ "Axum documentation — routing, extractors, responses, and Tower integration"
