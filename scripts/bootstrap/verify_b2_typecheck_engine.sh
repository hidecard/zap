#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

fixture="bootstrap/fixtures/typecheck/flow_engine.zp"
runner=$(mktemp "$ROOT_DIR/.zap-b2-engine-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT

cat > "$expected" <<'EOF'
list<list<number>>
map<text,list<list<number>>>
number
option<number>
number
any
1
true
0
EOF

{
  printf '%s\n' 'import "bootstrap/b2/typecheck_engine.zp"'
  cat "$fixture"
} > "$runner"

ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 complete engine fixture passed: recursive typing, generic substitution, guard narrowing, branch join, and operator diagnostics\n'
