# Zap Changelog

## [Unreleased]

### Package reliability

- Extended registry-backed lockfiles to version 2 with deterministic `[resolved]` entries for selected package versions, sources, and SHA-256 checksums.
- Made `zap lock` and `zap update` record resolved transitive packages, while `zap install` re-resolves and verifies the complete pinned graph.
- Preserved compatibility with existing v1 lockfiles and added integration coverage for offline cache reuse and checksum-pinned installs.

### Security audit remediation

- Added `ZAP_UNTRUSTED=1` restricted mode that denies filesystem, environment, process, network, local HTTP serving, and local registry-source capabilities by default.
- Added SSRF defenses for loopback, private, link-local, unspecified, broadcast, IPv6 unique-local, and IPv6 link-local destinations; automatic HTTP redirects are disabled in restricted mode.
- Added bounded HTTP request bodies, hard child-process deadlines with termination, and regression tests for capability denial, private destinations, and oversized request bodies.
- Documented the remaining requirement for OS-level sandboxing, least-privilege deployment, resource quotas, and network egress controls.

## [2.0.3] — 2026-08-20

Zap 2.0.3 completes the P3.3 Production Standard Library milestone.

### Added

- Bounded `url_parse`, `url_encode`, and `url_decode` builtins with deterministic validation.
- `http_get` and `http_request` builtins restricted to HTTP/HTTPS URLs with bounded timeouts and response size.
- Direct non-shell `process_run` with text arguments, UTF-8 stdout/stderr capture, status reporting, and output limits.
- Safe configuration helpers: `env_get` with deterministic defaults, platform-aware `config_dir`, and traversal-resistant `config_path` for one relative file name.
- Bounded `http_serve_once` local server that binds to loopback, serves one request, and enforces request, response, and wait limits.
- Deterministic standard-library catalog entries and bilingual documentation for the new APIs.
- Initial `vscode-extension` folder with `.zp` registration, TextMate syntax highlighting, snippets, completion, CLI-backed diagnostics, workspace checking, and run-current-file commands.
- Native and VS Code LSP signature help for Zap function calls, including active-parameter tracking after `(` and `,`.
- LSP-backed document formatting with normalized line endings, four-space indentation, and trailing-whitespace cleanup.

### Verification

- Native suite: **235 tests passed**.
- Focused P3.3 URL, process, HTTP validation, configuration, local-server argument, and compatibility regressions passed.
- Cross-platform test hardening covers native path separators in evaluator expectations, JSON-escaped Windows file fixtures, option-aware URL-port assertions, and CRLF-safe process-output normalization in the Windows smoke gate.
- The Linux native suite passed **235 tests**; Windows and macOS target-native tests, strict Clippy, and release packaging remain enforced by GitHub Actions.

## [2.0.1] — 2026-08-20

Zap v2.0.1 is a production-quality maintenance release following the P2 Ecosystem foundation. It completes the P3.1 module/workspace architecture slice and closes the remaining v2 audit findings.

### Added and fixed

- Explicit `module <name>` declarations and `import <path> as <alias>` syntax with deterministic manifest-backed resolution.
- Recursive multi-module graph validation with traversal protection, missing-target diagnostics, repeated-import caching, and full circular-dependency chains.
- LSP indexing for module declarations and import aliases across completion, hover, definitions, and workspace symbols.
- Stable runtime `Error` and `KeyError` diagnostic categories with structured human-readable and JSON output.
- Declaration-time annotation validation for supported scalar and generic forms.
- Canonical CLI help and usage output for help, invalid arguments, and invalid paths.
- JSON-RPC `-32601 Method not found` responses for unknown LSP requests while preserving notification behavior.
- Collection literal parsing and AST/legacy parity for `join` and map-key `contains` behavior.
- Cross-process integration and end-to-end tests for annotation errors, CLI help, and framed LSP requests.
- Hardened cross-platform GitHub Actions release packaging for Linux x86_64, macOS ARM64, and Windows x86_64, including archive checksums and smoke tests.

### Verification

- Native unit and integration suite: **229 tests passed**.
- Audit regression and end-to-end tests: **3 passed**.
- Formatting, whitespace, release-build, CLI smoke, example execution, and package checksum checks passed.
- GitHub Actions release workflow validates tag/Cargo version matching and publishes verified platform archives.

## [2.0.0] — 2026-08-20

Zap P2 completes the Ecosystem milestone for the native runtime, deterministic package registry, async foundation, and editor integration.

### Added

