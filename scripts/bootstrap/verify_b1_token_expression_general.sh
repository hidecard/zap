#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b1-token-expression.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp "${TMPDIR:-/tmp}/zap-b1-token-expression-out.XXXXXX")
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
let arithmetic = from_json(lex("1 + 2 * 3", "arithmetic.zp"))
let logic = from_json(lex("not false or true and false", "logic.zp"))
let grouped = from_json(lex("(1 + 2) * 3", "grouped.zp"))
let list = from_json(lex("[1, 2 + 3]", "list.zp"))
let suffix = from_json(lex("service.fetch(1, key = 2)[0].value", "suffix.zp"))
say parse_token_expression(arithmetic["tokens"])
say parse_token_expression(logic["tokens"])
say parse_token_expression(grouped["tokens"])
say parse_token_expression(list["tokens"])
say parse_token_expression(suffix["tokens"])
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
grep -q '"op":"multiply"' "$out"
grep -q '"op":"or"' "$out"
grep -q '"op":"and"' "$out"
grep -q '"op":"not"' "$out"
grep -q '"kind":"list"' "$out"
grep -q '"member":"value"' "$out"
grep -q '"kind":"named"' "$out"
printf 'B1 token expression gate passed: precedence, unary, logical, grouped, and list expressions\n'
