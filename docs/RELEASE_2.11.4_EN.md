# Zap v2.11.4 Release Notes

**Release line:** v2.11.4
**Scope:** B2 collection-element conformance increment
**Status:** Incremental bootstrap and type-checking release

## Summary

Zap v2.11.4 adds a bounded collection-element inference increment to the provisional Zap-owned B2 type-checker candidate. For the supported corpus, a `list<number>` variable indexed by a numeric literal now produces the element type, and an incompatible annotated assignment receives a deterministic structured `TypeError` diagnostic. The native Rust checker and the Zap candidate are both covered by permanent positive and negative evidence.

This release remains explicitly **B0**. Rust remains the reference owner for complete parsing, type checking, typed IR, package/build behavior, VM execution, diagnostics, and platform boundaries. The new Zap implementation is corpus-limited evidence; it does not claim a fully Zap-only compiler, self-hosting, or B4.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Added numeric indexing of a tracked `list<T>` value and propagation of `T` for the supported fixture path. | Provisional and corpus-limited |
| Negative conformance | Added `collection_incompatible.zp`: assigning `values[0]` from `list<number>` to `text` is rejected with stable line 2/column 1 diagnostics. | Rust and Zap candidate gates |
| Native regression evidence | Added the matching native conformance case beside the existing positive TC-008 collection-element case. | Native Rust reference remains authoritative |
| Ownership | Added provisional `BOOT-022` ownership metadata for the new candidate fixture. | No stage advancement |
| Documentation | Synchronized bilingual bootstrap contracts, type-checking baseline surfaces, README/version surfaces, changelogs, and release notes. | B0 wording retained |

## Explicit limits

This increment does not implement general collection inference, map-element inference in the Zap candidate, arbitrary index expressions, nested collection propagation, control-flow-sensitive element facts, or user-defined generic declarations. The candidate typed-IR producer still covers only its existing annotated declaration fixture. Broader collection inference, deeper nested inference, and generic declarations require separate design and evidence gates.

The existing v2.11.3 tag and release history are unchanged. v2.11.4 is a new incremental release and does not rewrite or retag any prior release.

## Verification

The release candidate must pass the repository’s version, documentation, link, ownership, formatting, bootstrap, native-test, dependency, and security gates before publication. Final preflight totals are recorded from the exact committed v2.11.4 candidate rather than copied from an earlier release.

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../bootstrap/fixtures/typecheck/collection_incompatible.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[6]: ../native/tests/core.rs
