#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
contract="bootstrap/contracts/AST_SCHEMA.toml"
grep -qx 'schema = "zap.ast"' "$contract"
grep -qx 'version = 1' "$contract"
grep -qx 'schema_version = 1' <(sed -n '/^\[envelope\]/,/^\[/p' "$contract" | grep '^schema_version')
grep -qx 'schema_version = 1' <(sed -n '/^\[diagnostics\]/,/^\[/p' "$contract" | grep '^schema_version')
runner=$(mktemp "$ROOT_DIR/.zap-b3-canonical-schema.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typed_ir.zp"
import "bootstrap/b3/lower.zp"
let member = expression_node("user.name")
let map = expression_node("{\"ok\": true}")
let canonical_member = {"kind": "member", "member": "name", "target": {"kind": "literal", "literal_kind": "map", "value": {"name": "Zap"}, "span": {"line": 1, "column": 1, "length": 9}}, "span": {"line": 1, "column": 1, "length": 14}}
let lowered_member = lower_expression(canonical_member)
let source = "let answer = 1"
let tokens = from_json(lex(source, "schema.zp"))
let parsed = from_json(parse_or_diagnostics(source, tokens["tokens"], "schema.zp"))
say member["kind"]
say member["member"]
say member["target"]["kind"]
say map["kind"]
say map["entries"][0]["key"]["kind"]
say map["entries"][0]["value"]["kind"]
say lowered_member["error"]
say parsed["kind"]
say parsed["schema_version"]
EOF
cat > "$expected" <<'EOF'
member
name
name
map
literal
literal
none
zap.ast
1
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B3 canonical AST schema gate passed: versioned envelope, member/map nodes, and lowering fields\n'
