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
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-extended.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let variables = seed_compile_source("let base: number = 20\nlet extra: number = 22\nsay base + extra", "variables.zp")
let identity = seed_compile_source("say identity(42)", "identity.zp")
let absolute = seed_compile_source("say abs(-7)", "abs.zp")
let text = seed_compile_source("say \"Zap\"", "text.zp")
let boolean = seed_compile_source("say true and false", "boolean.zp")
let negation = seed_compile_source("say not false", "negation.zp")
let list_literal = seed_compile_source("say [1, 2]", "list.zp")
let map_literal = seed_compile_source("say {\"ok\": true}", "map.zp")
let bad = seed_compile_source("say missing", "bad.zp")
let rebuilt = seed_self_rebuild("say 2 * 3", "rebuild.zp")
let variable_state = vm_run(variables["instructions"])
let identity_state = vm_run(identity["instructions"])
let absolute_state = vm_run(absolute["instructions"])
let text_state = vm_run(text["instructions"])
let boolean_state = vm_run(boolean["instructions"])
let negation_state = vm_run(negation["instructions"])
let list_state = vm_run(list_literal["instructions"])
let map_state = vm_run(map_literal["instructions"])
say variables["status"]
say len(variables["instructions"])
say variable_state["halted"]
say variable_state["error"]
say variable_state["output"][0]
say identity_state["output"][0]
say absolute_state["output"][0]
say text_state["output"][0]
say boolean_state["output"][0]
say negation_state["output"][0]
say list_literal["status"]
say list_state["halted"]
say list_state["error"]
say map_literal["status"]
say map_state["halted"]
say bad["status"]
say bad["error"]
say rebuilt["status"]
EOF
cat > "$expected" <<'EOF'
compiled_slice
9
true
none
42
42
7
Zap
false
true
compiled_slice
true
none
compiled_slice
true
compile_error
unknown_name:missing
reproducible
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 extended source-to-VM gate passed: declarations, loads, calls, literals, errors, and self-rebuild determinism\n'
