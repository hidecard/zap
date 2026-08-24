#!/usr/bin/env bash
# Zap v2.1-E release preflight
#
# This script validates release inputs before a tag-triggered build/publish job.
# It intentionally does not create tags, publish artifacts, or handle secrets.
#
# Usage:
#   bash scripts/release_preflight.sh
#   RELEASE_TAG=v2.1.0 RUN_CARGO_CHECKS=1 bash scripts/release_preflight.sh
#
# Supported environment variables:
#   RELEASE_TAG       Expected tag, for example v2.1.0. Defaults to GITHUB_REF_NAME
#                     when available, otherwise the current exact tag name.
#   EXPECTED_VERSION  Expected semantic version without the leading v. Defaults
#                     to the native Cargo package version.
#   RELEASE_TARGETS   Comma-separated targets. Defaults to the v2.1-E matrix.
#   ALLOW_DIRTY=1     Permit a dirty working tree. Never use for publishing.
#   RUN_CARGO_CHECKS=1
#                     Run fmt, clippy, check, and tests locally.
#   RUN_CARGO_AUDIT=1
#                     Run the modern cargo-audit gate against native/Cargo.lock.
#   SKIP_DEPLOYMENT_VALIDATION=1
#                     Skip the deployment-policy validator, intended only for
#                     source-only development environments.

set -euo pipefail
IFS=$'\n\t'

