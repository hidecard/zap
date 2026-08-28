#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-a9-closure-member.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let closure_source = "fn outer():\n    let value: number = 7\n    fn inner():\n        return value"
let member_source = "let data = {\"user\": {\"name\": \"zap\"}}\nlet name = data.user.name"
let index_source = "let items = [1, 2]\nlet first = items[0]"
let closure_first = from_json(emit_inferred_program_typed_ir(closure_source, "closure.zp"))
let closure_second = from_json(emit_inferred_program_typed_ir(closure_source, "closure.zp"))
let member_first = from_json(emit_inferred_program_typed_ir(member_source, "member.zp"))
let member_second = from_json(emit_inferred_program_typed_ir(member_source, "member.zp"))
let index_first = from_json(emit_inferred_program_typed_ir(index_source, "index.zp"))
let index_second = from_json(emit_inferred_program_typed_ir(index_source, "index.zp"))
let closure_contract = typed_ir_general_contract(closure_first, closure_second)
let member_contract = typed_ir_general_contract(member_first, member_second)
let index_contract = typed_ir_general_contract(index_first, index_second)
let closure_root = typed_ir_root_contract(closure_first, closure_second)
let member_root = typed_ir_root_contract(member_first, member_second)
let index_root = typed_ir_root_contract(index_first, index_second)
let inner = closure_first["ir"]["nodes"][0]["body"][1]
let name_node = member_first["ir"]["nodes"][1]["value"]
let first_node = index_first["ir"]["nodes"][1]["value"]
say closure_contract["valid"]
say member_contract["valid"]
say index_contract["valid"]
say closure_root["valid"]
say member_root["valid"]
say index_root["valid"]
say inner["is_closure"]
say inner["captures"]
say name_node["expression_kind"]
say name_node["member_path"]
say name_node["inferred_type"]
say first_node["expression_kind"]
say first_node["index_depth"]
say first_node["inferred_type"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true true true true true true [value] member [data, user, name] any index 1 number" ]]; then
  echo "unexpected A9 closure/member output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 closure/member semantics gate passed: nested capture metadata, member-chain path, index depth, inferred types, recursive shape, and deterministic root contracts\n'
