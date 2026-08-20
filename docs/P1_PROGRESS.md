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
| Native AST function-body storage | Implemented | Native AST execution tests |

## Current verification baseline

The native Rust suite currently passes **85 tests**: 25 unit tests and 60 integration tests. The repository also includes the runnable example [`examples/default_parameters.zp`](../examples/default_parameters.zp). `cargo fmt --check` and `git diff --check` pass for verified changes. The local sandbox does not provide the Rust Clippy component, so Clippy remains a CI/environment release gate and is not claimed as locally verified.

## Ordered remaining P1 work

| Priority | Work item | Current state | Next acceptance criteria |
|---:|---|---|---|
| 1 | Direct AST call evaluation | In progress | Native AST now directly evaluates literals, collections, operators, user-function calls, member access, object methods, list/map indexing, and pure built-ins such as `len`, `range`, `sum`, `split`, `join`, `ok`, `some`, and `unwrap`; filesystem/time-sensitive built-in fallback remains to be migrated |
| 2 | Named arguments | Not implemented | Parse `name = expression` only inside calls; reject unknown, duplicate, positional-after-named, and missing arguments; bind functions and methods consistently |
| 3 | Control-flow type narrowing | Not implemented | `if is_some(value):` and `if is_ok(result):` provide safe branch-local payload types, including `else` handling and nested branches |
| 4 | OOP visibility and initialization rules | Partial | Define and enforce public/private/protected members, constructor rules, and diagnostics across inheritance and modules |
| 5 | Standard-library extraction and stabilization | Partial | Stabilize filesystem, JSON, path, time, environment, text, math, and collection APIs with documented error behavior |
| 6 | Package determinism and CLI tooling | Partial | Lockfile/deterministic dependency behavior plus stable diagnostics, filtering, formatting, and project checks |
| 7 | Cross-platform and release gates | Pending | Linux, Windows, and macOS verification; bilingual changelog/release documentation; final P1 release only after all gates pass |

The direct-AST migration is **in progress** and is not yet the final call architecture. Named arguments are **not** available in the current release line. The documentation deliberately describes positional defaults only. P2 work such as async execution, LSP/editor integration, and a full package registry will begin only after the P1 acceptance criteria are verified.
