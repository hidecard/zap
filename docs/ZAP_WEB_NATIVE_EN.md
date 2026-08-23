# Zap-first Web Framework Guide

**Verified baseline:** Zap v2.2.7 on the `Framework` branch.

## Purpose

Zap Web is being designed as a **Zap-native full-stack Web framework**. The goal is similar to the developer experience Django provides—one coherent project structure, routing, application modules, models, migrations, authentication, administration, testing, and deployment checks—without turning Zap into Python syntax or reproducing Django's implementation.

The guiding principle is:

> **Write less. Understand more. Deploy safely.**

The framework should provide safe defaults and integrated tooling while keeping the behavior visible in ordinary Zap modules. Convention is welcome; hidden magic is not.

## What is runnable today

The current repository contains a runnable Web project scaffold. It uses Zap source files and the native Zap project checker; it does not require a Python or JavaScript application layer.

```text
zap new shop
cd shop
zap check
zap web check
zap db check
zap db inspect --json
zap db plan
zap db migrate --dry-run
zap db migrate --check
zap db migrate
zap test tests
zap run main.zp
zap dev
```

`zap new` creates a Web-first project with `zap.toml`, `main.zp`, `routes.zp`, `models/`, `services/`, `views/`, `public/`, `migrations/`, `middleware.zp`, `admin.zp`, `server.zp`, and `tests/`. The generated `public/` directory contains a plain HTML entrypoint, CSS, and a browser ES module that consumes `/api/tasks`; it does not require Node.js to run. The generated entrypoint prints a deterministic application description, route table, model metadata, middleware order, and admin registry. The generated `server.zp` is a bounded native development server entrypoint and can be started with `zap dev`; it is not yet a complete production Web platform.

The native CLI now understands a constrained `[web]` section and an optional `[database]` section. It verifies that the declared routes, model directory, middleware, migration directory, admin registry, and server entrypoint exist, that paths are relative and safe, and that the first Web profile uses JSON-by-default serialization. Generic non-Web `zap.toml` projects remain valid. `zap web check` validates project structure, `zap db check` validates structured migration declarations and their deterministic SQL plan, and `zap dev` runs the manifest-declared `server.zp` after Web validation.

## Project and app model

A Zap Web project is the deployable site boundary. A Web app is a directory or module group that owns a cohesive feature such as accounts, catalog, billing, or devices. A project may compose several apps, but each app should expose a small explicit surface rather than registering hidden global state.

The scaffold uses a deliberately readable layout:

```text
shop/
├── zap.toml
├── main.zp
├── web.zp
├── routes.zp
├── middleware.zp
├── admin.zp
├── server.zp
├── models/
│   └── user.zp
├── services/
│   └── user_service.zp
├── migrations/
│   └── 0001_initial.zp
├── views/
├── public/
│   ├── index.html
│   └── assets/
│       ├── app.css
│       └── app.js
└── tests/
    └── web_test.zp
```

`routes.zp` owns the route catalog, `models/` owns data metadata, `services/` owns business operations, `middleware.zp` owns ordered cross-cutting policy, `migrations/` owns versioned schema intent, `admin.zp` owns explicit management registration, and `tests/` owns project tests. The generated `zap.toml` also declares `[database] driver = "sqlite"` and `url = "data/zap.sqlite3"`. This structure is a convention backed by the `[web]` manifest, the optional `[database]` validator, and the project checker.

## Runtime independence and frontend integration

An installed Zap executable is intended to be sufficient for project validation, testing, and server execution. Users should not need Python, Node.js, Rust, Java, or another language runtime on the deployment host. Rust is used to implement and distribute the native Zap executable; it is not a runtime dependency of a Zap project. Cross-platform releases must ship a pinned executable or installer for each supported operating system.

The browser boundary is deliberately ordinary. A project can serve HTML, CSS, and JavaScript from the declared `public` directory through `web_static`, and a browser application can call Zap JSON routes. React, Vue, Svelte, Alpine, or another JavaScript framework may be used as an optional build-time toolchain; the emitted files can be copied into `public/assets/`, after which the deployed process needs only Zap and those emitted files. Zap does not install npm packages, execute a JavaScript framework, or replace its compiler/bundler.

