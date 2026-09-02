#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-class-trait.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let class_source = "class Box:\n    let score: number = 1\n    fn value(self) -> number:\n        return 7"
let trait_source = "trait Printable:\n    fn print(self) -> text"
let class_first = from_json(emit_inferred_program_typed_ir(class_source, "class.zp"))
let class_second = from_json(emit_inferred_program_typed_ir(class_source, "class.zp"))
let trait_first = from_json(emit_inferred_program_typed_ir(trait_source, "trait.zp"))
let trait_second = from_json(emit_inferred_program_typed_ir(trait_source, "trait.zp"))
let class_contract = typed_ir_general_contract(class_first, class_second)
let trait_contract = typed_ir_general_contract(trait_first, trait_second)
let class_root = typed_ir_root_contract(class_first, class_second)
let trait_root = typed_ir_root_contract(trait_first, trait_second)
let field = class_first["ir"]["nodes"][0]["member_semantics"][0]
let method = class_first["ir"]["nodes"][0]["member_semantics"][1]
let trait_method = trait_first["ir"]["nodes"][0]["member_semantics"][0]
say class_contract["valid"]
say trait_contract["valid"]
say class_root["valid"]
say trait_root["valid"]
say field["kind"]
say field["name"]
say field["inferred_type"]
say method["kind"]
say method["name"]
say method["inferred_return_type"]
say trait_method["kind"]
say trait_method["name"]
say trait_method["inferred_return_type"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true true true field score number method value number method print text" ]]; then
  echo "unexpected A9 class/trait output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 class/trait semantics gate passed: field and method projections, inferred types, trait methods, recursive shape, root contracts, and deterministic repeats\n'
