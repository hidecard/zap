#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a9-root-contract.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typed_ir.zp"
let source = "let value: number = 7\nsay value"
let first = from_json(emit_inferred_program_typed_ir(source, "root.zp"))
let second = from_json(emit_inferred_program_typed_ir(source, "root.zp"))
let valid = typed_ir_root_contract(first, second)
let bad = typed_ir_root_contract({"candidate_only": false, "ir": {"nodes": []}, "kind": "zap.typed_ir", "schema_version": 3, "source_name": "bad.zp"}, {"candidate_only": false, "ir": {"nodes": []}, "kind": "zap.typed_ir", "schema_version": 3, "source_name": "bad.zp"})
say valid["status"]
say valid["valid"]
say valid["schema_version"]
say valid["node_count"]
say bad["status"]
say bad["valid"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_typed_ir_root true 4 2 candidate_typed_ir_root_error false" ]]; then
  echo "unexpected A9 root contract output: ${lines[*]}" >&2
  exit 1
fi
printf 'A9 typed-IR root contract gate passed: schema-4 root shape, candidate boundary, source/node envelope, determinism, and invalid-root rejection\n'
