# Zap Native Benchmark Harness

**Status:** Repeatable benchmark baseline and regression gate for Zap v2.2.7

The repository now includes `scripts/benchmark_native.sh`, a dependency-free benchmark runner for the native interpreter. It builds `native/target/release/zap` with the locked dependency graph when necessary, creates temporary source fixtures, runs each fixture a configurable number of times, and writes stable raw CSV columns: `suite`, `iteration`, and `elapsed_seconds`. M2-BENCH-01 also writes a provenance sidecar, `ZAP_BENCH_PROVENANCE` (defaulting to the raw CSV basename with `.provenance.tsv`), containing the run status, UTC timestamp, commit, target triple, operating-system/kernel/architecture, CPU description when available, Rust/Cargo versions, binary and benchmark-script SHA-256 digests, repeat/warm-up counts, suite list, and raw-observation path.

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

The fixtures are generated in a temporary directory and do not modify the repository. The output is written to `benchmark-results/native.csv` by default. Set `ZAP_BENCH_REPEATS` to a positive integer to control measured repetitions, `ZAP_BENCH_WARMUPS` to a non-negative warm-up count per suite, and `ZAP_BENCH_OUTPUT` to choose another CSV path. For example:

```sh
ZAP_BENCH_REPEATS=10 scripts/benchmark_native.sh
```

The harness uses Bash's built-in `time` facility rather than an optional external timing package, so it remains usable in the minimal CI environment. `ZAP_BENCH_REPEATS` must be between 1 and 64, and `ZAP_BENCH_WARMUPS` must be between 0 and 16; these caps keep CI and release-preflight work bounded. Measurements are wall-clock process times and are intended for regression comparison on the same machine and toolchain, not for cross-machine performance claims.

## Interpretation and limits

A baseline run is useful only when the binary, compiler profile, operating system, CPU conditions, repetition count, and fixture source are recorded together. M2-BENCH-01 records those run conditions in the provenance sidecar and keeps the raw observation CSV separate so the original measurements remain auditable. The current suite includes explicit module/import dispatch without external registry access. `scripts/aggregate_benchmark.sh` consumes the CSV and emits deterministic per-suite min/mean/p95/max summaries plus population standard deviation, population variance, and coefficient of variation (`cv_percent`) while preserving the raw observations. Closure environments, JSON conversion, and deterministic async scheduling are included in the current suite.

This harness does not claim to be a statistically rigorous microbenchmark framework. It does not isolate CPU frequency, pin a process to a core, or measure allocations at the allocator level. Performance claims must therefore report the environment and should compare repeated runs of the same commit. The benchmark is a CI-visible regression gate. CI runs a seven-suite smoke, aggregates the CSV, compares mean and p95 values with `benchmark-results/native-summary.csv`, and uploads the raw CSV, provenance TSV, summary, and comparison artifacts. The default threshold is a 200% increase over the checked-in baseline; a run that exceeds the threshold fails the quality job. Because measurements are machine-dependent, baseline updates require an explicit reviewed change rather than an automatic rewrite.

## Verification

The harness and aggregator are exercised with positive and zero warm-up settings, bounded repeat validation, repeated measurements, p95 and variance aggregation, malformed-input rejection, and expected slow-run failures. `scripts/test_benchmark_regression.sh` validates the expanded summary schema and rejects malformed variance fields. CI and release preflight require provenance fields, compare mean and p95 against the checked-in baseline, and upload the raw CSV, provenance TSV, summary, and comparison log. The current seven suites complete successfully, emit the configured observations, and produce deterministic summary output. Native formatter, full native tests, and `git diff --check` remain required before committing benchmark changes.

## P1-05 deterministic test-layer runner

`scripts/test_p105_layers.sh` is the dependency-free CI-visible runner for the broader conformance and property layer. It executes deterministic parser and lexer corpora, malformed-program and JSON security corpora, malformed-lockfile cases, standard-library security inputs, registry provenance/property mutations, collection/filesystem regressions, and async cancellation/scheduler determinism cases. Each invocation uses a single stable Cargo test filter and fails immediately on a non-zero result.

The quality job runs this Linux corpus gate, while the build matrix independently compiles and tests Linux, Windows, and macOS targets. This separation keeps corpus diagnostics deterministic without weakening cross-platform compilation and test coverage. The remaining P1-05 gaps are dedicated fuzz targets and allocator/heap-level regression counters; the M2-VERIFY-02 platform-specific input slice is implemented in the native build matrix.

For the current validation command:

```sh
scripts/test_p105_layers.sh
```

The runner is a deterministic regression gate, not a timing benchmark and not a substitute for long-running fuzz campaigns.

## References

[1]: ../scripts/test_p105_layers.sh "P1-05 deterministic test-layer runner"
[2]: ../.github/workflows/ci.yml "Zap CI quality and cross-platform build matrix"
[3]: ../scripts/benchmark_native.sh "Native benchmark runner and provenance sidecar"
[4]: ../scripts/aggregate_benchmark.sh "Deterministic benchmark aggregation with variance fields"
[5]: ../scripts/check_benchmark_regression.sh "Mean and p95 benchmark regression comparator"
[6]: ../scripts/test_benchmark_regression.sh "Benchmark schema and regression contract harness"
