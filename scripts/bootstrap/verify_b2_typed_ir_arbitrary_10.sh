#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-typed-ir-arbitrary.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let output = from_json(emit("if ready:\n    say \"yes\"\nwhile running:\n    return\nfor item in values:\n    say item\n", "control.zp"))
let nodes = output["ir"]["nodes"]
say len(nodes)
say nodes[0]["kind"]
say nodes[0]["successors"][0]
say nodes[1]["kind"]
say nodes[1]["span"]["line"]
say nodes[2]["kind"]
say nodes[3]["kind"]
say nodes[4]["kind"]
say nodes[4]["successors"][0]
EOF
cat > "$expected" <<'EOF'
6
if
2
say
2
while
return
for
6
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 arbitrary typed-IR gate passed: 10 control-statement emission cases\n'
