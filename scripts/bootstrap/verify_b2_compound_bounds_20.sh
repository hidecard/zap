#!/usr/bin/env bash
# Verify B2 compound generic bounds (T: A + B syntax)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true

echo "Testing compound generic bounds..."

valid_source="$ROOT_DIR/.zap-compound-valid.zp"
invalid_source="$ROOT_DIR/.zap-compound-invalid.zp"
runner="$ROOT_DIR/.zap-compound-runner.zp"
trap 'rm -f "$valid_source" "$invalid_source" "$runner"' EXIT

cat > "$valid_source" << 'EOF'
fn bounded<T: number + text>(value: T) -> T:
    return value

let result: number = bounded(1)
EOF

cat > "$invalid_source" << 'EOF'
fn bounded<T: number + text>(value: T) -> T:
    return value

let result: bool = bounded(true)
EOF

cat > "$runner" << 'EOF'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

let source_v = read_text(".zap-compound-valid.zp")
let parsed_v = from_json(parse_general(source_v, "compound_valid.zp"))
let result_v = b2c_check_program(parsed_v["ast"], "compound_valid.zp")
if result_v["ok"]:
    say "VALID_OK"
else:
    say "VALID_FAIL"

let source_i = read_text(".zap-compound-invalid.zp")
let parsed_i = from_json(parse_general(source_i, "compound_invalid.zp"))
let result_i = b2c_check_program(parsed_i["ast"], "compound_invalid.zp")
if not result_i["ok"]:
    say "INVALID_REJECTED"
else:
    say "INVALID_ACCEPTED"
EOF

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  out=$("$ZAP_BIN" "$runner_rel")
else
  out=$(cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel")
fi

echo "$out" | grep -q "VALID_OK" || { echo "✗ Valid compound bounds rejected"; exit 1; }
echo "✓ Valid compound bounds accepted"

echo "$out" | grep -q "INVALID_REJECTED" || { echo "✗ Invalid compound bounds accepted"; exit 1; }
echo "✓ Invalid compound bounds rejected"

echo "Compound bounds verification passed"
