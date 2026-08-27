#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-function-return.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
import "bootstrap/b2/typed_ir.zp"
let source = "fn infer_number():\n    return 1\nlet answer: number = infer_number()"
let parsed = from_json(parse_general(source, "function_return.zp"))
let checked = b2c_check_program(parsed["ast"], "function_return.zp")
let inferred_functions = b2c_infer_function_returns(checked["functions"], "function_return.zp")
let inferred_return = inferred_functions[0]["return_type"]
let typed = from_json(emit_inferred_program_typed_ir(source, "function_return.zp"))
let answer = typed["ir"]["nodes"][1]
say checked["ok"]
say inferred_return
say answer["value"]["inferred_type"]
say len(checked["diagnostics"])
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true number number 0" ]]; then
  echo "unexpected function-return output: ${lines[*]}" >&2
  exit 1
fi
printf 'B2 function-return inference gate passed: unannotated return propagated to checker and typed-IR call metadata\n'
