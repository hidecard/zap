#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-generic-bounds-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let signature = function_signature_info("fn pair<T,U>(left: T, right: U) -> map<T,U> where T: number and U: text:")
say signature["name"]
say len(signature["generic_parameters"])
say len(signature["constraints"])
say signature["constraints"][0]["parameter"]
say signature["constraints"][0]["bound"]
say signature["constraints"][1]["parameter"]
say signature["constraints"][1]["bound"]
say infer_function_call([signature], "pair", ["number", "text"])
say infer_function_call([signature], "pair", ["text", "number"])
say generic_container_valid("map<number,text>")
EOF
cat > "$expected" <<'EOF'
pair
2
2
T
number
U
text
map<number,text>
constraint_error
true
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B2 generic-bounds gate passed: 10 declaration, bound, and instantiation cases\n'
