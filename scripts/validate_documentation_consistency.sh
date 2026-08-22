#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

read_cargo_version() {
  sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1
}

VERSION=${EXPECTED_VERSION:-$(read_cargo_version)}
REPORT=${ZAP_DOCS_REPORT:-$ROOT_DIR/target/documentation-consistency.tsv}

if [[ -z "$VERSION" ]]; then
  printf 'documentation consistency: could not determine package version\n' >&2
  exit 1
fi
mkdir -p "$(dirname "$REPORT")"
: > "$REPORT"

pass=0
failures=0
record() {
  local status="$1"; shift
  printf '%s\t%s\n' "$status" "$*" | tee -a "$REPORT"
  if [[ "$status" == PASS ]]; then pass=$((pass + 1)); else failures=$((failures + 1)); fi
}

require_file() {
  local file="$1"
  if [[ -f "$file" ]]; then record PASS "file:$file"; else record FAIL "missing-file:$file"; fi
}

require_text() {
  local file="$1" text="$2"
  if [[ -f "$file" ]] && grep -Fq -- "$text" "$file"; then record PASS "text:$file:$text"; else record FAIL "missing-text:$file:$text"; fi
}

pairs=(
  'docs/DOCUMENTATION_NAVIGATION_EN.md:docs/DOCUMENTATION_NAVIGATION_MM.md'
  'docs/LANGUAGE_SPEC_EN.md:docs/LANGUAGE_SPEC_MM.md'
  'docs/ASYNC_BOUNDARIES_EN.md:docs/ASYNC_BOUNDARIES_MM.md'
  'docs/BENCHMARK_HARNESS_EN.md:docs/BENCHMARK_HARNESS_MM.md'
  'docs/TYPECHECK_GENERIC_DESIGN_EN.md:docs/TYPECHECK_GENERIC_DESIGN_MM.md'
  'docs/RUNTIME_STATE_EN.md:docs/RUNTIME_STATE_MM.md'
  'docs/MEMORY_BUDGET_OBJECT_STORE_EN.md:docs/MEMORY_BUDGET_OBJECT_STORE_MM.md'
  'docs/P0_FOUNDATION_STATUS_EN.md:docs/P0_FOUNDATION_STATUS_MM.md'
  'docs/STDLIB_POLICY_EN.md:docs/STDLIB_POLICY_MM.md'
  'docs/TRAITS_RFC_EN.md:docs/TRAITS_RFC_MM.md'
  'docs/P2_PROGRESS.md:docs/P2_PROGRESS_MM.md'
  'docs/FRAMEWORK_EN.md:docs/FRAMEWORK_MM.md'
  'docs/WEB_FRAMEWORK_EN.md:docs/WEB_FRAMEWORK_MM.md'
  'docs/ZAP_HOST_EN.md:docs/ZAP_HOST_MM.md'
  'docs/POST_V2.2.0_REMEDIATION_EN.md:docs/POST_V2.2.0_REMEDIATION_MM.md'
  "docs/RELEASE_${VERSION}_EN.md:docs/RELEASE_${VERSION}_MM.md"
)

for pair in "${pairs[@]}"; do
  en=${pair%%:*}
  mm=${pair##*:}
  require_file "$en"
  require_file "$mm"
  if [[ -f "$en" && -f "$mm" ]]; then
    en_sections=$(grep -c '^## ' "$en" || true)
    mm_sections=$(grep -c '^## ' "$mm" || true)
    if [[ "$en_sections" == "$mm_sections" ]]; then
      record PASS "section-parity:$en:$mm:$en_sections"
    else
      record FAIL "section-parity:$en:$mm:en=$en_sections:mm=$mm_sections"
    fi
    en_fences=$(grep -c '^```' "$en" || true)
    mm_fences=$(grep -c '^```' "$mm" || true)
    if [[ "$en_fences" == "$mm_fences" ]]; then
      record PASS "code-fence-parity:$en:$mm:$en_fences"
    else
      record FAIL "code-fence-parity:$en:$mm:en=$en_fences:mm=$mm_fences"
    fi
  fi
done

for file in \
  docs/DOCUMENTATION_NAVIGATION_EN.md docs/DOCUMENTATION_NAVIGATION_MM.md \
  docs/SYNTAX_GUIDE_EN.md docs/SYNTAX_GUIDE.md \
  docs/LANGUAGE_SPEC_EN.md docs/LANGUAGE_SPEC_MM.md \
  docs/ASYNC_BOUNDARIES_EN.md docs/ASYNC_BOUNDARIES_MM.md \
  docs/BENCHMARK_HARNESS_EN.md docs/BENCHMARK_HARNESS_MM.md \
  docs/TYPECHECK_GENERIC_DESIGN_EN.md docs/TYPECHECK_GENERIC_DESIGN_MM.md \
  docs/P2_PROGRESS.md docs/P2_PROGRESS_MM.md \
  docs/FRAMEWORK_EN.md docs/FRAMEWORK_MM.md \
  docs/WEB_FRAMEWORK_EN.md docs/WEB_FRAMEWORK_MM.md \
  docs/ZAP_HOST_EN.md docs/ZAP_HOST_MM.md \
  docs/RUNTIME_STATE_EN.md docs/RUNTIME_STATE_MM.md \
  docs/MEMORY_BUDGET_OBJECT_STORE_EN.md docs/MEMORY_BUDGET_OBJECT_STORE_MM.md \
  docs/P0_FOUNDATION_STATUS_EN.md docs/P0_FOUNDATION_STATUS_MM.md \
  docs/STDLIB_POLICY_EN.md docs/STDLIB_POLICY_MM.md \
  docs/TRAITS_RFC_EN.md docs/TRAITS_RFC_MM.md \
  "docs/RELEASE_${VERSION}_EN.md" "docs/RELEASE_${VERSION}_MM.md"; do
  require_text "$file" "v$VERSION"
  if grep -Eq 'v2\.1\.(0|6|7|8)([^0-9]|$)' "$file"; then
    record FAIL "stale-version:$file"
  else
    record PASS "no-stale-version:$file"
  fi
done

require_text README.md 'docs/DOCUMENTATION_NAVIGATION_EN.md'
require_text README.md 'docs/DOCUMENTATION_NAVIGATION_MM.md'
require_text README_MM.md 'docs/DOCUMENTATION_NAVIGATION_EN.md'
require_text README_MM.md 'docs/DOCUMENTATION_NAVIGATION_MM.md'
require_text docs/DOCUMENTATION_NAVIGATION_EN.md 'benchmark-results/native-summary.csv'
require_text docs/DOCUMENTATION_NAVIGATION_MM.md 'benchmark-results/native-summary.csv'
require_text docs/DOCUMENTATION_NAVIGATION_EN.md 'FRAMEWORK_EN.md'
require_text docs/DOCUMENTATION_NAVIGATION_MM.md 'FRAMEWORK_MM.md'
require_text docs/DOCUMENTATION_NAVIGATION_EN.md 'ZAP_HOST_EN.md'
require_text docs/DOCUMENTATION_NAVIGATION_MM.md 'ZAP_HOST_MM.md'

if (( failures > 0 )); then
  printf 'documentation consistency failed: %d failure(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'documentation consistency passed: %d checks; report=%s\n' "$pass" "$REPORT"
