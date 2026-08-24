#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

VERSION=${EXPECTED_VERSION:-$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1)}
REPORT=${ZAP_FRAMEWORK_REPORT:-$ROOT_DIR/target/framework-starters.tsv}
READY_PATH="/ready"
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
require_file frameworks/web/database_adapter.zp
require_file frameworks/web/database_adapter_test.zp
require_file frameworks/web/frontend_contract.zp
require_file frameworks/web/frontend_contract_test.zp
require_file frameworks/web/public/index.html
require_file frameworks/web/public/assets/app.css
require_file frameworks/web/public/assets/app.js
require_file frameworks/web/auth_contract.zp
require_file frameworks/web/rate_limit_contract.zp
require_file docs/FRAMEWORK_EN.md
require_file docs/FRAMEWORK_MM.md
require_file docs/WEB_FRAMEWORK_EN.md
require_file docs/WEB_FRAMEWORK_MM.md
require_file docs/ZAP_HOST_EN.md
require_file docs/ZAP_HOST_MM.md
require_file docs/ZAP_HOST_QUICKSTART_EN.md
require_file docs/ZAP_HOST_QUICKSTART_MM.md
require_file docs/ZAP_WEB_NATIVE_EN.md
require_file docs/ZAP_WEB_NATIVE_MM.md
require_file docs/FRONTEND_INTEGRATION_EN.md
require_file docs/FRONTEND_INTEGRATION_MM.md
require_text docs/FRAMEWORK_EN.md "Framework Foundation v0.1"
require_text docs/FRAMEWORK_MM.md "Framework Foundation v0.1"
require_text docs/WEB_FRAMEWORK_EN.md "Web Foundation v0.2"
require_text docs/WEB_FRAMEWORK_MM.md "Web Foundation v0.2"
require_text docs/WEB_FRAMEWORK_EN.md "api_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "api_contract.zp"
require_text docs/WEB_FRAMEWORK_EN.md "database_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "database_contract.zp"
require_text docs/WEB_FRAMEWORK_EN.md "database_adapter.zp"
require_text docs/WEB_FRAMEWORK_MM.md "database_adapter.zp"
require_text docs/WEB_FRAMEWORK_EN.md "frontend_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "frontend_contract.zp"
require_text docs/WEB_FRAMEWORK_EN.md "web_static"
require_text docs/WEB_FRAMEWORK_MM.md "web_static"
require_text docs/FRAMEWORK_EN.md "Self-contained runtime"
require_text docs/FRAMEWORK_MM.md "Self-contained runtime"
require_text docs/FRAMEWORK_EN.md "web_static_spa"
require_text docs/FRAMEWORK_MM.md "web_static_spa"
require_text docs/FRONTEND_INTEGRATION_EN.md "React, Vue, and Svelte"
require_text docs/FRONTEND_INTEGRATION_MM.md "React, Vue နှင့် Svelte"
require_text docs/WEB_FRAMEWORK_EN.md "rate_limit_contract.zp"
require_text docs/WEB_FRAMEWORK_MM.md "rate_limit_contract.zp"
require_text docs/ZAP_HOST_EN.md "host/zap-host"
require_text docs/ZAP_HOST_MM.md "host/zap-host"
require_text docs/ZAP_HOST_EN.md "/ready"
require_text docs/ZAP_HOST_MM.md "/ready"
require_text docs/ZAP_HOST_EN.md "ZAP_HOST_SHUTDOWN_TIMEOUT_MS"
require_text docs/ZAP_HOST_MM.md "ZAP_HOST_SHUTDOWN_TIMEOUT_MS"
require_text docs/ZAP_HOST_QUICKSTART_EN.md "cargo run"
require_text docs/ZAP_HOST_QUICKSTART_MM.md "cargo run"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap new"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap new"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap web check"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap web check"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap db check"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap db check"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap db plan"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap db plan"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap db inspect"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap db inspect"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap db migrate"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap db migrate"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap db migrate --check"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap db migrate --check"
require_text docs/ZAP_WEB_NATIVE_EN.md "zap dev"
require_text docs/ZAP_WEB_NATIVE_MM.md "zap dev"
require_text docs/ZAP_WEB_NATIVE_EN.md "ZAP_WEB_PORT"
require_text docs/ZAP_WEB_NATIVE_MM.md "ZAP_WEB_PORT"
require_text docs/ZAP_HOST_QUICKSTART_EN.md "$READY_PATH"
require_text docs/ZAP_HOST_QUICKSTART_MM.md "$READY_PATH"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "FRAMEWORK_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "FRAMEWORK_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "WEB_FRAMEWORK_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "WEB_FRAMEWORK_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "ZAP_HOST_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "ZAP_HOST_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "ZAP_WEB_NATIVE_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "ZAP_WEB_NATIVE_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "FRONTEND_INTEGRATION_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "FRONTEND_INTEGRATION_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "PRODUCTION_DEPLOYMENT_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "PRODUCTION_DEPLOYMENT_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "DATABASE_PRODUCTION_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "DATABASE_PRODUCTION_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "AUTH_OAUTH2_JWT_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "AUTH_OAUTH2_JWT_MM.md"
require_text docs/DOCUMENTATION_NAVIGATION_EN.md "LOAD_CHAOS_TESTING_EN.md"
require_text docs/DOCUMENTATION_NAVIGATION_MM.md "LOAD_CHAOS_TESTING_MM.md"
require_file docs/PRODUCTION_DEPLOYMENT_EN.md
require_file docs/PRODUCTION_DEPLOYMENT_MM.md
require_file docs/DATABASE_PRODUCTION_EN.md
require_file docs/DATABASE_PRODUCTION_MM.md
require_file docs/AUTH_OAUTH2_JWT_EN.md
require_file docs/AUTH_OAUTH2_JWT_MM.md
require_file docs/LOAD_CHAOS_TESTING_EN.md
require_file docs/LOAD_CHAOS_TESTING_MM.md
require_text docs/AUTH_OAUTH2_JWT_EN.md "ZAP_AUTH_MODE=jwt"
require_text docs/AUTH_OAUTH2_JWT_MM.md "ZAP_AUTH_MODE=jwt"
require_text docs/LOAD_CHAOS_TESTING_EN.md "load_zap_host.py"
require_text docs/LOAD_CHAOS_TESTING_MM.md "load_zap_host.py"
require_text docs/PRODUCTION_DEPLOYMENT_EN.md "zap-web-migrate.service"
require_text docs/PRODUCTION_DEPLOYMENT_MM.md "zap-web-migrate.service"
require_text docs/DATABASE_PRODUCTION_EN.md "ZAP_DB_MAX_CONNECTIONS"
require_text docs/DATABASE_PRODUCTION_MM.md "ZAP_DB_MAX_CONNECTIONS"
require_text host/zap-host/.env.example "ZAP_DB_MAX_CONNECTIONS=16"
require_text host/zap-host/.env.example "ZAP_DB_ACQUIRE_TIMEOUT_MS=1000"
require_text host/zap-host/.env.example "ZAP_DB_QUERY_TIMEOUT_MS=5000"
require_text host/zap-host/.env.example "ZAP_AUTH_MODE=jwt"
require_text host/zap-host/.env.example "ZAP_AUTH_ALLOWED_ALGORITHMS=RS256"
for native_web_file in \
  docs/ZAP_WEB_NATIVE_EN.md \
  docs/ZAP_WEB_NATIVE_MM.md \
  native/src/cli.rs \
  native/src/database.rs \
  native/src/project.rs; do
  require_file "$native_web_file"
