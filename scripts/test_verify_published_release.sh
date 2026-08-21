#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
GNUPGHOME="$WORK/gnupg"
RELEASE_DIR="$WORK/release"
PACKAGE_ROOT="$WORK/package-root"
mkdir -m 0700 "$GNUPGHOME" "$RELEASE_DIR" "$PACKAGE_ROOT"
mkdir -p "$PACKAGE_ROOT/zap/bin" "$PACKAGE_ROOT/zap/docs" "$PACKAGE_ROOT/zap/examples"
printf 'fixture readme\n' > "$PACKAGE_ROOT/zap/README.md"
printf 'Version: 2.1.0\n' > "$PACKAGE_ROOT/zap/RELEASE.txt"
printf 'fixture syntax\n' > "$PACKAGE_ROOT/zap/docs/SYNTAX_GUIDE_EN.md"
printf 'print("hello")\n' > "$PACKAGE_ROOT/zap/examples/hello.zp"
printf 'binary\n' > "$PACKAGE_ROOT/zap/bin/zap"
printf 'binary\n' > "$PACKAGE_ROOT/zap/bin/zap.exe"

(cd "$PACKAGE_ROOT" && tar -czf "$RELEASE_DIR/zap-2.1.0-linux-x86_64.tar.gz" zap)
(cd "$PACKAGE_ROOT" && tar -czf "$RELEASE_DIR/zap-2.1.0-macos-arm64.tar.gz" zap)
(cd "$PACKAGE_ROOT" && zip -qr "$RELEASE_DIR/zap-2.1.0-windows-x86_64.zip" zap)
for archive in "$RELEASE_DIR"/zap-2.1.0-*.tar.gz "$RELEASE_DIR"/zap-2.1.0-*.zip; do
  (cd "$RELEASE_DIR" && sha256sum "$(basename "$archive")" > "$(basename "$archive").sha256")
done

RELEASE_REF=refs/tags/v2.1.0 RELEASE_COMMIT=fixture-commit WORKFLOW_RUN_ID=fixture-run \
  "$ROOT/scripts/aggregate_release_manifest.sh" 2.1.0 "$RELEASE_DIR" "$RELEASE_DIR" >/tmp/zap-verify-manifest.out

cat > "$WORK/key.conf" <<'KEYCONF'
%no-protection
Key-Type: RSA
Key-Length: 2048
Name-Real: Zap Verify Test
Name-Email: zap-verify@example.invalid
Expire-Date: 1d
%commit
KEYCONF
gpg --batch --homedir "$GNUPGHOME" --generate-key "$WORK/key.conf" >/dev/null 2>&1
KEY_ID="$(gpg --batch --homedir "$GNUPGHOME" --list-secret-keys --with-colons zap-verify@example.invalid | awk -F: '$1 == "sec" { print $5; exit }')"
KEY_FINGERPRINT="$(gpg --batch --homedir "$GNUPGHOME" --list-secret-keys --with-colons "$KEY_ID" | awk -F: '$1 == "fpr" { print $10; exit }')"
test -n "$KEY_FINGERPRINT"
GNUPGHOME="$GNUPGHOME" SIGNING_KEY_ID="$KEY_ID" TRUSTED_SIGNING_FINGERPRINTS="$KEY_FINGERPRINT" \
  "$ROOT/scripts/sign_release_artifacts.sh" 2.1.0 "$RELEASE_DIR" >/tmp/zap-verify-signing.out

GNUPGHOME="$GNUPGHOME" "$ROOT/scripts/verify_published_release.sh" 2.1.0 "$RELEASE_DIR" >/tmp/zap-verify-success.out
grep -Fq 'published release verification: PASSED' /tmp/zap-verify-success.out

# A valid signature over malformed provenance must still be rejected by the
# verifier's semantic identity contract. This keeps field mutation failures
# distinct from ordinary detached-signature tamper failures.
PROVENANCE="$RELEASE_DIR/zap-2.1.0-provenance.json"
PROVENANCE_SIG="$PROVENANCE.asc"
cp "$PROVENANCE" "$WORK/provenance.valid.json"
cp "$PROVENANCE_SIG" "$WORK/provenance.valid.json.asc"
expect_provenance_rejection() {
  local name="$1"
  local filter="$2"
  cp "$WORK/provenance.valid.json" "$PROVENANCE"
  jq -e "$filter" "$WORK/provenance.valid.json" > "$PROVENANCE"
  GNUPGHOME="$GNUPGHOME" gpg --batch --yes --armor --detach-sign --local-user "$KEY_ID" \
    --output "$PROVENANCE_SIG" "$PROVENANCE"
  if GNUPGHOME="$GNUPGHOME" "$ROOT/scripts/verify_published_release.sh" 2.1.0 "$RELEASE_DIR" \
      >/tmp/zap-verify-"$name".out 2>&1; then
    echo "expected provenance rejection did not occur: $name" >&2
    exit 1
  fi
  grep -Fq 'provenance identity, signing, or subject contract is invalid' \
    /tmp/zap-verify-"$name".out
}
expect_provenance_rejection missing-commit 'del(.source.commit)'
expect_provenance_rejection invalid-commit '.source.commit = (("f" * 39) + "g")'
expect_provenance_rejection insecure-source '.source.uri = "http://insecure.invalid"'
expect_provenance_rejection invalid-ref '.source.ref = "refs/heads/main"'
expect_provenance_rejection invalid-workflow '.build.workflow_run_id = "0"'
expect_provenance_rejection unsigned-mode '.signing.mode = "unsigned"'
cp "$WORK/provenance.valid.json" "$PROVENANCE"
jq -e '.subjects[0].name = "tampered-artifact.tar.gz"' "$WORK/provenance.valid.json" > "$PROVENANCE"
GNUPGHOME="$GNUPGHOME" gpg --batch --yes --armor --detach-sign --local-user "$KEY_ID" \
  --output "$PROVENANCE_SIG" "$PROVENANCE"
if GNUPGHOME="$GNUPGHOME" "$ROOT/scripts/verify_published_release.sh" 2.1.0 "$RELEASE_DIR" \
    >/tmp/zap-verify-subject.out 2>&1; then
  echo 'expected subject-list rejection did not occur' >&2
  exit 1
fi
grep -Fq 'provenance subjects do not match manifest artifacts' /tmp/zap-verify-subject.out
cp "$WORK/provenance.valid.json" "$PROVENANCE"
cp "$WORK/provenance.valid.json.asc" "$PROVENANCE_SIG"
GNUPGHOME="$GNUPGHOME" "$ROOT/scripts/verify_published_release.sh" 2.1.0 "$RELEASE_DIR" \
  >/tmp/zap-verify-restored.out
grep -Fq 'published release verification: PASSED' /tmp/zap-verify-restored.out

rm -f "$RELEASE_DIR/zap-2.1.0-provenance.json.asc"
if GNUPGHOME="$GNUPGHOME" "$ROOT/scripts/verify_published_release.sh" 2.1.0 "$RELEASE_DIR" >/tmp/zap-verify-failure.out 2>&1; then
  echo 'expected missing-signature failure did not occur' >&2
  exit 1
fi
grep -Fq 'missing signature' /tmp/zap-verify-failure.out

echo 'published release verification fixture tests passed'
