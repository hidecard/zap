#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b2-ast-inference.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp "${TMPDIR:-/tmp}/zap-b2-ast-inference-out.XXXXXX")
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let number = {"kind": "literal", "literal_kind": "number", "value": 1}
let two = {"kind": "literal", "literal_kind": "number", "value": 2}
let list = {"kind": "list", "elements": [number, two]}
let ast = {"statements": [{"annotation": none, "kind": "declaration", "name": "x", "value": number}, {"annotation": none, "kind": "declaration", "name": "xs", "value": list}]}
let result = infer_ast_program(ast)
say ast_lookup_type(result["environment"], "x")
say ast_lookup_type(result["environment"], "xs")
say infer_ast_expression({"kind": "binary", "left": number, "op": "add", "right": two}, [], [])
say infer_ast_expression({"kind": "index", "target": list, "index": number}, [], [])
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
grep -q '^number$' "$out"
grep -q '^list<number>$' "$out"
# The expression and index results are both number.
[ "$(grep -c '^number$' "$out")" -ge 3 ]
printf 'B2 AST inference gate passed: literals, recursive collections, binary expressions, and indexing\n'