done
for host_file in \
  host/zap-host/.env.example \
  host/zap-host/Cargo.toml \
  host/zap-host/Cargo.lock \
  host/zap-host/README.md \
  host/zap-host/src/lib.rs \
  host/zap-host/src/main.rs \
  host/zap-host/tests/http_contract.rs; do
  require_file "$host_file"
done
for deployment_file in \
  deploy/zap-host.service \
  deploy/zap-host.nginx.conf \
  deploy/zap-host-deployment-policy.toml \
  deploy/zap-host.env.example \
  deploy/zap-web.service \
  deploy/zap-web-migrate.service \
  deploy/zap-web.nginx.conf \
  deploy/zap-web-deployment-policy.toml \
  deploy/zap-web.env.example \
  scripts/validate_zap_host_deployment.sh \
  scripts/validate_zap_web_deployment.sh \
  scripts/load_zap_host.py \
  scripts/chaos_zap_host.py; do
  require_file "$deployment_file"
done
if [[ -x scripts/validate_zap_host_deployment.sh ]] && scripts/validate_zap_host_deployment.sh >/dev/null 2>&1; then
  record PASS "host-deployment-policy:zap-host"
else
  record FAIL "host-deployment-policy:zap-host"
fi
if [[ -x scripts/validate_zap_web_deployment.sh ]] && scripts/validate_zap_web_deployment.sh >/dev/null 2>&1; then
  record PASS "web-deployment-policy:zap-web"
