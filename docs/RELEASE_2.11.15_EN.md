# Zap v2.11.15

**Release status:** Candidate preparation; publication requires complete validation. Zap remains B0.

## Summary

Zap v2.11.15 prepares a provisional, corpus-limited B2 type-checker increment for one exact direct map-literal annotation shape. The candidate recognizes the exact literal `{"score": 7}` as `map<text,number>` and accepts `let scores: map<text,number> = {"score": 7}`. A paired negative fixture rejects assigning the same direct map literal to `text` with the stable diagnostic `variable 'wrong' expects text, got map<text,number>` at line 1, column 1.

This is evidence for one deterministic fixture pair only. It does not implement general map-literal inference, arbitrary map keys or values, nested maps, aliases, collection expression inference, or complete static type checking.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Adds exact direct `{"score": 7}` inference as `map<text,number>`. | One literal spelling, one key, and one value type |
| Diagnostics | Adds paired rejection for assigning that map literal to `text`. | Stable line 1, column 1 diagnostic |
| Native reference | Confirms the Rust checker accepts the positive fixture and reports `variable 'wrong' expects text, got map<text,number>` for the negative fixture. | Rust remains authoritative |
| Evidence gates | Extends native and candidate B2 verifiers with the map-literal pair. | Provisional corpus evidence only |
| Ownership | Adds `BOOT-031` to the bootstrap ledger. | Candidate-owned evidence; not compiler ownership |
| Test reliability | Reads test HTTP responses through declared `Content-Length` rather than waiting for socket EOF. | Test harness only; no production networking behavior change |
| Documentation | Updates English and Burmese contracts, matrices, current status, roadmap, and release notes. | Broader inference and self-hosting remain deferred |

## Verification contract

The candidate must pass the native and Zap candidate B2 verifiers, malformed-source safety, native tests, typecheck matrix parity, specification ownership, Markdown links, VS Code packaging, formatting, release-version validation, documentation consistency, and the exact committed release preflight. The public workflow must independently pass source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts must pass checksum, manifest, provenance, and detached-signature verification.

## Deferred scope

General map-literal inference, arbitrary map keys and values, nested maps, collection/map inference beyond the existing bounded corpus, compound guards, loop mutation, reassignment invalidation, aliases, arbitrary control flow, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Bootstrap boundary

Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner. The Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` remains provisional and corpus-limited. This candidate does not claim a fully Zap-only, self-hosted, B1, B2, B3, or B4 compiler.

## References

[1]: RELEASE_ROLLBACK_RUNBOOK_EN.md
[2]: ../bootstrap/contracts/OWNERS.tsv
[3]: ../bootstrap/fixtures/typecheck/map_annotation.zp
[4]: ../bootstrap/fixtures/typecheck/map_annotation_incompatible.zp
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[7]: ../native/src/evaluator.rs
