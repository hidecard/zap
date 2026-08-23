# Zap Standard Library Public Modules

**Verified baseline:** Zap v2.2.4
**Purpose:** Public standard-library reference for language users and package authors; stability rules are owned by the linked policy.
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Learning guide](LEARN_ZAP_EN.md) · [Syntax reference](SYNTAX_GUIDE_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Package author guide](PACKAGE_EN.md) · [Stability policy](STDLIB_POLICY_EN.md)

Zap's standard library is organized into stable public domains. The runtime dispatch remains centralized for compatibility, while this domain index provides a deterministic public organization for documentation, tooling, and future package modules. The normative stability, deprecation, semver, platform, limit, timeout, error, and schema-2 determinism-class rules are defined in the [standard-library stability policy](STDLIB_POLICY_EN.md).

| Public module | Scope | Representative APIs |
|---|---|---|
| `text` | Text conversion and manipulation | `len`, `str`, `type`, `upper`, `lower`, `trim`, `split`, `join`, `contains`, `replace`, `char_at`, `substring`, `codepoints` |
| `math` | Numeric operations | `abs`, `min`, `max`, `pow`, `sqrt` |
| `collections` | Lists and maps | `sum`, `range`, `keys`, `entries`, `enumerate`, `count`, `reverse`, `sort`, `get` |
| `filesystem` | Bounded text and line I/O | `read_text`, `write_text`, `read_lines`, `write_lines`, `exists`, `file_metadata`, `atomic_write` |
| `json` | JSON serialization and runtime-category validation | `json`, `from_json`, `from_json_typed` |
| `system` | Environment, configuration, and paths | `env`, `has_env`, `env_get`, `config_dir`, `config_path`, `path_join`, `basename`, `dirname`, `now`, `sleep` |
| `time` | UTC timestamps and signed duration decomposition | `utc_now`, `duration_parts`, `duration_between` |
| `logging` | Deterministic structured log records and JSON lines | `log_record`, `log_json` |
| `runtime` | Assertions, bounded memory diagnostics, lifecycle counters, and capability reporting | `assert`, `memory_stats` |
| `async` | Deterministic executor-backed tasks, cancellation, timeout, and capability reporting | `spawn`, `task_join`, `task_is_ready`, `task_cancel`, `task_join_timeout`, `async_capabilities` |
| `network` | URL handling, bounded HTTP requests, and a local one-request server | `url_parse`, `url_encode`, `url_decode`, `http_get`, `http_request`, `http_serve_once` |
| `process` | Non-shell process execution | `process_run` |

All public builtins use explicit argument validation and return structured runtime errors rather than silently accepting invalid input. The `runtime` domain exposes `assert(condition, message)` for fail-fast validation and `memory_stats()` with live-object, object allocation/deallocation, validation/cleanup lifecycle, logical budget, value-size-limit, and deferred-capability fields, including `cycle_policy=explicit_clear_object_fields`. Logical budget failures use stable `ZAP-MEMORY-001` diagnostics. Public weak references are reported as unsupported and tracing collection as not implemented. Public builtin boundaries reject oversized or excessively deep/cyclic value graphs deterministically. Filesystem, JSON, and HTTP response operations use documented 8 MiB safety limits. URL inputs are limited to 8 KiB. `process_run` invokes a program directly without shell interpretation, accepts only a text command and list of text arguments, captures UTF-8 stdout/stderr, and rejects output larger than 1 MiB. HTTP requests accept only `http` and `https` URLs and use bounded connect, read, and write timeouts. `http_serve_once` binds to `127.0.0.1`, serves exactly one HTTP request, and enforces a 64 KiB request limit, an 8 MiB response limit, and a 10-second wait limit. `env_get` provides a deterministic text fallback without mutating the process environment. `config_dir` resolves the platform configuration directory using XDG configuration rules on Unix-like systems, `Application Support` on macOS, and `APPDATA`/`LOCALAPPDATA` on Windows. `config_path` accepts only one relative file name and rejects path separators and traversal components. The `time` APIs use UTC and integer millisecond precision: `utc_now()` returns `unix_seconds` and `unix_millis`; `duration_parts(milliseconds)` returns signed `days`, `hours`, `minutes`, `seconds`, `millis`, and `milliseconds`; and `duration_between(end_millis, start_millis)` decomposes the checked difference `end_millis - start_millis`. Overflow is reported as a runtime error rather than wrapping. The `logging` APIs are pure record builders: `log_record(level, message, fields)` returns a map with `level`, `message`, and `fields`, while `log_json(level, message, fields)` returns one canonical JSON line with alphabetically ordered field names. Levels are limited to `trace`, `debug`, `info`, `warn`, and `error`; messages are limited to 8 KiB, fields to 64 entries, field names to 256 bytes, and encoded output to 64 KiB. These APIs do not write to process streams, which keeps output deterministic and lets applications choose their own sink.

The async domain exposes a context-owned language task facade through executor-backed `ScheduledFuture` values. `async_capabilities()` reports which work is deterministic, worker-backed, bounded, cancellable, deferred, or unsupported; it reports runtime-state scheduling, cooperative language cancellation, and poll-budget timeouts, while typed resource-limit preflight remains enforced and does not start any worker, network, or process operation. The public catalog is deterministic: each builtin appears once and belongs to one domain. The native runtime includes a catalog used by tooling and tests so documentation and future module exports can remain synchronized with implementation.

```zap
let endpoint = url_parse("https://example.com:8443/api?q=zap")
say endpoint["host"]
say url_encode("a b/c")

let result = process_run("printf", ["zap"])
say result["success"]
say result["stdout"]

let fallback = env_get("ZAP_OPTIONAL_SETTING", "default")
let settings = config_path("settings.json")

let current = utc_now()
let elapsed = duration_between(current["unix_millis"], current["unix_millis"] - 1500)
say elapsed["milliseconds"]

let event = log_record("info", "server started", {"port": 8080, "mode": "dev"})
say log_json(event["level"], event["message"], event["fields"])

# Bind to loopback, serve one request, then return request metadata.
let served = http_serve_once(8080, "Hello from Zap")
say served["path"]
```

The current release line exposes these APIs as direct builtins. Every listed domain and builtin is cataloged as stable for v2.2.4 with no active deprecation window, the release-target platform matrix, and an explicit schema-2 determinism class; see the [stability policy](STDLIB_POLICY_EN.md) for the change checklist. Namespace import syntax and remote standard-library packages remain later ecosystem milestones after P1 verification.
