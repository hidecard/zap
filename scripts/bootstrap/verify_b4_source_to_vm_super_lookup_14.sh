#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-super-lookup.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let chain = seed_compile_source("class Base:\n    fn value(self):\n        return 1\n    fn __init__(self, value):\n        set self.value = value\n        return self\nclass Middle extends Base:\n    fn value(self):\n        return super().value() + 1\nclass Child extends Middle:\n    fn value(self):\n        return super().value() + 1\n    fn __init__(self, value):\n        super().__init__(value)\n        return self\nlet child = Child(7)\nsay child.value()\nsay child.value", "chain.zp")
let override = seed_compile_source("class Base:\n    fn value(self):\n        return 1\nclass Child extends Base:\n    fn value(self):\n        return super().value() + 4\nlet child = Child()\nsay child.value()", "override.zp")
let no_parent = seed_compile_source("class Base:\n    fn value(self):\n        return super().value()\nlet base = Base()\nsay base.value()", "no_parent.zp")
let canonical = seed_compile_ast_source("class Base:\n    fn value(self):\n        return 1\n    fn __init__(self, value):\n        self.value = value\n        return self\nclass Middle extends Base:\n    fn value(self):\n        return super().value() + 1\nclass Child extends Middle:\n    fn value(self):\n        return super().value() + 1\n    fn __init__(self, value):\n        super().__init__(value)\n        return self\nlet child = Child(7)\nsay child.value()\nsay child.value", "canonical.zp")
let chain_result = vm_run(chain["instructions"])
let override_result = vm_run(override["instructions"])
let no_parent_result = vm_run(no_parent["instructions"])
let canonical_result = vm_run(canonical["instructions"])
say chain_result["error"]
say chain_result["output"][0]
say chain_result["output"][1]
say override_result["output"][0]
say no_parent_result["error"]
say canonical["status"]
say canonical_result["error"]
say canonical_result["output"][0]
say canonical_result["output"][1]
EOF
cat > "$expected" <<'EOF'
none
3
7
5
super_no_parent:Base
compiled_ast_slice
none
3
7
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 super lookup gate passed: multi-level parent dispatch, super method/constructor calls, overrides, canonical AST path, and no-parent diagnostics\n'
