# Zap P3 Roadmap — Production Language and Tooling

## Purpose

Zap P3 continues after the verified `v2.0.1` maintenance release, which includes the completed P3.1 module/workspace architecture slice. The objective is to move Zap from a stable language and ecosystem foundation toward a production-ready development platform for web, AI, automation, and systems-oriented projects while preserving deterministic behavior and Rust 1.75 compatibility.

## Baseline

P2 delivered the native runtime, deterministic registry transport and resolution, signed-index verification, cache lifecycle controls, async/future foundations, cancellation and suspension primitives, and a practical LSP foundation with diagnostics, hover, completion, formatting, definitions, and workspace symbols.

## Prioritized milestones

| Milestone | Scope | Acceptance criteria | Status |
|---|---|---|---|
| P3.1 | Module and workspace architecture | Explicit module/import syntax, deterministic search paths, duplicate/cycle diagnostics, and cross-platform workspace tests | Complete |
| P3.2 | Structured error model | Native `raise`/`try`/`catch` propagation, stable diagnostics, catch binding restoration, re-raise support, and deterministic runtime behavior | Complete |
| P3.3 | Production standard library | HTTP client/server primitives, URL handling, process execution boundaries, and safe environment/configuration APIs | In progress |
| P3.4 | Async I/O integration | Deterministic runtime interfaces for timers, sockets, files, cancellation, backpressure, and resource budgets | Planned |
| P3.5 | Type-system productivity | Generic functions and collections, richer inference, pattern matching, and improved exhaustiveness diagnostics | Planned |
| P3.6 | Tooling and language server | Full formatter, workspace indexing, rename/references, import assistance, semantic tokens, and project-aware diagnostics | Planned |
| P3.7 | Quality and release engineering | Benchmarks, fuzz/property tests, security audit, reproducible artifacts, and cross-platform install verification | Planned |

## Completed implementation: P3.2

P3.2 adds `raise <expression>` and same-level `try`/`catch <binding>:` syntax to the AST runtime. Raised values propagate through functions, loops, nested blocks, and module execution until caught; catch bindings are restored after the catch body, including when the catch re-raises. Bare `raise`, missing catch clauses, malformed bindings, and missing catch bodies produce stable parser diagnostics. Uncaught values reach the process boundary as deterministic `raised error: <value>` diagnostics. The implementation preserves Rust 1.75 compatibility and AST/legacy execution parity where compatibility paths remain available.

Focused parser/evaluator coverage verifies expression-required raise syntax, uncaught flow, nested catch shadow restoration, normal completion, non-text payloads, and re-raising. The complete native suite and strict release checks remain required before publishing the next release artifact.

## First implementation target: P3.1

The P3.3 implementation has now added bounded `url_parse`, `url_encode`, `url_decode`, `http_get`, `http_request`, and direct non-shell `process_run` builtins. It also registers these APIs in the deterministic standard-library catalog and documents URL, process, and HTTP size and timeout limits. The remaining P3.3 work is to complete the safe configuration surface, add a deterministic local HTTP server primitive, and finish cross-platform network/process verification.

P3.1 is the first priority because modules and workspaces are prerequisites for reusable web, AI, and standard-library packages. The implementation will extend the existing parser-owned spans and project resolver rather than introducing a second module model.

The P3.1 implementation defines an optional deterministic `[module]` manifest section with a relative `root` and explicit `.zp` `entries`. It rejects absolute paths, traversal, missing files, duplicate entries, and unknown module fields through stable CLI diagnostics. Explicit `module` declarations and `import ... as ...` paths are resolved below the module root, imported files are traversed in deterministic source order, and circular dependencies report their complete cycle. The LSP indexes module declarations and explicit import aliases for hover, definitions, completion, and workspace symbols. Cross-platform workspace integration coverage now verifies valid graphs, nested imports, stable cycle diagnostics, and legacy compatibility. Local path dependencies and lockfile behavior from P2 remain unchanged.

## Engineering rules

Zap P3 must remain compatible with stable Rust 1.75, avoid Edition 2024-only dependencies, preserve deterministic ordering and diagnostics, and keep security-sensitive operations explicitly bounded. Every milestone requires focused regression tests, complete native tests, formatting, strict Clippy in GitHub Actions, and bilingual documentation updates.

## Release policy

No P3 release tag will be created until the selected milestone has a documented acceptance checklist, a clean working tree, passing native tests, green strict Clippy, and successful Linux, Windows, and macOS verification. The first P3 release version will be selected after P3.1 scope is implemented and reviewed.
