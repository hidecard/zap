# Zap Native Benchmark Harness

**Status:** Initial repeatable baseline for Zap v2.1.6

The repository now includes `scripts/benchmark_native.sh`, a dependency-free benchmark runner for the native interpreter. It builds `native/target/release/zap` with the locked dependency graph when necessary, creates temporary source fixtures, runs each fixture a configurable number of times, and writes stable CSV columns: `suite`, `iteration`, and `elapsed_seconds`.

## Current benchmark suites

| Suite | Workload | Purpose |
|---|---|---|
| `loops` | A bounded `while` loop with integer accumulation. | Establish loop and arithmetic dispatch baseline. |
| `calls` | Repeated user-defined function calls inside a bounded loop. | Establish call-frame and return-value baseline. |
| `closures` | A nested function mutating captured state across repeated calls. | Establish closure environment and captured-state dispatch baseline. |
| `allocations` | `range(10000)` followed by `enumerate`. | Establish list allocation and collection transformation baseline. |
| `json` | JSON encoding and decoding of a deterministic numeric list. | Establish conversion and nested-value traversal baseline. |
| `async` | Spawn, readiness check, and join of a deterministic async task. | Establish task scheduling and completion baseline. |
| `imports` | Explicit module/import dispatch fixture with a deterministic helper call. | Establish module loading and dispatch coverage without external dependencies. |

The fixtures are generated in a temporary directory and do not modify the repository. The output is written to `benchmark-results/native.csv` by default. Set `ZAP_BENCH_REPEATS` to a positive integer to control repetitions and `ZAP_BENCH_OUTPUT` to choose another CSV path. For example:

```sh
ZAP_BENCH_REPEATS=10 scripts/benchmark_native.sh
```

The harness uses Bash's built-in `time` facility rather than an optional external timing package, so it remains usable in the minimal CI environment. Measurements are wall-clock process times and are intended for regression comparison on the same machine and toolchain, not for cross-machine performance claims.

## Interpretation and limits

A baseline run is useful only when the binary, compiler profile, operating system, CPU conditions, repetition count, and fixture source are recorded together. The CSV intentionally contains raw observations; aggregation and plotting should be performed separately so that the original measurements remain auditable. The current suite includes explicit module/import dispatch without external registry access. `scripts/aggregate_benchmark.sh` consumes the CSV and emits deterministic per-suite min/mean/max summaries while preserving the raw observations. Closure environments, JSON conversion, and deterministic async scheduling are included in the current suite.

This harness does not claim to be a statistically rigorous microbenchmark framework. It does not isolate CPU frequency, pin a process to a core, or measure allocations at the allocator level. Performance claims must therefore report the environment and should compare repeated runs of the same commit. The benchmark is a CI-visible baseline gate rather than a timing-threshold release gate. CI runs a seven-suite smoke, aggregates the CSV, and uploads the raw and summary artifacts; it intentionally does not fail on machine-dependent elapsed-time thresholds.

## Verification

The harness and aggregator are exercised with one repetition per suite. The current seven suites complete successfully, emit seven raw CSV observations, and produce deterministic summary output. Native formatter, full native tests, and `git diff --check` remain required before committing benchmark changes.
