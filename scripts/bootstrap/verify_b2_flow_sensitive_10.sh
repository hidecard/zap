#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-flow-sensitive.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let base = [{"name": "value", "type": "number"}, {"name": "flag", "type": "bool"}]
let same_a = [{"name": "value", "type": "number"}, {"name": "flag", "type": "bool"}]
let same_b = [{"name": "value", "type": "number"}, {"name": "flag", "type": "bool"}]
let diff = [{"name": "value", "type": "text"}, {"name": "flag", "type": "bool"}]
let merged_same = branch_environment_merge_many(base, [same_a, same_b])
let merged_diff = branch_environment_merge_many(base, [same_a, diff])
let declared = branch_path_declaration_merge(base, [[{"name": "local", "type": "number"}], [{"name": "local", "type": "number"}]], ["local"])
let missing = branch_path_declaration_merge(base, [[{"name": "local", "type": "number"}], []], ["local"])
let invalidated = flow_reassignment_invalidate([{"name": "value", "type": "option<number>"}], "value", "text", ["value"])
let stable = loop_fixpoint_many(base, [same_a, same_b], ["value", "flag"], 4)
let divergent = loop_fixpoint_many(base, [same_a, diff], ["value"], 4)
say ast_lookup_type(merged_same, "value")
say ast_lookup_type(merged_diff, "value")
say ast_lookup_type(declared, "local")
say symbol_environment_has(missing, "local")
say ast_lookup_type(invalidated, "value")
say ast_lookup_type(stable, "value")
say ast_lookup_type(stable, "flag")
say ast_lookup_type(divergent, "value")
say branch_environment_merge_paths(base, same_a, same_b, false, false)[0]["type"]
say scope_exit(base, scope_enter(base, [{"name": "inner", "type": "text"}]))[0]["type"]
EOF
cat > "$expected" <<'EOF'
number
any
number
false
text
number
bool
any
number
number
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 flow-sensitive gate passed: 10 multi-path, invalidation, loop, and scope cases\n'
