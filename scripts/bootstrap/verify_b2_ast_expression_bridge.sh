#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-ast-type-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let one = {"kind": "literal", "literal_kind": "number"}
let two = {"kind": "literal", "literal_kind": "number"}
let text_a = {"kind": "literal", "literal_kind": "text"}
let text_b = {"kind": "literal", "literal_kind": "text"}
let add_num = {"kind": "binary", "left": one, "op": "add", "right": two}
let add_text = {"kind": "binary", "left": text_a, "op": "add", "right": text_b}
let compare = {"kind": "binary", "left": one, "op": "less", "right": two}
let negate = {"kind": "unary", "op": "negate", "value": one}
let values = {"kind": "list", "elements": [one, two]}
let empty = {"kind": "list", "elements": []}
say ast_expression_type(add_num)
say ast_expression_type(add_text)
say ast_expression_type(compare)
say ast_expression_type(negate)
say ast_expression_type(values)
say ast_expression_type(empty)
EOF
cat > "$expected" <<'EOF'
number
text
bool
number
list<number>
list<any>
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 AST expression bridge gate passed: literal, unary, binary, list, and empty-list inference\n'
