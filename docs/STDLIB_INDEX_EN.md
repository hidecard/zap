# Zap Standard Library Public Modules

Zap's standard library is organized into stable public domains. The runtime dispatch remains centralized for compatibility, while this domain index provides a deterministic public organization for documentation, tooling, and future package modules.

| Public module | Scope | Representative APIs |
|---|---|---|
| `text` | Text conversion and manipulation | `len`, `str`, `type`, `upper`, `lower`, `trim`, `split`, `join`, `contains`, `replace`, `char_at`, `substring`, `codepoints` |
| `math` | Numeric operations | `abs`, `min`, `max`, `pow`, `sqrt` |
| `collections` | Lists and maps | `sum`, `range`, `keys`, `entries`, `enumerate`, `count`, `reverse`, `get` |
| `filesystem` | Bounded text and line I/O | `read_text`, `write_text`, `read_lines`, `write_lines`, `exists`, `file_metadata`, `atomic_write` |
| `json` | JSON serialization and runtime-category validation | `json`, `from_json`, `from_json_typed` |
| `system` | Environment, configuration, paths, and time | `env`, `has_env`, `env_get`, `config_dir`, `config_path`, `path_join`, `basename`, `dirname`, `now`, `sleep` |
| `network` | URL handling, bounded HTTP requests, and a local one-request server | `url_parse`, `url_encode`, `url_decode`, `http_get`, `http_request`, `http_serve_once` |
| `process` | Non-shell process execution | `process_run` |

All public builtins use explicit argument validation and return structured runtime errors rather than silently accepting invalid input. Filesystem, JSON, and HTTP response operations use documented 8 MiB safety limits. URL inputs are limited to 8 KiB. `process_run` invokes a program directly without shell interpretation, accepts only a text command and list of text arguments, captures UTF-8 stdout/stderr, and rejects output larger than 1 MiB. HTTP requests accept only `http` and `https` URLs and use bounded connect, read, and write timeouts. `http_serve_once` binds to `127.0.0.1`, serves exactly one HTTP request, and enforces a 64 KiB request limit, an 8 MiB response limit, and a 10-second wait limit. `env_get` provides a deterministic text fallback without mutating the process environment. `config_dir` resolves the platform configuration directory using XDG configuration rules on Unix-like systems, `Application Support` on macOS, and `APPDATA`/`LOCALAPPDATA` on Windows. `config_path` accepts only one relative file name and rejects path separators and traversal components.

The public catalog is deterministic: each builtin appears once and belongs to one domain. The native runtime includes a catalog used by tooling and tests so documentation and future module exports can remain synchronized with implementation.

```zap
let endpoint = url_parse("https://example.com:8443/api?q=zap")
say endpoint["host"]
say url_encode("a b/c")

let result = process_run("printf", ["zap"])
say result["success"]
say result["stdout"]

let fallback = env_get("ZAP_OPTIONAL_SETTING", "default")
let settings = config_path("settings.json")

# Bind to loopback, serve one request, then return request metadata.
let served = http_serve_once(8080, "Hello from Zap")
say served["path"]
```

The current release line exposes these APIs as direct builtins. Namespace import syntax and remote standard-library packages remain later ecosystem milestones after P1 verification.
