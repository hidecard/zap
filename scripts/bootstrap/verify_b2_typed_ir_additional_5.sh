#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-typed-additional.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let a = from_json(emit("raise \"bad\"", "raise.zp"))
let b = from_json(emit("import std.io", "import.zp"))
let c = from_json(emit("module demo", "module.zp"))
let d = from_json(emit("try:", "try.zp"))
let e = from_json(emit("catch:", "catch.zp"))
say a["ir"]["nodes"][0]["kind"]
say b["ir"]["nodes"][0]["kind"]
say c["ir"]["nodes"][0]["kind"]
say d["ir"]["nodes"][0]["kind"]
say e["ir"]["nodes"][0]["kind"]
EOF
cat > "$expected" <<'EOF'
raise
import
module
try
catch
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 typed-IR additional-statement gate passed: 5 raise/import/module/try/catch cases\n'
