#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

VERSION=${EXPECTED_VERSION:-$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1)}
REPORT=${ZAP_FRAMEWORK_REPORT:-$ROOT_DIR/target/framework-starters.tsv}
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

starters=(web mobile ai iot)
for name in "${starters[@]}"; do
  dir="frameworks/$name"
  manifest="$dir/zap.toml"
  lockfile="$dir/zap.lock"
  source="$dir/main.zp"

  require_file "$manifest"
  require_file "$lockfile"
  require_file "$source"

  if [[ -f "$manifest" ]]; then
    if grep -Eq '^name[[:space:]]*=[[:space:]]*"zap-framework-[a-z-]+-contract"' "$manifest"; then
      record PASS "manifest-contract-name:$manifest"
    else
      record FAIL "manifest-contract-name:$manifest"
    fi
    if grep -Fq 'main = "main.zp"' "$manifest"; then
      record PASS "manifest-entry:$manifest"
    else
      record FAIL "manifest-entry:$manifest"
    fi
    if grep -Fq 'status = "contract-prototype"' "$manifest"; then
      record PASS "manifest-status:$manifest"
    else
      record FAIL "manifest-status:$manifest"
    fi
    if awk '
      /^\[dependencies\]/{inside=1; next}
      /^\[/{inside=0}
      inside && $0 !~ /^[[:space:]]*(#|$)/ {bad=1}
      END {exit bad ? 1 : 0}
    ' "$manifest"; then
      record PASS "dependency-free:$manifest"
    else
      record FAIL "unexpected-dependency:$manifest"
    fi
  fi

  if [[ -f "$lockfile" ]] && grep -Fq 'lockfile_version = 1' "$lockfile" && grep -Fq "name = \"zap-framework-" "$lockfile"; then
    record PASS "canonical-lockfile:$lockfile"
  else
    record FAIL "canonical-lockfile:$lockfile"
  fi

  if [[ -f "$source" ]]; then
    if grep -Eq '^(use (web|mobile|ai|iot)|app\.|device\.|assistant[[:space:]]*=)' "$source"; then
      record FAIL "unsupported-placeholder-syntax:$source"
    else
      record PASS "current-zap-syntax:$source"
    fi
    if grep -Fq 'contract' "$source"; then
      record PASS "contract-marker:$source"
    else
      record FAIL "contract-marker:$source"
    fi
  fi
done

require_file frameworks/web/web_contract.zp
require_file frameworks/web/web_contract_test.zp
require_file frameworks/web/api_contract.zp
require_file frameworks/web/api_contract_test.zp
require_file frameworks/web/dto_contract.zp
require_file frameworks/web/database_contract.zp
require_file frameworks/web/auth_contract.zp
require_file frameworks/web/rate_limit_contract.zp
require_file docs/FRAMEWORK_EN.md
require_file docs/FRAMEWORK_MM.md
require_file docs/WEB_FRAMEWORK_EN.md
require_file docs/WEB_FRAMEWORK_MM.md
require_file docs/ZAP_HOST_EN.md
require_file docs/ZAP_HOST_MM.md
require_text docs/FRAMEWORK_EN.md "Framework Foundation v0.1"
require_text docs/FRAMEWORK_MM.md "Framework Foundation v0.1"
require_text docs/WEB_FRAMEWORK_EN.md "Web Foundation v0.2"
require_text docs/WEB_FRAMEWORK_MM.md "Web Foundation v0.2"
require_text docs/WEB_FRAMEWORK_EN.md "api_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "api_contract.zp"
require_text docs/WEB_FRAMEWORK_EN.md "database_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "database_contract.zp"
require_text docs/WEB_FRAMEWORK_EN.md "rate_limit_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "rate_limit_contract.zp"
require_text docs/ZAP_HOST_EN.md "host/zap-host"
require_text docs/ZAP_HOST_MM.md "host/zap-host"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "FRAMEWORK_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "FRAMEWORK_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "WEB_FRAMEWORK_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "WEB_FRAMEWORK_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "ZAP_HOST_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "ZAP_HOST_MM.md"
for host_file in \
  host/zap-host/Cargo.toml \
  host/zap-host/Cargo.lock \
  host/zap-host/README.md \
  host/zap-host/src/lib.rs \
  host/zap-host/src/main.rs \
  host/zap-host/tests/http_contract.rs; do
  require_file "$host_file"
done

if command -v cargo >/dev/null 2>&1; then
  if cargo fmt --manifest-path host/zap-host/Cargo.toml -- --check >/dev/null 2>&1; then
    record PASS "host-format:host/zap-host"
  else
    record FAIL "host-format:host/zap-host"
  fi
  if cargo clippy --manifest-path host/zap-host/Cargo.toml --all-targets --all-features -- -D warnings >/dev/null 2>&1; then
    record PASS "host-clippy:host/zap-host"
  else
    record FAIL "host-clippy:host/zap-host"
  fi
  if cargo test --manifest-path host/zap-host/Cargo.toml --all-targets >/dev/null 2>&1; then
    record PASS "host-test:host/zap-host"
  else
    record FAIL "host-test:host/zap-host"
  fi
else
  record PASS "host-quality:skipped-no-cargo"
fi

ZAP_BIN=${ZAP_BIN:-}
if [[ -z "$ZAP_BIN" && -x "$ROOT_DIR/target/release/zap" ]]; then
  ZAP_BIN="$ROOT_DIR/target/release/zap"
fi
if [[ -z "$ZAP_BIN" ]]; then
  ZAP_BIN=$(command -v zap || true)
fi

if [[ -n "$ZAP_BIN" && -x "$ZAP_BIN" ]]; then
  record PASS "runtime-binary:$ZAP_BIN"
    for name in "${starters[@]}"; do
    dir="frameworks/$name"
    output=$(mktemp)

    cleanup() { rm -f "$output"; }
    trap cleanup RETURN
    if "$ZAP_BIN" check "$dir" >>"$output" 2>&1 && "$ZAP_BIN" build "$dir" >>"$output" 2>&1 && "$ZAP_BIN" run "$dir/main.zp" >>"$output" 2>&1; then
      if grep -Fq '"contract"' "$output"; then
        record PASS "runtime-smoke:$dir"
      else
        record FAIL "runtime-smoke-output:$dir"
      fi
    else
      record FAIL "runtime-smoke:$dir"
    fi
    if [[ "$name" == "web" ]]; then
      if "$ZAP_BIN" test "$dir" >>"$output" 2>&1; then
        record PASS "runtime-test:$dir"
      else
        record FAIL "runtime-test:$dir"
      fi
    fi
    trap - RETURN
    cleanup
  done
elif [[ "${ZAP_FRAMEWORK_DOCS_ONLY:-0}" == "1" ]]; then
  record PASS "runtime-smoke:skipped-docs-only"
else
  record FAIL "runtime-binary:missing; set ZAP_BIN or build target/release/zap"
fi

if (( failures > 0 )); then
  printf 'framework starter validation failed: %d failure(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'framework starter validation passed: %d checks; report=%s\n' "$pass" "$REPORT"
