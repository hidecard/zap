#!/usr/bin/env bash
# Zap P0 release version single-source-of-truth validator.
# The native Cargo package version is authoritative. Every user-facing release
# surface and, when supplied, the release tag must agree with it.

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT_DIR" ]]; then
  printf '%s\n' 'version validation: must run inside the Zap Git repository' >&2
  exit 1
fi
cd "$ROOT_DIR"

EXPECTED_VERSION="${1:-${EXPECTED_VERSION:-}}"
RELEASE_TAG="${RELEASE_TAG:-}"
if [[ -z "$RELEASE_TAG" && "${GITHUB_REF_NAME:-}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([.-].*)?$ ]]; then
  RELEASE_TAG="$GITHUB_REF_NAME"
fi
REPORT="${ZAP_VERSION_REPORT:-$ROOT_DIR/target/version-consistency.tsv}"
mkdir -p "$(dirname "$REPORT")"

failures=0
rows=0
: > "$REPORT"
printf 'source\texpected\tobserved\tstatus\n' >> "$REPORT"

record() {
  local source="$1"
  local expected="$2"
  local observed="$3"
  local status="$4"
  rows=$((rows + 1))
  printf '%s\t%s\t%s\t%s\n' "$source" "$expected" "$observed" "$status" >> "$REPORT"
  if [[ "$status" == "PASS" ]]; then
    printf 'PASS: %s = %s\n' "$source" "$observed"
  else
    printf 'FAIL: %s expected %s but observed %s\n' "$source" "$expected" "$observed" >&2
    failures=$((failures + 1))
  fi
}

require_text() {
  local source="$1"
  local expected_text="$2"
  if grep -Fq -- "$expected_text" "$source"; then
    record "$source" "$expected_text" "$expected_text" PASS
  else
    record "$source" "$expected_text" '<missing>' FAIL
  fi
}

read_cargo_version() {
  sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1
}

read_lock_version() {
  awk '
    $0 == "name = \"zap-native\"" { found = 1; next }
    found && $0 ~ /^version = / {
      sub(/^version = "/, "", $0)
      sub(/"$/, "", $0)
      print
      exit
    }
    found && /^\[\[package\]\]/ { exit }
  ' native/Cargo.lock
}

read_cli_version() {
  local output
  if [[ -n "${ZAP_CLI_BINARY:-}" ]]; then
    output="$($ZAP_CLI_BINARY --version)"
  else
    output="$(cargo run --quiet --manifest-path native/Cargo.toml -- --version 2>/dev/null)"
  fi
  printf '%s\n' "$output" | sed -n 's/^zap \([^[:space:]]*\) .*/\1/p' | head -n 1
}

cargo_version="$(read_cargo_version)"
if [[ -z "$cargo_version" ]]; then
  printf '%s\n' 'version validation: native Cargo version is missing' >&2
  exit 1
fi
if [[ -z "$EXPECTED_VERSION" ]]; then
  EXPECTED_VERSION="$cargo_version"
fi

if [[ "$EXPECTED_VERSION" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]]; then
  record 'semver' "$EXPECTED_VERSION" "$EXPECTED_VERSION" PASS
else
  record 'semver' 'valid semantic version' "$EXPECTED_VERSION" FAIL
fi
record 'native/Cargo.toml' "$EXPECTED_VERSION" "$cargo_version" "$([[ "$cargo_version" == "$EXPECTED_VERSION" ]] && printf PASS || printf FAIL)"

lock_version="$(read_lock_version)"
record 'native/Cargo.lock zap-native' "$EXPECTED_VERSION" "${lock_version:-<missing>}" "$([[ "$lock_version" == "$EXPECTED_VERSION" ]] && printf PASS || printf FAIL)"

cli_version="$(read_cli_version)"
record 'zap --version' "$EXPECTED_VERSION" "${cli_version:-<missing>}" "$([[ "$cli_version" == "$EXPECTED_VERSION" ]] && printf PASS || printf FAIL)"

if [[ -n "$RELEASE_TAG" ]]; then
  if [[ "$RELEASE_TAG" == v* ]]; then
    tag_version="${RELEASE_TAG#v}"
    record 'release tag' "$EXPECTED_VERSION" "$tag_version" "$([[ "$tag_version" == "$EXPECTED_VERSION" ]] && printf PASS || printf FAIL)"
  else
    record 'release tag' "v$EXPECTED_VERSION" "$RELEASE_TAG" FAIL
  fi
else
  printf '%s\n' 'WARN: RELEASE_TAG is not set; tag/version comparison was skipped'
fi

for changelog in CHANGELOG.md CHANGELOG_EN.md CHANGELOG_MM.md; do
  require_text "$changelog" "$EXPECTED_VERSION"
done

require_text README.md "| Current release line | \`v$EXPECTED_VERSION\` |"
require_text README.md "releases/tag/v$EXPECTED_VERSION"
require_text README.md "zap-$EXPECTED_VERSION-linux-x86_64.tar.gz"
require_text README.md "zap-$EXPECTED_VERSION-macos-arm64.tar.gz"
require_text README.md "zap-$EXPECTED_VERSION-windows-x86_64.zip"
require_text README_MM.md "| လက်ရှိ release line | \`v$EXPECTED_VERSION\` |"
require_text README_MM.md "releases/tag/v$EXPECTED_VERSION"
require_text README_MM.md "zap-$EXPECTED_VERSION-linux-x86_64.tar.gz"
require_text README_MM.md "zap-$EXPECTED_VERSION-macos-arm64.tar.gz"
require_text README_MM.md "zap-$EXPECTED_VERSION-windows-x86_64.zip"

major_minor="${EXPECTED_VERSION%%.*}.${EXPECTED_VERSION#*.}"
major_minor="${major_minor%%.*}.$(printf '%s' "${EXPECTED_VERSION#*.}" | cut -d. -f1)"
require_text SECURITY.md "Latest \`v${major_minor}.x\`"
require_text SECURITY.md "releases/tag/v$EXPECTED_VERSION"
require_text docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md "$EXPECTED_VERSION"
require_text docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md "$EXPECTED_VERSION"
require_text "docs/RELEASE_${EXPECTED_VERSION}_EN.md" "$EXPECTED_VERSION"
require_text "docs/RELEASE_${EXPECTED_VERSION}_MM.md" "$EXPECTED_VERSION"

if grep -Eq 'v[0-9]+\.[0-9]+\.[0-9]+' .github/release_template.md; then
  record '.github/release_template.md' 'placeholder release version' 'hard-coded semver found' FAIL
else
  record '.github/release_template.md' 'placeholder release version' 'no hard-coded semver' PASS
fi

for installer in install.sh install_windows.bat; do
  if grep -Eq 'v?[0-9]+\.[0-9]+\.[0-9]+' "$installer"; then
    record "$installer" 'version-agnostic installer' 'hard-coded semver found' FAIL
  else
    record "$installer" 'version-agnostic installer' 'no hard-coded semver' PASS
  fi
done

if (( failures > 0 )); then
  printf 'version validation failed: %s issue(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'version validation passed: %s checks; report=%s\n' "$rows" "$REPORT"
