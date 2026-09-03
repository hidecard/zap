#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-c3-mro.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let diamond = seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "diamond.zp")
let reverse = seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Right, Left:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "reverse.zp")
let super_chain = seed_compile_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return super().value() + 10\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "super-chain.zp")
let constructors = seed_compile_source("class Root:\n    fn __init__(self):\n        set self.value = 1\n        return self\nclass Left extends Root:\n    fn __init__(self):\n        super().__init__()\n        set self.value = 2\n        return self\nclass Right extends Root:\n    fn __init__(self):\n        super().__init__()\n        set self.value = 3\n        return self\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value", "constructors.zp")
let conflict = seed_compile_source("class A:\n    fn marker(self):\n        return 0\nclass B:\n    fn marker(self):\n        return 0\nclass C extends A, B:\n    fn marker(self):\n        return 0\nclass D extends B, A:\n    fn marker(self):\n        return 0\nclass E extends C, D:\n    fn marker(self):\n        return 0\nlet value = E()", "conflict.zp")
let unknown = seed_compile_source("class Child extends Missing:\n    fn marker(self):\n        return 0\nlet value = Child()", "unknown.zp")
let cycle = seed_compile_source("class A extends B:\n    fn marker(self):\n        return 0\nclass B extends A:\n    fn marker(self):\n        return 0\nlet value = A()", "cycle.zp")
let canonical = seed_compile_ast_source("class Root:\n    fn value(self):\n        return 1\nclass Left extends Root:\n    fn value(self):\n        return 2\nclass Right extends Root:\n    fn value(self):\n        return 3\nclass Child extends Left, Right:\n    fn marker(self):\n        return 0\nlet child = Child()\nsay child.value()", "canonical.zp")
let diamond_result = vm_run(diamond["instructions"])
let reverse_result = vm_run(reverse["instructions"])
let super_result = vm_run(super_chain["instructions"])
let constructor_result = vm_run(constructors["instructions"])
let conflict_result = vm_run(conflict["instructions"])
let unknown_result = vm_run(unknown["instructions"])
let cycle_result = vm_run(cycle["instructions"])
let canonical_result = vm_run(canonical["instructions"])
say diamond["status"]
say diamond_result["error"]
say diamond_result["output"][0]
say reverse_result["output"][0]
say super_result["output"][0]
say constructor_result["output"][0]
say conflict_result["error"]
say unknown_result["error"]
say cycle_result["error"]
say canonical["status"]
say canonical_result["error"]
say canonical_result["output"][0]
EOF
cat > "$expected" <<'EOF'
compiled_slice
none
2
3
13
2
inconsistent_mro:E
unknown_class:Missing
inheritance_cycle:A
compiled_ast_slice
none
2
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 C3 MRO gate passed: deterministic diamond precedence, MRO-based super continuation, inherited cooperative constructors, canonical AST multiple parents, and stable hierarchy diagnostics\n'
