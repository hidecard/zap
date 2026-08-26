#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-decls.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let typed = parse_declaration("let count: number = 1", 1)
let text = parse_declaration("let label: text = \"zap\"", 2)
let untyped = parse_declaration("let value = none", 3)
say typed["name"]
say typed["annotation"]
say typed["value"]["kind"]
say text["annotation"]
say text["value"]["literal_kind"]
say untyped["annotation"]
EOF
cat > "$expected" <<'EOF'
count
number
literal
text
text
none
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B1 declaration-route gate passed: 6 typed and untyped declaration cases\n'
