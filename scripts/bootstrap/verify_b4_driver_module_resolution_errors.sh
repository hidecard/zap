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
[ -f bootstrap/b4/compiler_driver.zp ]
runner=$(mktemp "$ROOT_DIR/.zap-b4-driver-module-errors.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let missing = driver_resolve_modules(["import \"does_not_exist\"\nsay 1"], ["app.zp"])
let duplicate = driver_resolve_modules(["say 1", "say 2"], ["same.zp", "same.zp"])
let ambiguous = driver_resolve_modules(["import \"shared\"\nsay 1", "say 2", "say 3"], ["app.zp", "a/shared.zp", "b/shared.zp"])
let cycle = driver_resolve_modules(["import \"b\"\nsay 1", "import \"a\"\nsay 2"], ["a.zp", "b.zp"])
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
