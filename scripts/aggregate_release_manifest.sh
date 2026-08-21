#!/usr/bin/env bash
# Zap v2.1-E release artifact manifest and checksum aggregation
#
# This script consumes the platform archives produced by release.yml. It verifies
# every archive against its sidecar checksum, requires the complete supported
# target matrix, and emits deterministic JSON/sha256 outputs.
#
# Usage:
#   bash scripts/aggregate_release_manifest.sh VERSION ARTIFACT_DIR OUTPUT_DIR
#
# Optional environment variables:
#   RELEASE_TARGETS   Comma-separated targets; defaults to the v2.1-E matrix.
#   RELEASE_REF       Git ref recorded in the manifest.
#   RELEASE_COMMIT    Commit SHA recorded in the manifest.
#   WORKFLOW_RUN_ID   CI run ID recorded in the manifest.
#   ALLOW_EXTRA=1     Allow non-release files in ARTIFACT_DIR; archives and
#                     checksum sidecars are still validated strictly.

set -euo pipefail
IFS=$'\n\t'

usage() {
  sed -n '1,28p' "$0"
}

fail() {
  echo "artifact manifest: $*" >&2
  exit 1
}

VERSION="${1:-}"
ARTIFACT_DIR="${2:-}"
OUTPUT_DIR="${3:-}"

if [[ "$VERSION" == "--help" || "$VERSION" == "-h" ]]; then
  usage
  exit 0
fi
[[ -n "$VERSION" && -n "$ARTIFACT_DIR" && -n "$OUTPUT_DIR" ]] || { usage >&2; exit 2; }
[[ "$VERSION" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]] || \
  fail "invalid release version: $VERSION"
[[ -d "$ARTIFACT_DIR" ]] || fail "artifact directory does not exist: $ARTIFACT_DIR"

ARTIFACT_DIR="$(cd "$ARTIFACT_DIR" && pwd -P)"
mkdir -p "$OUTPUT_DIR"
OUTPUT_DIR="$(cd "$OUTPUT_DIR" && pwd -P)"

RELEASE_TARGETS="${RELEASE_TARGETS:-x86_64-unknown-linux-gnu,aarch64-apple-darwin,x86_64-pc-windows-msvc}"
RELEASE_REF="${RELEASE_REF:-}"
RELEASE_COMMIT="${RELEASE_COMMIT:-}"
WORKFLOW_RUN_ID="${WORKFLOW_RUN_ID:-}"
ALLOW_EXTRA="${ALLOW_EXTRA:-0}"

command -v sha256sum >/dev/null 2>&1 || fail "sha256sum is required"
command -v stat >/dev/null 2>&1 || fail "stat is required"

archive_for_target() {
  case "$1" in
    x86_64-unknown-linux-gnu) printf 'zap-%s-linux-x86_64.tar.gz' "$VERSION" ;;
    aarch64-apple-darwin) printf 'zap-%s-macos-arm64.tar.gz' "$VERSION" ;;
    x86_64-pc-windows-msvc) printf 'zap-%s-windows-x86_64.zip' "$VERSION" ;;
    *) fail "unsupported release target: $1" ;;
  esac
}

target_for_archive() {
  case "$1" in
    "zap-$VERSION-linux-x86_64.tar.gz") printf 'x86_64-unknown-linux-gnu' ;;
    "zap-$VERSION-macos-arm64.tar.gz") printf 'aarch64-apple-darwin' ;;
    "zap-$VERSION-windows-x86_64.zip") printf 'x86_64-pc-windows-msvc' ;;
    *) return 1 ;;
  esac
}

IFS=',' read -r -a TARGETS <<< "$RELEASE_TARGETS"
EXPECTED_ARCHIVES=()
for target in "${TARGETS[@]}"; do
  target="${target//[[:space:]]/}"
  [[ -n "$target" ]] || fail "release target list contains an empty target"
  archive="$(archive_for_target "$target")"
  EXPECTED_ARCHIVES+=("$archive")
done

