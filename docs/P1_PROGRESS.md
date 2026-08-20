# Zap P1 Language Core Progress

## Current status

Zap P1 Language Core is complete for release candidate `v1.0.0`. The release tag is created only after the documented source, local verification, and GitHub Actions release gates are satisfied.

## Verified milestones

| Milestone | Status | Verification |
|---|---|---|
| Generic `list<T>` and `map<K,V>` annotation matching | Implemented | Nested generic and mismatch regression tests |
| Generic `result<T>` and `option<T>` payload matching | Implemented | Runtime and static annotation tests |
| Typed `option_none()` assignment | Implemented | `option<T> = option_none()` check test |
| Static annotated-variable reassignment checks | Implemented | Incompatible reassignment regression test |
| Explicit `super.init()` dispatch | Implemented | Native OOP integration test |
| Explicit `super.method()` dispatch | Implemented | Parent override integration test |
| Runtime map-key validation | Implemented | Full runtime annotation path covered |
| Result/Option question-operator propagation | Implemented | Result and Option propagation regression tests |
| Duplicate function-parameter rejection | Implemented | Parser and integration regression test |
| Function return-type validation | Implemented | Static and runtime return diagnostics |
| Persistent mutable closure state | Implemented | Nested closure mutation regression test |
| Positional default function parameters | Implemented | Parser, AST, runtime binding, static arity checking, and integration example |
| Named arguments | Implemented | Structured AST parsing, deterministic binding, function/method/closure integration tests, and diagnostics |
| Native AST function-body storage | Implemented | Native AST execution tests |
| Control-flow Option/Result narrowing | Implemented | Single guards, complex boolean conjunction/disjunction, alias propagation, else-branch scope restoration, and static-check regression tests |
| OOP method visibility | Implemented | Private/protected same-class and inheritance access checks with external-access diagnostics |
| OOP field visibility and initialization | Implemented (initial) | Public/private/protected fields, inherited protected access, default initialization, assignment checks, and external-access regression tests |
| OOP constructor rules | Implemented | Constructor visibility enforcement, module-aware private access, field initialization, and explicit/implicit parent delegation |
| Module-aware OOP visibility | Implemented (initial) | Declaring-module identity propagation, cross-module private access rejection, and imported-class visibility regression tests |
| Filesystem and JSON standard-library APIs | Stabilized (initial) | Direct-AST JSON round trip, malformed-input diagnostics, bounded 8 MiB JSON payloads, and existing file I/O regression coverage |
| Text, math, and collection standard-library APIs | Stabilized (initial) | Direct-AST dispatch, explicit validation, checked integer behavior, and integration regression coverage |
| Public standard-library organization | Implemented (initial) | Deterministic `text`, `math`, `collections`, `filesystem`, `json`, and `system` domain catalog with bilingual indexes |
| Direct-AST edge-case audit | Hardened (initial) | Nested helper-call regression coverage across text, collections, math, filesystem, JSON, and environment APIs |

## Current verification baseline

The native Rust suite currently passes **109 tests**: 31 unit tests and 78 integration tests. The repository includes the runnable example [`examples/default_parameters.zp`](../examples/default_parameters.zp), synchronized English/Burmese type-narrowing, package, standard-library, and release guides, and README links for the current learning materials. The native CLI version output matches the `v1.0.0` release line. `cargo fmt --check` and `git diff --check` pass. Stable Rust Clippy remains enforced by the GitHub Actions release workflow; it is not claimed as locally verified when the local sandbox lacks the Clippy component.

## P1 completion and next roadmap

| Priority | Work item | Current state | Next acceptance criteria |
|---:|---|---|---|
| 1 | Direct AST call evaluation | In progress | Native AST directly evaluates the current runtime call set, including functions, methods, closures, indexing, pure built-ins, filesystem I/O, environment, path, and time helpers; final edge-case audit remains |
| 2 | Named arguments | Implemented | Continue advanced diagnostics and decide whether named arguments should be supported by selected built-ins |
| 3 | Control-flow type narrowing | Implemented | Continue broader nested-flow analysis and negative narrowing diagnostics for additional guard forms |
| 4 | OOP visibility and initialization rules | Implemented | Continue broader module-aware field coverage and constructor diagnostic refinement |
| 5 | Standard-library extraction and stabilization | Implemented (initial) | Continue API contract hardening and future namespace exposure; deterministic public domain catalog and bilingual indexes are complete |
| 6 | Package determinism and CLI tooling | Implemented (initial) | Canonical `zap.lock` generation, sorted dependency entries, missing/stale lockfile rejection, stable diagnostics, project checks, and version/help consistency |
| 7 | Cross-platform and release gates | Completed for P1 release | Linux, Windows, and macOS matrix builds run CLI version/help/example smoke checks; bilingual release changelogs and release workflow packaging are complete |

The direct-AST migration is complete for the current runtime call set, with nested-call audit coverage. The standard-library public surface has deterministic domain metadata and bilingual indexes. CI includes platform-specific CLI smoke checks. Package tooling supports deterministic local dependency declarations and canonical lockfile validation; remote registry resolution and publishing remain later ecosystem work. Named arguments are available for user-defined functions, methods, and closures. OOP covers method and field modifiers, protected inheritance access, field default initialization, field assignment checks, module-aware visibility, and constructor delegation rules. Control-flow narrowing handles single guards, `and`/`or` guard combinations, aliases, and restoration of the original option/result type after an `else` branch. P1 release documentation is synchronized in English and Burmese. P2 work such as async execution, LSP/editor integration, and a full package registry is now the next roadmap phase.
