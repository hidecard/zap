#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-general-contract.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let source1 = "let value: number = 7\nsay value"
let source2 = "let items = [1, 2]\nlet score = {\"value\": 7}"
let source3 = "let value: number = 7\nif value > 2:\n    say value"
let first1 = from_json(emit_inferred_program_typed_ir(source1, "a9-1.zp"))
let second1 = from_json(emit_inferred_program_typed_ir(source1, "a9-1.zp"))
let first2 = from_json(emit_inferred_program_typed_ir(source2, "a9-2.zp"))
let second2 = from_json(emit_inferred_program_typed_ir(source2, "a9-2.zp"))
let first3 = from_json(emit_inferred_program_typed_ir(source3, "a9-3.zp"))
let second3 = from_json(emit_inferred_program_typed_ir(source3, "a9-3.zp"))
let contract1 = typed_ir_general_contract(first1, second1)
let contract2 = typed_ir_general_contract(first2, second2)
let contract3 = typed_ir_general_contract(first3, second3)
let reference = from_json(emit("let value: number = 7", "a9-reference.zp"))
let candidate = from_json(emit_inferred_program_typed_ir("let value: number = 7", "a9-reference.zp"))
let parity = typed_ir_compare_reference_program(reference, candidate)
say contract1["valid"]
say contract1["node_count"]
say contract2["valid"]
say contract2["node_count"]
say contract3["valid"]
say contract3["node_count"]
say parity["semantic_equal"]
say parity["node_count_match"]
say parity["diagnostic_equal"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true 2 true 2 true 2 true true true" ]]; then
  echo "unexpected A9 general typed-IR output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 general typed-IR contract gate passed: schema-4 metadata, spans/node shapes, deterministic repeated emission, nested collection/conditional forms, and bounded reference parity\n'
