#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SOURCE_VALIDATOR="$ROOT_DIR/scripts/validate_documentation_consistency.sh"
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-doc-consistency.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT
mkdir -p "$TMP_DIR/scripts" "$TMP_DIR/native" "$TMP_DIR/docs"
cp "$SOURCE_VALIDATOR" "$TMP_DIR/scripts/validate_documentation_consistency.sh"
chmod +x "$TMP_DIR/scripts/validate_documentation_consistency.sh"
printf '[package]\nname = "zap-native"\nversion = "2.1.14"\n' > "$TMP_DIR/native/Cargo.toml"
printf '%s\n' 'documentation navigation' > "$TMP_DIR/README.md"
printf '%s\n' 'documentation navigation' > "$TMP_DIR/README_MM.md"

pairs=(
  DOCUMENTATION_NAVIGATION_EN DOCUMENTATION_NAVIGATION_MM
  SYNTAX_GUIDE_EN SYNTAX_GUIDE
  LANGUAGE_SPEC_EN LANGUAGE_SPEC_MM
  ASYNC_BOUNDARIES_EN ASYNC_BOUNDARIES_MM
  BENCHMARK_HARNESS_EN BENCHMARK_HARNESS_MM
  TYPECHECK_GENERIC_DESIGN_EN TYPECHECK_GENERIC_DESIGN_MM
  TYPECHECK_GENERIC_DECLARATIONS_A3_EN TYPECHECK_GENERIC_DECLARATIONS_A3_MM
  RUNTIME_STATE_EN RUNTIME_STATE_MM
  MEMORY_BUDGET_OBJECT_STORE_EN MEMORY_BUDGET_OBJECT_STORE_MM
  P0_FOUNDATION_STATUS_EN P0_FOUNDATION_STATUS_MM
  STDLIB_POLICY_EN STDLIB_POLICY_MM
  TRAITS_RFC_EN TRAITS_RFC_MM
  P2_PROGRESS P2_PROGRESS_MM
  FRAMEWORK_EN FRAMEWORK_MM
  WEB_FRAMEWORK_EN WEB_FRAMEWORK_MM
  ZAP_HOST_EN ZAP_HOST_MM
  ZAP_HOST_QUICKSTART_EN ZAP_HOST_QUICKSTART_MM
  ZAP_WEB_NATIVE_EN ZAP_WEB_NATIVE_MM
  CURRENT_STATUS_EN CURRENT_STATUS_MM
  POST_V2.2.0_REMEDIATION_EN POST_V2.2.0_REMEDIATION_MM
  POST_V2.4.0_REMEDIATION_EN POST_V2.4.0_REMEDIATION_MM
  USAGE_EN USAGE
  RELEASE_2.1.14_EN RELEASE_2.1.14_MM
)
for file in "${pairs[@]}"; do
  cat >"$TMP_DIR/docs/$file.md" <<'DOC'
# Zap v2.1.14

## Contract

```text
contract
```
DOC
done
printf '%s\n' 'benchmark-results/native-summary.csv' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'benchmark-results/native-summary.csv' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'FRAMEWORK_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'FRAMEWORK_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'WEB_FRAMEWORK_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'WEB_FRAMEWORK_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'ZAP_HOST_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'ZAP_HOST_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'ZAP_HOST_QUICKSTART_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'ZAP_HOST_QUICKSTART_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'ZAP_WEB_NATIVE_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'ZAP_WEB_NATIVE_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'CURRENT_STATUS_EN.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_EN.md"
printf '%s\n' 'CURRENT_STATUS_MM.md' >> "$TMP_DIR/docs/DOCUMENTATION_NAVIGATION_MM.md"
printf '%s\n' 'docs/DOCUMENTATION_NAVIGATION_EN.md' >> "$TMP_DIR/README.md"
printf '%s\n' 'docs/DOCUMENTATION_NAVIGATION_MM.md' >> "$TMP_DIR/README.md"
printf '%s\n' 'docs/DOCUMENTATION_NAVIGATION_EN.md' >> "$TMP_DIR/README_MM.md"
printf '%s\n' 'docs/DOCUMENTATION_NAVIGATION_MM.md' >> "$TMP_DIR/README_MM.md"

(cd "$TMP_DIR" && EXPECTED_VERSION=2.1.14 ZAP_DOCS_REPORT="$TMP_DIR/positive.tsv" scripts/validate_documentation_consistency.sh >/dev/null)

sed -i 's/v2\.1\.14/v2.1.13/' "$TMP_DIR/docs/SYNTAX_GUIDE_EN.md"
if (cd "$TMP_DIR" && EXPECTED_VERSION=2.1.14 scripts/validate_documentation_consistency.sh >/dev/null 2>&1); then
  printf 'expected version drift to fail\n' >&2
  exit 1
fi
sed -i 's/v2\.1\.13/v2.1.14/' "$TMP_DIR/docs/SYNTAX_GUIDE_EN.md"
printf '\n## Drift\n' >> "$TMP_DIR/docs/LANGUAGE_SPEC_MM.md"
if (cd "$TMP_DIR" && EXPECTED_VERSION=2.1.14 scripts/validate_documentation_consistency.sh >/dev/null 2>&1); then
  printf 'expected section parity drift to fail\n' >&2
  exit 1
fi

printf 'documentation consistency regression harness passed\n'
