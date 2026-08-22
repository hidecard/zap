# Zap v2.2.2 Release Notes

Zap v2.2.2 is a corrective maintenance release following the published v2.2.1 release. It packages the remaining audited core-runtime safety work, canonical AST compatibility repairs, standard-library catalog synchronization, editor grammar parity, and bilingual documentation updates completed on `master`.

## Release provenance

The v2.2.2 release is built from the post-v2.2.1 `master` history. The historical v2.2.0 tag and release remain immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb), together with their signed assets, checksums, provenance, and release notes. The published v2.2.1 tag and release also remain unchanged; no historical tag or asset was rewritten.

## Runtime borrow and cycle safety

Canonical AST execution now uses checked `EnvFrame` operations at active lexical-frame boundaries. A frame that is already borrowed returns the stable `BorrowError` result instead of panicking through an unchecked `RefCell` path. Strong `Rc` object and capture cycles remain supported, but their cleanup responsibility is explicit through `clear_object_fields()`; `memory_stats()` reports `cycle_policy=explicit_clear_object_fields`. No public weak-reference API or automatic tracing collector is introduced.

## Canonical helper compatibility

The canonical AST dispatcher now restores the already documented `assert`, `sort`, and `sqrt` helpers that were missing from the native execution path. `assert` reports deterministic expected/observed failures, `sort` returns an ascending clone of a number-only or text-only list, and `sqrt` accepts a non-negative integer and returns its rounded integer square root. The standard-library catalog and bilingual policy/index metadata now cover the complete 76-builtin editor surface.

## VS Code grammar and catalog parity

Both the canonical `vscode-extension/` package and the `editors/vscode/` mirror now highlight the complete public builtin catalog, including `assert`, `sort`, and `sqrt`. The package validation confirms metadata, grammar parity, catalog coverage, archive integrity, and exclusion of generated or VCS entries. The extension manifest is versioned at 2.2.2.

## Documentation and traceability

The English and Burmese README, release, standard-library, typecheck, runtime, memory, roadmap, policy, and security surfaces identify v2.2.2 as the active verified release. The documentation retains explicit v2.2.0 and v2.2.1 provenance and states that the cycle policy is explicit cleanup rather than automatic collection. No new parser, runtime, or traits syntax is introduced.

## Compatibility and known limitations

This release preserves the existing Zap language and package contracts while repairing the canonical execution path and hardening checked borrow boundaries. Async functions retain the documented eager scheduled-value behavior and are not lazy. LSP synchronization remains full-text only with `change: 1`, rename remains file-local, the runtime remains single-threaded `Rc`/`RefCell` infrastructure, and traits/composition remains a design-only RFC with unsupported syntax. Public weak references, automatic cycle collection, and tracing collection remain unsupported or deferred.

## Verification and reproducibility

The release candidate passed the complete serial native suite with 229 unit tests and 256 integration tests, strict Clippy with warnings denied, formatting, LSP protocol synchronization, LSP semantic parity, the canonical VS Code package contract, standard-library policy, parity/replay/async matrix, benchmark, packaging, documentation consistency and regression harnesses, specification ownership, and `git diff --check`. GitHub Actions run [`32584437606`](https://github.com/hidecard/zap/actions/runs/32584437606) completed successfully for the grammar-synchronized state. Cargo.lock was not regenerated or updated; only the `zap-native` package version stanza was patched from 2.2.1 to 2.2.2.

## Upgrade guidance

Download the archive matching the target platform from the [v2.2.2 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.2), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). The published [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1) and historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) remain available without modification.

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/commit/73a1fb5840af4e36789f9572078b0215282291ea "Checked EnvFrame borrows and explicit cycle policy"
[4]: https://github.com/hidecard/zap/commit/4db20741a34100c99cacca1811eea551b2040ce5 "Builtin grammar catalog synchronization"
