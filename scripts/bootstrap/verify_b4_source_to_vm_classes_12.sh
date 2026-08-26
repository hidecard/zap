#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-classes.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let receiver = seed_compile_source("class Counter:\n    fn get(self):\n        return self\nlet counter = Counter()\nsay counter.get()", "receiver.zp")
let method_args = seed_compile_source("class Calculator:\n    fn add(self, value):\n        return value + 2\nlet calculator = Calculator()\nsay calculator.add(5)", "method_args.zp")
let two_methods = seed_compile_source("class Pair:\n    fn left(self):\n        return 1\n    fn right(self):\n        return 2\nlet pair = Pair()\nsay pair.left()\nsay pair.right()", "two_methods.zp")
let missing_body = seed_compile_source("class Empty:", "missing_body.zp")
let nested_class = seed_compile_source("fn outer():\n    class Inner:\n        fn get(self):\n            return 1\n    return 2", "nested_class.zp")
let receiver_result = vm_run(receiver["instructions"])
let method_result = vm_run(method_args["instructions"])
let two_result = vm_run(two_methods["instructions"])
say receiver_result["error"]
say json(receiver_result["output"][0])
say method_result["output"][0]
say two_result["output"][0]
say two_result["output"][1]
say missing_body["status"]
say missing_body["error"]
say nested_class["status"]
say nested_class["error"]
EOF
cat > "$expected" <<'EOF'
none
{"class_name":"Counter","fields":[],"object":true}
7
1
2
compile_error
missing_class_body
compile_error
nested_class_unsupported
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 classes gate passed: class descriptors, instances, receiver frames, method arguments, multiple methods, and diagnostics\n'
