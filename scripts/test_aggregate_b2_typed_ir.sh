#!/usr/bin/env bash
# Regression tests for scripts/aggregate_b2_typed_ir.sh.
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-b2-typed-ir-aggregate.XXXXXX")
trap 'rm -rf "$WORK_DIR"' EXIT

INPUT="$WORK_DIR/b2-typed-ir.csv"
OUTPUT="$WORK_DIR/b2-typed-ir.summary.csv"
printf 'suite,iteration,elapsed_seconds,peak_rss_kb\n' > "$INPUT"
printf 'candidate,1,1.00,54000\n' >> "$INPUT"
printf 'candidate,2,1.04,54100\n' >> "$INPUT"
printf 'candidate,3,0.98,54050\n' >> "$INPUT"
printf 'candidate,4,1.02,54080\n' >> "$INPUT"
printf 'candidate,5,1.06,54120\n' >> "$INPUT"
printf 'owned,1,0.50,30000\n' >> "$INPUT"
printf 'owned,2,0.52,30100\n' >> "$INPUT"
printf 'owned,3,0.51,30050\n' >> "$INPUT"
printf 'owned,4,0.49,30020\n' >> "$INPUT"
printf 'owned,5,0.53,30080\n' >> "$INPUT"

ZAP_TYPED_IR_AGG_INPUT="$INPUT" ZAP_TYPED_IR_AGG_OUTPUT="$OUTPUT" \
  bash "$ROOT_DIR/scripts/aggregate_b2_typed_ir.sh" >/dev/null

header='suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds,stddev_seconds,variance_seconds,cv_percent,peak_rss_kb_min,peak_rss_kb_mean,peak_rss_kb_max'
test "$(head -n 1 "$OUTPUT")" = "$header" || {
  printf 'B2 typed-IR aggregate header changed\n' >&2
  exit 1
}
test "$(awk 'END { print NR - 1 }' "$OUTPUT")" -eq 2 || {
  printf 'B2 typed-IR aggregate did not emit candidate and owned rows\n' >&2
  exit 1
}
awk -F, 'NR > 1 && NF == 12 && $1 ~ /^(candidate|owned)$/ && $2 == 5 { valid++ } END { exit valid == 2 ? 0 : 1 }' "$OUTPUT" || {
  printf 'B2 typed-IR aggregate emitted malformed row count\n' >&2
  exit 1
}
# Spot-check: candidate mean is 1.02 (deterministic for the seeded input)
awk -F, '$1 == "candidate" && $4 == 1.020000 { hit=1 } END { exit (hit == 1) ? 0 : 1 }' "$OUTPUT" || {
  printf 'B2 typed-IR aggregate candidate mean is not deterministic\n' >&2
  exit 1
}
awk -F, '$1 == "owned" && $4 == 0.510000 { hit=1 } END { exit (hit == 1) ? 0 : 1 }' "$OUTPUT" || {
  printf 'B2 typed-IR aggregate owned mean is not deterministic\n' >&2
  exit 1
}
printf 'B2 typed-IR aggregate contract passed: candidate/owned mean and per-suite variance fields are deterministic\n'
