# Zap Database Provider Migration Strategy

**Status:** Deferred until self-hosting certified (M3)
**Priority:** Low — SQLite-first remains primary until M3 complete

---

## 1. Migration Strategy Overview

The roadmap prioritizes SQLite-first for Zap's database layer. PostgreSQL, MySQL, and MariaDB are planned as post-SQLite expansion. This document outlines the strategy for adding these providers without breaking existing SQLite deployments.

### Design Principles

1. **SQLite remains the default.** No migration is required for existing projects.
2. **Provider-agnostic interface.** Applications should target the Zap DB interface, not a specific provider's SQL dialect.
3. **Migration contracts are schema-level.** `zap db migrate` operates on schema definitions, not provider-specific DDL.
4. **New providers are additive.** Adding PostgreSQL support does not change SQLite behavior.

---

## 2. PostgreSQL

### 2.1 Priority: High (first non-SQLite provider)

### 2.2 Rationale
- Most popular production RDBMS
- Strong Zap ecosystem alignment (JSON support, extensibility)
- Well-documented migration patterns from SQLite
- Azure, AWS RDS, GCP Cloud SQL all support it natively

### 2.3 Migration Path

```text
SQLite Project
     │
     ▼  [zap db init --provider postgres]
┌────────────────┐
│  Schema Export │  (existing schema → SQL-92 compatible DDL)
└────────┬───────┘
         │
         ▼  [zap db migrate --provider postgres]
┌────────────────┐
│  PostgreSQL    │  (equivalent schema on PostgreSQL)
│  Deployment    │
└────────────────┘
```

### 2.4 Key Technical Considerations

| Aspect | SQLite | PostgreSQL |
|---|---|---|
| Connection | File-based | TCP/IP (pool required) |
| Concurrency | Serialized | MVCC (reader-writer) |
| Identity | AUTOINCREMENT | SERIAL/BIGSERIAL/GENERATED |
| JSON | JSON1 extension | Native JSON/JSONB |
| Full-text | FTS5 | tsvector/tsquery |
| Constraints | Limited | CHECK, EXCLUDE, deferrable |
| Schema introspection | PRAGMA-based | information_schema/pg_catalog |

### 2.5 Provider Configuration

```toml
# zap.toml
[database]
provider = "postgres"
host = "localhost"
port = 5432
database = "myapp"
username = "${ZAP_DB_USER}"
password = "${ZAP_DB_PASSWORD}"
schema = "public"
pool_max = 32
pool_acquire_timeout_ms = 5000
pool_query_timeout_ms = 30000
ssl_mode = "require"
```

### 2.6 Expected API

```zap
# Schema definition (provider-agnostic)
model User:
    id: number = autoincrement
    name: text
    email: text
    created_at: time = now()

# Query (provider-agnostic)
users = User.where(name == "YHA")
user = User.find(42)
all = User.all()

# Migration (provider-agnostic)
# zap db migrate       — applies pending migrations
# zap db plan          — shows planned migrations
# zap db rollback      — reverts last migration batch
# zap db check         — validates schema consistency
```

---

## 3. MySQL / MariaDB

### 3.1 Priority: Medium (after PostgreSQL)

### 3.2 Rationale
- Widely deployed in Asian markets
- MariaDB fork provides open-source continuity
- Common in shared hosting and smaller deployments

### 3.3 Key Differences from PostgreSQL

| Aspect | MySQL/MariaDB | PostgreSQL |
|---|---|---|
| SQL dialect | MySQL-specific | SQL:2016 closer |
| Auto-increment | AUTO_INCREMENT | SERIAL |
| String quoting | Double quotes = identifier | Double quotes = identifier (SQL standard) |
| JSON type | JSON (MySQL 5.7+) | JSON/JSONB |
| Grouping | ONLY_FULL_GROUP_BY mode | Standard |

### 3.4 Expected Configuration

```toml
[database]
provider = "mysql"
host = "localhost"
port = 3306
database = "myapp"
username = "${ZAP_DB_USER}"
password = "${ZAP_DB_PASSWORD}"
pool_max = 32
```

---

