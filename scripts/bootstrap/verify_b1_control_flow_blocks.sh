#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-control-flow-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let for_program = from_json(parse("for item in items:\n    break\nsay finished", "for.zp"))
let while_program = from_json(parse("while ready:\n    continue\nsay finished", "while.zp"))
let nested = parse_block_program(["for item in items:", "    if ready:", "        while active:", "            continue"])
let branch = parse_block_program(["if ready:", "    say yes", "else:", "    say no"])
let chain = parse_block_program(["if first:", "    say one", "elif second:", "    say two", "else:", "    say three"])
let valid = indentation_stack_diagnostic(["if ready:", "    say yes", "say done"], "valid.zp")
let missing = missing_block_diagnostic(["while ready:"], "missing.zp")
let jump = indentation_stack_diagnostic(["while ready:", "        continue"], "jump.zp")
let dedent = indentation_stack_diagnostic(["if ready:", "    say yes", "  say bad"], "dedent.zp")
let branch_json = json(branch)
say for_program["ast"]["statements"][0]["kind"]
say for_program["ast"]["statements"][0]["body"]["statements"][0]["kind"]
say while_program["ast"]["statements"][0]["body"]["statements"][0]["kind"]
say nested[0]["body"]["statements"][0]["then_branch"]["statements"][0]["kind"]
say branch[0]["else_branch"]["statements"][0]["kind"]
say chain[0]["else_branch"]["statements"][0]["condition"]["name"]
say valid
say missing["message"]
say jump["message"]
say contains(branch_json, "else_branch")
EOF
cat > "$expected" <<'EOF'
for
break
continue
while
say
second
none
block requires an indented body at line 1
unexpected indentation at line 2
true
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B1 generic control-flow gate passed: 10 recursive block and diagnostic cases\n'
