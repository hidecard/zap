#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-typed-ir-expression.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let call = expression_node("identity(1)")
let nested = expression_node("identity(identity(1))")
let binary = expression_node("1 + 2")
let typed = from_json(emit("let value: number = identity(1)\n", "expr.zp"))
say call["kind"]
say call["callee"]["name"]
say call["args"][0]["value"]["literal_kind"]
say nested["kind"]
say nested["args"][0]["value"]["kind"]
say binary["kind"]
say binary["op"]
say typed["ir"]["nodes"][0]["value"]["kind"]
say typed["ir"]["nodes"][0]["value"]["callee"]["name"]
say typed["ir"]["nodes"][0]["inferred_type"]
EOF
cat > "$expected" <<'EOF'
call
identity
number
call
call
binary
add
call
identity
number
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR expression gate passed: 10 call, nested-call, binary, and declaration cases\n'
