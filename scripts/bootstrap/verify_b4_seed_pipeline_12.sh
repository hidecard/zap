#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-seed-pipeline.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/seed_pipeline.zp"
let plan = seed_pipeline_plan("platform-seed-0")
let manifest = seed_pipeline_manifest("platform-seed-0", "source-digest", ["tokens-digest", "ast-digest", "typed-ast-digest", "ir-digest", "bytecode-digest", "execution-digest"])
say len(plan["stages"])
say plan["stages"][0]["name"]
say plan["stages"][5]["output_kind"]
say manifest["execution"]["status"]
say manifest["execution"]["final_kind"]
say manifest["execution"]["final_digest"]
say len(manifest["execution"]["executions"])
say len(manifest["artifacts"])
say manifest["artifacts"][4]["artifact_kind"]
say manifest["native_independent"]
say manifest["status"]
say seed_pipeline_execute("platform-seed-0", "source-digest", ["tokens-digest"])["error"]
EOF
cat > "$expected" <<'EOF'
6
lexer
execution
executed
execution
execution-digest
6
6
bytecode
false
contract_only
digest_count_mismatch
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 seed-pipeline gate passed: 12 source-to-VM stage-chain and blocker cases\n'
