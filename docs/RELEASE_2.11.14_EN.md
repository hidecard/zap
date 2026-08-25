# Zap v2.11.14

**Release status:** Published after complete validation and public artifact/signature verification. Zap remains B0.

## Summary

Zap v2.11.14 publishes a provisional, corpus-limited B2 type-checker increment for one direct list-number literal annotation shape. The published candidate recognizes the exact literal `[1, 2]` as `list<number>` and accepts `let numbers: list<number> = [1, 2]`. A paired negative fixture rejects assigning the same direct list-number literal to `text` with the stable diagnostic `variable 'wrong' expects text, got list<number>` at line 1, column 1.

This is evidence for one deterministic fixture pair only. It does not implement general list-literal inference, arbitrary list contents, variable aliases, nested list construction, collection expression inference, or complete static type checking.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Adds exact direct `[1, 2]` inference as `list<number>`. | One literal spelling and one element type |
| Diagnostics | Adds paired rejection for assigning that list literal to `text`. | Stable line 1, column 1 diagnostic |
| Native reference | Confirms the Rust checker accepts the positive fixture and reports `variable 'wrong' expects text, got list<number>` for the negative fixture. | Rust remains authoritative |
| Evidence gates | Extends native and candidate B2 verifiers from 22 to 24 deterministic cases. | Provisional corpus evidence only |
| Ownership | Adds `BOOT-030` to the bootstrap ledger. | Candidate-owned evidence; not compiler ownership |
| Documentation | Updates English and Burmese contracts, matrices, current status, and roadmap. | Broader inference remains deferred |

## Verification contract

The published candidate passed the native and Zap candidate B2 verifiers, malformed-source safety, native tests, typecheck matrix parity, specification ownership, Markdown links, VS Code packaging, formatting, release-version validation, documentation consistency, and the exact committed release preflight. The public workflow independently passed source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts passed checksum, manifest, provenance, and detached-signature verification.

## Deferred scope

General list-literal inference, arbitrary list element expressions, nested list construction, collection/map inference beyond the existing bounded corpus, compound guards, loop mutation, reassignment invalidation, aliases, arbitrary control flow, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Bootstrap boundary

Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner. The Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` remains provisional and corpus-limited. This release does not claim a fully Zap-only, self-hosted, B1, B2, B3, or B4 compiler.

## References

[1]: RELEASE_ROLLBACK_RUNBOOK_EN.md
[2]: ../bootstrap/contracts/OWNERS.tsv
[3]: ../bootstrap/fixtures/typecheck/list_annotation.zp
[4]: ../bootstrap/fixtures/typecheck/list_annotation_incompatible.zp
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[7]: ../native/src/main.rs
