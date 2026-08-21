#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
ARTIFACTS="$WORK/artifacts"
OUTPUT="$ARTIFACTS"
mkdir -p "$ARTIFACTS" "$OUTPUT"
for name in \
  zap-2.1.0-linux-x86_64.tar.gz \
  zap-2.1.0-macos-arm64.tar.gz \
  zap-2.1.0-windows-x86_64.zip; do
  printf 'fixture:%s\n' "$name" > "$ARTIFACTS/$name"
  (cd "$ARTIFACTS" && sha256sum "$name" > "$name.sha256")
done
RELEASE_REF=refs/tags/v2.1.0 \
RELEASE_COMMIT=fixture-commit \
WORKFLOW_RUN_ID=fixture-run \
  "$ROOT/scripts/aggregate_release_manifest.sh" 2.1.0 "$ARTIFACTS" "$OUTPUT" >/tmp/zap-manifest-success.out

test -s "$OUTPUT/zap-2.1.0-manifest.json"
test -s "$OUTPUT/zap-2.1.0-checksums.sha256"

grep -Fq '"schema": "zap.release-manifest.v1"' "$OUTPUT/zap-2.1.0-manifest.json"
grep -Fq '"version": "2.1.0"' "$OUTPUT/zap-2.1.0-manifest.json"
grep -Fq 'zap-2.1.0-linux-x86_64.tar.gz' "$OUTPUT/zap-2.1.0-manifest.json"
grep -Fq 'zap-2.1.0-macos-arm64.tar.gz' "$OUTPUT/zap-2.1.0-manifest.json"
grep -Fq 'zap-2.1.0-windows-x86_64.zip' "$OUTPUT/zap-2.1.0-manifest.json"
test "$(wc -l < "$OUTPUT/zap-2.1.0-checksums.sha256")" -eq 3

RELEASE_REF=refs/tags/v2.1.0 \
RELEASE_COMMIT=fixture-commit \
WORKFLOW_RUN_ID=fixture-run \
  "$ROOT/scripts/aggregate_release_manifest.sh" 2.1.0 "$ARTIFACTS" "$OUTPUT" >/tmp/zap-manifest-rerun.out

grep -Fq 'artifact manifest: passed' /tmp/zap-manifest-rerun.out

rm -f "$ARTIFACTS/zap-2.1.0-windows-x86_64.zip"
if "$ROOT/scripts/aggregate_release_manifest.sh" 2.1.0 "$ARTIFACTS" "$WORK/failure-output" >/tmp/zap-manifest-failure.out 2>&1; then
  echo 'expected missing-artifact failure did not occur' >&2
  exit 1
fi
grep -Fq 'missing release archive: zap-2.1.0-windows-x86_64.zip' /tmp/zap-manifest-failure.out

echo 'aggregate release manifest fixture tests passed'
