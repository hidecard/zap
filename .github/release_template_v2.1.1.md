# Zap v2.1.1

## Release summary

Zap v2.1.1 is the first protected v2.1 release prepared with the v2.1-E release-engineering pipeline. It includes the production async/runtime, tooling, registry, bounded I/O, deterministic artifact, signing, provenance, and post-publish verification work completed for this release line.

## Release metadata

- Tag: `v2.1.1`
- Source commit: `<COMMIT_SHA>`
- GitHub Actions release run: `<WORKFLOW_RUN_URL>`
- Release date: `2026-08-21`
- Previous stable release: `<PREVIOUS_VERSION>`

## Highlights

- Structured async runtime with joinable tasks, deterministic timeout behavior, cancellation propagation, and language-level task builtins.
- Completed TC-006 through TC-012 type-checking conformance baseline, stable L3 JSON diagnostics, and L4 CLI/LSP TypeError parity.
- Threaded runtime and bounded non-blocking file, TCP, and process adapters.
- Forced cancellation and deadline-aware child-process execution with output limits.
- Formatter, LSP, document-symbol indexing, package indexing, and VS Code vocabulary synchronization.
- Authenticated loopback registry service with signed-index persistence and safe path handling.
- Deployment reference artifacts for systemd, Nginx, and machine-readable registry policy.
- Deterministic release preflight, version/changelog automation, artifact manifest, aggregate checksums, GPG signatures, provenance, and post-publish verification.

## Breaking changes and migration

<!-- Describe any breaking changes. Write `None` if no migration is required. -->

None known at release preparation time.

## Verification instructions

Download the release assets together with `zap-2.1.1-release-signing-key.asc`, import the public key into an isolated verification keyring, and run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.1 ./published-release
```

The verifier must report `published release verification: PASSED`. Do not install or distribute assets when a checksum, signature, manifest, provenance, or archive-content check fails.

## Supported release targets

| Target | Artifact |
|---|---|
| Linux x86_64 | `zap-2.1.1-linux-x86_64.tar.gz` |
| macOS ARM64 | `zap-2.1.1-macos-arm64.tar.gz` |
| Windows x86_64 | `zap-2.1.1-windows-x86_64.zip` |

## Release assets checklist

- [ ] All three platform archives are attached.
- [ ] Per-artifact `.sha256` sidecars are attached.
- [ ] `zap-2.1.1-checksums.sha256` is attached.
- [ ] `zap-2.1.1-manifest.json` is attached.
- [ ] `zap-2.1.1-provenance.json` is attached.
- [ ] Detached `.asc` signatures are attached for every archive and metadata file.
- [ ] `zap-2.1.1-release-signing-key.asc` is attached.
- [ ] Post-publish verification completed successfully.

## Approval and rollback

- [ ] Release preflight passed on the tagged commit.
- [ ] Strict Clippy, formatting, check, full tests, and target-native tests passed.
- [ ] Protected release-environment approval recorded.
- [ ] Signing secret was supplied through GitHub Actions secrets and was not stored in the repository.
- [ ] Last known-good release and rollback owner are recorded.
- [ ] Rollback runbook reviewed: `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` / `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md`.

## Documentation

- English release notes: `docs/RELEASE_2.1.1_EN.md`
- Burmese release notes: `docs/RELEASE_2.1.1_MM.md`
- English generic syntax decision: `docs/TYPECHECK_GENERIC_DESIGN_EN.md`
- Burmese generic syntax decision: `docs/TYPECHECK_GENERIC_DESIGN_MM.md`
- Signing guide: `docs/RELEASE_SIGNING_EN.md` / `docs/RELEASE_SIGNING_MM.md`
- Deployment guide: `docs/DEPLOYMENT_EN.md` / `docs/DEPLOYMENT_MM.md`

## Known limitations

<!-- List known limitations, operational boundaries, or follow-up work. -->

Production GPG secret provisioning, public-key trust distribution, and operator rollback approval remain protected-environment responsibilities.
