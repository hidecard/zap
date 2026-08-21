#!/usr/bin/env bash
# Zap v2.1-E artifact signing and provenance generation
#
# The script expects an already validated artifact directory containing the
# platform archives plus zap-VERSION-manifest.json and
# zap-VERSION-checksums.sha256. It never imports keys, creates tags, pushes
# commits, or publishes releases. CI must inject the signing key separately.
#
# Signed mode:
#   GNUPGHOME=/secure/ci/gnupg SIGNING_KEY_ID=release@example.com \
#     bash scripts/sign_release_artifacts.sh 2.1.0 artifacts
#
# Provenance-only mode for development/testing:
#   bash scripts/sign_release_artifacts.sh 2.1.0 artifacts --unsigned
#
# Optional environment variables:
#   SIGNING_KEY_ID  GPG key fingerprint, long key ID, or exact user ID.
#   TRUSTED_SIGNING_FINGERPRINTS
#                   Required in signed mode. Comma/space-separated full
#                   fingerprints allowed during an explicit rotation window.
#   GNUPGHOME       Keyring supplied by CI; never commit this directory.
#   RELEASE_REF     Git ref recorded in provenance.
#   RELEASE_COMMIT  Commit SHA recorded in provenance.
#   WORKFLOW_RUN_ID CI workflow run ID recorded in provenance.
#   SOURCE_URI      Source repository URI recorded in provenance.
#   GPG_PASSPHRASE_FILE
#                   Optional chmod-600 file injected by CI for a protected key.
#   ALLOW_EXTRA=1   Permit unrelated files in the artifact directory.

set -euo pipefail
IFS=$'\n\t'

usage() {
  sed -n '1,34p' "$0"
}

fail() {
  echo "artifact signing: $*" >&2
  exit 1
}

VERSION="${1:-}"
ARTIFACT_DIR="${2:-}"
MODE="signed"

if [[ "$VERSION" == "--help" || "$VERSION" == "-h" ]]; then
  usage
  exit 0
