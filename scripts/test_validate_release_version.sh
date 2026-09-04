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

GITHUB_REF_NAME=master \
  EXPECTED_VERSION="$CURRENT_VERSION" \
  ZAP_VERSION_REPORT="$WORK_DIR/branch-ref.tsv" \
  scripts/validate_release_version.sh "$CURRENT_VERSION" > "$WORK_DIR/branch-ref.log"
if grep -Fq $'release tag\t' "$WORK_DIR/branch-ref.tsv"; then
  printf '%s\n' 'version validator regression: branch ref was treated as a release tag' >&2
  exit 1
fi

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

# Regression for core.autocrlf=true Windows checkouts: the lockfile may carry
# trailing CR bytes that previously caused the package match to silently skip
# and report <missing>. The validator must remain correct under that checkout.
CRLF_WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR" "$CRLF_WORK_DIR" "$BIN_LAG_WORK_DIR"' EXIT
CRLF_LOCK="$CRLF_WORK_DIR/native/Cargo.lock"
mkdir -p "$(dirname "$CRLF_LOCK")"
# Build a lockfile-shaped copy of the real one, but with CRLF endings, to
# mirror what a Windows checkout produces.
{ tr -d '\r' < native/Cargo.lock; } > "$CRLF_LOCK.tmp"
python3 - "$CRLF_LOCK.tmp" "$CRLF_LOCK" <<'PY'
import sys
src, dst = sys.argv[1], sys.argv[2]
with open(src, "rb") as fh:
    data = fh.read()
with open(dst, "wb") as fh:
    fh.write(data.replace(b"\n", b"\r\n"))
PY
rm -f "$CRLF_LOCK.tmp"

CRLF_AWK_OUTPUT="$(
  awk '
    { sub(/\r$/, "") }
    $0 == "name = \"zap-native\"" { found = 1; next }
    found && $0 ~ /^version = / {
      sub(/^version = "/, "", $0)
      sub(/"$/, "", $0)
      print
      exit
    }
    found && /^\[\[package\]\]/ { exit }
  ' "$CRLF_LOCK"
)"
[[ "$CRLF_AWK_OUTPUT" == "$CURRENT_VERSION" ]] || {
  printf '%s\n' "version validator regression: CRLF lockfile parsing failed (got '$CRLF_AWK_OUTPUT', expected '$CURRENT_VERSION')" >&2
  exit 1
}

# Regression for native-binary lag: when ZAP_CLI_BINARY points at a binary
# whose `--version` output reports an older release line than native/Cargo.toml,
# the validator must report FAIL on the `zap --version` row instead of
# silently passing. Reproduces the exact mode where the committed bin/zap was
# last rebuilt against an earlier release.
BIN_LAG_WORK_DIR="$(mktemp -d)"
LAG_BIN="$BIN_LAG_WORK_DIR/lagged-zap"
cat > "$LAG_BIN" <<EOF
#!/usr/bin/env bash
printf 'zap ${DRIFT_VERSION} (native)\n'
EOF
chmod +x "$LAG_BIN"

if ZAP_CLI_BINARY="$LAG_BIN" \
   EXPECTED_VERSION="$CURRENT_VERSION" \
   ZAP_VERSION_REPORT="$BIN_LAG_WORK_DIR/binary-drift.tsv" \
   scripts/validate_release_version.sh "$CURRENT_VERSION" > "$BIN_LAG_WORK_DIR/binary-drift.log" 2>&1; then
  printf '%s\n' "version validator regression: lagged binary was accepted (validator exit 0)" >&2
  exit 1
fi
grep -Fq "$(printf 'zap --version\t%s\t%s\tFAIL' "$CURRENT_VERSION" "$DRIFT_VERSION")" "$BIN_LAG_WORK_DIR/binary-drift.tsv" || {
  printf '%s\n' "version validator regression: expected FAIL row 'zap --version <expected> <observed> FAIL' missing in binary-drift.tsv" >&2
  cat "$BIN_LAG_WORK_DIR/binary-drift.tsv" >&2
  exit 1
}

# Companion regression: a fresh binary whose `--version` matches CURRENT_VERSION
# must report PASS so the validator continues to work for normal releases.
FRESH_BIN="$BIN_LAG_WORK_DIR/fresh-zap"
cat > "$FRESH_BIN" <<EOF
#!/usr/bin/env bash
printf 'zap ${CURRENT_VERSION} (native)\n'
EOF
chmod +x "$FRESH_BIN"

ZAP_CLI_BINARY="$FRESH_BIN" \
  EXPECTED_VERSION="$CURRENT_VERSION" \
  ZAP_VERSION_REPORT="$BIN_LAG_WORK_DIR/binary-fresh.tsv" \
  scripts/validate_release_version.sh "$CURRENT_VERSION" > "$BIN_LAG_WORK_DIR/binary-fresh.log" 2>&1
grep -Fq "$(printf 'zap --version\t%s\t%s\tPASS' "$CURRENT_VERSION" "$CURRENT_VERSION")" "$BIN_LAG_WORK_DIR/binary-fresh.tsv" || {
  printf '%s\n' "version validator regression: matching binary was reported as not-PASS" >&2
  cat "$BIN_LAG_WORK_DIR/binary-fresh.tsv" >&2
  exit 1
}

printf '%s\n' 'version consistency regression tests: passed'
