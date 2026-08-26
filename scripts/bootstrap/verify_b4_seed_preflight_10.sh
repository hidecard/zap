#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-seed-preflight.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b3/package.zp"
import "bootstrap/b3/seed.zp"
let seed = platform_seed("0.0.1", "linux-x86_64", "B3")
let manifest_value = manifest("seed", "0.0.1", "main.zp", [])
let lock = lockfile(manifest_value, [])
let artifact = seed_artifact(seed, lock, 1)
let acceptance = seed_acceptance(seed, "B3", "linux-x86_64")
say seed["version"]
say seed["target"]
say seed["compiler_stage"]
say seed["schema_version"]
say artifact["lock_schema"]
say artifact["typed_ir_schema"]
say acceptance["accepted"]
say acceptance["stage"]
say acceptance["target"]
say seed_is_reproducible(seed, platform_seed("0.0.1", "linux-x86_64", "B3"))
EOF
cat > "$expected" <<'EOF'
0.0.1
linux-x86_64
B3
1
1
1
true
B3
linux-x86_64
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 seed preflight gate passed: 10 deterministic seed cases\n'
