#!/usr/bin/env bash
# Verify B2 alias expansion and validation
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

echo "Testing alias expansion..."

f1="$ROOT_DIR/.zap-alias-nested.zp"
f2="$ROOT_DIR/.zap-alias-of-alias.zp"
f3="$ROOT_DIR/.zap-alias-generic.zp"
f4="$ROOT_DIR/.zap-alias-undeclared.zp"
runner="$ROOT_DIR/.zap-alias-runner.zp"
runner_rel=$(basename "$runner")
trap 'rm -f "$f1" "$f2" "$f3" "$f4" "$runner"' EXIT

cat > "$f1" << 'EOF'
type IntList = list<number>
let values: IntList = [1, 2, 3]
EOF

cat > "$f2" << 'EOF'
type Inner = list<number>
type Outer = Inner
let values: Outer = [1, 2, 3]
EOF

cat > "$f3" << 'EOF'
type Box<T> = option<T>
let boxed: Box<number> = some(42)
EOF

cat > "$f4" << 'EOF'
type Box<T> = option<U>
let boxed: Box<number> = some(42)
EOF

cat > "$runner" << 'EOF'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

let s1 = read_text(".zap-alias-nested.zp")
let p1 = from_json(parse_general(s1, "f1.zp"))
if b2c_check_program(p1["ast"], "f1.zp")["ok"]:
    say "C1_OK"

let s2 = read_text(".zap-alias-of-alias.zp")
let p2 = from_json(parse_general(s2, "f2.zp"))
if b2c_check_program(p2["ast"], "f2.zp")["ok"]:
    say "C2_OK"

let s3 = read_text(".zap-alias-generic.zp")
let p3 = from_json(parse_general(s3, "f3.zp"))
if b2c_check_program(p3["ast"], "f3.zp")["ok"]:
    say "C3_OK"

let s4 = read_text(".zap-alias-undeclared.zp")
let p4 = from_json(parse_general(s4, "f4.zp"))
if not b2c_check_program(p4["ast"], "f4.zp")["ok"]:
    say "C4_REJECTED"
EOF

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  out=$("$ZAP_BIN" "$runner_rel")
else
  out=$(run_zap "$runner_rel")
fi

echo "$out" | grep -q "C1_OK" || { echo "✗ Nested alias expansion rejected"; exit 1; }
echo "✓ Nested alias expansion accepted"

echo "$out" | grep -q "C2_OK" || { echo "✗ Alias of alias rejected"; exit 1; }
echo "✓ Alias of alias accepted"

echo "$out" | grep -q "C3_OK" || { echo "✗ Generic alias rejected"; exit 1; }
echo "✓ Generic alias accepted"

echo "$out" | grep -q "C4_REJECTED" || { echo "✗ Undeclared parameter accepted"; exit 1; }
echo "✓ Undeclared parameter rejected"

echo "Alias expansion verification passed"
