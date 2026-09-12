#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  local seed="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap.exe}}"
  [[ -x "$seed" ]] || { echo "B4 supported-subset rebuild blocked: prebuilt Zap seed required; set ZAP_BOOTSTRAP_BIN" >&2; exit 2; }
  "$seed" "$@"
}
runner=$(mktemp "$ROOT_DIR/.zap-b4-subset-rebuild.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let sources = ["let value: number = 7\nsay value", "let value: number = 0\nif value == 0:\n    say 1\nelse:\n    say 2", "fn add(a, b):\n    return a + b\nsay add(2, 3)"]
let names = ["literal.zp", "branch.zp", "function.zp"]
let rebuild = driver_subset_rebuild(sources, names)
say rebuild["status"]
say rebuild["native_independent"]
say rebuild["count"]
say rebuild["all_successful"]
say rebuild["byte_equal"]
say rebuild["first"][0]["execution"]["output"][0]
say rebuild["first"][1]["execution"]["output"][0]
say rebuild["first"][2]["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}}"
[ -x "$ZAP_BIN" ] || { echo "B4 supported-subset rebuild blocked: prebuilt Zap seed required" >&2; exit 2; }
"$ZAP_BIN" "$runner_rel" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_driver_subset_rebuild false 3 true true 7 1 5" ]]; then
  echo "unexpected supported subset output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 supported-subset rebuild gate passed: three source forms, two-pass artifact equality, and VM result parity\n'
