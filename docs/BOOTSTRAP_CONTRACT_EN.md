# Zap Bootstrap and Self-Hosting Contract

**Status:** B0 reference baseline for Zap v2.9.2

Zap’s self-hosting roadmap is staged. The current release remains a **Rust reference/native implementation**; it is not yet a fully Zap-only compiler. The normative stage contract, independent version identities, and machine-readable ownership records are maintained under [`bootstrap/contracts`](../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md).

## Current B0 boundary

The reference pipeline is:

```text
Zap source -> Rust lexer -> AST parser -> evaluator/runtime
```

Rust/Cargo is therefore still required to build the current compiler. The operating-system loader and explicitly documented platform boundary are accepted as infrastructure boundaries, while other language runtimes and frameworks are not required by the current Zap compiler path.

## Canonical inspection commands

The native CLI now exposes read-only B0 inspection commands:

```text
zap bootstrap status
zap bootstrap tokens <file.zp>
zap bootstrap ast <file.zp>
zap bootstrap diagnostics <file.zp>
```

The first batch freezes representative token, AST, diagnostic, metadata, platform-boundary, and standard-library fixtures under [`bootstrap/fixtures`](../bootstrap/fixtures). Run [`scripts/bootstrap/verify_b0_artifacts.sh`](../scripts/bootstrap/verify_b0_artifacts.sh) to rebuild those artifacts and compare them byte-for-byte with the committed corpus.

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

The next implementation gate is B1: a Zap-owned lexer that emits the token schema defined by the B0 fixture corpus and a differential runner that compares candidate output with the Rust reference. Parser migration, type checking, VM work, and native-backend work must remain behind that gate rather than being claimed prematurely.
