#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}

fixtures=(
  "bootstrap/fixtures/parser/compound.zp|bootstrap/fixtures/parser/compound.ast.json|ast"
  "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp|bootstrap/fixtures/diagnostics/missing_closing_bracket.json|diagnostics"
)

run_native() {
  local mode=$1
  local fixture=$2
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" bootstrap "$mode" "$fixture"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" bootstrap "$mode" "$fixture"
  else
    run_zap bootstrap "$mode" "$fixture"
  fi
}

for entry in "${fixtures[@]}"; do
  IFS='|' read -r fixture expected mode <<<"$entry"
  [[ -f "$fixture" && -f "$expected" ]] || {
    printf 'missing B1 parser fixture or expected artifact: %s\n' "$fixture" >&2
    exit 2
  }
  first=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-first.XXXXXX")
  second=$(mktemp "${TMPDIR:-/tmp}/zap-b1-parser-second.XXXXXX")
  trap 'rm -f "$first" "$second"' EXIT
  run_native "$mode" "$fixture" > "$first"
  run_native "$mode" "$fixture" > "$second"
  cmp "$first" "$second"
  cmp "$first" "$expected"
  if [[ "$mode" == ast ]]; then
    jq -e '.kind == "zap.ast" and .schema_version == 1 and (.ast.statements | length) > 0' "$first" >/dev/null
  else
    jq -e '.kind == "zap.diagnostics" and .schema_version == 1 and .diagnostics[0].code == "ZAP-SYNTAX-001"' "$first" >/dev/null
  fi
  rm -f "$first" "$second"
  trap - EXIT
  printf 'B1 reference parser differential passed: %s (%s)\n' "$fixture" "$mode"
done
