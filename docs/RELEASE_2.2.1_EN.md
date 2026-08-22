# Zap v2.2.1 Release Notes

Zap v2.2.1 is the corrective patch release following the published v2.2.0 release. It packages the post-release reliability, LSP interoperability, editor-delivery, standard-library metadata, and documentation corrections completed on `master`.

## Release provenance

The v2.2.1 release is built from the corrected post-v2.2.0 `master` history. The published v2.2.0 tag remains immutable at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb), together with its signed assets, checksums, provenance, and release notes. No v2.2.0 tag or asset was rewritten to contain the later fixes.

## LSP document synchronization

The native LSP server now advertises and implements standard full-document synchronization. `initialize` reports `textDocumentSync` with `openClose: true` and `change: 1`; `didOpen` and `didChange` retain the accepted document text and version; and diagnostics are produced from the accepted buffer. Stale versioned updates are ignored without replacing newer state. Incremental range changes are rejected safely because v2.2.1 intentionally exposes full-sync only.

## File-local semantic rename

Rename now resolves lexical bindings within the active document instead of changing every same-spelled lexer token. Declarations and references for functions, classes, modules, `let`, `for`, `catch`, parameters, nested closures, and imported aliases are resolved with nearest-scope shadowing. Strings, comments, keywords, catalog builtins, and member names after `.` are excluded. Cross-file rename remains unsupported and returned edits are limited to the active URI.

## LSP interoperability and bounds

The server negotiates UTF-8, UTF-16, or UTF-32 position encoding from the client capability list, defaulting to UTF-16 when no preference is provided. File URIs use strict percent-decoding and reject malformed escapes, URI hosts, NUL bytes, and traversal segments. Workspace indexing enforces bounded document count, import depth, and total workspace bytes so oversized or unsafe inputs are skipped or rejected without evicting accepted document state.

## Canonical VS Code extension delivery

`vscode-extension/` is the canonical distributable source for the Zap extension. Its manifest is versioned at 2.2.1, retains publisher `ArkarYan`, delegates rename to the native LSP, and is checked by the package smoke contract. `editors/vscode/` remains the catalog-aligned static grammar and configuration mirror. The package contract validates metadata, grammar/configuration parity, catalog builtin coverage, archive integrity, and exclusion of generated or VCS entries.

## Standard-library determinism taxonomy

The standard-library catalog uses schema version 2 and exposes `determinism_class` for every public domain and builtin. The public labels are `pure`, `input-deterministic`, `runtime-dependent`, and `external-io`. Domain defaults and reviewed builtin exceptions are explicit, including pure builders, input-dependent transforms, clock/environment access, and network/process operations. The legacy `deterministic` boolean remains available as a compatibility view: it is true only for `pure` and `input-deterministic` entries.

## Documentation and traceability

English and Burmese policy, index, roadmap, TODO, progress, navigation, README, and changelog surfaces describe the same correction boundary. The [post-v2.2.0 remediation record](POST_V2.2.0_REMEDIATION_EN.md) identifies which commits landed after the immutable v2.2.0 tag and which limitations remain. The release notes and documentation consistency harness preserve bilingual section and code-fence parity.

## Compatibility and known limitations

This release preserves the existing Zap language and package contracts while making the corrective metadata and editor behavior explicit. LSP synchronization is full-text only; incremental range edits are not applied. Rename is file-local; cross-file edits are not returned. The async runtime remains a deterministic, bounded foundation rather than a full production reactor, and multi-thread-safe task state and external production deployment controls remain outside this patch’s scope. Traits and composition remain design-only and unsupported syntax.

## Verification and reproducibility

The release candidate passed the pinned Rust formatting check, strict Clippy with warnings denied, the complete native unit and integration suite, the wire-level LSP synchronization probe, LSP semantic parity, the canonical VS Code package contract, the standard-library policy harness, Cargo-authoritative release-version validation, bilingual documentation consistency and regression tests, specification ownership validation, and `git diff --check`. Cargo.lock was not regenerated or updated; only the `zap-native` package version stanza was patched from 2.2.0 to 2.2.1.

## Upgrade guidance

Download the archive matching the target platform from the [v2.2.1 GitHub release](https://github.com/hidecard/zap/releases/tag/v2.2.1), verify its published checksum and signature/provenance information, and follow the installation instructions in the [English README](../README.md) or [Burmese README](../README_MM.md). Users who need the historical v2.2.0 artifacts can continue to use the immutable [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0).

## References

[1]: https://github.com/hidecard/zap/releases/tag/v2.2.0 "Zap v2.2.0 release"
[2]: https://github.com/hidecard/zap/releases/tag/v2.2.1 "Zap v2.2.1 release"
[3]: https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb "Zap v2.2.0 tag commit"
