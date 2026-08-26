#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-build-plan.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b3/package.zp"
import "bootstrap/b3/build.zp"
let manifest_value = manifest("demo", "0.1.0", "main.zp", [])
let lock = lockfile(manifest_value, [])
let plan = build_plan(manifest_value, lock, "linux-x86_64", "0.0.1")
let ir = artifact_descriptor(plan, "typed_ir", "ir-1")
let bytecode = artifact_descriptor(plan, "bytecode", "bc-1")
let runtime = artifact_descriptor(plan, "runtime", "rt-1")
let result = build_manifest(plan, [ir, bytecode, runtime])
let policy = build_network_policy()
say plan["package"]
say plan["target"]
say plan["seed_version"]
say len(plan["artifacts"])
say plan["lock_schema"]
say ir["kind"]
say bytecode["digest"]
say runtime["package"]
say len(result["artifacts"])
say policy["allow_network"]
say policy["require_lock"]
say policy["require_seed"]
say build_is_reproducible(result, result)
EOF
cat > "$expected" <<'EOF'
demo
linux-x86_64
0.0.1
3
1
typed_ir
bc-1
demo
3
false
true
true
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B3 build-plan gate passed: 13 deterministic build ownership cases\n'
