#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then "$ROOT_DIR/native/target/debug/zap" "$@"
  else cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-extended.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let variables = driver_seed_compile_source("let base: number = 20\nlet extra: number = 22\nsay base + extra", "variables.zp")
let identity = driver_seed_compile_source("say identity(42)", "identity.zp")
let absolute = driver_seed_compile_source("say abs(-7)", "abs.zp")
let text = driver_seed_compile_source("say \"Zap\"", "text.zp")
let boolean = driver_seed_compile_source("say true and false", "boolean.zp")
let negation = driver_seed_compile_source("say not false", "negation.zp")
let list_literal = driver_seed_compile_source("say [1, 2]", "list.zp")
let map_literal = driver_seed_compile_source("say {\"ok\": true}", "map.zp")
let bad = driver_seed_compile_source("say missing", "bad.zp")
let rebuilt = driver_seed_self_rebuild("say 2 * 3", "rebuild.zp")
let identity_state = vm_run(identity["instructions"])
let text_state = vm_run(text["instructions"])
let negation_state = vm_run(negation["instructions"])
say variables["status"]
say identity["status"]
say identity_state["halted"]
say identity_state["output"][0]
say text["status"]
say text_state["output"][0]
say negation["status"]
say negation_state["output"][0]
say absolute["status"]
say boolean["status"]
say list_literal["status"]
say map_literal["status"]
say bad["status"]
say bad["error"]
say rebuilt["status"]
EOF
cat > "$expected" <<'EOF'
compiled_slice
compiled_slice
true
42
compiled_slice
Zap
compiled_slice
true
compile_error
compiled_slice
compile_error
compile_error
compile_error
typed_ir_promotion_error
reproducible
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then "$ZAP_BIN" "$runner_rel"; else run_zap "$runner_rel"; fi > "$out"
cmp "$out" "$expected"
printf 'B4 extended source-to-VM gate passed: driver-owned supported execution, explicit unsupported-feature failures, and self-rebuild determinism\n'
