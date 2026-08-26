#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b3-canonical-schema.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
import "bootstrap/b3/lower.zp"
let member = expression_node("user.name")
let map = expression_node("{\"ok\": true}")
let canonical_member = {"kind": "member", "member": "name", "target": {"kind": "literal", "literal_kind": "map", "value": {"name": "Zap"}, "span": {"line": 1, "column": 1, "length": 9}}, "span": {"line": 1, "column": 1, "length": 14}}
let lowered_member = lower_expression(canonical_member)
say member["kind"]
say member["member"]
say member["target"]["kind"]
say map["kind"]
say map["entries"][0]["key"]["kind"]
say map["entries"][0]["value"]["kind"]
say lowered_member["error"]
EOF
cat > "$expected" <<'EOF'
member
name
name
map
literal
literal
none
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B3 canonical AST schema gate passed: member, map, and lowering fields\n'
