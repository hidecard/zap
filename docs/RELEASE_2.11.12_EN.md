# Zap v2.11.12 Release Notes

**Release status:** Incremental bootstrap-evidence release; the repository remains at B0.

## Summary

Zap v2.11.12 adds a narrowly scoped, provisional B2 candidate slice for the direct negative option predicate form `if is_option_none(name): ... else:`. For one tracked `option<number>` variable, the true body retains the option wrapper and one indented `else` body may use the payload as `number`. Paired fixtures cover the accepted form and the exact incompatible diagnostic at line 5, column 1: `variable 'payload' expects text, got number`.

The native Rust checker remains the complete/reference owner. This release adds candidate-side evidence and deterministic differential verification for behavior already represented by the native reference; it does not transfer compiler ownership or advance the bootstrap stage.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added direct `is_option_none` else-body narrowing for one tracked `option<number>`. | One variable, one direct conditional shape |
| True branch | Retains the original `option<number>` wrapper. | No automatic payload widening |
| Else branch | Narrows the tracked value to `number` in one indented body. | No compound or arbitrary predicates |
| Conformance | Added positive and negative fixtures with exact line/column diagnostics. | Corpus-limited evidence |
| Native gate | Extended the Rust reference verifier for the paired fixtures. | Rust remains the reference owner |
| Candidate gate | Extended deterministic candidate parity from 14 to 16 JSON outputs. | Unsupported syntax remains fail-closed |
| Ownership | Added `BOOT-027` for the provisional else-branch slice. | Provisional only |
| Documentation | Synchronized English/Burmese contracts, matrices, narrowing guides, TODO roadmap, and release notes. | B0 language preserved |

## Verification contract

The candidate must pass native reference conformance, candidate differential determinism, malformed-source no-panic safety, matrix parity, specification ownership, Markdown links, formatting, version consistency, Cargo checks, and the exact committed release preflight. The public release workflow must independently pass source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts must pass checksum, manifest, provenance, and signature verification.

## Deferred scope

Multiple option variables, compound guards, nested or compound control flow, loop mutation, reassignment invalidation, alias propagation, arbitrary user-defined predicates, broader collection/map inference, nested maps, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Bootstrap boundary

Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner. The Zap lexer, parser, type-checker, and typed-IR implementations under `bootstrap/` are provisional and corpus-limited; this release does not claim a fully Zap-only, self-hosted, B1, B2, B3, or B4 compiler.

Published tags are immutable. v2.11.11 and all earlier release history must not be rewritten; v2.11.12 uses a new annotated tag and is published only after the release gates pass.

## References

[1]: ../bootstrap/contracts/OWNERS.tsv
[2]: ../bootstrap/fixtures/typecheck/else_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/TYPE_NARROWING_EN.md
