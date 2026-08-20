# Zap P1 Language Core Progress

## Current status

Zap P1 implementation is in progress. The final P1 release and tag are intentionally deferred until all planned milestones, documentation updates, and cross-platform release gates pass.

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
| Control-flow Option/Result narrowing | Implemented | Guarded-branch static-check regression tests |
| OOP method visibility | Implemented | Private/protected same-class and inheritance access checks with external-access diagnostics |
| OOP field visibility and initialization | Implemented (initial) | Public/private/protected fields, inherited protected access, default initialization, assignment checks, and external-access regression tests |
| OOP constructor rules | Implemented | Constructor visibility enforcement, module-aware private access, field initialization, and explicit/implicit parent delegation |
| Module-aware OOP visibility | Implemented (initial) | Declaring-module identity propagation, cross-module private access rejection, and imported-class visibility regression tests |
| Filesystem and JSON standard-library APIs | Stabilized (initial) | Direct-AST JSON round trip, malformed-input diagnostics, bounded 8 MiB JSON payloads, and existing file I/O regression coverage |
| Text, math, and collection standard-library APIs | Stabilized (initial) | Direct-AST dispatch, explicit validation, checked integer behavior, and integration regression coverage |

## Current verification baseline

The native Rust suite currently passes **103 tests**: 30 unit tests and 73 integration tests. The repository also includes the runnable example [`examples/default_parameters.zp`](../examples/default_parameters.zp). `cargo fmt --check` and `git diff --check` pass for verified changes. The local sandbox does not provide the Rust Clippy component, so Clippy remains a CI/environment release gate and is not claimed as locally verified.

## Ordered remaining P1 work

| Priority | Work item | Current state | Next acceptance criteria |
|---:|---|---|---|
| 1 | Direct AST call evaluation | In progress | Native AST directly evaluates the current runtime call set, including functions, methods, closures, indexing, pure built-ins, filesystem I/O, environment, path, and time helpers; final edge-case audit remains |
| 2 | Named arguments | Implemented | Continue advanced diagnostics and decide whether named arguments should be supported by selected built-ins |
| 3 | Control-flow type narrowing | Implemented (initial branch-local support) | Extend else-specific negative narrowing, complex boolean guards, alias variables, and broader nested-flow analysis |
| 4 | OOP visibility and initialization rules | Implemented | Continue broader module-aware field coverage and constructor diagnostic refinement |
| 5 | Standard-library extraction and stabilization | Partial (filesystem/JSON/text/math/collection initial stabilization) | Complete API contracts, documented error behavior, broader edge-case coverage, and public module organization |
| 6 | Package determinism and CLI tooling | Partial | Lockfile/deterministic dependency behavior plus stable diagnostics, filtering, formatting, and project checks |
| 7 | Cross-platform and release gates | Pending | Linux, Windows, and macOS verification; bilingual changelog/release documentation; final P1 release only after all gates pass |

The direct-AST migration is **in progress** but covers the current runtime built-in set. Named arguments are available for user-defined functions, methods, and closures. OOP now covers method and field modifiers, protected inheritance access, field default initialization, field assignment checks, and constructor visibility enforcement. Additional module-aware field coverage and constructor diagnostic refinement remain P1 hardening work. P2 work such as async execution, LSP/editor integration, and a full package registry will begin only after the P1 acceptance criteria are verified.
