# Zap Release Signing and Verification

## Scope

Zap release signing uses an ephemeral CI GPG keyring. The repository contains scripts and policy only; it never contains a private signing key or passphrase. The release workflow fails closed when the required private-key secret is unavailable.

## Required protected secrets

Configure these values in the protected GitHub Actions release environment:

| Secret | Required | Purpose |
|---|---:|---|
| `ZAP_RELEASE_GPG_PRIVATE_KEY` | Yes | ASCII-armored private key used only inside the ephemeral runner keyring |
| `ZAP_RELEASE_GPG_PASSPHRASE` | Optional | Passphrase for a protected private key; omit only for an intentionally unprotected CI signing key |

The private key must be imported only into the runner’s temporary `GNUPGHOME`. It must not be committed, placed in an example file, printed in logs, or included in a release asset.

## Public verification artifact

The workflow exports only the public portion of the configured signing key with:

```bash
GNUPGHOME="$GNUPGHOME" \
SIGNING_KEY_ID="$SIGNING_KEY_ID" \
  bash scripts/export_release_public_key.sh \
    "artifacts/zap-${GITHUB_REF_NAME#v}-release-signing-key.asc"
```

The public key is distributed with the release so users and downstream automation can verify the `.asc` signatures. The helper rejects empty output and refuses to publish output containing a private-key armor block.

The machine-readable controls are defined in `deploy/release-signing-policy.toml`.

## Local verification

After downloading a release and importing the trusted public key into an isolated verification keyring, run:

```bash
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.0 ./published-release
```

The verifier checks the archive set, per-artifact checksums, aggregate checksums, manifest/provenance consistency, expected archive entries, and detached signatures. It fails closed on missing assets, mismatched hashes, missing signatures, unsafe names, or invalid provenance.

## Key rotation

Key rotation requires a new key ID, protected secret update, public-key distribution, a successful signed fixture and release verification run, and a bilingual release notice. If the old key is revoked, the revocation and the first release signed by the new key must be announced together. Existing releases must remain verifiable with their original trusted key unless a security incident requires quarantine.

## Release gates

A public release requires all of the following: release preflight, deterministic artifact manifest, aggregate checksum, provenance, signatures, post-publish verification, and manual approval in the protected release environment. Automatic tag creation, automatic secret rotation, and private-key export are disabled by policy.

## Operational boundary

Secret provisioning, key custody, public-key trust distribution, and key rotation are operator responsibilities. The repository provides reproducible procedures and validation scripts but does not contain production credentials or authorize production access by itself.