The current `web_static` builtin is constrained to UTF-8 text assets and safe allow-listed extensions, with root confinement, traversal rejection, canonicalization, and a 2 MiB file limit. The final `*name` route wildcard supports nested paths such as `/assets/chunks/app.js`; it does not provide binary image/font streaming, cache fingerprinting, server-side rendering, or a production static-file CDN.

## Current route declaration contract

The current language parser does not yet implement a first-class `route GET "/..." fn(req)` statement. The scaffold therefore uses an ordinary exported Zap function that returns a route table:

```zap
export fn routes():
    return [{"method": "GET", "path": "/", "handler": "home", "scope": ""}, {"method": "GET", "path": "/users/:id", "handler": "get_user", "scope": "users:read"}]
```

This is intentional. It makes the current project runnable and inspectable while reserving a future parser/AST change for a compatibility-reviewed RFC. A future concise route form may look like the following, but it is **design notation and must not be copied into a current project until the parser contract is implemented**:

```zap
route GET "/users/:id" handler get_user scope "users:read"
```

The long-term design should support ordered matching, typed path parameters, route names, reverse URL generation, conflict detection, method policy, and centralized `400/404/405/500` handling. The route catalog must remain inspectable through tooling so framework defaults never become invisible behavior.

## Request and response model

JSON is the default API representation for map and list values. HTML rendering should be explicit through a future standard template surface, not inferred from an ambiguous return value. The current Web contract already enforces path length, body length, request-ID bounds, method policy, traversal rejection, security headers, and stable error shapes.

A production route pipeline should follow this order:

| Order | Boundary | Responsibility |
|---:|---|---|
| 1 | Transport | Parse HTTP, enforce protocol and connection limits |
| 2 | Request policy | Normalize method/path, reject traversal, enforce request ID and body bounds |
| 3 | Correlation | Create or validate a request ID without echoing invalid untrusted values |
| 4 | Middleware | Apply security headers, trusted proxy policy, rate limit, session, and identity context |
| 5 | Router | Match an ordered route and convert path parameters |
| 6 | Authorization | Enforce scopes/permissions before application data access |
| 7 | Validation | Convert bounded JSON input to a typed DTO and return field errors |
| 8 | Service | Execute business policy and transaction boundary |
| 9 | Repository | Execute parameterized database operations through an injected adapter |
| 10 | Response | Serialize an explicit DTO and redact internal fields |

The current `frameworks/web` contract and `host/zap-host` adapter implement a safe subset of these boundaries. The Zap-first project scaffold records the same boundaries as ordinary Zap data so the project can be inspected before a full native server exists.

## Middleware design

Middleware is an ordered request/response pipeline, not a collection of decorators. Each middleware entry should identify its name, stage, order, and short-circuit behavior. A middleware may reject a request before the handler, enrich the request context, or add response headers on the way out.

The scaffold demonstrates request-ID handling, authentication placement, and security headers:

```zap
export fn middleware_stack():
    return [{"name": "request_id", "stage": "before", "order": 10}, {"name": "auth", "stage": "before_handler", "order": 40}, {"name": "security_headers", "stage": "after", "order": 90}]
```

The framework should reject duplicate names, invalid order values, impossible dependencies, and unsafe placement such as authorization after a database operation. Middleware order must be shown by `zap web check` or a future `zap routes`/`zap explain` command.

## Models, DTOs, and ORM direction

A model is the source of database schema intent. A DTO is the boundary for request and response data. These concepts must not be collapsed: accepting a model directly from an untrusted request can expose fields, bypass validation, or make a schema change an accidental API change.

The current scaffold records model metadata in ordinary Zap functions:

```zap
export fn user_model():
    return {"name": "User", "table": "users", "fields": {"id": "number primary_key", "name": "text required", "email": "email unique"}}
```

The planned ORM direction is deliberately smaller than a general-purpose dynamic ORM. It should provide typed model metadata, explicit field nullability and uniqueness, relationships, parameterized query construction, transaction handles, bounded pool acquisition, cancellation/deadlines, and stable database error classification. Query construction should be inspectable and should never concatenate untrusted values into SQL.

