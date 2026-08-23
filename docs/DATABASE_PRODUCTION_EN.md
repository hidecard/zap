# Zap Production Database Operations

Zap currently owns a deterministic, SQLite-first migration engine. A production application must keep schema migration, request-time database access, and connection-pool lifecycle as separate responsibilities. The migration command is a release operation; the repository owns the pool; the Web process owns readiness and graceful shutdown.

## What the native adapter owns today

The native adapter reads the `[database]` manifest section, supports `driver = "sqlite"`, resolves a relative project database path or a deployment-provided `ZAP_DATABASE_URL`, discovers declarative `.zp` migrations, validates dependency order, computes checksums, and applies pending operations in one SQLite transaction. The `__zap_migrations` ledger records the migration ID, application time, and checksum. If an applied migration disappears or its contents change, the adapter fails closed and requires a new migration.

| Concern | Native Zap behavior | Production implication |
|---|---|---|
| Migration format | Declarative `.zp` files with bounded operations | Review and test migration files as release artifacts |
| Ordering | Explicit `depends_on` plus deterministic ordering | Cycles and missing dependencies fail before apply |
| Drift | SHA-256 checksum ledger | Never edit an applied migration; create a forward migration |
| Apply | One transactional SQLite apply | Backup before release and verify post-apply state |
| Rollback | No automatic down migration | Use backup/restore or a tested forward compatibility migration |
| External providers | Not implemented by native adapter | Add a provider-specific host repository and migration tool |

## Release migration procedure

The release pipeline should build an immutable application artifact first. It should then validate the project and produce a read-only migration plan before touching the database.

```bash
zap build --locked /srv/zap/app
zap web check /srv/zap/app
zap db check /srv/zap/app
zap db inspect --json /srv/zap/app
zap db plan --json /srv/zap/app
zap db migrate --dry-run /srv/zap/app
```

For the current SQLite adapter, stop the Web process, take a verified copy of the database, and invoke the checked-in `zap-web-migrate.service`. That unit serializes migration work with `flock`, applies the transaction, and runs `zap db migrate --check` after completion. Keep the migration unit manual rather than attaching it to every worker boot.

A migration is safe only when the new application can work with both the old and new schema during the rollout window. Prefer expand-and-contract changes: add nullable/new columns or tables first, deploy code that can read both shapes, backfill in a bounded job, and remove old columns only after all old readers are gone. The current native migration format is intentionally smaller than a general SQL migration system; do not encode destructive or provider-specific assumptions in it.

## Failure and recovery

If `zap db migrate --check` reports pending migrations, the release is not ready. If the apply fails, inspect the journal and preserve the database before retrying. If the migration ledger reports checksum drift, restore the original migration file only if the repository history proves that it was the intended file; otherwise write a new migration. If a destructive operation has already been applied, use the tested backup/restore procedure or a forward corrective migration. Do not treat a successful systemd restart as schema recovery evidence.

For SQLite, use one writer at a time and keep transactions short. The native adapter already sets a bounded busy timeout and foreign-key enforcement for opened connections. A large connection pool does not create parallel SQLite write capacity; it can increase lock contention and file-descriptor pressure. Read/write behavior, backup strategy, WAL policy, and filesystem durability must be reviewed for the actual host filesystem.

## Connection-pool ownership

The Web framework must not put database credentials, SQL, or provider-specific pool objects into Zap source. The production `UserRepository` owns the provider pool and exposes typed operations to the `WebGateway`. The host adapter now exposes the following configuration policy through `AppConfig.database_pool`:

| Setting | Environment variable | Default | Bound |
|---|---|---:|---:|
| Maximum connections | `ZAP_DB_MAX_CONNECTIONS` | 16 | 1–256 |
| Acquisition timeout | `ZAP_DB_ACQUIRE_TIMEOUT_MS` | 1000 ms | 1 ms–30 s |
| Query/statement timeout | `ZAP_DB_QUERY_TIMEOUT_MS` | 5000 ms | 1 ms–120 s |

These fields define a contract; they do not turn `DemoRepository` into a real pool. A real repository must use the configured values when acquiring a connection and executing a statement, and must fail with a typed unavailable/internal error when acquisition or query limits are exceeded.

Pool sizing is a deployment calculation, not a language constant. Across all application instances, keep the sum of pool maxima below the database server's connection budget after reserving connections for administration, migrations, monitoring, and failover; the provider's connection settings and limits remain authoritative [1]. Start conservatively, measure pool wait time and database saturation, then tune. A pool is not a queue without limits: acquisition must time out, query execution must time out, and shutdown must stop new acquisition before waiting for in-flight work to finish.

## Repository transaction contract

A production repository should implement the following boundary:

```text
request
  -> authenticate and authorize
  -> acquire pool connection with deadline
  -> begin transaction only when multiple statements must be atomic
  -> use parameterized query and subject/tenant predicate
  -> commit or rollback
  -> release connection
  -> map provider error to typed DatabaseError
```

The repository must never return raw driver errors, SQL text, credentials, password material, or internal columns to the JSON DTO layer. Duplicate-key errors map to a stable conflict result, unavailable pool/database errors map to a dependency-unavailable result, and cancellation must release the connection. Readiness should perform a bounded database ping or equivalent health check, while liveness should remain independent of the database.

For external PostgreSQL/MySQL providers, use the provider's reviewed async pool implementation in the host adapter. Keep migrations in the provider's migration tool or a separately reviewed Zap adapter; do not attempt to make the SQLite `.zp` migration engine silently interpret another provider's SQL dialect. If a deployment has multiple Web instances, use a database advisory migration lock or an orchestrator lock instead of the local `/run/zap` lock.

## Shutdown and observability

On SIGTERM, mark the process draining so readiness fails, stop accepting new application work, cancel or time out outstanding queries, close the pool within the service drain budget, and exit with a deterministic status. Record pool acquisition latency, active/idle counts, timeout counts, migration ID/checksum, and database error category without recording URLs containing credentials or raw SQL values.

## References

[1]: https://www.postgresql.org/docs/current/runtime-config-connection.html PostgreSQL documentation — connection and authentication configuration.
[2]: https://documentation.suse.com/smart/security/html/systemd-securing/index.html SUSE Linux Enterprise Server — Securing systemd Services.
[3]: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/ NGINX — Reverse Proxy Administration Guide.
