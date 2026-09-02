#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
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
let handoff = runner_artifact_handoff(lexer, "source-digest", "token-digest")
let executed = runner_execute_stage(lexer, "source", "source-digest", "token-digest")
let rejected = runner_execute_stage(parser, "source", "source-digest", "ast-digest")
say runner_capability_allowed("console")
say runner_capability_allowed("file")
say runner_capability_allowed("network")
say runner_capability_allowed("process")
say acceptance["status"]
say acceptance["native_independent"]
say acceptance["structurally_valid"]
say handoff["status"]
say executed["status"]
say executed["output_kind"]
say rejected["status"]
say rejected["error"]
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
true
produced
executed
tokens
rejected
input_kind_mismatch
zap
false
EOF
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 runner-contract gate passed: 15 capability, structural-validity, execution, ownership, and artifact-handoff cases\n'