A production repository must be injected behind a provider-neutral interface. The deterministic `database_contract.zp` and `WebGateway` seam remain useful for contract tests, but they are not a real database driver. The native runtime now includes a SQLite-first adapter for the structured migration workflow; PostgreSQL, MySQL, and other backends still require explicit adapters with separate capability, query, transaction, and migration tests.

## Migrations

Migrations are versioned schema intent committed with the application. The scaffold begins with:

```zap
export fn migration():
    return {"id": "0001_initial", "depends_on": [], "operations": [{"kind": "create_table", "table": "users", "columns": {"id": "integer primary key", "name": "text not null", "email": "text not null unique"}}]}
```

The first native adapter is **SQLite-first**. Migration files must contain one exported, zero-argument `migration()` function whose return value is a literal map/list tree. The supported first operations are `create_table` and `add_column`; identifiers are allow-listed, column types/modifiers are bounded, and arbitrary SQL, function calls, names, and interpolation are rejected.

`zap db check` validates the migration declarations and compiles their deterministic SQL plan without opening a database. `zap db plan` reads the SQLite migration ledger when present and prints pending SQL; `zap db plan --json` emits machine-readable output. `zap db migrate --dry-run` performs the same read-only plan, while `zap db migrate` creates the SQLite database, applies pending migrations inside one transaction, enables foreign keys, and records each applied migration in the `__zap_migrations` ledger with a checksum. A previously applied migration cannot be edited silently; the command fails and requires a new migration. `ZAP_DATABASE_URL` may override the manifest URL for a controlled deployment or test environment.

The production migration workflow should still be: generate or write a migration, inspect its dependency graph and SQL plan, apply it in an isolated environment, run compatibility checks, deploy application code that supports both schema versions during rolling updates, and record the applied migration atomically. The current native slice deliberately supports only additive table/column operations; destructive operations, PostgreSQL/MySQL adapters, distributed migration locks, rollback orchestration, connection pools, and production deployment policy remain future work.

## Authentication and authorization

Authentication answers “who is this?” Authorization answers “what may this identity do?” Zap Web should keep them separate. The host or a standard identity adapter verifies a credential and passes a small verified identity object into the application. Raw bearer tokens, cookies, passwords, or private keys must not be exposed to arbitrary handlers or written to logs.

The application contract should support users, groups or roles, scopes/permissions, session or token identity, password hashing through a reviewed standard library, CSRF policy for cookie sessions, login throttling, and audit events. Authorization must run before repository access, and admin routes must require an explicit administrative permission plus a secure session policy.

The current scaffold only records a scope in a route table, and `web_serve` treats that field as metadata rather than enforcing authorization. The earlier Web contracts implement deterministic `401` and `403` decisions; a real identity backend and session store remain provider-specific work. Applications must perform explicit authorization before protected operations.

## Admin direction

Admin is an internal, model-centric management surface. It should be opt-in and explicit, never automatically expose every database column. The scaffold records a User registration with public fields and separate admin permissions:

```zap
export fn admin_registry():
    return [{"model": "User", "list": ["id", "name", "email"], "permissions": ["admin:read", "admin:write"]}]
```

A future built-in admin package should use the same model/DTO/authorization boundaries as public APIs. Secret hashes, credentials, internal flags, and audit internals must be excluded by default. The admin is not a replacement for a product front end.

## Testing model

`zap test tests` discovers nested `*_test.zp` files and resolves imports from the nearest project root containing `zap.toml`. This makes the generated project layout usable without copying shared modules into the test directory.

The test layers should grow in this order:

| Layer | Evidence |
|---|---|
| Language | Parser, type, memory, and deterministic runtime tests |
| Contract | Route catalog, DTO, auth, rate-limit, and migration metadata tests |
| Handler | Request/response tests with injected fake repositories and identities |
| Database | Adapter tests against an isolated test database and rollback fixtures |
| HTTP | Loopback end-to-end tests for headers, status, limits, and graceful shutdown |
| Security | Invalid input, credential leakage, CSRF, SSRF, traversal, timing, and permission corpus |
| Operations | Readiness, drain, restart, migration lock, log redaction, and resource-boundary tests |

