# Zap v<VERSION>

## Release summary

Zap v<VERSION> is a protected release prepared with the repository release-engineering pipeline. Replace the placeholders in this template from the authoritative Cargo package version and the tag-triggered workflow metadata; do not manually copy a version from an older release.

## Release metadata

- Tag: `v<VERSION>`
- Source commit: `<COMMIT_SHA>`
- GitHub Actions release run: `<WORKFLOW_RUN_URL>`
- Release date: `<RELEASE_DATE>`
- Previous stable release: `<PREVIOUS_VERSION>`

## Highlights

- <Summarize the verified implementation and documentation changes.>
- <List compatibility or migration notes, or write `None`.>
- <List cross-platform, replay, parity, ownership, and release-gate evidence when applicable.>

## Breaking changes and migration

<!-- Describe any breaking changes. Write `None` if no migration is required. -->

None known at release preparation time.

## Verification instructions

Download the release assets together with `zap-<VERSION>-release-signing-key.asc`, import the public key into an isolated verification keyring, and run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh <VERSION> ./published-release
```

The verifier must report `published release verification: PASSED`. Do not install or distribute assets when a checksum, signature, manifest, provenance, or archive-content check fails.

## Supported release targets

| Target | Artifact |
|---|---|
| Linux x86_64 | `zap-<VERSION>-linux-x86_64.tar.gz` |
| macOS ARM64 | `zap-<VERSION>-macos-arm64.tar.gz` |
| Windows x86_64 | `zap-<VERSION>-windows-x86_64.zip` |

## Release assets checklist

- [ ] All three platform archives are attached.
- [ ] Per-artifact `.sha256` sidecars are attached.
- [ ] `zap-<VERSION>-checksums.sha256` is attached.
- [ ] `zap-<VERSION>-manifest.json` is attached.
- [ ] `zap-<VERSION>-provenance.json` is attached.
- [ ] Detached `.asc` signatures are attached for every archive and metadata file.
- [ ] `zap-<VERSION>-release-signing-key.asc` is attached.
- [ ] Post-publish verification completed successfully.

## Approval and rollback

- [ ] Release preflight passed on the tagged commit.
- [ ] Strict Clippy, formatting, check, full tests, and target-native tests passed.
- [ ] Protected release-environment approval recorded.
- [ ] Signing secret was supplied through GitHub Actions secrets and was not stored in the repository.
- [ ] Last known-good release and rollback owner are recorded.
- [ ] Rollback runbook reviewed: `docs/RELEASE_ROLLBACK_RUNBOOK_EN.md` / `docs/RELEASE_ROLLBACK_RUNBOOK_MM.md`.

## Documentation

- English release notes: `docs/RELEASE_<VERSION>_EN.md`
- Burmese release notes: `docs/RELEASE_<VERSION>_MM.md`
- Signing guide: `docs/RELEASE_SIGNING_EN.md` / `docs/RELEASE_SIGNING_MM.md`
- Deployment guide: `docs/DEPLOYMENT_EN.md` / `docs/DEPLOYMENT_MM.md`

## Known limitations

<!-- List known limitations, operational boundaries, or follow-up work. -->

Production GPG secret provisioning, public-key trust distribution, and operator rollback approval remain protected-environment responsibilities.
