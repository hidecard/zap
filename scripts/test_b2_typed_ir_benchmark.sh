#!/usr/bin/env bash
# Regression tests for the P1-09 cross-platform B2 typed-IR benchmark
# baseline. Verifies the raw CSV contract (existing), the new provenance
# sidecar schema, and the cross-platform baseline table.
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-b2-typed-ir-benchmark.XXXXXX")
trap 'rm -rf "$WORK_DIR"' EXIT

# Allow the validator to run on hosts without cargo by passing a fake
# binary; the harness records the binary SHA-256 of the present binary.
ZAP_TYPED_IR_BENCH_REPEATS=1 \
ZAP_TYPED_IR_BENCH_WARMUPS=0 \
ZAP_TYPED_IR_BENCH_OUTPUT="$WORK_DIR/b2-typed-ir.csv" \
ZAP_TYPED_IR_BENCH_PROVENANCE="$WORK_DIR/b2-typed-ir.provenance.tsv" \
ZAP_TYPED_IR_BENCH_BASELINE="$WORK_DIR/b2-typed-ir.baseline.tsv" \
ZAP_TYPED_IR_BENCH_TIME_CMD="echo 1.00 1" \
scripts/benchmark_b2_typed_ir.sh >/dev/null

# Raw CSV contract: header + 1 candidate row + 1 owned row.
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

# Provenance sidecar: required fields must all be present and the
# time_backend row must record which timing path was used.
test -s "$WORK_DIR/b2-typed-ir.provenance.tsv" || {
  printf 'B2 typed-IR benchmark did not emit a provenance sidecar\n' >&2
  exit 1
}
for field in schema_version status git_commit target_triple os arch rust_version cargo_version binary_sha256 script_sha256 repeats warmups suites time_backend raw_csv; do
  grep -q "^${field}\b" "$WORK_DIR/b2-typed-ir.provenance.tsv" || {
    printf 'B2 typed-IR provenance sidecar is missing field: %s\n' "$field" >&2
    exit 1
  }
done
awk -F'\t' '$1 == "time_backend" { print $2 }' "$WORK_DIR/b2-typed-ir.provenance.tsv" | grep -qE '^(override|gnu-time|gtime|fallback)$' || {
  printf 'B2 typed-IR provenance time_backend field is not a recognized backend\n' >&2
  exit 1
}

# Cross-platform baseline table: header + 1 candidate row + 1 owned row,
# all sorted by target_triple then suite. Values are machine-dependent;
# only the schema and row count are checked here.
baseline_header='target_triple	suite	min_seconds	mean_seconds	max_seconds	peak_rss_kb_min	peak_rss_kb_max	git_commit	binary_sha256	timestamp_utc'
test "$(head -n 1 "$WORK_DIR/b2-typed-ir.baseline.tsv")" = "$baseline_header" || {
  printf 'B2 typed-IR baseline table header changed\n' >&2
  exit 1
}
test "$(awk 'END { print NR - 1 }' "$WORK_DIR/b2-typed-ir.baseline.tsv")" -eq 2 || {
  printf 'B2 typed-IR baseline table did not emit candidate and owned rows\n' >&2
  exit 1
}
awk -F'\t' 'NR > 1 && NF == 10 && $2 ~ /^(candidate|owned)$/ && $3 ~ /^[0-9]+([.][0-9]+)?$/ && $4 ~ /^[0-9]+([.][0-9]+)?$/ && $5 ~ /^[0-9]+([.][0-9]+)?$/ && $6 ~ /^[0-9]+$/ && $7 ~ /^[0-9]+$/ { valid++ } END { exit valid == 2 ? 0 : 1 }' "$WORK_DIR/b2-typed-ir.baseline.tsv" || {
  printf 'B2 typed-IR baseline table emitted malformed numeric fields\n' >&2
  exit 1
}

printf 'B2 typed-IR benchmark contract passed: candidate/owned timing, peak RSS, provenance, and cross-platform baseline rows are valid\n'