Tests that use a database must use a disposable isolated database. Production credentials and production data must never be used by the test runner.

## CLI workflow

The current supported workflow is:

```bash
zap new shop
cd shop
zap check
zap web check
zap db check
zap db inspect --json
zap db plan
zap db migrate --dry-run
zap db migrate --check
zap db migrate
zap test tests
zap run main.zp
zap dev
```

`zap db inspect` is a read-only adapter/status view; it does not create the SQLite file when it is absent. `zap db migrate --check` is a deployment-friendly check: it validates the migration ledger and exits successfully only when no migration is pending. With `--json`, the check includes `ok: true` or `ok: false` for automation. `zap dev` now runs the manifest-declared `server.zp` entrypoint. The generated server reads `ZAP_WEB_PORT` and defaults to `3000`, accepts bounded HTTP/1.0 or HTTP/1.1 requests on loopback, resolves exact, `:parameter`, and final `*wildcard` route segments, passes a request map to a Zap handler, and returns a framed response with security headers. The generated Web scaffold serves `public/index.html`, `public/assets/app.css`, and `public/assets/app.js` through `web_static`; the browser module calls `/api/tasks`. For a different local port, run `ZAP_WEB_PORT=3100 zap dev`. It is intentionally single-threaded and blocking; it is a development/reference server until concurrency, cancellation, TLS/edge policy, readiness integration, and operational evidence are complete.

The next CLI additions should be implemented only when their semantics are real and testable. The roadmap is `zap routes` for a resolved route/middleware table, `zap explain route <path>` for execution tracing, `zap docs` for generated API documentation, and `zap deploy preflight` for environment and security policy checks. The current `zap db migrate` implementation is SQLite-first and additive; it must not be mistaken for a provider-neutral production migration system.

`zap run main.zp` remains a contract preview. `zap dev` is the first Zap-native HTTP execution path, while `host/zap-host` remains the operational Axum/Tower reference adapter. Neither should be described as a complete production Web platform until the production rule below is satisfied.

## Difference from Django

Zap adopts the useful full-stack workflow idea: a project contains apps, URL declarations, model metadata, migrations, auth, admin, tests, and an integrated command line. It intentionally differs in several ways.

| Concern | Django-inspired idea | Zap-native choice |
|---|---|---|
| Syntax | Python modules, classes, decorators | Plain Zap modules first; new syntax only through parser/AST RFCs |
| Defaults | Convention over configuration | Safe conventions with inspectable manifest and route metadata |
| ORM | Broad dynamic model/query API | Smaller typed boundary with explicit DTO and adapter capabilities |
| Async | Framework adapts sync/async callables | Zap must expose I/O boundaries and avoid hiding blocking behavior |
| Errors | Exceptions mapped by framework | Result/Option and centralized response mapping, with explicit error classification |
| Admin | Model-centric internal interface | Explicit registration, least-privilege fields, and permission policy |
| Deployment | External WSGI/ASGI server choices | Native Zap server target, but only after runtime and operational gates pass |

## Production rule

A Zap Web project becomes production-ready only when the native runtime, HTTP server, database adapter, identity system, rate-limit store, migrations, admin, observability, deployment, and security test evidence are all versioned and verified together. The current scaffold is the first Zap-native project layer; it is not yet that complete platform.

## References

[1]: https://docs.djangoproject.com/en/6.1/intro/tutorial01/ "Django first-app tutorial"
[2]: https://docs.djangoproject.com/en/6.1/topics/http/urls/ "Django URL dispatcher"
[3]: https://docs.djangoproject.com/en/6.1/topics/http/middleware/ "Django middleware"
[4]: https://docs.djangoproject.com/en/6.1/topics/db/models/ "Django models"
[5]: https://docs.djangoproject.com/en/6.1/topics/migrations/ "Django migrations"
[6]: https://docs.djangoproject.com/en/6.1/topics/auth/ "Django authentication"
[7]: https://docs.djangoproject.com/en/6.1/ref/contrib/admin/ "Django admin"
[8]: https://docs.djangoproject.com/en/6.1/topics/testing/overview/ "Django testing"
