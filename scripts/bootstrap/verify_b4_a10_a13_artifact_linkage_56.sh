#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a10-a13-artifacts.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "let value: number = 7\nsay value"
let pipeline = seed_execute_owned_pipeline(source, "main.zp")
let pipeline_contract = seed_owned_pipeline_contract(pipeline)
let package = seed_build_package_owned("zap_app", "1.0.0", "main.zp", [], [], source, "main.zp")
let package_contract = seed_package_artifact_contract(package["artifact_bytes"], package["artifact_digest"])
let vm_contract = vm_execution_contract([{"op": "const", "value": 7}, {"op": "print"}, {"op": "halt"}])
say pipeline_contract["status"]
say pipeline_contract["valid"]
say pipeline_contract["artifact_count"]
say pipeline_contract["stage_count"]
say package["status"]
say package_contract["status"]
say package_contract["valid"]
say vm_contract["valid"]
say pipeline["artifacts"][0]["kind"]
say pipeline["artifacts"][1]["kind"]
say pipeline["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_owned_pipeline_contract true 2 3 candidate_package_build_executed candidate_package_artifact_contract true true typed_ir bytecode 7" ]]; then
  echo "unexpected A10-A13 artifact linkage output: ${lines[*]}" >&2
  exit 1
fi
printf 'A10-A13 artifact linkage gate passed: typed-IR/bytecode digest records, stage chain, package artifact, and VM contract\n'