# Ensure the expected matrix itself has no duplicates.
for ((i = 0; i < ${#EXPECTED_ARCHIVES[@]}; i++)); do
  for ((j = i + 1; j < ${#EXPECTED_ARCHIVES[@]}; j++)); do
    [[ "${EXPECTED_ARCHIVES[$i]}" != "${EXPECTED_ARCHIVES[$j]}" ]] || fail "duplicate release target/archive: ${EXPECTED_ARCHIVES[$i]}"
  done
done

for archive in "${EXPECTED_ARCHIVES[@]}"; do
  archive_path="$ARTIFACT_DIR/$archive"
  checksum_path="$archive_path.sha256"
  [[ -f "$archive_path" ]] || fail "missing release archive: $archive"
  [[ -f "$checksum_path" ]] || fail "missing checksum sidecar: $archive.sha256"
  [[ ! -L "$archive_path" && ! -L "$checksum_path" ]] || fail "symlinked artifact or checksum is not allowed: $archive"
  [[ "$(wc -l < "$checksum_path")" -eq 1 ]] || fail "checksum sidecar must contain exactly one line: $archive.sha256"
  (cd "$ARTIFACT_DIR" && sha256sum -c "$archive.sha256") >/dev/null || fail "checksum verification failed: $archive"
done

if [[ "$ALLOW_EXTRA" != "1" ]]; then
  while IFS= read -r -d '' path; do
    base="$(basename "$path")"
    expected=0
    for archive in "${EXPECTED_ARCHIVES[@]}"; do
      [[ "$base" == "$archive" || "$base" == "$archive.sha256" ]] && expected=1
    done
    [[ "$base" == "zap-$VERSION-manifest.json" || "$base" == "zap-$VERSION-checksums.sha256" ]] && expected=1
    [[ "$expected" -eq 1 ]] || fail "unexpected file in artifact directory: $base"
  done < <(find "$ARTIFACT_DIR" -mindepth 1 -maxdepth 1 -type f -print0 | sort -z)
else
  echo "WARN: ALLOW_EXTRA=1; non-release files in artifact directory are ignored" >&2
fi

# Sort the final archive list by artifact name, independent of matrix order.
SORTED_ARCHIVES=()
while IFS= read -r archive; do
  SORTED_ARCHIVES+=("$archive")
done < <(printf '%s\n' "${EXPECTED_ARCHIVES[@]}" | LC_ALL=C sort)

WORK_DIR="$(mktemp -d)"
cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT
CHECKSUMS_TMP="$WORK_DIR/zap-$VERSION-checksums.sha256"
MANIFEST_TMP="$WORK_DIR/zap-$VERSION-manifest.json"

: > "$CHECKSUMS_TMP"
for archive in "${SORTED_ARCHIVES[@]}"; do
  (cd "$ARTIFACT_DIR" && sha256sum "$archive") >> "$CHECKSUMS_TMP"
done

json_escape() {
  # All generated artifact names and metadata are expected to be UTF-8 text.
  # Escape JSON control characters and quotes without introducing locale-order
  # differences. Metadata values are supplied by CI and are not shell-evaluated.
  printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' -e ':a' -e 'N' -e '$!ba' -e 's/\n/\\n/g' -e 's/\r/\\r/g' -e 's/\t/\\t/g'
}

json_value() {
  json_escape "$1"
}

{
  printf '{\n'
  printf '  "schema": "zap.release-manifest.v1",\n'
  printf '  "version": "%s",\n' "$(json_value "$VERSION")"
  printf '  "release_ref": "%s",\n' "$(json_value "$RELEASE_REF")"
  printf '  "commit": "%s",\n' "$(json_value "$RELEASE_COMMIT")"
  printf '  "workflow_run_id": "%s",\n' "$(json_value "$WORKFLOW_RUN_ID")"
  printf '  "artifacts": [\n'
  for ((i = 0; i < ${#SORTED_ARCHIVES[@]}; i++)); do
    archive="${SORTED_ARCHIVES[$i]}"
    checksum="$(awk '{print $1}' "$ARTIFACT_DIR/$archive.sha256")"
    size="$(stat -c '%s' "$ARTIFACT_DIR/$archive")"
    target="$(target_for_archive "$archive")"
    comma=','
    [[ "$i" -eq $((${#SORTED_ARCHIVES[@]} - 1)) ]] && comma=''
    printf '    {"name":"%s","target":"%s","sha256":"%s","size_bytes":%s}%s\n' \
      "$(json_value "$archive")" "$(json_value "$target")" "$checksum" "$size" "$comma"
  done
  printf '  ]\n'
  printf '}\n'
} > "$MANIFEST_TMP"

# Atomic publication prevents a partially-written manifest from being consumed.
install -m 0644 "$CHECKSUMS_TMP" "$OUTPUT_DIR/zap-$VERSION-checksums.sha256"
install -m 0644 "$MANIFEST_TMP" "$OUTPUT_DIR/zap-$VERSION-manifest.json"

# Validate the generated JSON structure when jq is available; the script does
# not require jq so it remains usable on minimal release runners.
if command -v jq >/dev/null 2>&1; then
  jq -e --arg version "$VERSION" \
    '.schema == "zap.release-manifest.v1" and .version == $version and (.artifacts | length > 0)' \
    "$OUTPUT_DIR/zap-$VERSION-manifest.json" >/dev/null || fail "generated manifest failed jq validation"
fi

printf '%s\n' "artifact manifest: generated"
printf '  manifest: %s\n' "$OUTPUT_DIR/zap-$VERSION-manifest.json"
printf '  checksums: %s\n' "$OUTPUT_DIR/zap-$VERSION-checksums.sha256"
printf '  artifacts: %d\n' "${#SORTED_ARCHIVES[@]}"
printf '%s\n' 'artifact manifest: passed'
