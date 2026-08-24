# Zap v2.11.6 Release Notes

**Release line:** v2.11.6
**Scope:** B2 nested-list inference conformance slice
**Status:** Incremental bootstrap-evidence release

## Summary

Zap v2.11.6 adds a narrowly scoped nested-list inference slice to the provisional Zap B2 type-checker candidate. The candidate now recognizes the numeric element type of `list<list<number>>` for the paired expression `rows[0][1]`. A matching negative fixture rejects assignment of that numeric result to a `text` annotation with a stable structured diagnostic.

The native Rust checker and the provisional Zap candidate are both covered by deterministic release-gated checks. The native conformance test records the same positive and negative cases, and the ownership ledger records the new candidate fixture as provisional.

The bootstrap stage remains **B0**. Rust remains the reference owner for complete compiler and runtime semantics. This release does not claim general nested expression inference, broad collection inference, a fully Zap-only compiler, self-hosting, or B4.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added bounded nested-list index inference for `list<list<number>>`. | Candidate-only and corpus-limited |
| Native conformance | Added positive and negative `TC-008` nested collection cases. | Rust remains reference owner |
| Differential evidence | Expanded native and Zap candidate gates to paired nested fixtures with deterministic JSON diagnostics. | No stage advancement |
| Ownership | Added `BOOT-023` for the nested collection candidate fixture. | Provisional ownership record |
| Documentation | Updated English/Burmese bootstrap contracts, type-check matrix, TODO checkpoint, and current-status pages. | B0 boundary unchanged |

## Verification

The exact committed v2.11.6 candidate must pass version consistency, bilingual documentation parity, Markdown links, ownership, formatting, native and host tests, all bootstrap gates, dependency audit, and release preflight before publication. The nested candidate scope is limited to the explicit paired fixtures; unsupported or unknown expressions remain outside the candidate contract.

## Historical and release policy

The published v2.11.5 release remains immutable. v2.11.6 uses a new annotated tag and does not rewrite or retag v2.11.4 or v2.11.5. Historical changelog entries and prior release notes remain unchanged.

## References

[1]: ../bootstrap/b2/typecheck.zp
[2]: ../bootstrap/fixtures/typecheck/nested_collection.zp
[3]: ../bootstrap/fixtures/typecheck/nested_collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../docs/BOOTSTRAP_CONTRACT_EN.md
