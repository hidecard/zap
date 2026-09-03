#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-parser-stmt.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let empty_return = parse_statement("return", 1, 1)
let raise_statement = parse_statement("raise \"bad\"", 2, 1)
let import_statement = parse_statement("import std.io", 3, 1)
let module_statement = parse_statement("module demo", 4, 1)
say empty_return["kind"]
say empty_return["value"] == none
say raise_statement["kind"]
say raise_statement["value"]["literal_kind"]
say import_statement["kind"]
say import_statement["path"]
say module_statement["kind"]
say module_statement["name"]
EOF
cat > "$expected" <<'EOF'
return
true
raise
text
import
std.io
module
demo
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B1 statement coverage gate passed: 8 return/raise/import/module cases\n'
