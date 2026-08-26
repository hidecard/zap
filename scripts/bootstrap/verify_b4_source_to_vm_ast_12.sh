#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-ast.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let add = seed_compile_ast_source("fn add(a, b):\n    return a + b\nsay add(2, 3)", "ast_add.zp")
let class_value = seed_compile_ast_source("class Counter:\n    fn get(self):\n        return self\nlet counter = Counter()\nsay counter.get()", "ast_class.zp")
let return_none = seed_compile_ast_source("fn no_value():\n    return\nsay no_value()", "ast_none.zp")
let malformed = seed_compile_ast_source("break", "ast_bad.zp")
let add_result = vm_run(add["instructions"])
let class_result = vm_run(class_value["instructions"])
let none_result = vm_run(return_none["instructions"])
say add["status"]
say add_result["error"]
say add_result["output"][0]
say class_value["status"]
say class_result["error"]
say json(class_result["output"][0])
say none_result["output"][0]
say malformed["status"]
say malformed["error"]
EOF
cat > "$expected" <<'EOF'
compiled_ast_slice
none
5
compiled_ast_slice
none
{"class_name":"Counter","object":true}
none
compile_error
unsupported_ast_statement:break
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 AST gate passed: parser-AST functions, return, class methods, dotted calls, and unsupported-node diagnostics\n'
