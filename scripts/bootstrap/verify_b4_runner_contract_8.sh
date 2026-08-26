#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-runner.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/rebuild.zp"
import "bootstrap/b4/runner.zp"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let plan = rebuild_plan("platform-seed-0", [lexer, parser])
let acceptance = runner_acceptance(plan)
let record = runner_stage_record(lexer, "source-digest", "token-digest")
say runner_capability_allowed("console")
say runner_capability_allowed("file")
say runner_capability_allowed("network")
say runner_capability_allowed("process")
say acceptance["status"]
say acceptance["native_independent"]
say record["owner"]
say record["native_owner"]
EOF
cat > "$expected" <<'EOF'
true
true
false
false
contract_only
false
zap
false
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected"
printf 'B4 runner-contract gate passed: 8 capability, ownership, and explicit contract-only cases\n'
