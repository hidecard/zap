#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-artifact-rebuild.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/rebuild.zp"
let plan = rebuild_plan("platform-seed-0", [])
let first = rebuild_compiler_artifact(plan, "typed_ir", "bytecode", "ZAP-ARTIFACT-v1", "digest-1")
let second = rebuild_compiler_artifact(plan, "typed_ir", "bytecode", "ZAP-ARTIFACT-v1", "digest-1")
let changed = rebuild_compiler_artifact(plan, "typed_ir", "bytecode", "ZAP-ARTIFACT-v2", "digest-2")
let equal = rebuild_two_pass(first, second)
let unequal = rebuild_two_pass(first, changed)
say equal["status"]
say equal["byte_equal"]
say len(equal["first_bytes"])
say equal["first_bytes"] == equal["second_bytes"]
say unequal["status"]
say unequal["byte_equal"]
say unequal["first_bytes"] == unequal["second_bytes"]
EOF
cat > "$expected" <<'EOF'
reproducible
true
127
true
mismatch
false
false
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 compiler-artifact rebuild gate passed: 8 two-pass byte-equality and mismatch cases\n'
