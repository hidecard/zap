#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-malformed-safety.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
let first = seed_execute_owned_pipeline("let value: number =", "malformed.zp")
let second = seed_execute_owned_pipeline("let value: number =", "malformed.zp")
let valid = seed_execute_owned_pipeline("let value: number = 7\nsay value", "valid.zp")
say first["status"]
say first["error"]
say first["stage_chain_valid"]
say first["diagnostics"][0]["kind"]
say first["diagnostics"][0]["diagnostics"][0]["code"]
say json(first) == json(second)
say valid["status"]
say valid["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "compile_error syntax_diagnostic false zap.diagnostics ZAP-SYNTAX-001 true candidate_pipeline_executed 7" ]]; then
  echo "unexpected malformed pipeline output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 malformed-pipeline safety gate passed: deterministic syntax diagnostic, no crash, invalid-stage boundary, and valid-source regression\n'
