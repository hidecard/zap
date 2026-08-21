#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
GNUPGHOME="$WORK/gnupg"
ARTIFACTS="$WORK/artifacts"
mkdir -m 0700 "$GNUPGHOME" "$ARTIFACTS"

for name in \
  zap-2.1.0-linux-x86_64.tar.gz \
  zap-2.1.0-macos-arm64.tar.gz \
  zap-2.1.0-windows-x86_64.zip; do
  printf 'fixture:%s\n' "$name" > "$ARTIFACTS/$name"
  (cd "$ARTIFACTS" && sha256sum "$name" > "$name.sha256")
done

RELEASE_REF=refs/tags/v2.1.0 \
RELEASE_COMMIT=0123456789abcdef0123456789abcdef01234567 \
WORKFLOW_RUN_ID=123456789 \
SOURCE_URI=https://github.com/zap-lang/zap \
  "$ROOT/scripts/aggregate_release_manifest.sh" 2.1.0 "$ARTIFACTS" "$ARTIFACTS" >/tmp/zap-signing-manifest.out

cat > "$WORK/key.conf" <<'KEYCONF'
%no-protection
Key-Type: RSA
Key-Length: 2048
Name-Real: Zap Test Release
Name-Email: zap-test@example.invalid
Expire-Date: 1d
%commit
KEYCONF
gpg --batch --homedir "$GNUPGHOME" --generate-key "$WORK/key.conf" >/dev/null 2>&1
KEY_ID="$(gpg --batch --homedir "$GNUPGHOME" --list-secret-keys --with-colons zap-test@example.invalid | awk -F: '$1 == "sec" { print $5; exit }')"
test -n "$KEY_ID"
KEY_FINGERPRINT="$(gpg --batch --homedir "$GNUPGHOME" --with-colons --list-secret-keys "$KEY_ID" | awk -F: '$1 == "fpr" { print $10; exit }')"
[[ "$KEY_FINGERPRINT" =~ ^[A-Fa-f0-9]{40}$ ]]

GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID="$KEY_ID" TRUSTED_SIGNING_FINGERPRINTS="$KEY_FINGERPRINT" \
RELEASE_REF=refs/tags/v2.1.0 RELEASE_COMMIT=0123456789abcdef0123456789abcdef01234567 \
WORKFLOW_RUN_ID=123456789 SOURCE_URI=https://github.com/zap-lang/zap \
  "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$ARTIFACTS" >/tmp/zap-signing-success.out

test -s "$ARTIFACTS/zap-2.1.0-provenance.json"
test -s "$ARTIFACTS/zap-2.1.0-provenance.json.asc"
jq -e '.provenance_schema == "zap.provenance.v1" and .signing.mode == "signed" and (.signing.key_id | test("^[A-Fa-f0-9]{40}$")) and (.source.uri == "https://github.com/zap-lang/zap") and (.source.commit | test("^[A-Fa-f0-9]{40}$")) and (.build.workflow_run_id == "123456789") and (.subjects | length == 3)' "$ARTIFACTS/zap-2.1.0-provenance.json" >/dev/null
gpg --batch --homedir "$GNUPGHOME" --verify "$ARTIFACTS/zap-2.1.0-provenance.json.asc" "$ARTIFACTS/zap-2.1.0-provenance.json" >/dev/null 2>&1

# A signing key outside the configured rotation allowlist must fail closed.
printf 'fixture:zap-2.1.0-linux-x86_64.tar.gz\n' > "$ARTIFACTS/zap-2.1.0-linux-x86_64.tar.gz"
if GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID="$KEY_ID" TRUSTED_SIGNING_FINGERPRINTS=FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF RELEASE_REF=refs/tags/v2.1.0 RELEASE_COMMIT=0123456789abcdef0123456789abcdef01234567 WORKFLOW_RUN_ID=123456789 SOURCE_URI=https://github.com/zap-lang/zap "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$ARTIFACTS" >/tmp/zap-signing-rotation.out 2>&1; then
  echo 'expected rotation allowlist failure did not occur' >&2
  exit 1
fi
grep -Fq 'not trusted by the configured rotation allowlist' /tmp/zap-signing-rotation.out

# A changed subject must fail against its original sidecar checksum.
printf 'tampered\n' >> "$ARTIFACTS/zap-2.1.0-linux-x86_64.tar.gz"
if GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID="$KEY_ID" TRUSTED_SIGNING_FINGERPRINTS="$KEY_FINGERPRINT" "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$ARTIFACTS" >/tmp/zap-signing-tamper.out 2>&1; then
  echo 'expected tamper failure did not occur' >&2
  exit 1
fi
grep -Fq 'per-artifact checksum failed' /tmp/zap-signing-tamper.out

# Missing-key failure must happen before any signing operation.
rm -f "$ARTIFACTS/zap-2.1.0-linux-x86_64.tar.gz.asc"
printf 'fixture:zap-2.1.0-linux-x86_64.tar.gz\n' > "$ARTIFACTS/zap-2.1.0-linux-x86_64.tar.gz"
if GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID=missing-key@example.invalid TRUSTED_SIGNING_FINGERPRINTS="$KEY_FINGERPRINT" "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$ARTIFACTS" >/tmp/zap-signing-key.out 2>&1; then
  echo 'expected missing-key failure did not occur' >&2
  exit 1
fi
grep -Fq 'no usable secret signing key' /tmp/zap-signing-key.out

# Signed mode must fail closed when provenance identity is incomplete.
printf 'fixture:zap-2.1.0-linux-x86_64.tar.gz\n' > "$ARTIFACTS/zap-2.1.0-linux-x86_64.tar.gz"
if GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID="$KEY_ID" TRUSTED_SIGNING_FINGERPRINTS="$KEY_FINGERPRINT" RELEASE_REF=refs/heads/main RELEASE_COMMIT=fixture-commit WORKFLOW_RUN_ID=0 SOURCE_URI=http://insecure.invalid "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$ARTIFACTS" >/tmp/zap-signing-provenance.out 2>&1; then
  echo 'expected incomplete provenance failure did not occur' >&2
  exit 1
fi
grep -Fq 'signed provenance requires a semantic-version release tag ref' /tmp/zap-signing-provenance.out

echo 'signing and provenance fixture tests passed'
