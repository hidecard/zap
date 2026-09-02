#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-loops-try.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let while_program = seed_compile_source("let flag = true\nwhile flag:\n    let flag = false\nsay 3", "while.zp")
let for_program = seed_compile_source("for item in [1, 2, 3]:\n    say item\nsay 9", "for.zp")
let nested_program = seed_compile_source("let flag = true\nwhile flag:\n    for item in [1, 2]:\n        say item\n    let flag = false\nsay 9", "nested.zp")
let try_raise = seed_compile_source("try:\n    raise 7\ncatch:\n    say 8\nsay 9", "try_raise.zp")
let try_normal = seed_compile_source("try:\n    say 4\ncatch:\n    say 8\nsay 9", "try_normal.zp")
let missing_while = seed_compile_source("while true:", "missing_while.zp")
let bad_for = seed_compile_source("for item in values:\n    say item", "bad_for.zp")
let missing_catch = seed_compile_source("try:\n    say 4", "missing_catch.zp")
let raised = seed_compile_source("raise 7", "raised.zp")
let rebuilt = seed_self_rebuild("while false:\n    say 1\nsay 2", "rebuild.zp")
say while_program["status"]
say vm_run(while_program["instructions"])["output"][0]
say for_program["status"]
say len(vm_run(for_program["instructions"])["output"])
say vm_run(for_program["instructions"])["output"][0]
say vm_run(for_program["instructions"])["output"][3]
say nested_program["status"]
say len(vm_run(nested_program["instructions"])["output"])
say try_raise["status"]
say vm_run(try_raise["instructions"])["output"][0]
say vm_run(try_raise["instructions"])["output"][1]
say try_normal["status"]
say vm_run(try_normal["instructions"])["output"][0]
say vm_run(try_normal["instructions"])["output"][1]
say missing_while["status"]
say missing_while["error"]
say bad_for["status"]
say bad_for["error"]
say missing_catch["status"]
say missing_catch["error"]
say raised["status"]
say vm_run(raised["instructions"])["error"]
say rebuilt["status"]
say rebuilt["byte_equal"]
EOF
cat > "$expected" <<'EOF'
compiled_slice
3
compiled_slice
4
1
9
compiled_slice
3
compiled_slice
8
9
compiled_slice
4
9
compile_error
missing_while_body
compile_error
unsupported_for_iterable
compile_error
missing_catch
compiled_slice
raised
reproducible
true
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 loop/exception source-to-VM gate passed: while, literal-list for, nested loops, try/catch, diagnostics, and rebuild\n'
