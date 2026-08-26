#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-ir-expr.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let index = expression_node("items[0]")
let member = expression_node("user.name")
let awaited = expression_node("await task()")
let propagated = expression_node("try result")
say index["kind"]
say index["target"]["kind"]
say index["index"]["kind"]
say member["kind"]
say member["member"]
say awaited["kind"]
say propagated["kind"]
say propagated["value"]["kind"]
EOF
cat > "$expected" <<'EOF'
index
name
literal
member
name
await
propagate
name
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR expression-node gate passed: 8 index/member/await/propagate cases\n'
