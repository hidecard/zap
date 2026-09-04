#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
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
runner=$(mktemp "$ROOT_DIR/.zap-b4-provenance.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/rebuild.zp"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let typed = rebuild_stage("typed_ir", "ast", "typed_ir", "zap")
let plan = rebuild_plan("platform-seed-0", [lexer, parser, typed])
let first = rebuild_provenance_record(plan, lexer, "source-digest", "tokens-digest")
let second = rebuild_provenance_record(plan, parser, "tokens-digest", "ast-digest")
let third = rebuild_provenance_record(plan, typed, "ast-digest", "typed-ir-digest")
let records = [first, second, third]
let bad_link = [first, rebuild_provenance_record(plan, parser, "wrong-digest", "ast-digest"), third]
let bad_stage = [first, rebuild_provenance_record(plan, typed, "tokens-digest", "ast-digest"), third]
let bad_empty = [first, rebuild_provenance_record(plan, parser, "tokens-digest", ""), third]
let acceptance = rebuild_provenance_acceptance(plan, records)
say rebuild_provenance_chain_valid(plan, records)
say acceptance["status"]
say acceptance["native_independent"]
say acceptance["records"]
say first["input_kind"]
say first["output_kind"]
say first["input_digest"]
say rebuild_provenance_chain_valid(plan, bad_link)
say rebuild_provenance_chain_valid(plan, bad_stage)
say rebuild_provenance_chain_valid(plan, bad_empty)
say json(records) == json([rebuild_provenance_record(plan, lexer, "source-digest", "tokens-digest"), rebuild_provenance_record(plan, parser, "tokens-digest", "ast-digest"), rebuild_provenance_record(plan, typed, "ast-digest", "typed-ir-digest")])
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true contract_only false 3 source tokens source-digest false false false true" ]]; then
  echo "unexpected provenance output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 provenance-chain gate passed: ordered stage metadata, digest linkage, replay determinism, and negative rejection\n'
