#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-constructors-inheritance.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let constructor = seed_compile_source("class Box:\n    fn __init__(self, value):\n        set self.value = value\n        return self\nlet box = Box(7)\nsay box.value", "constructor.zp")
let inherited = seed_compile_source("class Animal:\n    fn speak(self):\n        return 1\n    fn __init__(self, value):\n        set self.value = value\n        return self\nclass Dog extends Animal:\n    fn bark(self):\n        return 2\nlet dog = Dog(7)\nsay dog.speak()\nsay dog.bark()\nsay dog.value", "inherited.zp")
let override = seed_compile_source("class Animal:\n    fn speak(self):\n        return 1\nclass Dog extends Animal:\n    fn speak(self):\n        return 3\nlet dog = Dog()\nsay dog.speak()", "override.zp")
let bad_arity = seed_compile_source("class Needs:\n    fn __init__(self, value):\n        return self\nlet needs = Needs()", "bad_arity.zp")
let ast = seed_compile_ast_source("class Animal:\n    fn speak(self):\n        return 1\n    fn __init__(self, value):\n        self.value = value\n        return self\nclass Dog extends Animal:\n    fn bark(self):\n        return 2\nlet dog = Dog(7)\nsay dog.speak()\nsay dog.bark()\nsay dog.value", "ast_inherited.zp")
let constructor_result = vm_run(constructor["instructions"])
let inherited_result = vm_run(inherited["instructions"])
let override_result = vm_run(override["instructions"])
let bad_arity_result = vm_run(bad_arity["instructions"])
let ast_result = vm_run(ast["instructions"])
say constructor_result["output"][0]
say inherited_result["output"][0]
say inherited_result["output"][1]
say inherited_result["output"][2]
say override_result["output"][0]
say bad_arity_result["error"]
say ast_result["output"][0]
say ast_result["output"][1]
say ast_result["output"][2]
EOF
cat > "$expected" <<'EOF'
7
1
2
7
3
arity_error:__init__
1
2
7
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 constructor/inheritance gate passed: constructor args, inherited init/methods, overrides, arity diagnostics, and canonical AST path\n'
