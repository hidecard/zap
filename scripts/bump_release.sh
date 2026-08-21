#!/usr/bin/env bash
# Zap release version and changelog automation
#
# The script is intentionally dry-run by default. It changes only the package
# version and changelog files; it never creates tags, pushes commits, publishes
# artifacts, or handles release secrets.
#
# Usage:
#   bash scripts/bump_release.sh 2.1.1 \
#     --en-note 'Added release preflight automation.' \
#     --mm-note 'Release preflight automation ကို ထည့်သွင်းထားပါသည်။'
#
# Apply changes only after reviewing the dry-run:
#   bash scripts/bump_release.sh 2.1.1 --apply \
#     --en-note 'Added release preflight automation.' \
#     --mm-note 'Release preflight automation ကို ထည့်သွင်းထားပါသည်။'
#
# Optional flags:
#   --from-version VERSION  Require the current Cargo version to match VERSION.
#   --date YYYY-MM-DD        Release date; defaults to UTC today.
#   --summary TEXT           Short summary for the canonical CHANGELOG.md.
#   --en-note TEXT           English release note; required for parity.
#   --mm-note TEXT           Burmese release note; required for parity.
#   --apply                  Replace files. Without this flag, show a diff only.
#   --skip-preflight         Do not run scripts/release_preflight.sh after apply.

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT_DIR" ]]; then
  echo "release bump: must run inside the Zap Git repository" >&2
  exit 1
fi
cd "$ROOT_DIR"

usage() {
  sed -n '1,36p' "$0"
}

fail() {
  echo "release bump: $*" >&2
  exit 1
}

