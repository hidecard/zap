# Zap v2.11.8 Release Notes

**Release line:** v2.11.8  
**Scope:** Bounded provisional map-element inference evidence  
**Status:** Incremental B2 evidence release; bootstrap stage remains B0

## Summary

Zap v2.11.8 adds a narrow, provisional map-element inference slice to the Zap-written B2 type-checker candidate. The owned corpus covers a tracked `map<text,number>` variable indexed by a text literal, together with a paired negative fixture that rejects assigning the inferred numeric element to `text`.

The native Rust reference checker now has permanent TC-008 regression coverage for the same positive and negative behavior. The B2 reference and candidate verifiers require deterministic output and stable `ZAP-TYPE-001` diagnostic fields for the mismatch. This evidence is intentionally limited: it does not establish arbitrary key-expression inference, nested-map inference, general collection inference, or complete type-checker parity.

The bootstrap stage remains **B0**. Rust remains the complete reference compiler and runtime owner. This release does not claim a fully Zap-only compiler, self-hosting, B4, or general Zap-owned type-checking semantics.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added a corpus-limited `map<text,number>` text-key element inference path. | Tracked map variable and text literal key only |
| Native conformance | Added positive and negative TC-008 map-element regression cases. | Rust remains the reference owner |
| Candidate gate | Extended deterministic candidate verification from eight to ten JSON cases. | Candidate evidence is not general compiler correctness |
| Ownership | Added provisional `BOOT-024` metadata and paired map fixtures. | Ownership remains provisional |
| Documentation | Updated English/Burmese contracts, matrices, current-status scope, roadmap checkpoint, and release metadata. | B0 boundary unchanged |

## Verification contract

The release candidate must pass the native B2 verifier, the Zap-written candidate verifier twice with byte-identical JSON output, the TC-008 native regression, documentation and ownership consistency checks, malformed-source safety, and the exact committed release preflight. The map mismatch must retain `kind=TypeError`, `code=ZAP-TYPE-001`, line 2, column 1, and the message `variable 'result' expects text, got number`.

## Historical and release policy

The published v2.11.7 release and all earlier tags remain immutable. v2.11.8 uses a new annotated tag and must not rewrite or retag prior release history. Current-status pages are updated to identify v2.11.8 as the latest release only after the publication workflow and public artifact verification succeed.

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/map_collection.zp
[3]: ../bootstrap/fixtures/typecheck/map_collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
[7]: ../bootstrap/contracts/OWNERS.tsv
[8]: ../docs/BOOTSTRAP_CONTRACT_EN.md
