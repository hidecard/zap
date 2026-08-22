#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

catalog="native/src/stdlib_catalog.rs"
english="docs/STDLIB_POLICY_EN.md"
burmese="docs/STDLIB_POLICY_MM.md"

fail() {
  printf 'FAIL\t%s\n' "$1" >&2
  exit 1
}
pass() {
  printf 'PASS\t%s\n' "$1"
}

[[ -f "$catalog" ]] || fail "missing $catalog"
[[ -f "$english" ]] || fail "missing $english"
[[ -f "$burmese" ]] || fail "missing $burmese"

for marker in \
  'CATALOG_SCHEMA_VERSION: u32 = 1' \
  'pub(crate) const PUBLIC_DOMAINS' \
  'pub(crate) const PUBLIC_BUILTINS' \
  'deprecation_window' \
  'SemverPolicy' \
  'PlatformSupport' \
  'error_contract' \
  'deterministic: bool'; do
  grep -Fq -- "$marker" "$catalog" || fail "catalog marker missing: $marker"
done
pass "catalog contains the complete stability metadata schema"

domains=(text math collections filesystem json system time logging runtime async network process)
for domain in "${domains[@]}"; do
  grep -Fq -- "\`$domain\`" "$english" || fail "English policy is missing domain $domain"
  grep -Fq -- "\`$domain\`" "$burmese" || fail "Burmese policy is missing domain $domain"
  grep -Fq -- "\"$domain\"," "$catalog" || fail "catalog is missing domain $domain"
done
pass "all twelve public domains have catalog and bilingual policy entries"

for document in "$english" "$burmese"; do
  grep -Fq -- '2.1.14' "$document" || fail "$document is missing current release metadata"
  grep -Fq -- 'minor-compatible' "$document" || fail "$document is missing semver policy"
  grep -Fq -- 'Deprecation window' "$document" || fail "$document is missing deprecation metadata"
  grep -Fq -- 'Platform' "$document" || fail "$document is missing platform metadata"
  grep -Fq -- 'Deterministic' "$document" || fail "$document is missing determinism metadata"
done
pass "bilingual policy documents contain release, semver, deprecation, platform, and determinism fields"

english_sections="$(grep -c '^## ' "$english")"
burmese_sections="$(grep -c '^## ' "$burmese")"
[[ "$english_sections" == "$burmese_sections" ]] || fail "policy section counts differ: $english_sections vs $burmese_sections"
pass "English/Burmese policy section counts match"

grep -Fq -- 'standard_library_catalog_metadata_is_complete_and_unique' "$catalog" || fail "catalog metadata regression test missing"
grep -Fq -- 'standard_library_catalog_domain_order_is_deterministic' "$catalog" || fail "catalog ordering regression test missing"
pass "catalog metadata regression tests are present"
