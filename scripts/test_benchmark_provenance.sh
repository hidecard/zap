#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
RUNNER="$ROOT_DIR/scripts/benchmark_native.sh"
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-benchmark-provenance.XXXXXX")
trap 'rm -rf "$WORK_DIR"' EXIT

fail() {
  printf 'benchmark provenance contract: %s\n' "$1" >&2
  exit 1
}

RAW="$WORK_DIR/raw.csv"
PROVENANCE="$WORK_DIR/provenance.tsv"
SUMMARY="$WORK_DIR/summary.csv"
ZAP_BENCH_REPEATS=1 \
  ZAP_BENCH_WARMUPS=0 \
  ZAP_BENCH_OUTPUT="$RAW" \
  ZAP_BENCH_PROVENANCE="$PROVENANCE" \
  "$RUNNER" >/dev/null

test -s "$RAW" || fail 'raw observation file is empty'
test -s "$PROVENANCE" || fail 'provenance sidecar is empty'
test "$(awk 'END { print NR - 1 }' "$RAW")" -eq 7 || fail 'one-repeat run did not emit seven suite rows'
test "$(head -n 1 "$RAW")" = 'suite,iteration,elapsed_seconds' || fail 'raw observation header changed'
for field in schema_version status generated_at_utc git_commit target_triple runner_os os_release kernel architecture cpu_model rustc cargo binary binary_sha256 benchmark_script_sha256 repeats warmups suites raw_observations; do
  awk -F '\t' -v expected="$field" 'NR > 1 && $1 == expected { found=1 } END { exit found ? 0 : 1 }' "$PROVENANCE" || fail "missing provenance field: $field"
done
grep -q $'^status\tpassed$' "$PROVENANCE" || fail 'successful benchmark did not record passed status'
grep -Eq $'^binary_sha256\t[0-9a-f]{64}$' "$PROVENANCE" || fail 'binary SHA-256 digest is not hexadecimal'
grep -Eq $'^benchmark_script_sha256\t[0-9a-f]{64}$' "$PROVENANCE" || fail 'script SHA-256 digest is not hexadecimal'
scripts/aggregate_benchmark.sh "$RAW" "$SUMMARY" >/dev/null
test "$(head -n 1 "$SUMMARY")" = 'suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds,stddev_seconds,variance_seconds,cv_percent' || fail 'summary variance header changed'
test "$(awk 'END { print NR - 1 }' "$SUMMARY")" -eq 7 || fail 'summary did not contain seven suites'
awk -F, 'NR > 1 && (NF != 9 || $7 != "0.000000" || $8 != "0.000000" || $9 != "0.000") { exit 1 }' "$SUMMARY" || fail 'one-repeat summary did not report zero spread'

cat >"$WORK_DIR/math.csv" <<'CSV'
suite,iteration,elapsed_seconds
alpha,1,1
alpha,2,3
CSV
scripts/aggregate_benchmark.sh "$WORK_DIR/math.csv" "$WORK_DIR/math-summary.csv" >/dev/null
test "$(tail -n 1 "$WORK_DIR/math-summary.csv")" = 'alpha,2,1.000000,2.000000,3.000000,3.000000,1.000000,1.000000,50.000' || fail 'variance math summary is incorrect'

for variable in ZAP_BENCH_REPEATS ZAP_BENCH_WARMUPS; do
  value=65
  expected='must not exceed'
  if [[ "$variable" == ZAP_BENCH_WARMUPS ]]; then
    value=17
  fi
  if env "$variable=$value" ZAP_BENCH_OUTPUT="$WORK_DIR/invalid.csv" "$RUNNER" >/dev/null 2>"$WORK_DIR/invalid.err"; then
    fail "$variable=$value was accepted"
  fi
  grep -q "$expected" "$WORK_DIR/invalid.err" || fail "$variable=$value did not fail at its cap"
done

printf 'benchmark provenance contract regression passed\n'
