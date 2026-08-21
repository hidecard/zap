#!/usr/bin/env bash
# Regression tests for the P0 release version single-source-of-truth gate.

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CURRENT_VERSION="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1)"
[[ -n "$CURRENT_VERSION" ]]
DRIFT_VERSION="9.9.9"
[[ "$DRIFT_VERSION" != "$CURRENT_VERSION" ]] || DRIFT_VERSION="8.8.8"
TAG_DRIFT_VERSION="0.0.0"
[[ "$TAG_DRIFT_VERSION" != "$CURRENT_VERSION" ]] || TAG_DRIFT_VERSION="0.0.1"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

EXPECTED_VERSION="$CURRENT_VERSION" \
  RELEASE_TAG="v$CURRENT_VERSION" \
  ZAP_VERSION_REPORT="$WORK_DIR/pass.tsv" \
  scripts/validate_release_version.sh "$CURRENT_VERSION" > "$WORK_DIR/pass.log"
grep -Fq "$(printf 'release tag\t%s\t%s\tPASS' "$CURRENT_VERSION" "$CURRENT_VERSION")" "$WORK_DIR/pass.tsv"
grep -Fq 'version validation passed:' "$WORK_DIR/pass.log"

if EXPECTED_VERSION="$DRIFT_VERSION" \
  ZAP_VERSION_REPORT="$WORK_DIR/version-drift.tsv" \
  scripts/validate_release_version.sh "$DRIFT_VERSION" > "$WORK_DIR/version-drift.log" 2>&1; then
  printf '%s\n' 'version validator regression: expected-version drift was accepted' >&2
  exit 1
fi
grep -Fq "$(printf 'native/Cargo.toml\t%s\t%s\tFAIL' "$DRIFT_VERSION" "$CURRENT_VERSION")" "$WORK_DIR/version-drift.tsv"

if EXPECTED_VERSION="$CURRENT_VERSION" \
  RELEASE_TAG="v$TAG_DRIFT_VERSION" \
  ZAP_VERSION_REPORT="$WORK_DIR/tag-drift.tsv" \
  scripts/validate_release_version.sh "$CURRENT_VERSION" > "$WORK_DIR/tag-drift.log" 2>&1; then
  printf '%s\n' 'version validator regression: tag drift was accepted' >&2
  exit 1
fi
grep -Fq "$(printf 'release tag\t%s\t%s\tFAIL' "$CURRENT_VERSION" "$TAG_DRIFT_VERSION")" "$WORK_DIR/tag-drift.tsv"

printf '%s\n' 'version consistency regression tests: passed'
