#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CHECKER="$ROOT_DIR/scripts/check_benchmark_regression.sh"
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-benchmark-regression.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT

cat >"$TMP_DIR/baseline.csv" <<'CSV'
suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds
alpha,5,1.000000,1.100000,1.200000,1.300000
beta,5,2.000000,2.100000,2.200000,2.300000
CSV
cat >"$TMP_DIR/current.csv" <<'CSV'
suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds
alpha,3,0.900000,1.200000,1.300000,1.400000
beta,3,1.900000,2.200000,2.300000,2.400000
CSV

"$CHECKER" "$TMP_DIR/baseline.csv" "$TMP_DIR/current.csv" 20 >/dev/null

cat >"$TMP_DIR/slow.csv" <<'CSV'
suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds
alpha,3,0.900000,1.500000,1.300000,1.600000
beta,3,1.900000,2.200000,2.300000,2.400000
CSV
if "$CHECKER" "$TMP_DIR/baseline.csv" "$TMP_DIR/slow.csv" 20 >/dev/null 2>&1; then
  printf 'expected slow benchmark regression to fail\n' >&2
  exit 1
fi

cat >"$TMP_DIR/missing.csv" <<'CSV'
suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds
alpha,3,0.900000,1.200000,1.300000,1.400000
CSV
if "$CHECKER" "$TMP_DIR/baseline.csv" "$TMP_DIR/missing.csv" 20 >/dev/null 2>&1; then
  printf 'expected missing suite to fail\n' >&2
  exit 1
fi

cat >"$TMP_DIR/malformed.csv" <<'CSV'
suite,iterations,elapsed_seconds
alpha,3,1.200000
CSV
if "$CHECKER" "$TMP_DIR/baseline.csv" "$TMP_DIR/malformed.csv" 20 >/dev/null 2>&1; then
  printf 'expected malformed summary to fail\n' >&2
  exit 1
fi

printf 'benchmark regression harness passed\n'
