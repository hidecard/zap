#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

fail() {
  printf 'platform archive regression: %s\n' "$1" >&2
  exit 1
}

PYTHON="${ZAP_PYTHON:-}"
if [[ -z "$PYTHON" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    PYTHON=python3
  elif command -v python >/dev/null 2>&1; then
    PYTHON=python
  else
    fail 'python3 or python is required'
  fi
fi
command -v tar >/dev/null 2>&1 || fail 'tar is required'

SOURCE="$WORK_DIR/source"
ARCHIVE_ONE="$WORK_DIR/one.tar.gz"
ARCHIVE_TWO="$WORK_DIR/two.tar.gz"
mkdir -p "$SOURCE/nested"
printf 'line1\r\nline2\n' > "$SOURCE/crlf.txt"
printf 'nested\n' > "$SOURCE/nested/value.txt"

"$PYTHON" "$ROOT_DIR/scripts/create_deterministic_tar_gz.py" "$SOURCE" "$ARCHIVE_ONE"
"$PYTHON" "$ROOT_DIR/scripts/create_deterministic_tar_gz.py" "$SOURCE" "$ARCHIVE_TWO"
cmp "$ARCHIVE_ONE" "$ARCHIVE_TWO" || fail 'repeated archive bytes differ'

tar -tzf "$ARCHIVE_ONE" > "$WORK_DIR/listing"
expected_listing=$'zap/\nzap/nested/\nzap/crlf.txt\nzap/nested/value.txt'
printf '%s\n' "$expected_listing" | cmp - "$WORK_DIR/listing" || {
  cat "$WORK_DIR/listing" >&2
  fail 'archive member order or names differ'
}
tar -xOf "$ARCHIVE_ONE" zap/crlf.txt | cmp - "$SOURCE/crlf.txt" || fail 'archive changed newline payload bytes'

printf 'platform archive regression passed: members=%s sha256=' "$(wc -l < "$WORK_DIR/listing" | tr -d ' ')"
if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$ARCHIVE_ONE" | awk '{print $1}'
else
  shasum -a 256 "$ARCHIVE_ONE" | awk '{print $1}'
fi
