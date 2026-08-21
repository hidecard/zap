# Zap v2.1.6 Release Notes

**Release date:** 2026-08-21  
**Release tag:** `v2.1.6`

## Overview

Zap v2.1.6 is a hardening release focused on type-checking conformance, diagnostic consistency, reproducible release tooling, and cross-platform CI reliability. It promotes the v2.1.5 preparation work into a versioned release candidate only after source, packaging, signing, and publication checks pass.

## Type-checking and diagnostics

The release documents and gates the verified TC-001 through TC-012 conformance baseline. Loop-boundary narrowing, conditional-expression typing, alias and wrapper narrowing, generic collection and variant annotation validation, stable JSON diagnostics, and CLI/LSP diagnostic agreement are covered by named test and CI gates.

The CLI and LSP use the shared source-diagnostic bridge for `TypeError` code, normalized messages, and source locations. Dedicated regression coverage protects diagnostic parity while preserving legacy lint-line behavior.

## Toolchain and CI hardening

The repository pins Rust `1.75.0` with the `rustfmt` and `clippy` components. Strict Clippy validation runs with `-D warnings`, and the CI quality job runs formatting, Clippy, Cargo check, native tests, conformance gates, and diagnostic-parity tests before any release build can start.

The release workflow validates Linux x86_64, macOS ARM64, and Windows x86_64 builds independently. Each target runs native tests, CLI smoke tests, installer checks, archive-content checks, and reproducibility checks.

## Release engineering

The release process uses the dry-run-first `scripts/bump_release.sh` helper, tag-gated `scripts/release_preflight.sh`, deterministic archive packaging, per-artifact SHA-256 sidecars, an aggregate checksum file, `zap.release-manifest.v1`, detached GPG signatures, `zap.provenance.v1`, and post-publication verification.

Unix archives are created with the repository-owned deterministic tar.gz helper. Windows ZIP entries use deterministic ordering and the ZIP-supported minimum timestamp. The published-release verifier consumes complete archive listings safely under `pipefail` and checks archive contents, checksums, manifest/provenance consistency, signatures, and the published signing key.

## Supported targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.6-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.6-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.6-windows-x86_64.zip` |

## Release verification

After downloading all release assets and the public verification key, import the key into an isolated GPG keyring and run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.6 ./published-release
```

The command must report `published release verification: PASSED`. Do not install or redistribute an asset when any checksum, signature, provenance, archive-content, or installer check fails.

## Upgrade and rollback

Keep the previous stable release until installation and CLI smoke testing have completed successfully. If a release check fails, quarantine the release and follow `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` or `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md`. Never reuse a tag for different bytes.

## Operational boundaries

The repository does not contain production private keys, passphrases, registry secrets, certificates, or infrastructure credentials. The protected release environment must provide `ZAP_RELEASE_GPG_PRIVATE_KEY` and, when required, `ZAP_RELEASE_GPG_PASSPHRASE`. Key distribution, rotation, release approval, and incident communication remain authorized-operator responsibilities.

## Documentation

The English and Burmese release notes are maintained as a pair. Related references include the type-check conformance matrices, `docs/RELEASE_SIGNING_EN.md`, `docs/RELEASE_SIGNING_MM.md`, deployment documentation, and the v2.1 roadmap documents.
