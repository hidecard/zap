#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

fixture="bootstrap/fixtures/typecheck/flow_engine.zp"
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$out" "$expected"' EXIT

cat > "$expected" <<'EOF'
list<list<number>>
map<text,list<list<number>>>
number
option<number>
number
any
1
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$fixture" > "$out"
cmp "$out" "$expected"
printf 'B2 complete engine fixture passed: recursive typing, generic substitution, guard narrowing, branch join, and operator diagnostics\n'
