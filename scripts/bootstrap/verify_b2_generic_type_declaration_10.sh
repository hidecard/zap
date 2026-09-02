#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-generic-type-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let pair = generic_type_declaration_info("type Pair<T,U> = map<T,list<U>>")
let box = generic_type_declaration_info("type Box<T> = option<T>")
let broken = generic_type_declaration_info("type Broken<T,T> = list<T>")
say pair["name"]
say len(pair["parameters"])
say pair["body"]
say pair["valid"]
say generic_type_declaration_instance("type Pair<T,U> = map<T,list<U>>", ["text", "number"])
say generic_type_declaration_instance("type Box<T> = option<T>", ["bool"])
say generic_type_declaration_instance("type Pair<T,U> = map<T,list<U>>", ["text"])
say broken["valid"]
say generic_container_valid("option<list<number>>")
say generic_container_element("map<text,list<number>>")
EOF
cat > "$expected" <<'EOF'
Pair
2
map<T,list<U>>
true
map<text,list<number>>
option<bool>
generic_arity_error
false
true
list<number>
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B2 generic-type-declaration gate passed: 10 metadata and nested-container cases\n'
