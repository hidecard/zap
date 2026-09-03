#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-subset-rebuild.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
let sources = ["let value: number = 7\nsay value", "let value: number = 0\nif value == 0:\n    say 1\nelse:\n    say 2", "fn add(a, b):\n    return a + b\nsay add(2, 3)"]
let names = ["literal.zp", "branch.zp", "function.zp"]
let rebuild = seed_supported_subset_rebuild(sources, names)
say rebuild["status"]
say rebuild["native_independent"]
say rebuild["count"]
say rebuild["all_successful"]
say rebuild["byte_equal"]
say rebuild["first"][0]["execution"]["output"][0]
say rebuild["first"][1]["execution"]["output"][0]
say rebuild["first"][2]["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_supported_subset_rebuild false 3 true true 7 1 5" ]]; then
  echo "unexpected supported subset output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 supported-subset rebuild gate passed: three source forms, two-pass artifact equality, and VM result parity\n'
