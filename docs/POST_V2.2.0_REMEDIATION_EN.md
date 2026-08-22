# Post-v2.2.0 Remediation and Provenance Record

**Record status:** v2.2.1 corrective release published; subsequent engineering queue remains tracked separately

## Scope and provenance

This record documents the corrective-release cycle initiated after the attached deep technical review of Zap v2.2.0. It is intentionally separate from the historical v2.2.0 release notes. The published **v2.2.0 tag remains immutable** at commit [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb), and its successful release workflow is recorded at [GitHub Actions run 32546657968](https://github.com/hidecard/zap/actions/runs/32546657968). No later correction is being backported into, force-pushed onto, or represented as part of that historical tag.

The current `master` branch contains subsequent corrective commits. Those changes were packaged and published as **v2.2.1**. Users who install the v2.2.0 archives receive the v2.2.0-tagged behavior, while the v2.2.1 release contains the corrective behavior described below.

| Provenance boundary | Commit or record | Meaning |
|---|---|---|
| Published v2.2.0 | [`7a2269b`](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb) and [release v2.2.0](https://github.com/hidecard/zap/releases/tag/v2.2.0) | Historical release assets, checksums, provenance, and release notes; unchanged. |
| LSP synchronization correction | [`c2a662f`](https://github.com/hidecard/zap/commit/c2a662f) | Standard full-document `didChange`, version tracking, accepted-buffer diagnostics, and safe rejection of unsupported range changes. |
| Semantic rename correction | [`eed2dc4`](https://github.com/hidecard/zap/commit/eed2dc4) | File-local lexical binding resolution with shadowing, closures, parameters, and import aliases. |
| LSP interoperability correction | [`cdf2aa1`](https://github.com/hidecard/zap/commit/cdf2aa1) | Negotiated position encoding, strict file URIs, workspace bounds, and encoding-aware ranges. |
| VS Code delivery correction | [`f77f265`](https://github.com/hidecard/zap/commit/f77f265) | Canonical extension source, package validation, catalog-aligned assets, and a native-LSP rename provider. |
| Standard-library determinism correction | [`2c4c928`](https://github.com/hidecard/zap/commit/2c4c928) | Schema-2 `determinism_class` taxonomy, legacy-boolean compatibility, explicit builtin exceptions, and bilingual policy updates. |

## Corrective milestones

| Milestone | Status | Corrected contract and evidence |
|---|---|---|
| LSP-SYNC-01 | Implemented | The server advertises `textDocumentSync` with `openClose: true` and `change: 1`. It stores versioned document text, consumes standard full-text `params.contentChanges`, preserves accepted state against stale updates, and publishes diagnostics from that state. Incremental range changes are rejected rather than approximately applied. Unit tests and `scripts/test_lsp_protocol_sync.sh` cover the wire-level behavior. |
| LSP-REN-01 | Implemented | Rename resolves the nearest lexical declaration within the active file for functions, classes, modules, `let`, `for`, `catch`, parameters, nested closures, and import aliases. Comments, strings, member names after `.`, keywords, and catalog builtins are excluded. Cross-file rename remains intentionally unsupported. |
| LSP-INTEROP-01 | Implemented | Position encoding is negotiated as UTF-8, UTF-16 by default, or UTF-32. File URIs reject malformed escapes, URI hosts, NUL bytes, and decoded traversal. Workspace indexing is bounded to 256 documents, 32 import levels, and 32 MiB of source text. |
| EXT-201 | Implemented | `vscode-extension/` is the canonical distributable source. Its manifest, grammar, configuration, catalog coverage, LSP content-change behavior, rename provider, and `.vsix` archive layout are checked by the package contract. `editors/vscode/` remains the catalog-aligned static asset mirror. |
| API-301 | Implemented | The catalog schema is version 2. `DeterminismClass` distinguishes `pure`, `input-deterministic`, `runtime-dependent`, and `external-io`. The schema-version-1 `deterministic` boolean is retained as a compatibility view: it is true only for the first two classes. All twelve domains and all cataloged builtins have explicit, regression-tested classifications, including builtin-level exceptions for path/log builders, URL transforms, duration transforms, clock access, and environment/configuration access. |
| DOC-401 | This record | This bilingual record, the roadmap/progress wording, navigation links, and README status text distinguish the immutable v2.2.0 release from post-release master corrections. |

## Public contract and limitations

The corrected LSP contract is deliberately bounded. It supports standard full-text synchronization and version-aware state, but it does not support incremental range application. It provides file-local semantic rename, but it does not provide cross-file rename. It negotiates UTF-8, UTF-16, or UTF-32 position columns and enforces strict file-URI and workspace-size boundaries. These limitations are normative and must remain visible in the README, the English/Burmese LSP guide, and the v2.2.1 release notes.

The canonical VS Code package delegates rename to the native LSP and packages the checked-in grammar and configuration. The repository package smoke test proves metadata, source coverage, archive integrity, and provider wiring; it is not a claim that every external VS Code host or Marketplace installation has been exercised. The extension version remains tied to the Cargo-authoritative release version.

The standard-library catalog now reports a determinism class rather than making a single coarse claim. Pure and input-deterministic transformations are distinguished from operations that depend on runtime state or external I/O. The traits/composition proposal remains design-only; this corrective cycle does not add trait, interface, composition, or method-resolution parser/runtime implementation.

## Verification and release policy

Each corrective milestone is required to pass the pinned Rust formatting check, strict Clippy, the complete native test suite, focused LSP protocol and semantic-parity tests, the canonical VS Code package contract, the standard-library policy contract, release-version validation, bilingual documentation consistency, specification ownership, and `git diff --check` before its focused commit is pushed. API-301 passed these gates while the authoritative version remained 2.2.0, so the taxonomy commit is a post-release master correction rather than a modification of the v2.2.0 artifact.

The v2.2.1 release was prepared from the clean commit with Cargo, the manually patched package lock entry, CLI output, both VS Code manifests, changelogs, bilingual README/archive links, security metadata, documentation, and release notes in agreement. Release preflight ran with `EXPECTED_VERSION=2.2.1` and the GitHub Actions workflow verified all platform, signing, checksum, provenance, and publication conditions. The v2.2.0 tag and release assets remained untouched throughout that process.

## Next release boundary

The published v2.2.1 notes summarize the corrected LSP synchronization, file-local rename, position/URI/workspace bounds, canonical VS Code package, and determinism taxonomy. They state the full-sync-only and file-local-only limitations and link back to this record. v2.2.1 is a new patch release; v2.2.0 was not retagged or rewritten.

## References

1. [Published v2.2.0 tag](https://github.com/hidecard/zap/releases/tag/v2.2.0) and [immutable tag commit](https://github.com/hidecard/zap/commit/7a2269bfb70863608156484453576cbbe4376deb).
2. [Successful v2.2.0 release workflow](https://github.com/hidecard/zap/actions/runs/32546657968).
3. [LSP synchronization contract](ASYNC_LSP_EN.md) and [protocol regression harness](../scripts/test_lsp_protocol_sync.sh).
4. [Standard-library stability and determinism policy](STDLIB_POLICY_EN.md).
5. [v2.2.0 historical release notes](RELEASE_2.2.0_EN.md); this record does not rewrite them.
6. [Published v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), [v2.2.1 release workflow](https://github.com/hidecard/zap/actions/runs/32575824809), and [v2.2.1 release notes](RELEASE_2.2.1_EN.md).