- Deterministic exact, caret, tilde, and comparator version-range selection for registry dependencies.
- HTTPS registry transport, SHA-256 artifact verification, signed-index HMAC verification, deterministic cache pruning, offline reuse, and authenticated local registry persistence.
- Checksum-verified package publishing with atomic artifact storage and signed index rewriting.
- `async fn`, deterministic `Future` values, `await`, poll-based timers, cancellation tokens, cancellable tasks, task limits, poll budgets, and deterministic suspension controls.
- LSP document synchronization, diagnostics, hover, context-aware completion, formatting, go-to-definition, and workspace symbols.
- English and Burmese documentation updates covering the complete P2 foundation.

### Verification

- Native test suite: **223 tests passed**.
- Formatting, `cargo check`, whitespace, and strict Clippy gates passed.
- Linux x86_64, Windows x86_64, and macOS ARM64 CI checks passed.
- Release artifacts are generated by the tag-triggered GitHub Actions workflow.

## [2.0.2] — 2026-08-20

### P3.2 Structured Error Model

- Added `raise <expression>` and same-level `try`/`catch <binding>:` syntax with deterministic parser diagnostics for bare `raise`, malformed bindings, missing catches, and missing catch bodies.
- Implemented structured raise propagation through functions, loops, nested blocks, and modules, including catch-binding restoration and re-raise behavior.
- Preserved uncaught raised values as stable process-boundary diagnostics using the `raised error: <value>` format.
- Verified the native suite with 229 passing tests while preserving Rust 1.75 compatibility and deterministic AST/legacy behavior.

### P2 Ecosystem progress

- Added HTTPS registry index and artifact transport, content-addressed caching, and SHA-256 integrity enforcement.
- Added validated remote publishing and deterministic nested dependency traversal with cycle diagnostics.
- Added a stable-Rust-compatible single-threaded async runtime foundation with `async fn`, deterministic `Future` values, and `await` expressions.
- Extended the stdio JSON-RPC LSP with text synchronization, deterministic diagnostics, parser-span hover, and context-aware completion.
- Synchronized the English and Burmese P2 roadmaps, async/LSP guides, and syntax references.

### Verification

- Native test suite: **223 tests passed**.
- Formatting, `cargo check`, and `git diff --check` pass locally.
- Strict Clippy and Linux, Windows, and macOS ARM64 checks pass in GitHub Actions.
- No release tag is created until every P2 track is green and verified.

## [1.0.0] — 2026-08-20

Zap P1 establishes the first complete Language Core milestone for the standalone native runtime. This release focuses on predictable language semantics, direct AST execution, safe diagnostics, and a stable foundation for future ecosystem work.

### Added

- Direct AST evaluation for functions, methods, closures, indexing, built-ins, filesystem, JSON, environment, path, and time helpers.
- Default parameters and named arguments for user-defined functions, methods, and closures.
- Static type narrowing for `option<T>` and `result<T>` guards, complex boolean conditions, aliases, and `else` branch restoration.
- OOP method and field visibility with `public`, `private`, and `protected` access rules.
- Module-aware private access checks and protected inheritance behavior.
- Constructor visibility, field default initialization, explicit `super.init()` delegation, and single implicit parent-constructor delegation.
- Stabilized text, math, collection, filesystem, JSON, environment, path, and time standard-library APIs.
- Deterministic public standard-library domain catalog and bilingual API indexes.
- Canonical `zap.lock` generation, sorted dependency entries, missing/stale lockfile rejection, and deterministic local dependency validation.
- Structured diagnostics, JSON diagnostics, source locations, secret redaction, and runtime resource limits.
- Cross-platform CI smoke checks for Linux, macOS, and Windows CLI version, help, and example execution.

### Documentation

- Updated the main README and bilingual learning guides.
- Added type-narrowing, package/lockfile, and public standard-library indexes in English and Burmese.
- Synchronized the P1 progress roadmap and release documentation.

### Verification

- 109 native tests pass: 31 unit tests and 78 integration tests.
- Formatting, whitespace, release build, CLI version/help, and runnable example checks pass locally.
- The GitHub Actions release workflow performs stable Rust formatting, Clippy, check, test, version/tag matching, and Linux/macOS/Windows artifact verification.

### Scope

P1 intentionally does not include remote package registries, package publishing, async execution, or LSP/editor integration. Those belong to the P2 Ecosystem roadmap.

## Previous releases

See the historical entries in [`CHANGELOG.md`](CHANGELOG.md).
