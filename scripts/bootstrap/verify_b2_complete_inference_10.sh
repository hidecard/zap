#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-inference.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let number = {"kind": "literal", "literal_kind": "number", "value": 1}
let text = {"kind": "literal", "literal_kind": "text", "value": "zap"}
let boolean = {"kind": "literal", "literal_kind": "bool", "value": true}
let list = {"kind": "list", "elements": [number, number]}
let nested = {"kind": "list", "elements": [list, list]}
let map = {"kind": "map", "entries": [{"key": text, "value": list}]}
let base = [{"name": "items", "type": "list<number>"}, {"name": "data", "type": "map<text,list<number>>"}]
let index_list = {"kind": "index", "target": {"kind": "name", "name": "items"}, "index": number}
let index_map = {"kind": "index", "target": {"kind": "name", "name": "data"}, "index": text}
let plus = {"kind": "binary", "left": number, "op": "add", "right": number}
let identity_call = {"kind": "call", "callee": {"kind": "name", "name": "identity"}, "args": [{"kind": "positional", "value": text}]}
let known_call = {"kind": "call", "callee": {"kind": "name", "name": "make_number"}, "args": []}
let call_environment = [{"name": "make_number", "type": "number"}]
let program = [{"kind": "declaration", "name": "x", "annotation": none, "value": number}, {"kind": "assignment", "name": "x", "value": text}]
say inferred_value_type(number, base)
say inferred_value_type(text, base)
say inferred_value_type(boolean, base)
say inferred_value_type(list, base)
say inferred_value_type(nested, base)
say inferred_value_type(map, base)
say inferred_value_type(index_list, base)
say inferred_value_type(index_map, base)
say inferred_value_type(plus, base)
say inferred_value_type(identity_call, base)
say inferred_value_type(known_call, call_environment)
say ast_lookup_type(infer_program_types(program, base), "x")
say type_wrapper_parts("map<text,map<number,list<bool>>>")["value"]
say generic_container_element("map<text,map<number,list<bool>>>")
say generic_container_valid("map<text,map<number,list<bool>>>")
EOF
cat > "$expected" <<'EOF'
number
text
bool
list<number>
list<list<number>>
map<text,list<number>>
number
list<number>
number
text
number
text
map<number,list<bool>>
map<number,list<bool>>
true
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 complete-inference gate passed: 15 recursive value, call, nested-container, and program-flow cases\n'
