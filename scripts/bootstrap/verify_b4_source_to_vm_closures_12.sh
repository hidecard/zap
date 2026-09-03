#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-closures.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let closure = seed_compile_source("fn make_adder(base):\n    fn add(value):\n        return base + value\n    return add\nlet add_two = make_adder(2)\nlet add_five = make_adder(5)\nsay add_two(3)\nsay add_five(3)", "closure.zp")
let captured_text = seed_compile_source("fn make_message(prefix):\n    fn message(value):\n        return prefix + value\n    return message\nlet greet = make_message(\"hello \" )\nsay greet(\"zap\")", "captured_text.zp")
let result = vm_run(closure["instructions"])
let text_result = vm_run(captured_text["instructions"])
say result["error"]
say result["output"][0]
say result["output"][1]
say text_result["output"][0]
EOF
cat > "$expected" <<'EOF'
none
5
8
hello zap
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 closures gate passed: nested definitions, immutable captures, distinct environments, and captured text\n'
