#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-mutable-closures.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let counter = seed_compile_source("fn make_counter(start):\n    let current = start\n    fn next():\n        let current = current + 1\n        return current\n    return next\nlet next = make_counter(0)\nsay next()\nsay next()\nsay next()", "counter.zp")
let independent = seed_compile_source("fn make_counter(start):\n    let current = start\n    fn next():\n        let current = current + 1\n        return current\n    return next\nlet first = make_counter(1)\nlet second = make_counter(10)\nsay first()\nsay second()\nsay first()\nsay second()", "independent.zp")
let result = vm_run(counter["instructions"])
let independent_result = vm_run(independent["instructions"])
say result["error"]
say result["output"][0]
say result["output"][1]
say result["output"][2]
say independent_result["output"][0]
say independent_result["output"][1]
say independent_result["output"][2]
say independent_result["output"][3]
EOF
cat > "$expected" <<'EOF'
none
1
2
3
2
11
3
12
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 mutable closures gate passed: shared stateful captures, repeated calls, and independent environments\n'
