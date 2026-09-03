#!/usr/bin/env bash
# Aggregator for the B2 typed-IR benchmark raw CSV.
#
# Emits a deterministic per-suite summary with the same fields as
# scripts/aggregate_benchmark.sh (min/mean/p95/max seconds, population
# standard deviation, population variance, and coefficient of variation
# in percent) plus a peak RSS min/mean/max row. The raw observations
# remain unchanged; this only summarizes them for cross-platform
# comparison tooling.
#
# The summary is machine-dependent. It is intended for repeated
# measurements on the same runner and for per-target comparison; it is
# NOT a portability claim.
#
# Environment variables (all optional):
#   ZAP_TYPED_IR_AGG_INPUT   raw CSV (default: benchmark-results/b2-typed-ir.csv)
#   ZAP_TYPED_IR_AGG_OUTPUT  summary CSV (default: <input>.summary.csv)

set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

INPUT=${ZAP_TYPED_IR_AGG_INPUT:-$ROOT_DIR/benchmark-results/b2-typed-ir.csv}
OUTPUT=${ZAP_TYPED_IR_AGG_OUTPUT:-"$INPUT.summary.csv"}

if [[ ! -s "$INPUT" ]]; then
  printf 'B2 typed-IR aggregate: missing raw CSV %s\n' "$INPUT" >&2
  exit 1
fi

mkdir -p "$(dirname "$OUTPUT")"
: > "$OUTPUT"
printf 'suite,iterations,min_seconds,mean_seconds,p95_seconds,max_seconds,stddev_seconds,variance_seconds,cv_percent,peak_rss_kb_min,peak_rss_kb_mean,peak_rss_kb_max\n' >> "$OUTPUT"

# Population statistics over a fixed window of observations. p95 is the
# floor of the 0.95 quantile index (round-up to 1) of the sorted elapsed
# list, which is deterministic for the same input.
awk -F, '
  NR == 1 { next }
  {
    suite = $1
    elapsed = $3 + 0
    rss = $4 + 0
    n[suite]++
    elapsed_list[suite] = elapsed_list[suite] " " elapsed
    rss_list[suite] = rss_list[suite] " " rss
    sum_e[suite] += elapsed
    sum_rss[suite] += rss
    if (!(suite in min_e) || elapsed < min_e[suite]) min_e[suite] = elapsed
    if (!(suite in max_e) || elapsed > max_e[suite]) max_e[suite] = elapsed
    if (!(suite in min_r) || rss < min_r[suite]) min_r[suite] = rss
    if (!(suite in max_r) || rss > max_r[suite]) max_r[suite] = rss
  }
  END {
    for (suite in n) {
      count = n[suite]
      mean = sum_e[suite] / count
      mean_rss = sum_rss[suite] / count
      # Build sorted elapsed list for stddev and p95
      split(elapsed_list[suite], arr, " ")
      # Simple insertion sort
      for (i = 2; i <= count; i++) {
        key = arr[i]
        j = i - 1
        while (j >= 1 && arr[j] + 0 > key + 0) {
          arr[j+1] = arr[j]
          j--
        }
        arr[j+1] = key
      }
      var = 0
      for (i = 1; i <= count; i++) {
        diff = arr[i] + 0 - mean
        var += diff * diff
      }
      var = var / count
      stddev = (var > 0) ? pop_sqrt(var) : 0
      p95_idx = int((count * 95 + 99) / 100)
      if (p95_idx < 1) p95_idx = 1
      if (p95_idx > count) p95_idx = count
      p95 = arr[p95_idx] + 0
      cv = (mean > 0) ? (stddev / mean) * 100 : 0
      printf "%s,%d,%.6f,%.6f,%.6f,%.6f,%.6f,%.6f,%.3f,%d,%d,%d\n", \
        suite, count, min_e[suite], mean, p95, max_e[suite], \
        stddev, var, cv, min_r[suite], mean_rss, max_r[suite]
    }
  }
  function pop_sqrt(x) { return x > 0 ? _pop_sqrt(x, x) : 0 }
  function _pop_sqrt(x, g) { d = (g - x/g) / 2; if (d < 0) d = -d; return (d < 0.0000001) ? g : _pop_sqrt(x, (g + x/g) / 2) }
' "$INPUT" | sort -t, -k1,1 >> "$OUTPUT"

printf 'B2 typed-IR aggregate written to %s\n' "$OUTPUT" >&2
