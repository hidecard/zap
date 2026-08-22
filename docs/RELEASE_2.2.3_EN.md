# Zap v2.2.3 Release Notes

Zap v2.2.3 is a post-v2.2.2 runtime-reliability release. It packages bounded cycle-safe equality, checked object and lexical-frame borrow propagation, deterministic task and frame invariant fallbacks, an LSP rename scope-stack hardening, and synchronized English/Burmese documentation.

## Release provenance

The v2.2.3 release is built from the `master` history after the published v2.2.2 release. The v2.2.0 tag and release remain immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb); v2.2.1 remains immutable at [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784); and v2.2.2 remains immutable at [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698). Their tags, releases, signed assets, checksums, provenance, and release notes are not rewritten.

## Runtime equality safety

Canonical AST `==` and `!=` now use a checked, bounded `try_values_equal` path. Lists, maps, `Result`, `Option`, and `Future` values compare recursively; object pairs are cycle-guarded; callable values use handle identity. Traversal is bounded by `MAX_RUNTIME_VALUE_NODES`, and an internal borrow conflict returns the typed `BorrowError` result rather than panicking. The infallible `PartialEq` compatibility view remains available and reports `false` when checked comparison cannot complete.

## Checked borrow and invariant hardening

Object and `EnvFrame` borrows in logical memory sizing, validation, and canonical AST member reads now propagate typed borrow failures. Task joining and function/method frame invariant paths use deterministic fallbacks instead of unchecked existence or frame-borrow `expect` paths. These changes harden the existing single-threaded `Rc`/`RefCell` runtime without introducing a new ownership model or automatic cycle collection.

## LSP rename boundary

The LSP rename scope-stack path now fails closed when its internal scope invariant is not available, removing the remaining audited `unwrap` path. Existing protocol boundaries remain unchanged: document synchronization is full-text with `change: 1`, unsupported range edits are rejected, and semantic rename remains file-local rather than cross-file.

## Documentation and traceability

The English and Burmese README, release, memory, diagnostics, runtime-state, type-check, roadmap, policy, security, learner-guide, and specification-ownership surfaces identify v2.2.3 as the active release. The bilingual release notes document the same runtime behavior and limitations. No new parser syntax, runtime syntax, traits implementation, or composition syntax is introduced.

## Compatibility and known limitations

This release preserves the existing Zap language and package contracts. The runtime remains single-threaded `Rc`/`RefCell` infrastructure. Strong reference cycles remain supported but require explicit `clear_object_fields()` cleanup; public weak references, automatic tracing collection, and a tracing collector remain unsupported. Async functions retain eager scheduled-value behavior and are not lazy. LSP synchronization remains full-text only, range changes remain rejected, and rename remains file-local. Traits and composition remain a design-only RFC with unsupported syntax, and no production reactor or multi-thread runtime is claimed.

## Verification and reproducibility

The release candidate is required to pass strict formatting and Clippy, the serial native unit and integration suites, bilingual documentation consistency, specification ownership validation, standard-library policy, LSP protocol and semantic-parity checks, VS Code package validation, parity/replay/async matrices, benchmarks, packaging and archive manifests, signing and registry policy harnesses, release-version validation, the clean-tree release preflight, and `git diff --check`. Cargo.lock is not regenerated or updated; only the `zap-native` package version stanza is manually synchronized from 2.2.2 to 2.2.3.

## Upgrade guidance

Download the archive matching the target platform from the [v2.2.3 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.3), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). The published [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2), [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), and historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) remain available without modification.

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/commit/ed1cb46 "Harden runtime borrows and cyclic equality"
[5]: https://github.com/hidecard/zap/commit/3e58e10 "Harden LSP rename scope stack"
[6]: https://github.com/hidecard/zap/commit/349f68a "Synchronize post-v2.2.2 documentation"

