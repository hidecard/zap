#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
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
runner=$(mktemp "$ROOT_DIR/.zap-typed-ir-arbitrary.XXXXXX.zp")
runner_rel=$(basename "$runner")
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
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 arbitrary typed-IR gate passed: 10 control-statement emission cases\n'
