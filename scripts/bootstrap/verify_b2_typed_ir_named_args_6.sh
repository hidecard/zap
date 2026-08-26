#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-named-ir.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let call = expression_node("render(title = \"Zap\", count = 2)")
say call["kind"]
say len(call["args"])
say call["args"][0]["kind"]
say call["args"][0]["name"]
say call["args"][1]["kind"]
say call["args"][1]["value"]["literal_kind"]
EOF
cat > "$expected" <<'EOF'
call
2
named
title
named
number
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR named-argument gate passed: 6 named-call argument cases\n'
