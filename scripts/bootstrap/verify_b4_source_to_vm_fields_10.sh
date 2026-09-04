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
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-fields.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let update = seed_compile_source("class Counter:\n    fn set(self, value):\n        set self.count = value\n        return self\nlet counter = Counter()\nlet updated = counter.set(7)\nsay updated.count", "update.zp")
let replace = seed_compile_source("class Counter:\n    fn set(self, value):\n        set self.count = value\n        return self\nlet counter = Counter()\nlet first = counter.set(1)\nlet second = first.set(9)\nsay second.count", "replace.zp")
let missing = seed_compile_source("class Empty:\n    fn get(self):\n        return self.missing\nlet empty = Empty()\nsay empty.get()", "missing.zp")
let update_result = vm_run(update["instructions"])
let replace_result = vm_run(replace["instructions"])
let missing_result = vm_run(missing["instructions"])
say update_result["output"][0]
say replace_result["output"][0]
say missing_result["error"]
EOF
cat > "$expected" <<'EOF'
7
9
unknown_field:missing
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 fields gate passed: field store/load, replacement, method-return rebinding, and missing-field diagnostics\n'
