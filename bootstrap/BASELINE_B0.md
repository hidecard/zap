# Zap B0 Baseline Freeze

**Baseline release:** v2.9.2  
**Bootstrap stage:** B0  
**Freeze date:** 2026-08-24  
**Reference commit:** `18dec9a028a406491187d1e79289410ee491d356`  
**Release tag:** `v2.9.2`
**Historical role:** This freeze remains the immutable v2.9.2 reference record. The current v2.10.0 branch carries the same B0 ownership and non-self-hosted boundary with independent version identities in [`contracts/VERSIONS.toml`](contracts/VERSIONS.toml).

## Baseline meaning

This document freezes the current Rust implementation as the reference behavior owner for the first Zap-only bootstrap batch. The baseline is **not self-hosted**. It records the behavior that future Zap-owned lexer, parser, diagnostics, type-checker, standard-library, IR, package, VM, and platform-seed implementations must reproduce or explicitly supersede through a compatibility decision.

## Reference pipeline

The B0 pipeline is:

```text
UTF-8 .zp source -> Rust lexer -> AST parser -> evaluator/runtime
```

The native reference also owns diagnostics, standard-library behavior, module/project validation, package/registry behavior, and the current runtime capability boundaries. Host adapter behavior remains separately covered by its Rust test suite.

## Version identities

| Identity | Value |
|---|---|
| Language version | 2.9.2 |
| Compiler version | 2.9.2 |
| Standard-library version | 2.9.2 |
| Token schema | 1 |
| AST schema | 1 |
| Diagnostic schema | 1 |
| Typed IR schema | 0 |
| Manifest schema | 1 |
| Lockfile schema | 1 |
| Platform-seed version | 0 |

The independent values are defined in [`contracts/VERSIONS.toml`](contracts/VERSIONS.toml).

## Test evidence

The baseline was executed from the reference commit with the pinned repository toolchain. Native tests completed with **258 passed, 0 failed**. Host tests completed with **9 unit tests and 10 HTTP contract tests passed, 0 failed**. The commands were:

```text
cargo test --manifest-path native/Cargo.toml --all-targets
cargo test --manifest-path host/zap-host/Cargo.toml --all-targets
```

The full command output was captured during the freeze and is retained as local audit evidence outside the repository. Future CI jobs must rerun these commands before accepting a bootstrap-layer change.

## Frozen behavior families

| Family | B0 owner | Required future evidence |
|---|---|---|
| Syntax and tokens | `native/src/lexer.rs` | Token kind, value, span, invalid-input, and deterministic hash fixtures |
| AST and precedence | `native/src/ast.rs`, `native/src/parser.rs` | Canonical AST snapshots and parser negative fixtures |
| Diagnostics | `native/src/diagnostics.rs` plus parser/evaluator errors | Stable code/severity/location/notes/help records |
| Runtime values | `native/src/value.rs`, `native/src/evaluator.rs` | stdout, stderr, exit-status, and error behavior fixtures |
| Standard library | `native/src/stdlib.rs`, `native/src/stdlib_catalog.rs` | Catalog, signature, determinism, and security classification |
| Project/package | `native/src/project.rs`, `native/src/registry.rs` | Manifest, lockfile, offline resolution, hash, and signature evidence |
| Host boundary | `host/zap-host` | Cross-process HTTP and capability-denial regression coverage |

## Freeze rules

Until a compatibility record is accepted, the bootstrap work MUST NOT silently change syntax, implicit coercion, runtime value representation, diagnostic meaning, standard-library contracts, lockfile semantics, or host capability behavior. A change must update the relevant English and Burmese contract, fixture, ownership record, and changelog entry.

The next gate is the canonical artifact and fixture batch. It will add machine-readable metadata and deterministic corpus files while keeping B0 execution unchanged.