VERSION=""
FROM_VERSION=""
RELEASE_DATE="$(date -u +%F)"
SUMMARY=""
EN_NOTE=""
MM_NOTE=""
APPLY=0
SKIP_PREFLIGHT=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --help|-h)
      usage
      exit 0
      ;;
    --from-version)
      [[ $# -ge 2 ]] || fail "--from-version requires a value"
      FROM_VERSION="$2"
      shift 2
      ;;
    --date)
      [[ $# -ge 2 ]] || fail "--date requires a value"
      RELEASE_DATE="$2"
      shift 2
      ;;
    --summary)
      [[ $# -ge 2 ]] || fail "--summary requires a value"
      SUMMARY="$2"
      shift 2
      ;;
    --en-note)
      [[ $# -ge 2 ]] || fail "--en-note requires a value"
      EN_NOTE="$2"
      shift 2
      ;;
    --mm-note)
      [[ $# -ge 2 ]] || fail "--mm-note requires a value"
      MM_NOTE="$2"
      shift 2
      ;;
    --apply)
      APPLY=1
      shift
      ;;
    --skip-preflight)
      SKIP_PREFLIGHT=1
      shift
      ;;
    --*)
      fail "unknown option: $1"
      ;;
    *)
      [[ -z "$VERSION" ]] || fail "only one target version may be supplied"
      VERSION="$1"
      shift
      ;;
  esac
done

[[ -n "$VERSION" ]] || { usage >&2; fail "target version is required"; }
[[ "$VERSION" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)([-+][0-9A-Za-z.-]+)?$ ]] || \
  fail "target version is not valid semver: $VERSION"
[[ "$RELEASE_DATE" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || \
  fail "release date must use YYYY-MM-DD: $RELEASE_DATE"
[[ -n "$EN_NOTE" ]] || fail "--en-note is required to maintain English changelog parity"
[[ -n "$MM_NOTE" ]] || fail "--mm-note is required to maintain Burmese changelog parity"
[[ "$EN_NOTE" != *$'\n'* && "$MM_NOTE" != *$'\n'* ]] || fail "release notes must be single-line values"

cargo_file="native/Cargo.toml"
[[ -f "$cargo_file" ]] || fail "missing $cargo_file"
CURRENT_VERSION="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' "$cargo_file" | head -n 1)"
[[ -n "$CURRENT_VERSION" ]] || fail "could not read the native Cargo version"
[[ -z "$FROM_VERSION" || "$FROM_VERSION" == "$CURRENT_VERSION" ]] || \
  fail "current version is $CURRENT_VERSION, not the required $FROM_VERSION"
[[ "$CURRENT_VERSION" != "$VERSION" ]] || fail "target version is already current: $VERSION"

for file in CHANGELOG_EN.md CHANGELOG_MM.md CHANGELOG.md; do
  [[ -f "$file" ]] || fail "missing changelog: $file"
  grep -Fqx '## [Unreleased]' "$file" || fail "$file has no exact '## [Unreleased]' heading"
done

if [[ -n "$(git status --porcelain)" && "$APPLY" == "1" ]]; then
  fail "working tree is dirty; commit or stash existing changes before --apply"
fi

SUMMARY="${SUMMARY:-$EN_NOTE}"

make_version_file() {
  local input="$1" output="$2"
  awk -v version="$VERSION" '
    BEGIN { replaced = 0 }
    /^version[[:space:]]*=[[:space:]]*"[^"]*"[[:space:]]*$/ && replaced == 0 {
      print "version = \"" version "\""
      replaced = 1
      next
    }
    { print }
    END { if (replaced != 1) exit 2 }
  ' "$input" > "$output" || fail "failed to update version in $input"
}

make_changelog_file() {
  local input="$1" output="$2" note="$3" heading="$4"
  awk -v version="$VERSION" -v release_date="$RELEASE_DATE" -v note="$note" -v heading="$heading" '
    BEGIN { inserted = 0 }
    {
      print
      if (inserted == 0 && $0 == "## [Unreleased]") {
        print ""
        print "## [" version "] — " release_date
        print ""
        print "### " heading
        print "- " note
        inserted = 1
      }
    }
    END { if (inserted != 1) exit 2 }
  ' "$input" > "$output" || fail "failed to update changelog $input"
}

make_canonical_changelog() {
  local input="$1" output="$2"
  awk -v version="$VERSION" -v release_date="$RELEASE_DATE" -v note="$SUMMARY" '
    BEGIN { inserted = 0 }
    {
      print
      if (inserted == 0 && $0 == "## [Unreleased]") {
        print ""
        print "## [" version "] - " release_date
        print ""
        print "### Release summary"
        print "- " note
        inserted = 1
      }
    }
    END { if (inserted != 1) exit 2 }
  ' "$input" > "$output" || fail "failed to update canonical changelog $input"
}

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

make_version_file native/Cargo.toml "$WORK_DIR/Cargo.toml"
make_changelog_file CHANGELOG_EN.md "$WORK_DIR/CHANGELOG_EN.md" "$EN_NOTE" "Release engineering"
make_changelog_file CHANGELOG_MM.md "$WORK_DIR/CHANGELOG_MM.md" "$MM_NOTE" "Release engineering"
make_canonical_changelog CHANGELOG.md "$WORK_DIR/CHANGELOG.md"

for file in "$WORK_DIR/CHANGELOG_EN.md" "$WORK_DIR/CHANGELOG_MM.md" "$WORK_DIR/CHANGELOG.md"; do
  grep -Fq -- "## [$VERSION]" "$file" || fail "generated changelog does not contain version $VERSION: $file"
done
grep -Fqx "version = \"$VERSION\"" "$WORK_DIR/Cargo.toml" || fail "generated Cargo.toml has no exact target version"

printf '%s\n' "Zap release bump: $CURRENT_VERSION -> $VERSION"
printf '%s\n' "Release date: $RELEASE_DATE"
printf '%s\n' "Mode: $([[ "$APPLY" == "1" ]] && echo apply || echo dry-run)"
printf '%s\n' '--- proposed changes'
for pair in \
  "native/Cargo.toml:$WORK_DIR/Cargo.toml" \
  "CHANGELOG_EN.md:$WORK_DIR/CHANGELOG_EN.md" \
  "CHANGELOG_MM.md:$WORK_DIR/CHANGELOG_MM.md" \
  "CHANGELOG.md:$WORK_DIR/CHANGELOG.md"; do
  original="${pair%%:*}"
  generated="${pair##*:}"
  diff -u --label "$original" --label "$original (generated)" "$original" "$generated" || true
done

if [[ "$APPLY" != "1" ]]; then
  printf '%s\n' 'Dry-run complete. Re-run with --apply after reviewing the diff.'
  exit 0
fi

install -m 0644 "$WORK_DIR/Cargo.toml" native/Cargo.toml
install -m 0644 "$WORK_DIR/CHANGELOG_EN.md" CHANGELOG_EN.md
install -m 0644 "$WORK_DIR/CHANGELOG_MM.md" CHANGELOG_MM.md
install -m 0644 "$WORK_DIR/CHANGELOG.md" CHANGELOG.md

if [[ "$SKIP_PREFLIGHT" != "1" && -x scripts/release_preflight.sh ]]; then
  RELEASE_TAG="" EXPECTED_VERSION="$VERSION" ALLOW_DIRTY=1 RUN_CARGO_CHECKS=0 \
    SKIP_DEPLOYMENT_VALIDATION=0 bash scripts/release_preflight.sh
fi

printf '%s\n' 'Release bump applied successfully.'
printf '%s\n' 'Review the diff, run the full release preflight, commit the changes, and create/push the tag separately.'
