#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-source-vm-control-flow.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let yes = seed_compile_source("if true:\n    say 7\nelse:\n    say 9", "yes.zp")
let no = seed_compile_source("if false:\n    say 7\nelse:\n    say 9", "no.zp")
let no_else = seed_compile_source("if false:\n    say 7\nsay 3", "no_else.zp")
let nested = seed_compile_source("if false:\n    if true:\n        say 1\n    else:\n        say 2\nelse:\n    say 3", "nested.zp")
let missing = seed_compile_source("if true:\nsay 1", "missing.zp")
let rebuilt = seed_self_rebuild("if false:\n    say 1\nelse:\n    say 2", "rebuild.zp")
say yes["status"]
say vm_run(yes["instructions"])["output"][0]
say no["status"]
say vm_run(no["instructions"])["output"][0]
say no_else["status"]
say vm_run(no_else["instructions"])["output"][0]
say nested["status"]
say vm_run(nested["instructions"])["output"][0]
say missing["status"]
say missing["error"]
say rebuilt["status"]
say rebuilt["byte_equal"]
EOF
cat > "$expected" <<'EOF'
compiled_slice
7
compiled_slice
9
compiled_slice
3
compiled_slice
3
compile_error
missing_if_body
reproducible
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 control-flow source-to-VM gate passed: if/else, nested branches, fall-through, diagnostics, and deterministic rebuild\n'
