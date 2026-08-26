#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-functions.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let add = seed_compile_source("fn add(a, b):\n    return a + b\nsay add(2, 3)", "add.zp")
let implicit = seed_compile_source("fn implicit():\n    say 4\nsay implicit()", "implicit.zp")
let nested = seed_compile_source("fn inner(value):\n    return value + 1\nfn outer(value):\n    return inner(value)\nsay outer(4)", "nested.zp")
let factorial = seed_compile_source("fn factorial(n):\n    if n == 0:\n        return 1\n    return n * factorial(n - 1)\nsay factorial(5)", "factorial.zp")
let cross_frame = seed_compile_source("fn raise_number():\n    raise 7\ntry:\n    say raise_number()\ncatch err:\n    say err\nsay 9", "cross_frame.zp")
let safe = seed_compile_source("fn safe():\n    try:\n        raise 6\n    catch err:\n        return err\nsay safe()", "safe.zp")
let arity = seed_compile_source("fn one(value):\n    return value\nsay one()", "arity.zp")
let outside = seed_compile_source("return 1", "outside.zp")
let nested_function = seed_compile_source("fn outer():\n    fn inner():\n        return 1\n    return 2", "nested_function.zp")
let add_result = vm_run(add["instructions"])
let implicit_result = vm_run(implicit["instructions"])
let nested_result = vm_run(nested["instructions"])
let factorial_result = vm_run(factorial["instructions"])
let cross_result = vm_run(cross_frame["instructions"])
let safe_result = vm_run(safe["instructions"])
let arity_result = vm_run(arity["instructions"])
say add_result["error"]
say add_result["output"][0]
say len(implicit_result["output"])
say implicit_result["output"][0]
say implicit_result["output"][1]
say nested_result["output"][0]
say factorial_result["output"][0]
say cross_result["output"][0]
say cross_result["output"][1]
say safe_result["output"][0]
say arity_result["error"]
say outside["status"]
say outside["error"]
say nested_function["status"]
EOF
cat > "$expected" <<'EOF'
none
5
2
4
none
5
120
7
9
6
arity_error:one
compile_error
return_outside_function
compiled_slice
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 functions gate passed: return, named calls, implicit none, recursion, frame-crossing catch, arity diagnostics, and scope boundaries\n'
