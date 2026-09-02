#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-source-vm.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let first = seed_compile_source("say 20 + 22", "seed.zp")
let second = seed_compile_source("say 20 + 22", "seed.zp")
let changed = seed_compile_source("say 20 - 22", "seed.zp")
let result = vm_run(first["instructions"])
say first["artifact_kind"]
say first["status"]
say first["native_independent"]
say len(first["instructions"])
say result["halted"]
say result["error"]
say result["output"][0]
say seed_compile_bytes(first) == seed_compile_bytes(second)
say seed_compile_bytes(first) == seed_compile_bytes(changed)
say changed["status"]
say len(changed["instructions"])
say vm_run(changed["instructions"])["output"][0]
EOF
cat > "$expected" <<'EOF'
bytecode
compiled_slice
false
5
true
none
42
true
false
compiled_slice
5
-2
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 source-to-VM gate passed: 10 bounded compiler-artifact and VM execution cases\n'
