#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-diagnostic-contract.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let source1 = "say @"
let reference1 = {"diagnostics": [{"code": "ZAP-LEX-CHAR-001", "column": 5, "line": 1, "message": "unexpected character at 1:5: @", "severity": "error"}]}
let first1 = from_json(emit_inferred_program_typed_ir(source1, "invalid-character.zp"))
let second1 = from_json(emit_inferred_program_typed_ir(source1, "invalid-character.zp"))
let source2 = "let values = [1, 2"
let reference2 = {"diagnostics": [{"code": "ZAP-SYNTAX-001", "column": 6, "line": 1, "message": "expected ']' at 1:6", "severity": "error"}]}
let first2 = from_json(emit_inferred_program_typed_ir(source2, "missing-bracket.zp"))
let second2 = from_json(emit_inferred_program_typed_ir(source2, "missing-bracket.zp"))
let contract1 = typed_ir_diagnostic_contract(reference1, first1, second1)
let contract2 = typed_ir_diagnostic_contract(reference2, first2, second2)
say contract1["status"]
say contract1["valid"]
say first1["diagnostics"][0]["code"]
say first1["diagnostics"][0]["line"]
say first1["diagnostics"][0]["column"]
say contract2["status"]
say contract2["valid"]
say first2["diagnostics"][0]["code"]
say first2["diagnostics"][0]["line"]
say first2["diagnostics"][0]["column"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_diagnostic_contract true ZAP-LEX-CHAR-001 1 5 candidate_diagnostic_contract true ZAP-SYNTAX-001 1 6" ]]; then
  echo "unexpected A9 diagnostic contract output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 diagnostic contract gate passed: parser negative code/line/column parity, deterministic repeats, and stable typed-IR diagnostic envelope\n'
