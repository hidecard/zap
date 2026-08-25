#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
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
let info = generic_declaration_info("fn pair<T, U>(value: T) -> U:")
say ast_expression_type_env(count_node, [{"name": "count", "type": "number"}])
say ast_expression_type_env(list_node, [{"name": "count", "type": "number"}])
say ast_expression_type_env(map_node, [{"name": "count", "type": "number"}])
say info["name"]
say len(info["parameters"])
say info["valid"]
say generic_parameters_valid(["T", "T"])
say generic_parameters_valid(["t"])
say generic_type_substitute("map<text,list<T>>", "T", "number")
say generic_identity_result("text", "option<result<T>>")
say generic_parameters_valid(["T", "U", "V", "E"])
EOF
cat > "$expected" <<'EOF'
number
list<number>
map<text,number>
pair
2
true
false
false
map<text,list<number>>
option<result<text>>
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 type/generic gate passed: 10 inference and generic declaration cases\n'
