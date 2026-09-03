#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b3-owner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b3/package.zp"
import "bootstrap/b3/vm.zp"
let dep = dependency("std", "2.11.16", "abc123")
let manifest_value = manifest("demo", "0.1.0", "main.zp", [dep])
let entry = lock_entry("std", "2.11.16", "abc123", "offline")
let lock = lockfile(manifest_value, [entry])
let policy = offline_policy(manifest_value)
let program = [{"op": "const", "value": 1}, {"op": "const", "value": 2}, {"op": "add"}, {"op": "print"}, {"op": "halt"}]
let state = vm_run(program)
say manifest_value["name"]
say manifest_value["version"]
say manifest_value["main"]
say len(manifest_value["dependencies"])
say package_identity(entry)
say lock["schema_version"]
say policy["allow_network"]
say lock_is_reproducible(lock, lock)
say state["halted"]
say state["ip"]
say len(state["output"])
say state["output"][0]
say len(state["stack"])
say vm_state()["ip"]
say vm_state()["halted"]
say dep["checksum"]
say entry["source"]
say policy["package"]
say len(lock["dependencies"])
EOF
cat > "$expected" <<'EOF'
demo
0.1.0
main.zp
1
std@2.11.16:abc123
1
false
true
true
5
1
3
0
0
false
abc123
offline
demo
1
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B3 Zap ownership-transition gate passed: 20 package and VM foundation cases\n'
