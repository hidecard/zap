#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-owned-pipeline.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
let source = "let value: number = 7\nsay value"
let result = seed_execute_owned_pipeline(source, "owned-pipeline.zp")
let replay = seed_pipeline_replay(source, "owned-pipeline.zp")
let function_result = seed_execute_owned_pipeline("fn add(a, b):\n    return a + b\nsay add(2, 3)", "function-pipeline.zp")
let invalid = seed_execute_owned_pipeline("return 1", "invalid-pipeline.zp")
let invalid_replay = seed_pipeline_replay("return 1", "invalid-pipeline.zp")
say result["status"]
say result["native_independent"]
say result["stage_chain_valid"]
say len(result["stages"])
say len(result["artifacts"])
say result["artifacts"][0]["kind"]
say result["artifacts"][1]["kind"]
say result["stages"][1]["input_digest"] == result["stages"][0]["output_digest"]
say result["stages"][2]["input_digest"] == result["stages"][1]["output_digest"]
say result["execution"]["error"]
say result["execution"]["output"][0]
say replay["status"]
say replay["byte_equal"]
say function_result["status"]
say function_result["execution"]["error"]
say function_result["execution"]["output"][0]
say invalid["status"]
say invalid["stage_chain_valid"]
say invalid["native_independent"]
say invalid_replay["byte_equal"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_pipeline_executed false true 3 2 typed_ir bytecode true true none 7 candidate_pipeline_replay true candidate_pipeline_executed none 5 candidate_pipeline_error true false true" ]]; then
  echo "unexpected owned pipeline output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 owned-pipeline gate passed: Zap source to inferred typed-IR to bytecode to VM, digest linkage, deterministic replay, and failure boundary\n'
