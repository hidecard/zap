# zap-host

`zap-host` is the first Axum/Tower HTTP adapter for the dependency-free Web contracts under [`../../frameworks/web`](../../frameworks/web). It owns HTTP routing, request extraction, bounded payload handling, request IDs, middleware order, timeout, graceful shutdown, and translation to replaceable authentication, gateway, repository, and rate-limit seams.

## Run

From this directory, run the quality gates first. To start the local demo with the checked-in non-secret configuration example:

```bash
cargo check --all-targets
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cp .env.example .env.local
set -a
. ./.env.local
set +a
cargo run
```

The executable binds to `127.0.0.1:3000` by default. Configuration is read from `ZAP_HOST_MODE`, `DATABASE_URL`, `ZAP_AUTH_MODE`, `ZAP_HOST_ADDR`, `ZAP_HOST_MAX_BODY_BYTES`, `ZAP_HOST_REQUEST_TIMEOUT_MS`, `ZAP_HOST_SHUTDOWN_TIMEOUT_MS`, `ZAP_HOST_RATE_LIMIT`, `ZAP_HOST_RATE_WINDOW_MS`, and `ZAP_HOST_RATE_KEY`. The public `/health` route is liveness-only; `/ready` reports the configured readiness probe. `GET /api/users` accepts `limit=1..100` and an optional `cursor` and returns `next_cursor` when the page is full.

```bash
curl -i http://127.0.0.1:3000/health
curl -i -H 'x-request-id: demo-1' http://127.0.0.1:3000/api/users/1
curl -i -H 'content-type: application/json' -d '{"name":"Bob","email":"bob@example.com"}' http://127.0.0.1:3000/api/users
```

The checked-in `.env.example` is explicitly local demo mode. Production mode uses the persistent SQLite repository and JWT authenticator, requires `DATABASE_URL` and the JWT settings, and fails closed if those dependencies are missing. The current native Zap runtime is a binary crate, so this adapter does not claim direct runtime embedding; a future `ZapGateway` requires a reviewed library/embedding seam.

See [`../../docs/ZAP_HOST_EN.md`](../../docs/ZAP_HOST_EN.md) and [`../../docs/ZAP_HOST_MM.md`](../../docs/ZAP_HOST_MM.md) for the architecture, production checklist, and integration boundary. For a step-by-step first-use workflow, read [`../../docs/ZAP_HOST_QUICKSTART_EN.md`](../../docs/ZAP_HOST_QUICKSTART_EN.md) or [`../../docs/ZAP_HOST_QUICKSTART_MM.md`](../../docs/ZAP_HOST_QUICKSTART_MM.md).