# Local shells may not inherit Cargo's user-level bin directory; CI setup steps
# already provide it, while this fallback keeps the preflight command reproducible.
if [[ -f "${HOME}/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "${HOME}/.cargo/env"
fi
export PATH="${HOME}/.cargo/bin:${PATH}"

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT_DIR" ]]; then
  echo "release preflight: must run inside the Zap Git repository" >&2
  exit 1
fi
cd "$ROOT_DIR"

EXPECTED_VERSION="${EXPECTED_VERSION:-}"
RELEASE_TAG="${RELEASE_TAG:-${GITHUB_REF_NAME:-}}"
RELEASE_TARGETS="${RELEASE_TARGETS:-x86_64-unknown-linux-gnu,aarch64-apple-darwin,x86_64-pc-windows-msvc}"
ALLOW_DIRTY="${ALLOW_DIRTY:-0}"
RUN_CARGO_CHECKS="${RUN_CARGO_CHECKS:-0}"
RUN_CARGO_AUDIT="${RUN_CARGO_AUDIT:-0}"
SKIP_DEPLOYMENT_VALIDATION="${SKIP_DEPLOYMENT_VALIDATION:-0}"

PASS_COUNT=0
WARN_COUNT=0
FAIL_COUNT=0

pass() {
  PASS_COUNT=$((PASS_COUNT + 1))
  printf 'PASS: %s\n' "$*"
}

warn() {
  WARN_COUNT=$((WARN_COUNT + 1))
  printf 'WARN: %s\n' "$*" >&2
}

fail() {
  FAIL_COUNT=$((FAIL_COUNT + 1))
  printf 'FAIL: %s\n' "$*" >&2
}

require_file() {
  local path="$1"
  if [[ -f "$path" ]]; then
    pass "required file exists: $path"
  else
    fail "required file is missing: $path"
  fi
}

require_text() {
  local path="$1"
  local text="$2"
  if [[ -f "$path" ]] && grep -Fq -- "$text" "$path"; then
    pass "required text found in $path: $text"
  else
    fail "required text missing from $path: $text"
  fi
}

read_cargo_version() {
  sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' native/Cargo.toml | head -n 1
}

is_semver() {
  [[ "$1" =~ ^0\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$|^[1-9][0-9]*\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]
}

check_version() {
  local cargo_version
  if [[ -x scripts/validate_release_version.sh ]]; then
    if EXPECTED_VERSION="$EXPECTED_VERSION" RELEASE_TAG="$RELEASE_TAG" \
      ZAP_VERSION_REPORT="${RELEASE_VERSION_REPORT:-$ROOT_DIR/target/version-consistency.tsv}" \
      scripts/validate_release_version.sh "$EXPECTED_VERSION"; then
      pass "single-source release version validation passed"
    else
      fail "single-source release version validation failed"
    fi
  else
    fail "missing executable scripts/validate_release_version.sh"
  fi
  cargo_version="$(read_cargo_version)"
  if [[ -z "$cargo_version" ]]; then
    fail "could not read native Cargo package version"
    return
  fi
  if [[ -z "$EXPECTED_VERSION" ]]; then
    EXPECTED_VERSION="$cargo_version"
  fi
  if [[ "$EXPECTED_VERSION" != "$cargo_version" ]]; then
    fail "expected version $EXPECTED_VERSION does not match native Cargo version $cargo_version"
  else
    pass "native Cargo version is $EXPECTED_VERSION"
  fi
  if is_semver "$EXPECTED_VERSION"; then
    pass "version has an accepted semantic-version shape: $EXPECTED_VERSION"
  else
    fail "version is not a supported semantic-version shape: $EXPECTED_VERSION"
  fi

  if [[ -n "$RELEASE_TAG" ]]; then
    if [[ "$RELEASE_TAG" == v* ]]; then
      local tag_version="${RELEASE_TAG#v}"
      if [[ "$tag_version" == "$EXPECTED_VERSION" ]]; then
        pass "release tag matches package version: $RELEASE_TAG"
      else
        fail "release tag $RELEASE_TAG does not match version $EXPECTED_VERSION"
      fi
    else
      fail "release tag must start with v: $RELEASE_TAG"
    fi
  else
    warn "RELEASE_TAG is not set; tag/version consistency check was not performed"
  fi

  for changelog in CHANGELOG_EN.md CHANGELOG_MM.md; do
    if grep -Fq -- "$EXPECTED_VERSION" "$changelog"; then
      pass "$changelog mentions version $EXPECTED_VERSION"
    else
      fail "$changelog does not mention version $EXPECTED_VERSION"
    fi
  done
}

check_clean_tree() {
  if [[ "$ALLOW_DIRTY" == "1" ]]; then
    warn "ALLOW_DIRTY=1: dirty working tree is permitted for this run"
    return
  fi
  if [[ -z "$(git status --porcelain)" ]]; then
    pass "working tree is clean"
  else
    git status --short >&2 || true
    fail "working tree is dirty; commit or use ALLOW_DIRTY=1 for non-publishing development checks"
  fi
}

check_release_files() {
  local required_files=(
    README.md
    CHANGELOG.md
    CHANGELOG_EN.md
    CHANGELOG_MM.md
    LICENSE
    native/Cargo.toml
    native/Cargo.lock
    .github/workflows/ci.yml
    .github/workflows/release.yml
    "docs/RELEASE_${EXPECTED_VERSION}_EN.md"
    "docs/RELEASE_${EXPECTED_VERSION}_MM.md"
    scripts/verify_release_artifacts.sh
    scripts/verify_installer_unix.sh
    scripts/test_p001_parity.sh
    scripts/test_p105_layers.sh
    scripts/test_p105_replay.sh
    scripts/test_m2_verify_replay.sh
    scripts/test_m2_verify_replay_contract.sh
    scripts/test_p005c_async_matrix.sh
    scripts/test_platform_archive.sh
    scripts/test_benchmark_regression.sh
    scripts/test_benchmark_provenance.sh
    scripts/test_stdlib_policy.sh
    scripts/test_lsp_semantic_parity.sh
    scripts/test_lsp_protocol_sync.sh
    scripts/test_vscode_extension.sh
    scripts/validate_vscode_assets.py
    editors/vscode/package.json
    editors/vscode/language-configuration.json
    editors/vscode/syntaxes/zap.tmLanguage.json
    vscode-extension/package.json
    vscode-extension/extension.js
    vscode-extension/lsp-client.js
    vscode-extension/language-configuration.json
    vscode-extension/syntaxes/zap.tmLanguage.json
    vscode-extension/snippets/zap.json
    vscode-extension/scripts/test-extension.js
    vscode-extension/scripts/package-extension.js
    vscode-extension/README.md
    vscode-extension/README_MM.md
    docs/ASYNC_LSP_EN.md
    docs/ASYNC_LSP_MM.md
    docs/FRAMEWORK_EN.md
    docs/FRAMEWORK_MM.md
    docs/WEB_FRAMEWORK_EN.md
    docs/WEB_FRAMEWORK_MM.md
    docs/ZAP_HOST_EN.md
    docs/ZAP_HOST_MM.md
    docs/ZAP_HOST_QUICKSTART_EN.md
    docs/ZAP_HOST_QUICKSTART_MM.md
    docs/ZAP_WEB_NATIVE_EN.md
    docs/ZAP_WEB_NATIVE_MM.md
    host/zap-host/.env.example
    host/zap-host/Cargo.toml
    host/zap-host/Cargo.lock
    host/zap-host/README.md
    host/zap-host/src/lib.rs
    host/zap-host/src/main.rs
    host/zap-host/tests/http_contract.rs
    deploy/zap-host.service
    deploy/zap-host.nginx.conf
    deploy/zap-host-deployment-policy.toml
    deploy/zap-host.env.example
    scripts/validate_zap_host_deployment.sh
    scripts/validate_framework_starters.sh
    frameworks/README.md
    frameworks/web/README.md
    frameworks/web/zap.toml
    frameworks/web/zap.lock
    frameworks/web/main.zp
    frameworks/web/web_contract.zp
    frameworks/web/web_contract_test.zp
    frameworks/web/api_contract.zp
    frameworks/web/api_contract_test.zp
    frameworks/web/dto_contract.zp
    frameworks/web/database_contract.zp
    frameworks/web/database_adapter.zp
    frameworks/web/database_adapter_test.zp
    frameworks/web/frontend_contract.zp
    frameworks/web/frontend_contract_test.zp
    frameworks/web/public/index.html
    frameworks/web/public/assets/app.css
    frameworks/web/public/assets/app.js
    frameworks/web/auth_contract.zp
    frameworks/web/rate_limit_contract.zp
    frameworks/mobile/README.md
    frameworks/mobile/zap.toml
    frameworks/mobile/zap.lock
    frameworks/mobile/main.zp
    frameworks/ai/README.md
    frameworks/ai/zap.toml
    frameworks/ai/zap.lock
    frameworks/ai/main.zp
    frameworks/iot/README.md
    frameworks/iot/zap.toml
    frameworks/iot/zap.lock
    frameworks/iot/main.zp
    scripts/validate_spec_ownership.sh
    scripts/validate_release_version.sh
    scripts/test_validate_release_version.sh
    scripts/check_rustsec_audit.sh
    scripts/validate_documentation_consistency.sh
    scripts/test_validate_documentation_consistency.sh
    scripts/validate_markdown_links.py
    scripts/check_benchmark_regression.sh
    scripts/test_benchmark_regression.sh
    benchmark-results/native-summary.csv
    scripts/verify_installer_windows.ps1
    scripts/validate_registry_deployment.sh
    deploy/registry-deployment-policy.toml
    deploy/registry.env.example
    deploy/zap-registry.service
    deploy/zap-registry.nginx.conf
    docs/SPEC_OWNERSHIP_INDEX.tsv
    docs/SPEC_OWNERSHIP_EN.md
    docs/SPEC_OWNERSHIP_MM.md
    docs/COMPATIBILITY_CHANGE_TEMPLATE_EN.md
    docs/COMPATIBILITY_CHANGE_TEMPLATE_MM.md
    docs/TRAITS_RFC_EN.md
    docs/TRAITS_RFC_MM.md
    docs/RELEASE_VERSION_POLICY_EN.md
    docs/RELEASE_VERSION_POLICY_MM.md
    docs/P001_PARITY_MATRIX_EN.md
    docs/P001_PARITY_MATRIX_MM.md
    docs/P105_REPLAY_EN.md
    docs/P105_REPLAY_MM.md
    docs/BENCHMARK_HARNESS_EN.md
    docs/BENCHMARK_HARNESS_MM.md
    docs/DOCUMENTATION_NAVIGATION_EN.md
    docs/DOCUMENTATION_NAVIGATION_MM.md
    docs/RUNTIME_STATE_EN.md
    docs/RUNTIME_STATE_MM.md
  )
  local path
  for path in "${required_files[@]}"; do
    require_file "$path"
  done
}

check_documentation_pairs() {
  local pairs=(
    'docs/ASYNC_LSP_EN.md:docs/ASYNC_LSP_MM.md'
    'docs/ASYNC_RUNTIME_EN.md:docs/ASYNC_RUNTIME_MM.md'
    'docs/DEPLOYMENT_EN.md:docs/DEPLOYMENT_MM.md'
    'docs/REGISTRY_AUTH_EN.md:docs/REGISTRY_AUTH_MM.md'
    'docs/V2.1_ROADMAP_EN.md:docs/V2.1_ROADMAP_MM.md'
    'docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md:docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md'
    'docs/FRAMEWORK_EN.md:docs/FRAMEWORK_MM.md'
    'docs/WEB_FRAMEWORK_EN.md:docs/WEB_FRAMEWORK_MM.md'
    'docs/ZAP_HOST_EN.md:docs/ZAP_HOST_MM.md'
    'docs/ZAP_HOST_QUICKSTART_EN.md:docs/ZAP_HOST_QUICKSTART_MM.md'
    'docs/ZAP_WEB_NATIVE_EN.md:docs/ZAP_WEB_NATIVE_MM.md'
    'docs/RUNTIME_STATE_EN.md:docs/RUNTIME_STATE_MM.md'
    "docs/RELEASE_${EXPECTED_VERSION}_EN.md:docs/RELEASE_${EXPECTED_VERSION}_MM.md"
  )
  local pair en mm
  for pair in "${pairs[@]}"; do
    en="${pair%%:*}"
    mm="${pair##*:}"
    if [[ -f "$en" && -f "$mm" ]]; then
      pass "bilingual documentation pair exists: $en / $mm"
    else
      fail "bilingual documentation pair is incomplete: $en / $mm"
    fi
  done

  require_text docs/V2.1_ROADMAP_EN.md 'v2.1.0'
  require_text docs/V2.1_ROADMAP_MM.md 'v2.1.0'
  require_text docs/DEPLOYMENT_EN.md 'production'
  require_text docs/DEPLOYMENT_MM.md 'production'
  require_text docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md "$EXPECTED_VERSION"
  require_text docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md "$EXPECTED_VERSION"
  require_text "docs/RELEASE_${EXPECTED_VERSION}_EN.md" "$EXPECTED_VERSION"
  require_text "docs/RELEASE_${EXPECTED_VERSION}_MM.md" "$EXPECTED_VERSION"
}

check_source_safety() {
  if git grep -n -I -E '^(<<<<<<<|=======|>>>>>>>)( |$)' -- ':!scripts/release_preflight.sh'; then
    fail "merge-conflict markers found in tracked source files"
  else
    pass "no merge-conflict markers found"
  fi

  if git diff --check; then
    pass "repository whitespace check passed"
  else
    fail "repository whitespace check failed"
  fi

  if find deploy -type f \( -name '*.key' -o -name '*.pem' -o -name 'registry.env' \) -print -quit | grep -q .; then
    fail "deploy/ contains a private key, certificate, or populated registry.env"
  else
    pass "deploy/ contains no private key, certificate, or populated registry.env"
  fi
}

check_targets() {
  local target
  local valid=0
  local expected
  IFS=',' read -r -a targets <<< "$RELEASE_TARGETS"
  for target in "${targets[@]}"; do
    target="${target//[[:space:]]/}"
    case "$target" in
      x86_64-unknown-linux-gnu|aarch64-apple-darwin|x86_64-pc-windows-msvc)
        pass "supported release target configured: $target"
        valid=$((valid + 1))
        ;;
      '')
        fail "release target list contains an empty target"
        ;;
      *)
        fail "unsupported release target configured: $target"
        ;;
    esac
  done
  expected=3
  if [[ "$valid" -lt "$expected" ]]; then
    fail "release target matrix contains only $valid supported target(s); expected at least $expected"
  else
    pass "release target matrix contains $valid supported targets"
  fi
}

