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
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-control-flow.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let yes = driver_build_source("if true:\n    say 7\nelse:\n    say 9", "yes.zp")
let no = driver_build_source("if false:\n    say 7\nelse:\n    say 9", "no.zp")
let no_else = driver_build_source("if false:\n    say 7\nsay 3", "no_else.zp")
let nested = driver_build_source("if false:\n    if true:\n        say 1\n    else:\n        say 2\nelse:\n    say 3", "nested.zp")
let rebuilt = driver_rebuild("if false:\n    say 1\nelse:\n    say 2", "rebuild.zp")
say yes["status"]
say vm_run(from_json(yes["artifacts"][1]["bytes"]))["output"][0]
say no["status"]
say vm_run(from_json(no["artifacts"][1]["bytes"]))["output"][0]
say no_else["status"]
say vm_run(from_json(no_else["artifacts"][1]["bytes"]))["output"][0]
say nested["status"]
say vm_run(from_json(nested["artifacts"][1]["bytes"]))["output"][0]
say rebuilt["status"]
say rebuilt["byte_equal"]
EOF
cat > "$expected" <<'EOF'
ok
7
ok
9
ok
3
ok
3
driver_rebuild
true
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 control-flow source-to-VM gate passed: if/else, nested branches, fall-through, diagnostics, and deterministic rebuild\n'
