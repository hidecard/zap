# Zap v2.11.11 Release Notes

**Release status:** Incremental bootstrap-evidence release; the repository remains at B0.

## Summary

Zap v2.11.11 adds a narrowly scoped, provisional B2 candidate slice for loop-local option narrowing. The Zap-written candidate recognizes a direct `while is_some(value):` guard for a tracked `option<number>` variable, permits the numeric payload only within one indented loop body, and restores the original option-wrapper type at the loop boundary. A positive fixture covers the body use, and a paired negative fixture verifies the post-loop `option<number>` mismatch with a stable structured diagnostic.

The native Rust checker already owns the corresponding TC-006 loop-boundary behavior. This release adds candidate-side evidence and deterministic differential verification; it does not transfer compiler ownership, make the candidate a general type checker, or advance the bootstrap stage.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added direct `while is_some` narrowing for a tracked `option<number>`. | One variable, one indented loop body |
| Loop boundary | Restores the original option wrapper after the loop. | No general data-flow or mutation analysis |
| Conformance | Added positive and negative loop fixtures with exact rejection shape. | Corpus-limited evidence |
| Native gate | Extended the Rust reference verifier alongside the existing TC-006 regression. | Rust remains the reference owner |
| Candidate gate | Extended deterministic two-run candidate parity from 12 to 14 corpus outputs. | Unsupported syntax remains fail-closed |
| Ownership | Added `BOOT-026` for the provisional loop-narrowing slice. | Provisional only |
| Documentation | Synchronized English/Burmese contracts, matrix, current status, roadmap, and release notes. | B0 language preserved |

## Verification contract

The candidate must pass native reference conformance, candidate differential determinism, malformed-source no-panic safety, matrix parity, specification ownership, Markdown links, formatting, version consistency, Cargo checks, and the exact committed release preflight. The public release workflow must independently pass source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts must pass checksum, manifest, provenance, and signature verification.

## Deferred scope

Compound guards, `is_option_none` else-branch candidate support, loop mutation, reassignment invalidation, nested loops, arbitrary control-flow expressions, broader collection/map inference, nested maps, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Bootstrap boundary

Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner. The Zap lexer, parser, type-checker, and typed-IR implementations under `bootstrap/` are provisional and corpus-limited; this release does not claim a fully Zap-only, self-hosted, B1, B2, B3, or B4 compiler.

Published tags are immutable. v2.11.10 and all earlier release history must not be rewritten; v2.11.11 uses a new annotated tag and is published only after the release gates pass.

## References

[1]: ../bootstrap/contracts/OWNERS.tsv
[2]: ../bootstrap/fixtures/typecheck/loop_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/loop_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md
