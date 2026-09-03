#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-ir-ops.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
say expression_node("1 + 2")["op"]
say expression_node("1 - 2")["op"]
say expression_node("1 * 2")["op"]
say expression_node("1 / 2")["op"]
say expression_node("1 % 2")["op"]
say expression_node("1 == 2")["op"]
say expression_node("true and false")["op"]
say expression_node("true or false")["op"]
EOF
cat > "$expected" <<'EOF'
add
subtract
multiply
divide
remainder
equal
and
or
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR operator gate passed: 8 arithmetic/comparison/logical cases\n'
