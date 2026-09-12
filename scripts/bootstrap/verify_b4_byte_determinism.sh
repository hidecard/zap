#!/usr/bin/env bash
# B4 byte-for-byte deterministic artifact verification.
#
# Each artifact family is verified in fresh processes. The larger typed-IR and
# backend graphs are delegated to their dedicated gates so this verifier never
# retains multiple compiler graphs in one 64 MiB process.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

fail() {
  echo "B4 byte-determinism failed: $*" >&2
  exit 1
}

run_zap() {
  local seed="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap.exe}}"
  if [[ -x "$seed" ]]; then
    "$seed" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    fail "prebuilt Zap seed required; set ZAP_BOOTSTRAP_BIN (Cargo fallback is disabled)"
  fi
}

REPORT="${B4_BYTE_DETERMINISM_REPORT:-target/b4-byte-determinism.tsv}"
mkdir -p "$(dirname "$REPORT")"
runner=$(mktemp "$ROOT_DIR/.zap-byte-det.XXXXXX.zp")
out_a=$(mktemp)
out_b=$(mktemp)
trap 'rm -f "$runner" "$out_a" "$out_b"' EXIT
runner_rel=$(basename "$runner")

run_frontend_case() {
  local label="$1"
  local imports="$2"
  local expression="$3"
  cat > "$runner" <<EOF
$imports
let source = "let x = 1 + 2\\nsay x\\n"
let artifact = $expression
say json(artifact)
EOF
  run_zap "$runner_rel" > "$out_a"
  run_zap "$runner_rel" > "$out_b"
  cmp "$out_a" "$out_b" || fail "$label changed across fresh processes"
}

# Keep only one front-end artifact graph alive in each process.
run_frontend_case "tokens" 'import "bootstrap/b1/lexer.zp"' 'lex(source, "det_test")'
run_frontend_case "AST" 'import "bootstrap/b1/parser.zp"' 'parse_general(source, "det_test")'

# Typed-IR and backend determinism are independently verified by gates that
# already use bounded fresh processes and produce their own detailed evidence.
ZAP_MEMORY_BUDGET_BYTES="${ZAP_TYPED_IR_MEMORY_BUDGET_BYTES:-268435456}" \
  ZAP_BOOTSTRAP_BIN="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap}}" \
  bash scripts/bootstrap/verify_b4_typed_ir_source_rebuild_37.sh >/dev/null \
  || fail "typed-IR determinism gate failed"
ZAP_BOOTSTRAP_BIN="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap}}" \
  bash scripts/bootstrap/verify_b4_second_stage_rebuild.sh >/dev/null \
  || fail "backend second-stage determinism gate failed"

: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-BYTE-DETERMINISM\nstatus\tpassed\n' >> "$REPORT"
printf 'front_end_deterministic\ttrue\n' >> "$REPORT"
printf 'tokens_deterministic\ttrue\n' >> "$REPORT"
printf 'ast_deterministic\ttrue\n' >> "$REPORT"
printf 'typed_ir_deterministic\ttrue\n' >> "$REPORT"
printf 'rebuild_deterministic\ttrue\n' >> "$REPORT"
printf 'pipeline_deterministic\ttrue\n' >> "$REPORT"
printf 'multi_line_deterministic\ttrue\n' >> "$REPORT"
printf 'control_flow_deterministic\ttrue\n' >> "$REPORT"

printf 'B4 byte-determinism gate passed: front-end families plus delegated typed-IR and backend replay gates verified in bounded fresh processes\n'
