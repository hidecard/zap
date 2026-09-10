#!/usr/bin/env bash
# B4 driver-owned source-to-VM verification.
#
# Verifies that the Zap compiler driver directly owns the full
# source → typed-IR → bytecode → VM execution pipeline and produces
# deterministic, correct results without depending on native_independent.zp.
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

fail() { echo "B4 driver source-to-VM failed: $*" >&2; exit 1; }

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
runner=$(mktemp "$ROOT_DIR/.zap-b4-driver-source-vm.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT

# Test 1: Basic arithmetic through driver pipeline
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let first = driver_execute_owned_pipeline("say 20 + 22", "arith1.zp")
let second = driver_execute_owned_pipeline("say 20 + 22", "arith1.zp")
let changed = driver_execute_owned_pipeline("say 20 - 22", "arith2.zp")

say first["status"]
say first["native_independent"]
say first["stage_chain_valid"]
say len(first["artifacts"])
say first["execution"]["error"]
say first["execution"]["output"][0]
say json(first["artifacts"][0]["bytes"]) == json(second["artifacts"][0]["bytes"])
say json(first["artifacts"][1]["bytes"]) == json(second["artifacts"][1]["bytes"])
say json(first["artifacts"][0]["bytes"]) == json(changed["artifacts"][0]["bytes"])
say changed["status"]
say changed["execution"]["output"][0]
ZP

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi

mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "pipeline_executed true true 2 none 42 true true false pipeline_executed -2" ]]; then
  echo "unexpected driver source-to-VM output: ${lines[*]}" >&2
  exit 1
fi

printf 'B4 driver source-to-VM gate passed: driver-owned pipeline produces deterministic arithmetic and VM results\n'
