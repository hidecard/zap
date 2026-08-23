# Zap v2.2.6 Release Notes

Zap v2.2.6 is a post-v2.2.5 non-framework core-reliability maintenance release. It hardens workspace line-I/O confinement, bounded synchronous operations, strict locked-build validation, URL-port parsing, cycle-safe test discovery, process descendant termination, registry-test isolation, and cross-platform editor/test parity. It introduces no parser, runtime language syntax, or Framework/Web/App/IoT behavior.

## Release provenance

The v2.2.6 release is prepared from `master` history after the published v2.2.5 release. The v2.2.0 tag and release remain immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb); v2.2.1 remains immutable at [`201fad4`](https://github.com/hidecard/zap/commit/201fad4c7fbee38e3fabf63bf17d50eb4d70f784); v2.2.2 remains immutable at [`f932e21`](https://github.com/hidecard/zap/commit/f932e21d190f59e722bf17dfdc214cef75ade698); v2.2.3 remains immutable at [`758d9fa`](https://github.com/hidecard/zap/commit/758d9faf04154721788016937b0963bd9d0872a8); v2.2.4 remains immutable at [`00d2847`](https://github.com/hidecard/zap/commit/00d2847eaf149821c88f1ed060085972eca993b2); and v2.2.5 remains immutable at commit [`e5f3ea7`](https://github.com/hidecard/zap/commit/e5f3ea7195d4b8bb1e3c38c4618be834bf50c558). Their tags, releases, signed assets, checksums, provenance, and release notes are not rewritten.

## Active runtime reliability baseline

Filesystem line builtins now use the same context-aware workspace confinement as text I/O in both the canonical AST path and the compatibility-only legacy path. The runtime rejects traversal, outside-workspace absolute paths, and symlink-resolved paths that leave the active workspace while retaining existing file-size and capability limits. Synchronous and async process timeouts/cancellation now place children in an isolated process group on Unix, use the explicit `kill -KILL -- -PID` form required for negative group identifiers, and request recursive tree termination through the platform process utility on Windows, followed by direct-child cleanup and wait. Focused cancellation and direct process-group regressions cover this boundary.

## Bounded operations and URL handling

`sleep` and `pow` share explicit bounded policies across canonical and legacy execution. Sleep rejects negative or over-limit durations, while exponentiation uses checked exponentiation-by-squaring and stable overflow/limit diagnostics rather than unbounded repeated multiplication. URL parsing rejects malformed, empty, and out-of-range authority ports while preserving valid host-only and bracketed IPv6 forms. These are defensive limits and correctness fixes; they do not add public syntax.

## CLI, project, and test-harness correctness

`build --locked` now requires a valid existing lockfile through the strict project-validation path, while ordinary non-locked build/check/install behavior remains unchanged. Test discovery canonicalizes real directories once, skips symlink directory entries, preserves deterministic ordering, and avoids symlink loops. The registry security-test fixtures use one shared environment guard, and the Windows legacy path regression encodes backslashes according to Zap string-literal rules. The standard-library catalog and both mirrored VS Code grammars now cover the same builtin set, including `sleep`.

## README and release-surface synchronization

The English and Burmese READMEs, navigation hubs, syntax and language references, runtime and memory records, standard-library policy, type-check matrices, learner guides, TODO registers, security metadata, changelogs, VS Code manifests, and release-note links are synchronized to the v2.2.6 active line. Historical v2.2.0 through v2.2.5 references remain explicit historical provenance rather than current-installation targets. No newly authored release document adds an author attribution.

## Compatibility and framework boundary

This release preserves the existing Zap language and package contracts. It does not add traits, interfaces, composition, generic declaration syntax, broad async syntax, public weak references, automatic cycle collection, a tracing collector, a production reactor, a multi-thread language runtime, ranged LSP changes, cross-file rename, or any Web/App/IoT host or adapter. The runtime remains single-threaded `Rc`/`RefCell` infrastructure; async retains eager scheduled-value behavior; and LSP remains full-text synchronization with rejected range edits and file-local rename only. Framework planning is intentionally deferred to a separate future branch.

## Verification and dependency advisory status

The v2.2.6 release passed strict formatting, warnings-denied Clippy, locked compilation, the complete native unit and integration suites, filesystem/process/network/security corpora, project and lockfile tests, LSP and VS Code parity, documentation consistency, standard-library policy, ownership/parity/replay/async matrices, packaging, and clean tagged-name release preflight. The release preflight and tag-triggered release workflow enforce a modern RustSec audit through `scripts/check_rustsec_audit.sh` and `RUN_CARGO_AUDIT=1`; the audit does not ignore advisories or mutate the lockfile. The released locked graph is `ureq 2.12.1`, `url 2.5.8`, `idna 1.1.0`, `rustls 0.23.40`, `rustls-webpki 0.103.15`, `rcgen 0.13.2`, and development-only `time 0.3.47`. The strict `cargo-audit 0.22.2` scan covered 87 locked crate dependencies and reported zero unresolved advisories. Because `time 0.3.47` requires Rust 1.88.0, the released source and CI quality job use Rust 1.88.0; this changes the build toolchain only, not the Zap language contract. The release workflow [32638479414](https://github.com/hidecard/zap/actions/runs/32638479414) published v2.2.6 from tagged commit [`d1d6816`](https://github.com/hidecard/zap/commit/d1d6816d7d39198b4a9778d531e29cd7b4e1f38a), and independent checksum/signature verification passed for all three platform archives and their signed release metadata.

## Historical release preservation

The v2.2.6 maintenance work preserves all prior release tags and published assets. The source changes are limited to bounded reliability, project validation, test-discovery safety, process cleanup, registry-test determinism, catalog/editor parity, and bilingual documentation. Future AST/typed-IR checker redesign, syntax-aware formatter/linter expansion, race-resistant descriptor-relative filesystem APIs, complete DNS-to-connection pinning, and universal descendant cleanup remain explicit follow-up boundaries unless separately implemented and verified without changing the preserved language contract.

## Upgrade guidance

After publication, download the archive matching the target platform from the [v2.2.6 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.6), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). The published [v2.2.5 release](https://github.com/hidecard/zap/releases/tag/v2.2.5), [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4), [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3), [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2), [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), and historical [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0) remain available without modification.

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/releases/tag/v2.2.2 "Zap v2.2.2 release"
[4]: https://github.com/hidecard/zap/releases/tag/v2.2.3 "Zap v2.2.3 release"
[5]: https://github.com/hidecard/zap/releases/tag/v2.2.4 "Zap v2.2.4 release"
[6]: https://github.com/hidecard/zap/releases/tag/v2.2.5 "Zap v2.2.5 release"
[7]: https://github.com/hidecard/zap/commit/cf614e2 "Fix Windows legacy path fixture"
[8]: https://github.com/hidecard/zap/commit/0b0e276 "Core hardening maintenance"
[9]: https://github.com/hidecard/zap/commit/d5c2cde "Align grammar with cataloged sleep builtin"
