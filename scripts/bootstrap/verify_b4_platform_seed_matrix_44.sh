#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-platform-seed.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/seed_pipeline.zp"
let targets = ["linux-x86_64", "macos-arm64", "windows-x86_64"]
let valid = [seed_platform_record("linux-x86_64", "bytecode-v1", "digest-v1", "executed"), seed_platform_record("macos-arm64", "bytecode-v1", "digest-v1", "executed"), seed_platform_record("windows-x86_64", "bytecode-v1", "digest-v1", "executed")]
let wrong_bytes = [seed_platform_record("linux-x86_64", "bytecode-v1", "digest-v1", "executed"), seed_platform_record("macos-arm64", "bytecode-v2", "digest-v2", "executed"), seed_platform_record("windows-x86_64", "bytecode-v1", "digest-v1", "executed")]
let wrong_status = [seed_platform_record("linux-x86_64", "bytecode-v1", "digest-v1", "executed"), seed_platform_record("macos-arm64", "bytecode-v1", "digest-v1", "failed"), seed_platform_record("windows-x86_64", "bytecode-v1", "digest-v1", "executed")]
let missing_target = [seed_platform_record("linux-x86_64", "bytecode-v1", "digest-v1", "executed"), seed_platform_record("macos-arm64", "bytecode-v1", "digest-v1", "executed")]
let acceptance = seed_platform_acceptance("platform-seed-0", valid, targets)
say seed_platform_matrix_valid(valid, targets)
say acceptance["artifact_equal"]
say acceptance["native_independent"]
say acceptance["status"]
say acceptance["targets"]
say seed_platform_matrix_valid(wrong_bytes, targets)
say seed_platform_matrix_valid(wrong_status, targets)
say seed_platform_matrix_valid(missing_target, targets)
say seed_platform_matrix_valid(valid, ["linux-x86_64", "macos-arm64", "linux-x86_64"])
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true false contract_only 3 false false false false" ]]; then
  echo "unexpected platform matrix output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 platform-seed matrix gate passed: target coverage, artifact byte/digest equality, status validation, and mismatch rejection\n'
