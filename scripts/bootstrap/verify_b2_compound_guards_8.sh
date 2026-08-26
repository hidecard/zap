#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-guards.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let base = [{"name": "a", "type": "option<number>"}, {"name": "b", "type": "result<text>"}]
let a = {"kind": "guard", "name": "a", "guard": "is_some"}
let b = {"kind": "guard", "name": "b", "guard": "is_ok"}
let both = {"kind": "logical", "operator": "and", "left": a, "right": b}
let either = {"kind": "logical", "operator": "or", "left": a, "right": b}
let both_paths = condition_expression_paths(base, both)
let either_paths = condition_expression_paths(base, either)
say ast_lookup_type(both_paths["then"], "a")
say ast_lookup_type(both_paths["then"], "b")
say ast_lookup_type(both_paths["else"], "a")
say ast_lookup_type(both_paths["else"], "b")
say ast_lookup_type(either_paths["then"], "a")
say ast_lookup_type(either_paths["then"], "b")
say ast_lookup_type(either_paths["else"], "a")
say ast_lookup_type(either_paths["else"], "b")
EOF
cat > "$expected" <<'EOF'
number
text
any
any
any
any
none
error
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 compound-guard gate passed: 8 recursive and/or narrowing-path cases\n'
