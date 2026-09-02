#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-type-generic-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let number_node = {"kind": "literal", "literal_kind": "number"}
let text_node = {"kind": "literal", "literal_kind": "text"}
let count_node = {"kind": "name", "name": "count"}
let list_node = {"kind": "list", "elements": [count_node, number_node]}
let map_node = {"kind": "map", "entries": [{"key": text_node, "value": count_node}]}
let nested_node = {"kind": "list", "elements": [list_node]}
let map_index = {"kind": "index", "target": map_node, "index": text_node}
let list_index = {"kind": "index", "target": nested_node, "index": number_node}
let info = generic_declaration_info("fn pair<T, U>(value: T) -> U:")
say ast_expression_type_env(count_node, [{"name": "count", "type": "number"}])
say ast_expression_type_env(list_node, [{"name": "count", "type": "number"}])
say ast_expression_type_env(map_node, [{"name": "count", "type": "number"}])
say ast_expression_type_env(list_index, [{"name": "count", "type": "number"}])
say ast_expression_type_env(map_index, [{"name": "count", "type": "number"}])
say info["name"]
say len(info["parameters"])
say info["valid"]
say generic_parameters_valid(["T", "T"])
say generic_parameters_valid(["t"])
say generic_type_substitute("map<text,list<T>>", "T", "number")
say generic_identity_result("text", "option<result<T>>")
say generic_instantiate_return(["T", "U"], ["number", "text"], "map<T,list<U>>")
say generic_constraint_satisfied(["number", "number"], "number")
say generic_constraint_satisfied(["number", "text"], "number")
say generic_parameters_valid(["T", "U", "V", "E"])
EOF
cat > "$expected" <<'EOF'
number
list<number>
map<text,number>
list<number>
number
pair
2
true
false
false
map<text,list<number>>
option<result<text>>
map<number,list<text>>
true
false
true
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 type/generic gate passed: 15 inference and generic declaration cases\n'
