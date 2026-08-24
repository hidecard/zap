# Zap Current Status

**Status label:** active
**Latest published release:** [v2.11.8](https://github.com/hidecard/zap/releases/tag/v2.11.8)
**Next release line:** v2.11.9 preparation
**Bootstrap stage:** B0

> Zap is a Rust reference/native implementation. The Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` is provisional, corpus-limited evidence and does not establish a fully Zap-only or self-hosted compiler.

## Release and provenance

The latest published release is v2.11.8. Its tag and workflow record are immutable release evidence; a later release must use a new tag and must not rewrite prior tags. Each published release includes a versioned manifest, aggregate checksums, detached signatures, and a signed provenance asset.

The versioned provenance asset is the canonical machine-readable record for release identity. It records the source URI, tag/ref, source commit, workflow run ID, reproducible manifest and checksums, signing metadata, and artifact subjects with their SHA-256 digests and sizes. The release verifier checks this record together with the downloaded archives and signatures.

## Active implementation status

| Area | Status label | Current boundary |
|---|---|---|
| Native compiler/runtime | active | Rust remains the reference owner for complete semantics and supported release targets. |
| B0 artifacts | completed | Canonical tokens, AST, diagnostics, metadata, VM, and platform-seed fixtures are reproducible. |
| B1 lexer/parser candidates | provisional | Candidate output is checked only against the owned corpus and does not replace the Rust pipeline. |
| B2 type-checker candidate | provisional | Includes selected declarations, conditionals, functions, calls, bounded list-element diagnostics, a paired nested-list index slice, a bounded text-key map-element slice, and a bounded direct-`is_some` branch-local option-narrowing slice. |
| Typed-IR candidate | provisional | Covers the existing annotated declaration slice only. |
| Malformed-source safety | regression-gated | A small invalid-source corpus must fail nonzero without panic or unchecked-unwrap signatures; this is a safety regression gate, not compiler-ownership evidence. |
| B3 package/build foundations | reference-only | Offline and deterministic foundation checks do not transfer compiler ownership to Zap. |
| B4 self-hosting | deferred | No B4 claim is allowed until self-rebuild acceptance passes. |

## Next bounded work

The v2.11.8 release added a bounded `map<text,number>` element indexed by a text literal and a paired incompatible assignment to the B2 evidence corpus. The current v2.11.9 preparation line adds a bounded direct-`is_some` branch-local narrowing case for a tracked `option<number>` inside one indented `if` body, with a paired incompatible payload assignment. The next roadmap work is to broaden B2 inference and diagnostics through separately evidenced fixtures beyond this bounded branch slice. Malformed-source no-panic behavior remains regression-gated, and candidate typed-IR production will be extended only from the same owned analysis. Generic declarations, nested maps, deeper nested inference, compound guards, loop narrowing, reassignment invalidation, arbitrary program parsing, package/build ownership, VM ownership, and platform-seed self-hosting remain deferred until their acceptance criteria are met.

## Developer environment

Run `make doctor` before local validation. It reports Cargo, Rust, rustup, Python, cargo-audit, the pinned toolchain, host target, and the selected `ZAP_BIN` or built runtime separately. Normal mode reports an incomplete environment without pretending that tests failed; `make doctor` can be followed by `bash scripts/doctor.sh --strict` when all prerequisites are required.

## Status policy

This page is the current-status index. Historical release notes and changelogs remain immutable records and should not be read as current implementation claims. Any behavior change must update the relevant English/Burmese contract, fixture, ownership record, validation gate, and release documentation together.
