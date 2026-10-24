#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then
    "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
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

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
run_case() {
  local runner_path="$1"
  local out="$2"
  local runner_name
  runner_name="$(basename "$runner_path")"
  cp "$runner_path" "$ROOT_DIR/.zap-break-$$-$runner_name"
  if [ -x "$ZAP_BIN" ]; then
    (cd "$ROOT_DIR" && "$ZAP_BIN" ".zap-break-$$-$runner_name") > "$out" 2>&1
  else
    (cd "$ROOT_DIR" && run_zap ".zap-break-$$-$runner_name") > "$out" 2>&1
  fi
  rm -f "$ROOT_DIR/.zap-break-$$-$runner_name"
}

TMPDIR_GATE=$(mktemp -d "$ROOT_DIR/.zap-break-gate.XXXXXX")
mkdir -p "$TMPDIR_GATE"

run_case() {
  local runner_path="$1"
  local out="$2"
  local runner_name
  runner_name="$(basename "$runner_path")"
  cp "$runner_path" "$ROOT_DIR/.zap-break-$$-$runner_name"
  if [ -x "$ZAP_BIN" ]; then
    (cd "$ROOT_DIR" && "$ZAP_BIN" ".zap-break-$$-$runner_name") > "$out" 2>&1
  else
    (cd "$ROOT_DIR" && run_zap ".zap-break-$$-$runner_name") > "$out" 2>&1
  fi
  rm -f "$ROOT_DIR/.zap-break-$$-$runner_name"
}

cat > "$TMPDIR_GATE/case1.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let while_break = driver_seed_compile_source("let flag = true\nwhile flag:\n    break\nsay 9", "while_break.zp")
say vm_run(while_break["instructions"])["output"][0]
EOF
run_case "$TMPDIR_GATE/case1.zp" "$TMPDIR_GATE/o1.out"

cat > "$TMPDIR_GATE/case2.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let while_continue = driver_seed_compile_source("let flag = true\nwhile flag:\n    let flag = false\n    continue\nsay 7", "while_continue.zp")
say vm_run(while_continue["instructions"])["output"][0]
EOF
run_case "$TMPDIR_GATE/case2.zp" "$TMPDIR_GATE/o2.out"

cat > "$TMPDIR_GATE/case3.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let for_control = driver_seed_compile_source("for item in [1, 2, 3]:\n    if item == 2:\n        continue\n    if item == 3:\n        break\n    say item\nsay 9", "for_control.zp")
let r = vm_run(for_control["instructions"])
say len(r["output"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case3.zp" "$TMPDIR_GATE/o3.out"

cat > "$TMPDIR_GATE/case4.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let nested_loop = driver_seed_compile_source("let flag = true\nwhile flag:\n    for item in [1, 2]:\n        if item == 2:\n            break\n        say item\n    let flag = false\nsay 9", "nested_loop.zp")
let r = vm_run(nested_loop["instructions"])
say len(r["output"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case4.zp" "$TMPDIR_GATE/o4.out"

cat > "$TMPDIR_GATE/case5.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let caught_number = driver_seed_compile_source("try:\n    raise 42\ncatch err:\n    say err\nsay 9", "caught_number.zp")
let r = vm_run(caught_number["instructions"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case5.zp" "$TMPDIR_GATE/o5.out"

cat > "$TMPDIR_GATE/case6.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let caught_text = driver_seed_compile_source("try:\n    raise \"oops\"\ncatch err:\n    say err\nsay 9", "caught_text.zp")
let r = vm_run(caught_text["instructions"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case6.zp" "$TMPDIR_GATE/o6.out"

cat > "$TMPDIR_GATE/case7.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let nested_caught = driver_seed_compile_source("try:\n    try:\n        raise 7\n    catch inner:\n        say inner\ncatch outer:\n    say outer\nsay 8", "nested_caught.zp")
let r = vm_run(nested_caught["instructions"])
say len(r["output"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case7.zp" "$TMPDIR_GATE/o7.out"

cat > "$TMPDIR_GATE/case8.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let normal_try = driver_seed_compile_source("try:\n    say 4\ncatch err:\n    say err\nsay 9", "normal_try.zp")
let r = vm_run(normal_try["instructions"])
say len(r["output"])
say r["output"][0]
say r["output"][1]
EOF
run_case "$TMPDIR_GATE/case8.zp" "$TMPDIR_GATE/o8.out"

cat > "$TMPDIR_GATE/case9.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let outside_break = driver_seed_compile_source("break", "outside_break.zp")
say outside_break["status"]
say outside_break["error"]
EOF
run_case "$TMPDIR_GATE/case9.zp" "$TMPDIR_GATE/o9.out"

cat > "$TMPDIR_GATE/case10.zp" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let outside_continue = driver_seed_compile_source("continue", "outside_continue.zp")
say outside_continue["status"]
say outside_continue["error"]
EOF
run_case "$TMPDIR_GATE/case10.zp" "$TMPDIR_GATE/o10.out"

out=$(mktemp)
{
  grep -v '^$' "$TMPDIR_GATE/o1.out"
  grep -v '^$' "$TMPDIR_GATE/o2.out"
  grep -v '^$' "$TMPDIR_GATE/o3.out"
  grep -v '^$' "$TMPDIR_GATE/o4.out"
  grep -v '^$' "$TMPDIR_GATE/o5.out"
  grep -v '^$' "$TMPDIR_GATE/o6.out"
  grep -v '^$' "$TMPDIR_GATE/o7.out"
  grep -v '^$' "$TMPDIR_GATE/o8.out"
  grep -v '^$' "$TMPDIR_GATE/o9.out"
  grep -v '^$' "$TMPDIR_GATE/o10.out"
} > "$out"

expected=$(mktemp)
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
unsupported_ast_statement:break
compile_error
unsupported_ast_statement:continue
EOF

cmp "$out" "$expected" || { echo '--- actual ---'; cat "$out"; echo '--- expected ---'; cat "$expected"; rm -rf "$TMPDIR_GATE" "$out" "$expected"; exit 1; }
rm -rf "$TMPDIR_GATE" "$out" "$expected"
printf 'B4 break/payload gate passed: loop exits, continue skipping, nested loops, catch bindings, normal flow, and payload types\n'
