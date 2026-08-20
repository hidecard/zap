# Zap Changelog

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
