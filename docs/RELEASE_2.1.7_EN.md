# Zap v2.1.7 Release Notes

**Release date:** 2026-08-21  
**Release tag:** `v2.1.7`

## Overview

Zap v2.1.7 is a reliability and release-engineering patch release. It expands the bilingual specification ownership index to 27 stable rule IDs, strengthens P0/P1 contract validation, preserves the repaired Windows and macOS cross-platform behavior, and makes the release preflight run ownership, native/legacy parity, fixed-seed replay, and focused async gates before deployment validation.

## Specification ownership and compatibility

The canonical English and Burmese language specifications now link to a machine-readable rule-to-section-to-fixture index. The index covers source execution, precedence, typing, functions, modules, memory, deterministic and production async boundaries, diagnostics, registry, lockfiles, JSON/filesystem limits, standard-library catalog, CLI JSON, compatibility policy, and CI enforcement. The validator rejects missing bilingual sections, missing fixture owners, duplicate IDs, invalid policy values, and missing required domains.

The release adds bilingual compatibility/deprecation change templates. Future normative, compatibility, deprecated, or rejected behavior must identify its canonical sections, fixture owner, migration path, version impact, and verification evidence rather than relying on legacy acceptance alone.

## Verification and CI hardening

The fixed-seed P1-05 replay layer retains durable parser, JSON, lockfile, registry, memory, and async failure fixtures with deterministic evidence. The P0-01 native/legacy matrix compares six versioned common, native-only, and rejected fixtures with normalized output digests. The P0-05 focused async matrix continues to run process, file, socket, deadline, cancellation, and output-limit cases on Linux x86_64, Windows x86_64, and macOS ARM64.

The repository pins Rust `1.75.0` with `rustfmt` and `clippy`. Strict Clippy validation runs with `-D warnings`, and CI runs formatting, Cargo check, native tests, conformance gates, ownership validation, parity, replay, async matrix, and deployment-policy validation before release builds.

## Release engineering

The release process uses the dry-run-first `scripts/bump_release.sh` helper and tag-gated `scripts/release_preflight.sh`. The preflight now runs the four P0/P1 contract gates before deployment validation, followed by formatting, strict Clippy, Cargo check/test when enabled, bilingual documentation checks, target validation, source safety, and repository cleanliness. Release artifacts continue to use deterministic archives, per-artifact SHA-256 sidecars, aggregate manifests, provenance, detached signatures, and post-publication verification.

## Supported targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.7-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.7-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.7-windows-x86_64.zip` |

## Release verification

After downloading all release assets and the public verification key, import the key into an isolated GPG keyring and run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.7 ./published-release
```

The command must report `published release verification: PASSED`. Do not install or redistribute an asset when any checksum, signature, provenance, archive-content, or installer check fails.

## Upgrade and rollback

Keep v2.1.6 available until installation and CLI smoke testing for v2.1.7 have completed successfully. If a release check fails, quarantine the release and follow [`RELEASE_ROLLBACK_RUNBOOK_EN.md`](RELEASE_ROLLBACK_RUNBOOK_EN.md) or [`RELEASE_ROLLBACK_RUNBOOK_MM.md`](RELEASE_ROLLBACK_RUNBOOK_MM.md). Never reuse a tag for different bytes.

## Deferred boundaries

Executor-backed language scheduling, language-level async cancellation/timeout syntax, public weak references, tracing collection, long-running fuzz targets, allocator-level telemetry, and the remaining fragmented specification ownership work remain explicitly deferred. This release does not begin traits/composition implementation or broad async syntax.

## Operational boundaries

The repository does not contain production private keys, passphrases, registry secrets, certificates, or infrastructure credentials. The protected release environment must provide `ZAP_RELEASE_GPG_PRIVATE_KEY` and, when required, `ZAP_RELEASE_GPG_PASSPHRASE`. Key distribution, rotation, release approval, and incident communication remain authorized-operator responsibilities.

## Documentation

The English and Burmese release notes are maintained as a pair. Related references include [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv), [`SPEC_OWNERSHIP_EN.md`](SPEC_OWNERSHIP_EN.md), [`SPEC_OWNERSHIP_MM.md`](SPEC_OWNERSHIP_MM.md), the type-checking conformance matrices, release-signing documentation, deployment documentation, and the v2.1 roadmap documents.
