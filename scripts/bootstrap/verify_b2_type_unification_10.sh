#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-unify-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
say type_unify("number", "number")
say type_unify("number", "text")
say type_unify("list<number>", "list<number>")
say type_unify("list<number>", "list<text>")
say type_unify("map<text,number>", "map<text,number>")
say type_unify("map<text,number>", "map<number,number>")
say type_unify("option<list<number>>", "option<list<number>>")
say type_unify("result<map<text,number>>", "result<map<text,number>>")
say type_unify("any", "map<text,number>")
say generic_unify("T", "text", ["T"])
EOF
cat > "$expected" <<'EOF'
true
false
true
false
true
false
true
true
true
true
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 type-unification gate passed: 10 recursive wrapper and generic cases\n'
