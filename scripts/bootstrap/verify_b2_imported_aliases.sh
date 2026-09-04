#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true

echo "Testing imported aliases..."

runner="$ROOT_DIR/.zap-imported-alias-runner.zp"
runner_rel=$(basename "$runner")
trap 'rm -f "$runner"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

let s1 = read_text("bootstrap/fixtures/typecheck/alias_imported.zp")
let p1 = from_json(parse_general(s1, "bootstrap/fixtures/typecheck/alias_imported.zp"))
let r1 = b2c_check_program(p1["ast"], "bootstrap/fixtures/typecheck/alias_imported.zp")
if r1["ok"]:
    say "C1_OK"

let s2 = read_text("bootstrap/fixtures/typecheck/alias_imported_nested.zp")
let p2 = from_json(parse_general(s2, "bootstrap/fixtures/typecheck/alias_imported_nested.zp"))
let r2 = b2c_check_program(p2["ast"], "bootstrap/fixtures/typecheck/alias_imported_nested.zp")
if r2["ok"]:
    say "C2_OK"

let s3 = read_text("bootstrap/fixtures/typecheck/alias_imported.zp")
let p3 = from_json(parse_general(s3, "bootstrap/fixtures/typecheck/alias_imported.zp"))
let aliases = b2c_collect_type_aliases(p3["ast"]["statements"], [], "bootstrap/fixtures/typecheck/alias_imported.zp")
let alias_names = []
for a in aliases:
    alias_names = append(alias_names, a["name"])
if contains(alias_names, "Boxed"):
    say "C3_ALIAS_FOUND"

let s4 = read_text("bootstrap/fixtures/typecheck/alias_imported_nested.zp")
let p4 = from_json(parse_general(s4, "bootstrap/fixtures/typecheck/alias_imported_nested.zp"))
let nested_aliases = b2c_collect_type_aliases(p4["ast"]["statements"], [], "bootstrap/fixtures/typecheck/alias_imported_nested.zp")
let nested_alias_names = []
for a in nested_aliases:
    nested_alias_names = append(nested_alias_names, a["name"])
if contains(nested_alias_names, "Inner"):
    say "C4_NESTED_ALIAS_FOUND"
EOF

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  out=$("$ZAP_BIN" "$runner_rel")
else
  out=$(run_zap "$runner_rel")
fi

echo "$out" | grep -q "C1_OK" || { echo "✗ Imported generic alias rejected"; exit 1; }
echo "✓ Imported generic alias accepted"

echo "$out" | grep -q "C2_OK" || { echo "✗ Imported nested alias rejected"; exit 1; }
echo "✓ Imported nested alias accepted"

echo "$out" | grep -q "C3_ALIAS_FOUND" || { echo "✗ Imported alias Boxed not found"; exit 1; }
echo "✓ Imported alias Boxed found"

echo "$out" | grep -q "C4_NESTED_ALIAS_FOUND" || { echo "✗ Imported nested alias Inner not found"; exit 1; }
echo "✓ Imported nested alias Inner found"

echo "Imported aliases verification passed"
