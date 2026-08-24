# Zap Bootstrap and Self-Hosting Contract

**Status:** B0 reference baseline for Zap v2.9.2

Zap’s self-hosting roadmap is staged. The current release remains a **Rust reference/native implementation**; it is not yet a fully Zap-only compiler. The normative stage contract, independent version identities, and machine-readable ownership records are maintained under [`bootstrap/contracts`](../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md).

## Current B0 boundary

The reference pipeline is:

```text
Zap source -> Rust lexer -> AST parser -> evaluator/runtime
```

Rust/Cargo is therefore still required to build the current compiler. The operating-system loader and explicitly documented platform boundary are accepted as infrastructure boundaries, while other language runtimes and frameworks are not required by the current Zap compiler path.

## B1 candidate status

A first Zap-owned lexer candidate is now checked in at [`bootstrap/b1/lexer.zp`](../bootstrap/b1/lexer.zp). It covers the current identifier, number, text, comment, whitespace, operator, delimiter, Unicode, and fail-closed diagnostic paths needed by the initial owned corpus. [`scripts/bootstrap/verify_b1_lexer.sh`](../scripts/bootstrap/verify_b1_lexer.sh) executes the candidate and compares its output with the B0 token/diagnostic artifacts.

This is a **corpus-limited B1 foundation**, not a completed B1 compiler. The candidate is not yet the reference owner, does not replace the Rust lexer, and must expand through differential fixtures before the repository can advance the bootstrap stage claim.

The native reference parser now has an additive differential corpus at [`bootstrap/fixtures/parser/compound.zp`](../bootstrap/fixtures/parser/compound.zp), with a canonical AST snapshot and a syntax-rejection diagnostic fixture. [`scripts/bootstrap/verify_b1_parser.sh`](../scripts/bootstrap/verify_b1_parser.sh) checks byte-for-byte reproducibility and reference output. This is parser contract evidence only; no Zap-owned parser is claimed yet.

## B3 foundation status

The repository now has a reference-only B3 foundation gate at [`scripts/bootstrap/verify_b3_foundations.sh`](../scripts/bootstrap/verify_b3_foundations.sh). It validates the catalog determinism taxonomy, generates a canonical dependency-free manifest lockfile, checks lockfile reproducibility, runs an offline locked build, and executes a Zap test fixture. These checks demonstrate existing package/build/test-runner behavior; they do not claim that the compiler pipeline is already Zap-owned.

## Reference VM and platform-seed status

The first isolated bytecode VM foundation is implemented in [`native/src/bytecode.rs`](../native/src/bytecode.rs). `zap bootstrap vm-demo` executes a bounded arithmetic program and emits the canonical [`bootstrap/fixtures/bytecode/vm_demo.json`](../bootstrap/fixtures/bytecode/vm_demo.json) artifact. The VM rejects unsupported schema versions, malformed stack shapes, missing halts, arithmetic failures, and step-budget exhaustion without panicking.

The platform boundary remains documented rather than self-hosted. The compiler core has no network or process capability; console, bounded file access, memory, and optional clock behavior remain explicit seed responsibilities. [`scripts/bootstrap/verify_vm_platform.sh`](../scripts/bootstrap/verify_vm_platform.sh) checks this boundary and the deterministic VM smoke artifact.

## Canonical inspection commands

The native CLI now exposes read-only B0 inspection commands:

```text
zap bootstrap status
zap bootstrap tokens <file.zp>
zap bootstrap ast <file.zp>
zap bootstrap typed-ir <file.zp>
zap bootstrap diagnostics <file.zp>
```

The first batch freezes representative token, AST, reference-only typed-IR, diagnostic, metadata, platform-boundary, and standard-library fixtures under [`bootstrap/fixtures`](../bootstrap/fixtures). Run [`scripts/bootstrap/verify_b0_artifacts.sh`](../scripts/bootstrap/verify_b0_artifacts.sh) to rebuild those artifacts and compare them byte-for-byte with the committed corpus.

## Stage policy

| Stage | Meaning | Allowed release claim |
|---|---|---|
| B0 | Rust owns reference behavior and fixtures | Rust reference/native implementation |
| B1 | Zap lexer/parser reproduces B0 artifacts | Zap bootstrap compiler foundation |
| B2 | Zap diagnostics/type checker reproduces B0 acceptance and rejection | Zap bootstrap compiler foundation |
| B3 | Zap stdlib, typed IR, package resolver, and test runner operate offline and deterministically | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler rebuilds itself from the documented platform seed | Fully Zap-only self-hosted compiler |

No release may use the B4 wording before the B4 bootstrap checks pass. Future semantic or artifact changes require bilingual contract updates, fixture changes, ownership records, compatibility decisions where applicable, and regression evidence.

## Next gate

The active implementation gate remains B1 parity expansion: the Zap-owned lexer must cover the full owned corpus and its differential runner must compare candidate output with the Rust reference for valid, Unicode, malformed, overflow, and determinism cases. The reference parser corpus is now included as a separate foundation gate; the next ownership step is a Zap-written parser candidate compared against these snapshots. Type checking, VM work, and native-backend work must remain behind that gate rather than being claimed prematurely.
