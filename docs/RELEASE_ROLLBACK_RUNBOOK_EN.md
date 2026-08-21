# Zap Release Rollback and Quarantine Runbook

## Purpose

This runbook defines the controlled response for a Zap release that is incomplete, unverifiable, compromised, or operationally unsafe. It is designed for the v2.1-E release pipeline and applies to GitHub Release assets, platform installers, checksums, signatures, provenance metadata, and registry deployment references.

Rollback is a controlled change. Do not delete evidence, rewrite an existing tag, or silently replace a published asset. Quarantine the affected release, preserve logs and hashes, and direct users to the last known-good release until the incident is reviewed.

## Preconditions

The operator must have access to the repository, the protected release environment, the release verification script, the public signing key, registry backup/index storage, and the incident communication channel. Production secrets must be supplied through the approved secret manager or CI environment and must not be copied into this repository.

Required commands and files include:

```text
scripts/verify_published_release.sh
scripts/validate_registry_deployment.sh
scripts/aggregate_release_manifest.sh
docs/RELEASE_ROLLBACK_RUNBOOK_EN.md
docs/RELEASE_ROLLBACK_RUNBOOK_MM.md
```

## Severity and trigger conditions

| Trigger | Initial severity | Required action |
|---|---:|---|
| Missing release asset or incomplete upload | High | Stop promotion and quarantine the release |
| Checksum mismatch | Critical | Do not install or distribute the affected asset |
| Signature or provenance verification failure | Critical | Quarantine all release assets until key and artifact state are reviewed |
| Installer failure or unsafe upgrade | High | Stop further promotion and direct users to the previous stable version |
| Registry index or cache corruption | Critical | Freeze writes and restore the last known-good signed state |
| Credential or signing-key exposure | Critical | Revoke/rotate credentials and quarantine related releases |
| Post-release functional regression | High | Stop promotion and begin rollback assessment |

## Immediate containment

1. Record the release tag, commit SHA, workflow run ID, publication timestamp, reporter, and first observed symptom.
2. Do not delete the GitHub Release or move the tag. Preserve the original state for evidence.
3. Mark the release as quarantined in the incident record and stop any pending promotion, marketplace publication, registry synchronization, or installer distribution.
4. Point release documentation and operator communication to the last known-good version. Do not claim that a replacement asset is safe until verification passes.
5. If credentials or signing material may be exposed, revoke or rotate them before rebuilding any artifact.

## Verification and evidence collection

Download the published assets into an isolated directory and run the verification script with the public verification key available through `GNUPGHOME`:

```bash
mkdir -m 700 /tmp/zap-release-incident
# Download assets using the approved GitHub/registry procedure.
GNUPGHOME=/secure/verification/gnupg \
  bash scripts/verify_published_release.sh 2.1.0 /tmp/zap-release-incident
```

Save the verifier output, the manifest, provenance file, aggregate checksum file, signature files, artifact listing, and relevant CI logs as incident evidence. Compare the manifest commit and release ref with the intended source commit. A mismatch is a release integrity failure.

## Rollback decision

Rollback is required when a checksum or signature cannot be verified, a required artifact is missing, an installer can damage an existing installation, a registry index cannot be trusted, or the release introduces a severe regression without a safe mitigation. A release owner and a security or operations reviewer must approve the rollback decision. The operator must record the reason, evidence, selected last known-good version, and expected user impact.

## GitHub Release quarantine

Use the GitHub repository controls to mark the affected release as a draft or otherwise prevent further distribution according to the repository’s release policy. Do not overwrite existing assets in place. Keep the original assets available to authorized reviewers or move them to restricted incident storage with their original hashes preserved.

If a corrected release is required, create a new version or approved corrective tag. Do not reuse a version tag for different bytes. The corrected release must pass the full preflight, artifact aggregation, signing, provenance, and post-publish verification gates before it is announced.

## Registry rollback

1. Freeze registry writes and package publication while the index state is assessed.
2. Identify the last known-good signed index and its checksum from protected backup storage.
3. Verify the backup signature and checksum before restoration.
4. Restore the index atomically, preserve the previous state for investigation, and restart only the managed service path approved by operations.
5. Re-run trusted-registry, cache-integrity, and package-resolution checks.
6. Resume writes only after an operations reviewer confirms that the restored index and service health are correct.

Never restore an unsigned or unverified index merely because it is newer or operationally convenient.

## User and stakeholder communication

The initial notice must identify the affected version, the action users should take, the last known-good version, and whether downloads or registry installs are paused. Do not publish private incident details, credentials, signing keys, or unverified root-cause claims. The final notice should include the corrected version, verification instructions, migration/rollback guidance, and the incident closure time.

English and Burmese release communications must describe the same affected versions, user actions, and limitations.

## Return-to-service checklist

| Check | Evidence required |
|---|---|
| Corrected source tag is immutable | Commit and tag references |
| Preflight passes | CI run link and preflight summary |
| All platform artifacts exist | Artifact manifest |
| Checksums match | Aggregate and per-artifact checksum output |
| Signatures verify | Public-key verification output |
| Provenance matches intended source | Provenance JSON and commit comparison |
| Install/upgrade checks pass | Platform installer test results |
| Registry state is trusted | Restored signed index and service health evidence |
| Documentation is updated | English/Burmese release notes and rollback notice |
| Reviewer approval recorded | Incident and release approval record |

Only after every check passes may the release be unquarantined, the corrected release announced, and registry or marketplace promotion resumed.

## Post-incident actions

Preserve the incident timeline, affected hashes, CI logs, operator commands, user impact, root cause, and corrective actions. Rotate any potentially exposed credentials. Add a regression test for the failure mode, update the release preflight or verification gate, and review whether signing, provenance, backup, or access-control policy needs strengthening.

## Safety boundaries

This runbook does not authorize production access, credential rotation, payment, or public communication by itself. Those actions require the repository’s approved operator and reviewer permissions. The runbook is a reproducible reference procedure; environment-specific hostnames, secrets, certificate paths, and private infrastructure details must remain outside the repository.
