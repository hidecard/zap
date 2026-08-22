#!/usr/bin/env bash
set -euo pipefail

BASELINE=${1:-}
CURRENT=${2:-}
MAX_REGRESSION_PERCENT=${3:-${ZAP_BENCH_MAX_REGRESSION_PERCENT:-200}}

if [[ -z "$BASELINE" || -z "$CURRENT" ]]; then
  printf 'usage: check_benchmark_regression.sh BASELINE.csv CURRENT.csv [max-regression-percent]\n' >&2
  exit 2
fi
[[ -f "$BASELINE" ]] || { printf 'benchmark baseline does not exist: %s\n' "$BASELINE" >&2; exit 1; }
[[ -f "$CURRENT" ]] || { printf 'current benchmark summary does not exist: %s\n' "$CURRENT" >&2; exit 1; }
[[ "$MAX_REGRESSION_PERCENT" =~ ^[0-9]+$ ]] || {
  printf 'max regression percent must be a non-negative integer: %s\n' "$MAX_REGRESSION_PERCENT" >&2
  exit 2
}

awk -F, -v max_percent="$MAX_REGRESSION_PERCENT" '
  function fail(message) {
    print "benchmark regression: " message > "/dev/stderr"
    failed=1
  }
  function valid_number(value) {
    return value ~ /^[0-9]+([.][0-9]+)?$/
  }
  function load(path, is_current,    header,line,line_no,suite,iterations,min,mean,p95,max,row,field_count) {
    line_no=0
    while ((getline line < path) > 0) {
      if (line_no++ == 0) {
        header=line
        if (header != "suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds,stddev_seconds,variance_seconds,cv_percent") {
          fail(path ": invalid header")
        }
        continue
      }
      if (line == "") continue
      field_count=split(line, row, ",")
      if (field_count != 9) {
        fail(path ": invalid row: " line)
        continue
      }
      suite=row[1]; iterations=row[2]; min=row[3]; mean=row[4]; p95=row[5]; max=row[6]; stddev=row[7]; variance=row[8]; cv=row[9]
      if (suite !~ /^[A-Za-z0-9_-]+$/ || iterations !~ /^[1-9][0-9]*$/ || !valid_number(min) || !valid_number(mean) || !valid_number(p95) || !valid_number(max) || !valid_number(stddev) || !valid_number(variance) || !valid_number(cv)) {
        fail(path ": invalid values for suite " suite)
        continue
      }
      if (is_current) {
        if (suite in current_seen) fail(path ": duplicate suite " suite)
        current_seen[suite]=1
        current_count++
        current_iterations[suite]=iterations
        current_mean[suite]=mean
        current_p95[suite]=p95
      } else {
        if (baseline_seen[suite]) fail(path ": duplicate suite " suite)
        baseline_seen[suite]=1
        baseline_count++
        baseline_iterations[suite]=iterations
        baseline_mean[suite]=mean
        baseline_p95[suite]=p95
      }
    }
    close(path)
  }
  BEGIN {
    load(ARGV[1], 0)
    load(ARGV[2], 1)
    if (baseline_count == 0) fail(ARGV[1] ": no benchmark suites")
    if (current_count == 0) fail(ARGV[2] ": no benchmark suites")
    for (suite in baseline_seen) {
      if (!(suite in current_seen)) {
        fail("missing current suite " suite)
        continue
      }
      if (current_iterations[suite] < 1) fail("invalid current iteration count for " suite)
      mean_limit=baseline_mean[suite] * (1 + max_percent / 100)
      p95_limit=baseline_p95[suite] * (1 + max_percent / 100)
      if (current_mean[suite] > mean_limit) {
        printf "benchmark regression: %s mean %.6f exceeds %.6f (baseline %.6f, limit %d%%)\n", suite,current_mean[suite],mean_limit,baseline_mean[suite],max_percent > "/dev/stderr"
        failed=1
      }
      if (current_p95[suite] > p95_limit) {
        printf "benchmark regression: %s p95 %.6f exceeds %.6f (baseline %.6f, limit %d%%)\n", suite,current_p95[suite],p95_limit,baseline_p95[suite],max_percent > "/dev/stderr"
        failed=1
      }
      printf "benchmark check: %s mean=%.6f p95=%.6f baseline_mean=%.6f baseline_p95=%.6f limit=%d%% PASS\n", suite,current_mean[suite],current_p95[suite],baseline_mean[suite],baseline_p95[suite],max_percent
    }
    for (suite in current_seen) if (!(suite in baseline_seen)) fail("unexpected current suite " suite)
    if (failed) exit 1
    print "benchmark regression check passed"
  }
' "$BASELINE" "$CURRENT"
