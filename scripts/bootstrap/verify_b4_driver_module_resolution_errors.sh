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
  elif [[ -n "${ZAP_BOOTSTRAP_BIN:-}" && -x "$ZAP_BOOTSTRAP_BIN" ]]; then
    "$ZAP_BOOTSTRAP_BIN" "$@"
  elif ! command -v cargo >/dev/null 2>&1; then
    echo "BLOCKED: no Zap runtime found; provide ZAP_BOOTSTRAP_BIN or build native/target/release/zap" >&2
    return 2
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f bootstrap/b4/compiler_driver.zp ]
runner=$(mktemp "$ROOT_DIR/.zap-b4-driver-module-errors.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let missing = driver_resolve_modules([read_text("bootstrap/fixtures/driver/module_missing_app.zp")], ["app.zp"])
let duplicate = driver_resolve_modules(["say 1", "say 2"], ["same.zp", "same.zp"])
let ambiguous = driver_resolve_modules([read_text("bootstrap/fixtures/driver/module_ambiguous_app.zp"), read_text("bootstrap/fixtures/driver/module_graph_base.zp"), read_text("bootstrap/fixtures/driver/module_graph_base.zp")], ["app.zp", "a/shared.zp", "b/shared.zp"])
let cycle = driver_resolve_modules([read_text("bootstrap/fixtures/driver/module_cycle_a.zp"), read_text("bootstrap/fixtures/driver/module_cycle_b.zp")], ["module_cycle_a.zp", "module_cycle_b.zp"])
say missing["status"]
say missing["diagnostics"][0]["code"]
say duplicate["status"]
say duplicate["diagnostics"][0]["code"]
say ambiguous["status"]
say ambiguous["diagnostics"][0]["code"]
say cycle["status"]
say cycle["diagnostics"][0]["code"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" >"$out"
else
  run_zap "$runner_rel" >"$out"
fi
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "module_resolution_error ZAP-MODULE-003 module_resolution_error ZAP-MODULE-002 module_resolution_error ZAP-MODULE-004 module_resolution_error ZAP-MODULE-005" ]]; then
  echo "unexpected module-resolution error output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 driver module-resolution error gate passed: missing, duplicate, ambiguous, and cyclic imports have stable diagnostics\n'