else
  record FAIL "web-deployment-policy:zap-web"
fi

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
      scaffold_dir=$(mktemp -d)
      scaffold_output=$(mktemp)
      if "$ZAP_BIN" new "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && [[ -f "$scaffold_dir/project/zap.lock" ]] \
        && [[ -d "$scaffold_dir/project/models" ]] \
        && [[ -d "$scaffold_dir/project/functions" ]] \
        && [[ -d "$scaffold_dir/project/ui" ]] \
        && [[ -d "$scaffold_dir/project/routes" ]] \
        && [[ -d "$scaffold_dir/project/middleware" ]] \
        && [[ -d "$scaffold_dir/project/migrations" ]] \
        && [[ -d "$scaffold_dir/project/admin" ]] \
        && [[ -d "$scaffold_dir/project/public" ]] \
        && [[ ! -e "$scaffold_dir/project/services" ]] \
        && [[ ! -e "$scaffold_dir/project/views" ]] \
        && [[ -f "$scaffold_dir/project/server.zp" ]] \
        && [[ -f "$scaffold_dir/project/ui/ui.zp" ]] \
        && [[ -f "$scaffold_dir/project/routes/routes.zp" ]] \
        && [[ -f "$scaffold_dir/project/functions/user_functions.zp" ]] \
        && [[ -f "$scaffold_dir/project/middleware/middleware.zp" ]] \
        && [[ -f "$scaffold_dir/project/admin/admin.zp" ]] \
        && [[ -f "$scaffold_dir/project/public/index.html" ]] \
        && [[ -f "$scaffold_dir/project/public/assets/app.css" ]] \
        && [[ -f "$scaffold_dir/project/public/assets/app.js" ]] \
        && grep -Fq 'assets = "public"' "$scaffold_dir/project/zap.toml" \
        && grep -Fq '[frontend]' "$scaffold_dir/project/zap.toml" \
        && grep -Fq 'spa_fallback = "index.html"' "$scaffold_dir/project/zap.toml" \
        && grep -Fq '/assets/*path' "$scaffold_dir/project/routes/routes.zp" \
        && grep -Fq '/*path' "$scaffold_dir/project/routes/routes.zp" \
        && grep -Fq 'web_static_spa' "$scaffold_dir/project/functions/user_functions.zp" \
        && grep -Fq '/api/tasks' "$scaffold_dir/project/routes/routes.zp" \
        && "$ZAP_BIN" check "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" web check "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db check "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db inspect --json "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db plan "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db migrate --dry-run "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db migrate "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db migrate --check --json "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" db plan --json "$scaffold_dir/project" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" test "$scaffold_dir/project/tests" >>"$scaffold_output" 2>&1 \
        && "$ZAP_BIN" run "$scaffold_dir/project/main.zp" >>"$scaffold_output" 2>&1; then
        record PASS "runtime-smoke:zap-new-web"
      else
        record FAIL "runtime-smoke:zap-new-web"
      fi
      rm -rf "$scaffold_dir" "$scaffold_output"
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
