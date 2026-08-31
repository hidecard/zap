#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-b2-typed-ir-benchmark.XXXXXX")
trap 'rm -rf "$WORK_DIR"' EXIT

ZAP_TYPED_IR_BENCH_REPEATS=1 \
ZAP_TYPED_IR_BENCH_WARMUPS=0 \
ZAP_TYPED_IR_BENCH_OUTPUT="$WORK_DIR/b2-typed-ir.csv" \
scripts/benchmark_b2_typed_ir.sh >/dev/null

header='suite,iteration,elapsed_seconds,peak_rss_kb'
test "$(head -n 1 "$WORK_DIR/b2-typed-ir.csv")" = "$header" || {
  printf 'B2 typed-IR benchmark header changed\n' >&2
  exit 1
}
test "$(awk 'END { print NR - 1 }' "$WORK_DIR/b2-typed-ir.csv")" -eq 2 || {
  printf 'B2 typed-IR benchmark did not emit candidate and owned rows\n' >&2
  exit 1
}
awk -F, 'NR > 1 && NF == 4 && $1 ~ /^(candidate|owned)$/ && $2 == 1 && $3 ~ /^[0-9]+([.][0-9]+)?$/ && $4 ~ /^[0-9]+$/ { valid++ } END { exit valid == 2 ? 0 : 1 }' "$WORK_DIR/b2-typed-ir.csv" || {
  printf 'B2 typed-IR benchmark emitted malformed timing or RSS metrics\n' >&2
  exit 1
}
printf 'B2 typed-IR benchmark contract passed: candidate/owned timing and peak RSS rows are valid\n'