## 4. Implementation Order

### Phase 1: SQLite Provider Maturation (current)
- [x] SQLite integration and migration contracts
- [x] ORM (model, all, find, where)
- [x] Migration first-run, dry-run, check, rollback

### Phase 2: PostgreSQL Provider (deferred)
1. Design provider-agnostic query planner
2. Create PostgreSQL connection pool
3. Map schema definitions to PostgreSQL DDL
4. Implement query translation (SQLite SQL → PostgreSQL SQL)
5. Test migration compatibility
6. Add provider configuration schema
7. Integration tests against real PostgreSQL instance

### Phase 3: MySQL/MariaDB Provider (deferred)
1. Reuse PostgreSQL provider infrastructure
2. Map schema definitions to MySQL DDL
3. Implement MySQL-specific query translation
4. Test migration compatibility
5. Add provider configuration schema

### Phase 4: Additional Providers (future)
- CockroachDB
- TiDB
- Amazon Aurora
- Azure SQL

---

## 5. Provider Abstraction Layers

```text
Application Code
    │  (model definitions, queries)
    ▼
┌────────────────┐
│  Query Engine  │  (provider-agnostic query AST)
└────────┬───────┘
         │
    ┌────┴────┬──────────┐
    │         │          │
    ▼         ▼          ▼
┌────────┐┌────────┐┌────────┐
│SQLite  ││Postgres││ MySQL  │
│Provider││Provider││Provider│
└───┬────┘└───┬────┘└───┬────┘
    │         │         │
    ▼         ▼         ▼
SQLite lib  libpq     MySQL driver
```

### 5.1 Connection Pool Interface

```zap
# Provider-agnostic connection interface (design target)
database.connect(): Connection
database.disconnect(): void
database.execute(sql: text): ResultSet
database.prepare(sql: text): PreparedStatement
database.transaction(): Transaction
```

### 5.2 Schema Migration Interface

```zap
# Provider-agnostic migration interface (design target)
database.migrate: MigrationRunner
database.migrate.plan(): MigrationPlan
database.migrate.apply(plan: MigrationPlan): void
database.migrate.rollback(last_n: number): void
database.migrate.check(): MigrationStatus
```

---

## 6. Prerequisites Before Implementation

These must be completed before PostgreSQL provider work begins:

1. **Self-hosting certified (M3)** — Backend ownership must be `zap_owned`
2. **Typed IR backend ownership** — Query planner must run through Zap-owned typed IR
3. **SQLite provider fully tested** — All edge cases documented
4. **Database testing infrastructure** — Provider-agnostic test harness exists
5. **Connection pool tested on SQLite** — Pool behavior validated on file-based DB

---

## 7. Risk Assessment

| Risk | Impact | Mitigation |
|---|---|---|
| SQL dialect differences break queries | High | Query planner with provider-specific SQL generators |
| Migration data loss | Critical | Dry-run mode + backup verification + checksum validation |
| Connection pool instability | High | Bounded pools, timeout enforcement, circuit breakers |
| Provider-specific feature gaps | Medium | Feature detection + graceful degradation |
| ORM abstraction leak | Medium | Integration tests against all providers |

---

## 8. Timeline Estimate

| Milestone | Estimate | Depends On |
|---|---|---|
| PostgreSQL provider research | 2 weeks | M3 certification |
| PostgreSQL provider implementation | 4 weeks | Research complete |
| PostgreSQL provider testing | 2 weeks | Implementation complete |
| MySQL/MariaDB provider implementation | 3 weeks | PostgreSQL provider stable |
| MySQL/MariaDB provider testing | 2 weeks | Implementation complete |

---

## References

- [DEPLOYMENT_CLOUD_EN.md](DEPLOYMENT_CLOUD_EN.md) — Cloud deployment with managed databases
- [PRODUCTION_DEPLOYMENT_EN.md](PRODUCTION_DEPLOYMENT_EN.md) — Production deployment runbook
- [DEPLOYMENT_EN.md](DEPLOYMENT_EN.md) — Registry deployment boundaries
- [SECURITY.md](../SECURITY.md) — Security policy
