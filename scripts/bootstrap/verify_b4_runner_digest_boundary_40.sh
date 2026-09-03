#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-runner-digest.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/rebuild.zp"
import "bootstrap/b4/runner.zp"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let plan = rebuild_plan("platform-seed-0", [lexer, parser])
let good_handoff = runner_artifact_handoff(lexer, "source-digest", "token-digest")
let bad_handoff = runner_artifact_handoff(lexer, "", "token-digest")
let good_stage = runner_execute_stage(lexer, "source", "source-digest", "token-digest")
let bad_input = runner_execute_stage(lexer, "source", "", "token-digest")
let bad_output = runner_execute_stage(lexer, "source", "source-digest", "")
let bad_plan = runner_execute_plan(plan, "source", "", ["token-digest", "ast-digest"])
let good_plan = runner_execute_plan(plan, "source", "source-digest", ["token-digest", "ast-digest"])
say runner_digest_valid("source-digest")
say runner_digest_valid(" ")
say good_handoff["status"]
say bad_handoff["status"]
say bad_handoff["error"]
say good_stage["status"]
say bad_input["status"]
say bad_input["error"]
say bad_output["status"]
say bad_output["error"]
say bad_plan["status"]
say bad_plan["error"]
say good_plan["status"]
say good_plan["final_kind"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true false produced rejected digest_missing executed rejected digest_missing rejected digest_missing rejected digest_missing executed ast" ]]; then
  echo "unexpected runner digest output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 runner digest-boundary gate passed: valid handoff and deterministic missing-digest rejection\n'
