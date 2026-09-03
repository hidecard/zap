#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-recursive-block-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let one = parse_block_program(["say value"])
let two = parse_block_program(["if ready:", "    say value"])
let three = parse_block_program(["if ready:", "    while active:", "        say value"])
let four = parse_block_program(["if ready:", "    while active:", "        say value", "    say done"])
let five = parse_block_program(["if ready:", "    say value", "say finished"])
let six = parse_block_program(["for item in items:", "    if ready:", "        say item"])
let seven = indentation_stack_diagnostic(["if ready:", "    say value", "say finished"], "valid.zp")
let eight = indentation_stack_diagnostic(["if ready:", "        say value"], "jump.zp")
let nine = indentation_stack_diagnostic(["if ready:", "    say value", "  say bad"], "dedent.zp")
let ten = parse("if ready:\n    while active:\n        say value\nsay finished", "generic.zp")
say len(one)
say two[0]["kind"]
say two[0]["then_branch"]["statements"][0]["kind"]
say three[0]["then_branch"]["statements"][0]["kind"]
say len(four[0]["then_branch"]["statements"])
say len(five)
say six[0]["body"]["statements"][0]["kind"]
say seven
say eight["message"]
say nine["message"]
say contains(ten, "\"kind\":\"if\"")
EOF
cat > "$expected" <<'EOF'
1
if
say
while
2
2
if
none
unexpected indentation at line 2
invalid indentation at line 3
true
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B1 recursive-block gate passed: 10 nested-block and indentation-stack cases\n'
