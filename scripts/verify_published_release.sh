#!/usr/bin/env bash
# Zap v2.1-E post-publish release verification
#
# Usage:
#   GNUPGHOME=/secure/ci/gnupg \
#     bash scripts/verify_published_release.sh 2.1.0 ./published
#
# The directory must contain the assets downloaded from a GitHub Release. This
# script performs read-only checks and never deletes, retags, or republishes a
# release. Signature verification is required by default.

set -euo pipefail
IFS=$'\n\t'

usage() {
  sed -n '1,25p' "$0"
}

fail() {
  echo "published release verification: $*" >&2
  exit 1
}

VERSION="${1:-}"
RELEASE_DIR="${2:-}"
REQUIRE_SIGNATURES="${REQUIRE_SIGNATURES:-1}"

if [[ "$VERSION" == "--help" || "$VERSION" == "-h" ]]; then
  usage
  exit 0
fi
[[ -n "$VERSION" && -n "$RELEASE_DIR" ]] || { usage >&2; exit 2; }
[[ "$VERSION" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]] || \
  fail "invalid release version: $VERSION"
[[ -d "$RELEASE_DIR" ]] || fail "release directory does not exist: $RELEASE_DIR"
RELEASE_DIR="$(cd "$RELEASE_DIR" && pwd -P)"

for tool in jq sha256sum stat tar unzip diff; do
  command -v "$tool" >/dev/null 2>&1 || fail "required command is missing: $tool"
done

MANIFEST="$RELEASE_DIR/zap-$VERSION-manifest.json"
CHECKSUMS="$RELEASE_DIR/zap-$VERSION-checksums.sha256"
PROVENANCE="$RELEASE_DIR/zap-$VERSION-provenance.json"
[[ -f "$MANIFEST" ]] || fail "missing manifest"
[[ -f "$CHECKSUMS" ]] || fail "missing aggregate checksum file"
[[ -f "$PROVENANCE" ]] || fail "missing provenance"

jq -e --arg version "$VERSION" \
  '.schema == "zap.release-manifest.v1" and .version == $version and (.artifacts | type == "array" and length == 3)' \
  "$MANIFEST" >/dev/null || fail "manifest schema, version, or target count is invalid"
jq -e --arg version "$VERSION" \
  '.provenance_schema == "zap.provenance.v1" and
   .version == $version and
   (.source.uri | type == "string" and startswith("https://")) and
   (.source.ref | type == "string" and test("^refs/tags/v(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$")) and
   (.source.commit | type == "string" and test("^[A-Fa-f0-9]{40}$")) and
   (.build.workflow_run_id | type == "string" and test("^[1-9][0-9]*$")) and
   (.signing.mode == "signed") and
   (.signing.key_id | type == "string" and test("^[A-Fa-f0-9]{40}$")) and
   (.subjects | type == "array" and length == 3)' \
  "$PROVENANCE" >/dev/null || fail "provenance identity, signing, or subject contract is invalid"

mapfile -t ARCHIVES < <(jq -r '.artifacts[].name' "$MANIFEST" | LC_ALL=C sort)
mapfile -t PROVENANCE_SUBJECTS < <(jq -r '.subjects[].name' "$PROVENANCE" | LC_ALL=C sort)
mapfile -t MANIFEST_SUBJECTS < <(jq -r '.artifacts[].name' "$MANIFEST" | LC_ALL=C sort)
[[ "${PROVENANCE_SUBJECTS[*]}" == "${MANIFEST_SUBJECTS[*]}" ]] || fail "provenance subjects do not match manifest artifacts"

for archive in "${ARCHIVES[@]}"; do
  [[ "$archive" != */* && "$archive" != .* ]] || fail "unsafe artifact name: $archive"
  [[ -f "$RELEASE_DIR/$archive" ]] || fail "missing published archive: $archive"
  [[ -f "$RELEASE_DIR/$archive.sha256" ]] || fail "missing published sidecar checksum: $archive"
  (cd "$RELEASE_DIR" && sha256sum -c "$archive.sha256") >/dev/null || fail "sidecar checksum failed: $archive"
  grep -Fq "  $archive" "$CHECKSUMS" || fail "aggregate checksum missing: $archive"
done
(cd "$RELEASE_DIR" && sha256sum -c "$(basename "$CHECKSUMS")") >/dev/null || fail 'aggregate checksum verification failed'

check_tar_entry() {
  local archive="$1"
  local entry="$2"
  tar -tzf "$RELEASE_DIR/$archive" | grep -Fx "$entry" >/dev/null || fail "missing tar entry $entry in $archive"
}

check_zip_entry() {
  local archive="$1"
  local entry="$2"
  unzip -Z1 "$RELEASE_DIR/$archive" | grep -Fx "$entry" >/dev/null || fail "missing zip entry $entry in $archive"
}

for archive in "${ARCHIVES[@]}"; do
  case "$archive" in
    *.tar.gz)
      check_tar_entry "$archive" 'zap/README.md'
      check_tar_entry "$archive" 'zap/RELEASE.txt'
      check_tar_entry "$archive" 'zap/docs/SYNTAX_GUIDE_EN.md'
      check_tar_entry "$archive" 'zap/examples/hello.zp'
      ;;
    *.zip)
      check_zip_entry "$archive" 'zap/README.md'
      check_zip_entry "$archive" 'zap/RELEASE.txt'
      check_zip_entry "$archive" 'zap/docs/SYNTAX_GUIDE_EN.md'
      check_zip_entry "$archive" 'zap/examples/hello.zp'
      ;;
    *)
      fail "unsupported archive format: $archive"
      ;;
  esac
done

SIGNATURES=("${ARCHIVES[@]}" "$(basename "$MANIFEST")" "$(basename "$CHECKSUMS")" "$(basename "$PROVENANCE")")
if [[ "$REQUIRE_SIGNATURES" == "1" ]]; then
  command -v gpg >/dev/null 2>&1 || fail 'gpg is required for signature verification'
  for subject in "${SIGNATURES[@]}"; do
    signature="$RELEASE_DIR/$subject.asc"
    [[ -f "$signature" ]] || fail "missing signature: $(basename "$signature")"
    gpg --batch --verify "$signature" "$RELEASE_DIR/$subject" >/dev/null 2>&1 || \
      fail "signature verification failed: $subject"
  done
else
  echo 'WARN: REQUIRE_SIGNATURES is not 1; signature checks were skipped' >&2
fi

printf '%s\n' 'published release verification: PASSED'
printf '  version: %s\n' "$VERSION"
printf '  archives: %d\n' "${#ARCHIVES[@]}"
printf '  signatures: %s\n' "$REQUIRE_SIGNATURES"
