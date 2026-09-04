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
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-recursive-coverage.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let function_source = "fn add(a, b):\n    return a + b\nsay add(2, 3)"
let class_source = "class Box:\n    fn value(self):\n        return 7"
let loop_source = "let value: number = 0\nwhile value < 3:\n    if value == 1:\n        continue\n    break"
let try_source = "try:\n    raise \"x\"\ncatch error:\n    say error"
let import_source = "import \"bootstrap/b1/lexer.zp\""
let module_source = "module demo"
let function_first = from_json(emit_inferred_program_typed_ir(function_source, "function.zp"))
let function_second = from_json(emit_inferred_program_typed_ir(function_source, "function.zp"))
let class_first = from_json(emit_inferred_program_typed_ir(class_source, "class.zp"))
let class_second = from_json(emit_inferred_program_typed_ir(class_source, "class.zp"))
let loop_first = from_json(emit_inferred_program_typed_ir(loop_source, "loop.zp"))
let loop_second = from_json(emit_inferred_program_typed_ir(loop_source, "loop.zp"))
let try_first = from_json(emit_inferred_program_typed_ir(try_source, "try.zp"))
let try_second = from_json(emit_inferred_program_typed_ir(try_source, "try.zp"))
let import_first = from_json(emit_inferred_program_typed_ir(import_source, "import.zp"))
let import_second = from_json(emit_inferred_program_typed_ir(import_source, "import.zp"))
let module_first = from_json(emit_inferred_program_typed_ir(module_source, "module.zp"))
let module_second = from_json(emit_inferred_program_typed_ir(module_source, "module.zp"))
let function_contract = typed_ir_general_contract(function_first, function_second)
let class_contract = typed_ir_general_contract(class_first, class_second)
let loop_contract = typed_ir_general_contract(loop_first, loop_second)
let try_contract = typed_ir_general_contract(try_first, try_second)
let import_contract = typed_ir_general_contract(import_first, import_second)
let module_contract = typed_ir_general_contract(module_first, module_second)
say function_contract["valid"]
say class_contract["valid"]
say loop_contract["valid"]
say try_contract["valid"]
say import_contract["valid"]
say module_contract["valid"]
say function_contract["node_count"]
say class_contract["node_count"]
say loop_contract["node_count"]
say try_contract["node_count"]
say import_contract["node_count"]
say module_contract["node_count"]
say function_first["ir"]["nodes"][0]["inferred_return_type"]
say function_first["ir"]["nodes"][0]["inferred_parameters"][0]["inferred_type"]
say function_first["ir"]["nodes"][0]["inferred_parameters"][1]["inferred_type"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true true true true true 2 1 2 1 1 1 any any any" ]]; then
  echo "unexpected recursive AST coverage output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 recursive AST coverage gate passed: function, class, loop break/continue, try/catch, import, module, recursive shape validation, and deterministic repeats\n'
