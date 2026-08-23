# Zap v2.2.5 Release Notes

Zap v2.2.5 is a post-v2.2.4 non-framework reliability maintenance release. It hardens the production HTTP request path by replacing internal URL-parser invariant `unreachable!` branches with deterministic errors, updates the active English/Burmese release surfaces, and introduces no parser, runtime, language syntax, or framework behavior.

## Release provenance

The v2.2.5 release is built from `master` history after the published v2.2.4 release. The v2.2.0 tag and release remain immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb); v2.2.1 remains immutable at [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784); v2.2.2 remains immutable at [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698); v2.2.3 remains immutable at [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8); and v2.2.4 remains immutable at [`00d2847`](https://github.com/hidecard/zap/commit/00d2847eaf149821c88f1ed060085972eca993b2). Their tags, releases, signed assets, checksums, provenance, and release notes are not rewritten.

## Active runtime reliability baseline

The HTTP request implementation continues to parse absolute URLs through the existing bounded `parse_url` helper and supports only `http` and `https` requests under the existing capability and network-destination checks. If an internal parser-result shape is ever inconsistent, `http_request` now returns a deterministic error for an invalid result or a missing scheme/host instead of terminating through `unreachable!`. Valid request behavior, URL limits, response limits, timeout behavior, and capability enforcement are unchanged.

## HTTP URL invariant handling

The production `http_request` path now handles three internal shape assumptions explicitly: the parser result must be a map, the map must contain a text scheme, and the map must contain a text host. These are defensive invariant boundaries rather than new public syntax or a new URL contract. Malformed user URLs continue to receive the existing typed parse errors, while unexpected internal shapes now fail closed with ordinary runtime errors.

## README and release-surface synchronization

The English and Burmese READMEs now identify v2.2.5 as the current release, point installation links to the v2.2.5 release, and list the v2.2.5 Linux, macOS ARM64, and Windows archives. `SECURITY.md`, both type-check conformance matrices, and both checked-in VS Code package manifests are synchronized to the same active release line. Historical v2.2.0 through v2.2.4 references remain explicit historical provenance rather than current-installation targets.

## Compatibility and framework boundary

This release preserves the existing Zap language and package contracts. It does not add traits, interfaces, composition, parser syntax, runtime syntax, public weak references, automatic cycle collection, a tracing collector, or any Web/App/IoT framework. Framework planning remains deferred to a separate branch after the remaining core maintenance work; no framework branch or framework implementation is part of v2.2.5. The runtime remains single-threaded `Rc`/`RefCell` infrastructure; async retains eager scheduled-value behavior rather than lazy continuation; and LSP remains full-text synchronization with rejected range edits and file-local rename only. No production reactor or multi-thread runtime is claimed.

## Verification and reproducibility

The focused HTTP-hardening validation passed strict formatting, Clippy with warnings denied, and the serial native suite: 232 unit tests and 256 core integration tests. The v2.2.5 release candidate must pass the complete release preflight, including bilingual documentation consistency, specification ownership, standard-library policy, LSP protocol and semantic parity, VS Code package validation, parity/replay/async matrices, benchmarks, packaging, signing-policy, registry-policy, deployment-policy, release-version validation, and `git diff --check`. `Cargo.lock` is not regenerated or updated; only the `zap-native` package version stanza is manually synchronized from 2.2.4 to 2.2.5.

## Historical release preservation

The fresh no-framework audit found no remaining actionable TODO/FIXME marker, no new framework implementation, and no other production `unreachable!` or `todo!` path requiring correction after the HTTP invariant hardening. Remaining roadmap items are intentionally deferred architecture or governance scope and are not silently represented as completed implementation. The focused reliability correction is recorded in commit [`f4470ab`](https://github.com/hidecard/zap/commit/f4470abdcc314311cf759fa023bf497b1bdd2a94).

## Upgrade guidance

Download the archive matching the target platform from the [v2.2.5 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.5), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). The published [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4), [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3), [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2), [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), and historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) remain available without modification.

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/releases/tag/v2.2.4 "Zap v2.2.4 release"
[6]: https://github.com/hidecard/zap/commit/f4470ab "Harden HTTP URL invariants"