fi
[[ -n "$VERSION" && -n "$ARTIFACT_DIR" ]] || { usage >&2; exit 2; }
shift 2
while [[ $# -gt 0 ]]; do
  case "$1" in
    --unsigned)
      MODE="unsigned"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      fail "unknown option: $1"
      ;;
  esac
done

[[ "$VERSION" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]] || \
  fail "invalid release version: $VERSION"
[[ -d "$ARTIFACT_DIR" ]] || fail "artifact directory does not exist: $ARTIFACT_DIR"
ARTIFACT_DIR="$(cd "$ARTIFACT_DIR" && pwd -P)"

command -v jq >/dev/null 2>&1 || fail "jq is required for deterministic provenance generation"
command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"
command -v stat >/dev/null 2>&1 || fail "stat is required"

MANIFEST="$ARTIFACT_DIR/zap-$VERSION-manifest.json"
CHECKSUMS="$ARTIFACT_DIR/zap-$VERSION-checksums.sha256"
PROVENANCE="$ARTIFACT_DIR/zap-$VERSION-provenance.json"
[[ -f "$MANIFEST" ]] || fail "missing artifact manifest: $MANIFEST"
[[ -f "$CHECKSUMS" ]] || fail "missing aggregate checksums: $CHECKSUMS"

jq -e --arg version "$VERSION" \
  '.schema == "zap.release-manifest.v1" and .version == $version and (.artifacts | type == "array" and length > 0)' \
  "$MANIFEST" >/dev/null || fail "invalid or mismatched artifact manifest"

mapfile -t ARCHIVES < <(jq -r '.artifacts[].name' "$MANIFEST" | LC_ALL=C sort)
[[ "${#ARCHIVES[@]}" -gt 0 ]] || fail "manifest contains no artifacts"

for archive in "${ARCHIVES[@]}"; do
  [[ "$archive" != */* && "$archive" != .* ]] || fail "unsafe artifact name in manifest: $archive"
  archive_path="$ARTIFACT_DIR/$archive"
  [[ -f "$archive_path" ]] || fail "manifest artifact is missing: $archive"
  grep -Fq "  $archive" "$CHECKSUMS" || fail "aggregate checksum is missing artifact: $archive"
  (cd "$ARTIFACT_DIR" && sha256sum -c "$archive.sha256" 2>/dev/null) || fail "per-artifact checksum failed: $archive"
done

# Sign the release subjects, not the per-artifact checksum sidecars. The
# aggregate checksum file and manifest are signed as the release index.
SIGNED_SUBJECTS=("${ARCHIVES[@]}" "$(basename "$MANIFEST")" "$(basename "$CHECKSUMS")")
SIGNATURE_FILES=()

if [[ "$MODE" == "signed" ]]; then
  command -v gpg >/dev/null 2>&1 || fail "gpg is required for signed mode"
  SIGNING_KEY_ID="${SIGNING_KEY_ID:-}"
  GPG_PASSPHRASE_FILE="${GPG_PASSPHRASE_FILE:-}"
  [[ -n "$SIGNING_KEY_ID" ]] || fail "SIGNING_KEY_ID is required in signed mode"
  [[ -n "${GNUPGHOME:-}" ]] || echo 'WARN: GNUPGHOME is not set; gpg default keyring will be used' >&2
  if [[ -n "$GPG_PASSPHRASE_FILE" ]]; then
    [[ -f "$GPG_PASSPHRASE_FILE" ]] || fail "GPG_PASSPHRASE_FILE does not exist"
    [[ "$(stat -c '%a' "$GPG_PASSPHRASE_FILE")" == "600" ]] || fail "GPG_PASSPHRASE_FILE must have mode 600"
  fi
  gpg --batch --list-secret-keys --with-colons "$SIGNING_KEY_ID" >/dev/null 2>&1 || \
    fail "no usable secret signing key found for SIGNING_KEY_ID"

  SIGNING_KEY_FINGERPRINT="$(gpg --batch --with-colons --list-secret-keys "$SIGNING_KEY_ID" |
    awk -F: '$1 == "fpr" { print $10; exit }')"
  [[ "$SIGNING_KEY_FINGERPRINT" =~ ^[A-Fa-f0-9]{40}$ ]] || \
    fail "signing key does not resolve to a full 40-hex fingerprint"

  TRUSTED_SIGNING_FINGERPRINTS="${TRUSTED_SIGNING_FINGERPRINTS:-}"
  [[ -n "$TRUSTED_SIGNING_FINGERPRINTS" ]] || \
    fail "TRUSTED_SIGNING_FINGERPRINTS is required in signed mode"
  trusted_match=0
  IFS=', ' read -r -a trusted_fingerprints <<< "$TRUSTED_SIGNING_FINGERPRINTS"
  for trusted_fingerprint in "${trusted_fingerprints[@]}"; do
    [[ -n "$trusted_fingerprint" ]] || continue
    [[ "$trusted_fingerprint" =~ ^[A-Fa-f0-9]{40}$ ]] || \
      fail "trusted signing fingerprint must be a full 40-hex fingerprint"
    if [[ "${trusted_fingerprint^^}" == "${SIGNING_KEY_FINGERPRINT^^}" ]]; then
      trusted_match=1
    fi
  done
  [[ "$trusted_match" -eq 1 ]] || \
    fail "signing key fingerprint is not trusted by the configured rotation allowlist"

  gpg_sign() {
    local input="$1"
    local output="$2"
    local args=(--batch --yes --armor --detach-sign --local-user "$SIGNING_KEY_ID")
    if [[ -n "$GPG_PASSPHRASE_FILE" ]]; then
      args+=(--pinentry-mode loopback --passphrase-file "$GPG_PASSPHRASE_FILE")
    fi
    gpg "${args[@]}" --output "$output" "$input"
  }

  for subject in "${SIGNED_SUBJECTS[@]}"; do
    output="$ARTIFACT_DIR/$subject.asc"
    gpg_sign "$ARTIFACT_DIR/$subject" "$output"
    gpg --batch --verify "$output" "$ARTIFACT_DIR/$subject" >/dev/null 2>&1 || \
      fail "signature verification failed: $subject"
    SIGNATURE_FILES+=("$(basename "$output")")
  done
else
  echo 'WARN: --unsigned selected; provenance will explicitly declare unsigned mode' >&2
fi

RELEASE_REF="${RELEASE_REF:-}"
RELEASE_COMMIT="${RELEASE_COMMIT:-}"
WORKFLOW_RUN_ID="${WORKFLOW_RUN_ID:-}"
SOURCE_URI="${SOURCE_URI:-}"
SIGNING_KEY_ID="${SIGNING_KEY_ID:-}"

if [[ "$MODE" == "signed" ]]; then
  [[ "$RELEASE_REF" =~ ^refs/tags/v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]] || \
    fail "signed provenance requires a semantic-version release tag ref"
  [[ "$RELEASE_COMMIT" =~ ^[A-Fa-f0-9]{40}$ ]] || \
    fail "signed provenance requires a full 40-hex commit SHA"
  [[ "$WORKFLOW_RUN_ID" =~ ^[1-9][0-9]*$ ]] || \
    fail "signed provenance requires a numeric workflow run ID"
  [[ "$SOURCE_URI" =~ ^https://[^[:space:]]+$ ]] || \
    fail "signed provenance requires an HTTPS source URI"
  SIGNING_KEY_ID="$SIGNING_KEY_FINGERPRINT"
fi

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT
PROVENANCE_TMP="$WORK_DIR/provenance.json"

# jq -S gives stable key ordering. Subject order is already stable from the
# sorted manifest names, making the generated provenance reviewable and repeatable.
jq -S \
  --arg version "$VERSION" \
  --arg release_ref "$RELEASE_REF" \
  --arg commit "$RELEASE_COMMIT" \
  --arg workflow_run_id "$WORKFLOW_RUN_ID" \
  --arg source_uri "$SOURCE_URI" \
  --arg signing_mode "$MODE" \
  --arg signing_key_id "$SIGNING_KEY_ID" \
  --argjson signature_files "$(printf '%s\n' "${SIGNATURE_FILES[@]}" | jq -R -s 'split("\n") | map(select(length > 0))')" \
  '. + {
    provenance_schema: "zap.provenance.v1",
    version: $version,
    source: {
      uri: $source_uri,
      ref: $release_ref,
      commit: $commit
    },
    build: {
      workflow_run_id: $workflow_run_id,
      reproducible_artifact_manifest: ("zap-" + $version + "-manifest.json"),
      aggregate_checksums: ("zap-" + $version + "-checksums.sha256")
    },
    signing: {
      mode: $signing_mode,
      key_id: $signing_key_id,
      signature_files: $signature_files
    },
    subjects: [.artifacts[] | {
      name: .name,
      target: .target,
      sha256: .sha256,
      size_bytes: .size_bytes
    }]
  } | del(.artifacts)' "$MANIFEST" > "$PROVENANCE_TMP"

jq -e --arg version "$VERSION" \
  '.provenance_schema == "zap.provenance.v1" and .version == $version and (.subjects | length > 0)' \
  "$PROVENANCE_TMP" >/dev/null || fail "generated provenance failed schema validation"
install -m 0644 "$PROVENANCE_TMP" "$PROVENANCE"

if [[ "$MODE" == "signed" ]]; then
  gpg_sign "$PROVENANCE" "$PROVENANCE.asc"
  gpg --batch --verify "$PROVENANCE.asc" "$PROVENANCE" >/dev/null 2>&1 || \
    fail 'provenance signature verification failed'
  SIGNATURE_FILES+=("$(basename "$PROVENANCE.asc")")
fi

printf '%s\n' 'artifact signing: passed'
printf '  mode: %s\n' "$MODE"
printf '  manifest: %s\n' "$MANIFEST"
printf '  provenance: %s\n' "$PROVENANCE"
printf '  signed subjects: %d\n' "${#SIGNED_SUBJECTS[@]}"
printf '  signature files: %d\n' "${#SIGNATURE_FILES[@]}"
