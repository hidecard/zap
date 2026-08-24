# Zap v2.11.9 Release Notes

**Release status:** Incremental B2 evidence release; the bootstrap stage remains B0.

## Summary

Zap v2.11.9 adds a narrowly bounded provisional branch-local option-narrowing slice to the Zap-written B2 type-checker candidate. The owned corpus tracks an `option<number>` variable, narrows it only after a direct `is_some` guard inside one indented `if` body, and uses the resulting numeric payload through an annotated function call. A paired negative fixture rejects assigning that numeric payload to `text`.

The Rust reference checker has permanent native regression coverage for the positive and negative cases. The native B2 verifier and the Zap candidate verifier require deterministic acceptance/rejection behavior and the stable `ZAP-TYPE-001` mismatch shape. This evidence is deliberately limited: it does not establish general branch analysis, compound-guard reasoning, loop narrowing, reassignment invalidation, generic inference, or complete type-checker parity.

The bootstrap stage remains **B0**. Rust remains the complete reference compiler and runtime owner. This release does not claim a fully Zap-only compiler, self-hosting, B4, or general Zap-owned type-checking semantics.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added a direct `is_some` branch-local narrowing path for a tracked `option<number>` variable. | One direct guard and one indented `if` body only |
| Native conformance | Added paired native acceptance and rejection coverage for the branch-local payload use. | Rust remains the reference owner |
| Candidate gate | Extended deterministic candidate verification from 10 to 12 JSON cases. | Corpus evidence is not general compiler correctness |
| Diagnostics | Preserved `ZAP-TYPE-001` for assigning the narrowed numeric payload to `text`. | The asserted mismatch is line 5, column 1 |
| Ownership | Added provisional `BOOT-025` metadata and paired branch fixtures. | Ownership remains provisional |
| Documentation | Updated bilingual bootstrap contracts, conformance matrices, current-status scope, roadmap checkpoint, and release metadata. | B0 boundary is unchanged |

## Verification contract

The release candidate must pass the native B2 verifier, the twice-run Zap-written candidate verifier with byte-identical JSON output, the permanent native TC-001 regression, documentation and ownership consistency checks, malformed-source safety, and the exact committed release preflight. The incompatible branch fixture must retain `kind=TypeError`, `code=ZAP-TYPE-001`, line 5, column 1, and the message `variable 'inside' expects text, got number`.

## Deferred scope

Compound boolean guards, loop-boundary narrowing beyond the existing Rust baseline, reassignment invalidation in the candidate, alias propagation beyond existing native evidence, nested maps, arbitrary nested expressions, user-defined generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Historical and release policy

Published v2.11.8 and all earlier tags remain immutable. v2.11.9 uses a new annotated tag and must not rewrite or retag prior release history. The current-status pages must identify v2.11.9 as latest only after the public workflow and artifact verification succeed.

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/branch_narrowing.zp
[3]: ../bootstrap/fixtures/typecheck/branch_narrowing_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
[7]: ../bootstrap/contracts/OWNERS.tsv
[8]: ../docs/BOOTSTRAP_CONTRACT_EN.md
