# Zap v2.2.4 Release Notes

Zap v2.2.4 is a post-v2.2.3 documentation-baseline maintenance release. It records the fresh audit corrections for active language-specification metadata and the generic type-check release-gate reference, synchronizes the current README and release surfaces, and introduces no parser, runtime, or generic-syntax behavior changes.

## Release provenance

The v2.2.4 release is built from the `master` history after the published v2.2.3 release. The v2.2.0 tag and release remain immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb); v2.2.1 remains immutable at [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784); v2.2.2 remains immutable at [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698); and v2.2.3 remains immutable at [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8). Their tags, releases, signed assets, checksums, provenance, and release notes are not rewritten.

## Active specification baseline

The English and Burmese language specifications now identify v2.2.4 as the active normative documentation baseline. The underlying syntax, typing, runtime, diagnostics, compatibility, and version contracts are unchanged; this correction removes stale active metadata discovered during the post-v2.2.3 audit.

## Generic type-check gate

The bilingual generic type-check decision records now identify the v2.2.4 release gate for the already implemented TC-012 baseline. Supported collection and variant annotations remain unchanged. User-defined generic declarations, advanced inference, and any broader generic parser syntax remain explicitly deferred and are not introduced by this release.

## README and release-surface synchronization

The English and Burmese READMEs now identify v2.2.4 as the current release, point installation links to the v2.2.4 release, and list the v2.2.4 Linux, macOS ARM64, and Windows archives. `SECURITY.md`, both type-check conformance matrices, and both checked-in VS Code package manifests are synchronized to the same active release line. Historical v2.2.0 through v2.2.3 references remain explicit historical provenance rather than current-installation targets.

## Compatibility and language boundary

This release preserves the existing Zap language and package contracts. It does not add traits, interfaces, composition, parser syntax, runtime syntax, public weak references, automatic cycle collection, or a tracing collector. The runtime remains single-threaded `Rc`/`RefCell` infrastructure; async retains eager scheduled-value behavior rather than lazy continuation; and LSP remains full-text synchronization with rejected range edits and file-local rename only. No production reactor or multi-thread runtime is claimed.

## Verification and reproducibility

The audited v2.2.3 baseline passed the complete release preflight after the documentation corrections: strict formatting and Clippy, the serial native unit and integration suites, bilingual documentation consistency, specification ownership validation, standard-library policy, LSP protocol and semantic-parity checks, VS Code package validation, parity/replay/async matrices, benchmarks, packaging and archive manifests, signing and registry policy harnesses, release-version validation, and `git diff --check`. The v2.2.4 candidate must pass the same gates. `Cargo.lock` is not regenerated or updated; only the `zap-native` package version stanza is manually synchronized from 2.2.3 to 2.2.4.

## Historical release preservation

The post-v2.2.3 audit found no production panic-capable calls before Rust test modules and no actionable runtime or tooling defect beyond the stale active documentation references corrected in the maintenance commit [`5cf2682`](https://github.com/hidecard/zap/commit/5cf2682dd14e62f13a0edba6df9718d76e83459e). Deferred roadmap items remain deferred architecture or governance scope and are not silently represented as completed implementation.

## Upgrade guidance

Download the archive matching the target platform from the [v2.2.4 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.4), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). The published [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3), [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2), [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), and historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) remain available without modification.

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/commit/5cf2682 "Fix stale active documentation baselines"
