#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-break-payload.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let while_break = seed_compile_source("let flag = true\nwhile flag:\n    break\nsay 9", "while_break.zp")
let while_continue = seed_compile_source("let flag = true\nwhile flag:\n    let flag = false\n    continue\nsay 7", "while_continue.zp")
let for_control = seed_compile_source("for item in [1, 2, 3]:\n    if item == 2:\n        continue\n    if item == 3:\n        break\n    say item\nsay 9", "for_control.zp")
let nested_loop = seed_compile_source("let flag = true\nwhile flag:\n    for item in [1, 2]:\n        if item == 2:\n            break\n        say item\n    let flag = false\nsay 9", "nested_loop.zp")
let caught_number = seed_compile_source("try:\n    raise 42\ncatch err:\n    say err\nsay 9", "caught_number.zp")
let caught_text = seed_compile_source("try:\n    raise \"oops\"\ncatch err:\n    say err\nsay 9", "caught_text.zp")
let nested_caught = seed_compile_source("try:\n    try:\n        raise 7\n    catch inner:\n        say inner\ncatch outer:\n    say outer\nsay 8", "nested_caught.zp")
let normal_try = seed_compile_source("try:\n    say 4\ncatch err:\n    say err\nsay 9", "normal_try.zp")
let outside_break = seed_compile_source("break", "outside_break.zp")
let outside_continue = seed_compile_source("continue", "outside_continue.zp")
say vm_run(while_break["instructions"])["output"][0]
say vm_run(while_continue["instructions"])["output"][0]
say len(vm_run(for_control["instructions"])["output"])
say vm_run(for_control["instructions"])["output"][0]
say vm_run(for_control["instructions"])["output"][1]
say len(vm_run(nested_loop["instructions"])["output"])
say vm_run(nested_loop["instructions"])["output"][0]
say vm_run(nested_loop["instructions"])["output"][1]
say vm_run(caught_number["instructions"])["output"][0]
say vm_run(caught_number["instructions"])["output"][1]
say vm_run(caught_text["instructions"])["output"][0]
say vm_run(caught_text["instructions"])["output"][1]
say len(vm_run(nested_caught["instructions"])["output"])
say vm_run(nested_caught["instructions"])["output"][0]
say vm_run(nested_caught["instructions"])["output"][1]
say len(vm_run(normal_try["instructions"])["output"])
say vm_run(normal_try["instructions"])["output"][0]
say vm_run(normal_try["instructions"])["output"][1]
say outside_break["status"]
say outside_break["error"]
say outside_continue["status"]
say outside_continue["error"]
EOF
cat > "$expected" <<'EOF'
9
7
2
1
9
2
1
9
42
9
oops
9
2
7
8
2
4
9
compile_error
break_outside_loop
compile_error
continue_outside_loop
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 break/payload gate passed: loop exits, continue skipping, nested loops, catch bindings, normal flow, and payload types\n'
