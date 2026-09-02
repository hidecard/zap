#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b1-general-parser.XXXXXX.zp")
out=$(mktemp "${TMPDIR:-/tmp}/zap-b1-general-parser-out.XXXXXX")
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
let mixed = "let first = 1\nsay first\nif first:\n    let second = 2\n    while second:\n        break\nelse:\n    say none"
let invalid = "if true:\n  say 1"
let missing = "while true:"
let invalid_delimiter = "let value = [1, 2"
let function_source = "fn choose(value):\n    if value:\n        return value\n    else:\n        return 0"
let class_source = "class Counter:\n    let value = 0"
let module_source = "module app.main\nimport app.core as core\nlet answer = 42"
say parse_general(mixed, "general-mixed.zp")
say parse_general(invalid, "general-invalid.zp")
say parse_general(missing, "general-missing.zp")
let delimiter_tokens = from_json(lex(invalid_delimiter, "general-delimiter.zp"))
say parse_general_with_tokens(invalid_delimiter, delimiter_tokens["tokens"], "general-delimiter.zp")
say parse_general(function_source, "general-function.zp")
say parse_general(class_source, "general-class.zp")
say parse_general(module_source, "general-module.zp")
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
grep -q '"kind":"zap.ast"' "$out"
grep -q '"name":"first"' "$out"
grep -q '"kind":"zap.diagnostics"' "$out"
grep -q 'invalid indentation' "$out"
grep -q 'block requires an indented body' "$out"
grep -q 'expected' "$out"
grep -q '"name":"choose"' "$out"
grep -q '"kind":"class"' "$out"
grep -q '"kind":"module"' "$out"
printf 'B1 general parser gate passed: mixed AST, indentation, and missing-block diagnostics\n'
