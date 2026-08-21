# Zap v2.1.1 Release Notes

**Release date:** 2026-08-21
**Release tag:** `v2.1.1`

## Overview

Zap v2.1.1 is the first protected v2.1 release built around the completed v2.1-D runtime/tooling work and the v2.1-E release-engineering pipeline. The release combines production-oriented asynchronous execution, bounded I/O adapters, registry service controls, synchronized developer tooling, reproducible artifacts, signed metadata, provenance, and post-publish verification.

## Runtime and language

The release includes structured asynchronous execution with joinable tasks, deterministic tick-based timeout behavior, cancellation-aware propagation, task readiness checks, typed task failure handling, and the language-level `spawn`, `task_join`, and `task_is_ready` builtins. Repeated joins and cancellation precedence are covered by regression tests.

The threaded runtime provides bounded asynchronous file reads, non-blocking TCP request/response exchange, and asynchronous process execution with hard deadlines and output limits. Forced child-process cancellation terminates the child and drains its output so that a cancelled operation does not leave an uncontrolled process behind.

## Type-checking and diagnostics

The release records the completed TC-006 through TC-012 conformance baseline, including loop-boundary restoration, option/result alias narrowing, conditional-expression typing, generic collection and variant annotation validation, stable L3 JSON diagnostics, and L4 CLI/LSP diagnostic agreement. The LSP now reuses the shared source-diagnostic bridge and emits the same `TypeError` code, normalized message, and source-location semantics as CLI checking while preserving legacy lint-line behavior.

## Tooling

The Formatter, LSP server, and VS Code extension share the finalized async vocabulary. LSP completion, diagnostics, formatting, signature help, hover, go-to-definition, recursive document symbols, and module-aware package indexing are synchronized for local and unopened files.

## Registry and deployment

Zap includes an authenticated loopback registry service with signed-index persistence, safe path handling, trusted-registry controls, bounded transport behavior, cache verification, and deterministic failure paths. Reference deployment artifacts cover systemd, Nginx TLS termination, environment boundaries, and a machine-readable deployment policy. Production host provisioning, certificates, DNS, WAF/rate limiting, monitoring, and secret-manager setup remain operator responsibilities.

## Release engineering

The release pipeline now includes a dry-run-first version/changelog bump helper, tag-gated release preflight, deterministic three-target artifact aggregation, per-artifact and aggregate SHA-256 verification, `zap.release-manifest.v1`, `zap.provenance.v1`, detached GPG signatures, the `zap-2.1.1-release-signing-key.asc` public signing-key asset, and post-publish release verification. This patch release also reconciles the bilingual conformance matrices, roadmap status, and diagnostic release records before publication. Unix archives now use the repository-owned platform-neutral deterministic tar.gz helper, while Windows ZIP entries use the ZIP-supported minimum timestamp for reproducible packaging. A bilingual rollback/quarantine runbook is included for release incidents.

## Supported targets

| Platform | Target | Artifact |
|---|---|---|
| Linux | x86_64 GNU | `zap-2.1.1-linux-x86_64.tar.gz` |
| macOS | ARM64 | `zap-2.1.1-macos-arm64.tar.gz` |
| Windows | x86_64 MSVC | `zap-2.1.1-windows-x86_64.zip` |

## Release verification

After downloading all release assets and the public verification key, import the trusted public key into an isolated GPG keyring and run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.1 ./published-release
```

The command must report `published release verification: PASSED`. Verification checks the complete archive set, per-artifact sidecars, aggregate checksums, manifest/provenance consistency, expected archive entries, and every detached signature. Do not install or redistribute an asset when any check fails.

## Upgrade and rollback

Users should retain the previous stable release until the new installation and CLI smoke test have completed successfully. If a checksum, signature, provenance, installer, registry index, or severe runtime check fails, quarantine the release and follow `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` or `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md`. Do not reuse a tag for different bytes.

## Known operational boundaries

The repository does not contain production private keys, passphrases, registry secrets, certificates, or infrastructure credentials. The protected release environment must provide `ZAP_RELEASE_GPG_PRIVATE_KEY` and, when required, `ZAP_RELEASE_GPG_PASSPHRASE`. Public-key trust distribution, key rotation, release approval, and incident communication require an authorized operator.

## Documentation

The English and Burmese release notes are maintained as a pair. Additional references are available in `docs/RELEASE_SIGNING_EN.md`, `docs/RELEASE_SIGNING_MM.md`, `docs/DEPLOYMENT_EN.md`, `docs/DEPLOYMENT_MM.md`, and the v2.1 roadmap documents.