run_optional_cargo_checks() {
  if [[ "$RUN_CARGO_CHECKS" != "1" ]]; then
    warn "RUN_CARGO_CHECKS is not 1; local Rust quality gates were not run"
    return
  fi
  cargo fmt --manifest-path native/Cargo.toml --all -- --check
  pass "cargo fmt check passed"
  cargo clippy --manifest-path native/Cargo.toml --all-targets --all-features -- -D warnings
  pass "strict cargo clippy passed"
  cargo check --manifest-path native/Cargo.toml --all-targets --all-features
  pass "cargo check passed"
  cargo test --manifest-path native/Cargo.toml --all-targets --all-features
  pass "native test suite passed"
  cargo fmt --manifest-path host/zap-host/Cargo.toml -- --check
  pass "zap-host cargo fmt check passed"
  cargo clippy --manifest-path host/zap-host/Cargo.toml --all-targets --all-features -- -D warnings
  pass "zap-host strict cargo clippy passed"
  cargo check --manifest-path host/zap-host/Cargo.toml --all-targets --all-features
  pass "zap-host cargo check passed"
  cargo test --manifest-path host/zap-host/Cargo.toml --all-targets
  pass "zap-host test suite passed"
}

run_contract_validation() {
  local report_dir="${RELEASE_CONTRACT_REPORT_DIR:-$ROOT_DIR/target/release-contract-reports}"
  local seed="${ZAP_CORPUS_SEED:-20260821}"
  mkdir -p "$report_dir"

  EXPECTED_VERSION="$EXPECTED_VERSION" \
    ZAP_DOCS_REPORT="$report_dir/documentation-consistency.tsv" \
    bash scripts/validate_documentation_consistency.sh
  pass "documentation consistency validation passed"

  bash scripts/test_validate_documentation_consistency.sh
  pass "documentation consistency regression validation passed"

  python3 scripts/validate_markdown_links.py
  pass "repository Markdown link validation passed"

  ZAP_FRAMEWORK_DOCS_ONLY=1 \
    bash scripts/validate_framework_starters.sh
  pass "Framework starter static contract validation passed"

  ZAP_SPEC_OWNERSHIP_REPORT="$report_dir/spec-ownership.tsv" \
    bash scripts/validate_spec_ownership.sh
  pass "specification ownership validation passed"

  ZAP_PARITY_REPORT="$report_dir/native-legacy-parity.tsv" \
    bash scripts/test_p001_parity.sh
  pass "native/legacy parity validation passed"

  ZAP_CORPUS_SEED="$seed" \
    ZAP_CORPUS_REPLAY_LOG="$report_dir/p105-replay.log" \
    bash scripts/test_p105_replay.sh
  pass "fixed-seed replay validation passed with seed $seed"

  ZAP_CORPUS_SEED="$seed" \
    ZAP_CORPUS_ROUNDS="${ZAP_CORPUS_ROUNDS:-12}" \
    ZAP_BOUNDED_REPLAY_REPORT="$report_dir/m2-verify-replay.tsv" \
    ZAP_BOUNDED_REPLAY_LOG="$report_dir/m2-verify-replay.log" \
    bash scripts/test_m2_verify_replay.sh
  pass "bounded replay validation passed with seed $seed"

  bash scripts/test_m2_verify_replay_contract.sh
  pass "bounded replay contract regression passed"

  ZAP_ASYNC_LOG="$report_dir/p005c-async.log" \
    bash scripts/test_p005c_async_matrix.sh
  pass "focused async boundary matrix passed"

  bash scripts/test_platform_archive.sh
  pass "platform archive determinism regression passed"

  cargo test --manifest-path native/Cargo.toml --bin zap registry_client --all-features -- --nocapture
  pass "registry transport edge-case corpus passed"

  bash scripts/test_benchmark_regression.sh
  pass "benchmark schema and regression contract passed"

  bash scripts/test_benchmark_provenance.sh
  pass "benchmark provenance contract passed"

  bash scripts/test_stdlib_policy.sh
  pass "standard-library stability policy contract passed"

  bash scripts/bootstrap/verify_b0_artifacts.sh --release
  pass "B0 bootstrap artifact reproducibility contract passed"

  bash scripts/bootstrap/verify_b1_lexer.sh
  pass "B1 Zap-owned lexer differential contract passed"

  bash scripts/bootstrap/verify_b3_foundations.sh
  pass "B3 package/build/test-runner foundation contract passed"

  bash scripts/bootstrap/verify_vm_platform.sh
  pass "reference VM and platform-seed foundation contract passed"

  bash scripts/test_lsp_semantic_parity.sh
  pass "LSP and VS Code semantic-parity contract passed"

  bash scripts/test_lsp_protocol_sync.sh
  pass "LSP protocol synchronization contract passed"

  bash scripts/test_vscode_extension.sh
  pass "canonical VS Code extension package contract passed"

  local benchmark_raw="$report_dir/benchmark-raw.csv"
  local benchmark_summary="$report_dir/benchmark-summary.csv"
  local benchmark_provenance="$report_dir/benchmark-provenance.tsv"
  ZAP_BENCH_REPEATS="${ZAP_BENCH_REPEATS:-5}" \
    ZAP_BENCH_WARMUPS="${ZAP_BENCH_WARMUPS:-1}" \
    ZAP_BENCH_OUTPUT="$benchmark_raw" \
    ZAP_BENCH_PROVENANCE="$benchmark_provenance" \
    bash scripts/benchmark_native.sh
  test -s "$benchmark_provenance"
  awk -F '\t' 'NR > 1 { seen[$1]++ } END { if (seen["schema_version"] != 1 || seen["status"] != 1 || seen["git_commit"] != 1 || seen["target_triple"] != 1 || seen["binary_sha256"] != 1 || seen["repeats"] != 1 || seen["warmups"] != 1) exit 1 }' "$benchmark_provenance"
  bash scripts/aggregate_benchmark.sh "$benchmark_raw" "$benchmark_summary"
  scripts/check_benchmark_regression.sh \
    benchmark-results/native-summary.csv "$benchmark_summary" \
    "${ZAP_BENCH_MAX_REGRESSION_PERCENT:-200}" \
    >"$report_dir/benchmark-regression.log" 2>&1
  pass "benchmark regression validation passed"
}

