# Zap v2.11.16

**Release status:** Published after complete validation and public artifact/signature verification. Zap remains B0.

## Summary

Zap v2.11.16 publishes one provisional, corpus-limited B2 type-checker increment for an exact direct option-constructor annotation shape. The published candidate recognizes the exact expression `some(1)` as `option<number>` and accepts `let selected: option<number> = some(1)`. A paired negative fixture rejects assigning the same direct expression to `text` with the stable diagnostic `variable 'wrong' expects text, got option<number>` at line 1, column 1.

This is evidence for one deterministic fixture pair only. It does not implement general option-constructor inference, arbitrary constructor payloads, result constructors, aliases, variant narrowing, collection expression inference, or complete static type checking.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Adds exact `some(1)` → `option<number>` recognition and direct mismatch evidence. | No general option or result constructor inference |
| Fixtures | Adds paired positive and incompatible assignment fixtures. | One exact expression and one direct annotation shape |
| Differential gates | Extends native and candidate B2 verifiers to 26 deterministic output cases. | Rust remains the reference owner |
| Ownership | Records provisional `BOOT-032`. | Published evidence remains corpus-limited |
| Documentation | Updates English and Burmese contracts, matrices, current status, roadmap, and release notes. | Broader inference and self-hosting remain deferred |

## Verification contract

The published source passed the native and Zap candidate B2 verifiers, malformed-source safety, native tests, typecheck matrix parity, specification ownership, Markdown links, VS Code packaging, formatting, release-version validation, documentation consistency, and the exact committed release preflight. The public workflow passed source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts passed checksum, manifest, provenance, and detached-signature verification.

## Deferred scope

General option-constructor inference, arbitrary payload expressions, `ok`/`err` result constructors, option/result aliases, variant narrowing, nested maps, collection inference beyond the existing bounded corpus, compound guards, loop mutation, reassignment invalidation, arbitrary control flow, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.
