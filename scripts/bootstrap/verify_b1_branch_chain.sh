#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-branch-chain-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
let simple = parse_block_program(["if ready:", "    say yes", "else:", "    say no"])
let chain = parse_block_program(["if first:", "    say one", "elif second:", "    say two", "else:", "    say three"])
let nested = parse_block_program(["if outer:", "    if inner:", "        say yes", "    else:", "        say no"])
let valid = indentation_stack_diagnostic(["if ready:", "    say yes", "say done"], "valid.zp")
let jump = indentation_stack_diagnostic(["if ready:", "        say yes"], "jump.zp")
let bad_dedent = indentation_stack_diagnostic(["if ready:", "    say yes", "  say no"], "bad.zp")
let missing_if = missing_block_diagnostic(["if ready:"], "missing-if.zp")
let missing_else = missing_block_diagnostic(["if ready:", "    say yes", "else:"], "missing-else.zp")
let generic = parse("if ready:\n    say yes\nelse:\n    say no", "generic.zp")
say simple[0]["else_branch"]["statements"][0]["kind"]
say chain[0]["else_branch"]["statements"][0]["kind"]
say chain[0]["else_branch"]["statements"][0]["condition"]["name"]
say chain[0]["else_branch"]["statements"][0]["else_branch"]["statements"][0]["kind"]
say nested[0]["then_branch"]["statements"][0]["else_branch"]["statements"][0]["kind"]
say valid
say jump["message"]
say bad_dedent["message"]
say missing_if["message"]
say missing_else["message"]
say contains(generic, "\"else_branch\"")
EOF
cat > "$expected" <<'EOF'
say
if
second
say
say
none
unexpected indentation at line 2
invalid indentation at line 3
block requires an indented body at line 1
block requires an indented body at line 3
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B1 branch-chain gate passed: 10 if/elif/else and missing-body cases\n'