run_cargo_audit_gate() {
  if [[ "$RUN_CARGO_AUDIT" != "1" ]]; then
    warn "RUN_CARGO_AUDIT is not 1; modern RustSec audit was not run"
    return
  fi
  if [[ ! -x scripts/check_rustsec_audit.sh ]]; then
    fail "scripts/check_rustsec_audit.sh is missing or not executable"
    return
  fi
  if bash scripts/check_rustsec_audit.sh; then
    pass "modern RustSec audit passed"
  else
    fail "modern RustSec audit found advisories or could not complete"
  fi
}

run_deployment_validation() {
  if [[ "$SKIP_DEPLOYMENT_VALIDATION" == "1" ]]; then
    warn "SKIP_DEPLOYMENT_VALIDATION=1; deployment-policy validation was skipped"
    return
  fi
  if [[ -x scripts/validate_registry_deployment.sh ]]; then
    bash scripts/validate_registry_deployment.sh
    pass "registry deployment policy validation passed"
  else
    fail "scripts/validate_registry_deployment.sh is missing or not executable"
  fi
}

printf '%s\n' "Zap v2.1-E release preflight"
printf '%s\n' "Repository: $ROOT_DIR"
printf '%s\n' "Commit: $(git rev-parse HEAD)"
printf '%s\n' "Release tag: ${RELEASE_TAG:-<unset>}"
printf '%s\n' "Expected version: ${EXPECTED_VERSION:-<derived from Cargo>}"
printf '%s\n' "Targets: $RELEASE_TARGETS"
printf '%s\n' '--- version and tag checks'
check_version
printf '%s\n' '--- repository state'
check_clean_tree
check_source_safety
printf '%s\n' '--- required release files'
check_release_files
printf '%s\n' '--- bilingual documentation'
check_documentation_pairs
printf '%s\n' '--- target matrix'
check_targets
printf '%s\n' '--- P0/P1 contract validation'
run_contract_validation
printf '%s\n' '--- dependency security audit'
run_cargo_audit_gate
printf '%s\n' '--- deployment policy'
run_deployment_validation
printf '%s\n' '--- optional Rust gates'
run_optional_cargo_checks

printf '%s\n' '--- summary'
printf 'passed=%d warnings=%d failures=%d\n' "$PASS_COUNT" "$WARN_COUNT" "$FAIL_COUNT"
if [[ "$FAIL_COUNT" -ne 0 ]]; then
  echo 'release preflight: FAILED' >&2
  exit 1
fi
printf '%s\n' 'release preflight: PASSED'
