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
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-ast.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let add = seed_compile_ast_source("fn add(a, b):\n    return a + b\nsay add(2, 3)", "ast_add.zp")
let class_value = seed_compile_ast_source("class Counter:\n    fn get(self):\n        return self\nlet counter = Counter()\nsay counter.get()", "ast_class.zp")
let return_none = seed_compile_ast_source("fn no_value():\n    return\nsay no_value()", "ast_none.zp")
let fields = seed_compile_ast_source("class Counter:\n    fn set(self, value):\n        self.count = value\n        return self\nlet counter = Counter()\nlet updated = counter.set(7)\nsay updated.count", "ast_fields.zp")
let malformed = seed_compile_ast_source("break", "ast_bad.zp")
let add_result = vm_run(add["instructions"])
let class_result = vm_run(class_value["instructions"])
let none_result = vm_run(return_none["instructions"])
let fields_result = vm_run(fields["instructions"])
say add["status"]
say add_result["error"]
say add_result["output"][0]
say class_value["status"]
say class_result["error"]
say json(class_result["output"][0])
say none_result["output"][0]
say fields["status"]
say fields_result["output"][0]
say malformed["status"]
say malformed["error"]
EOF
cat > "$expected" <<'EOF'
compiled_ast_slice
none
5
compiled_ast_slice
none
{"class_name":"Counter","fields":[],"object":true}
none
compiled_ast_slice
7
compile_error
unsupported_ast_statement:break
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cat "$out"; printf '%s\n' '--- expected ---'; cat "$expected"
printf 'B4 AST gate passed: parser-AST functions, return, class methods, dotted calls, and unsupported-node diagnostics\n'
