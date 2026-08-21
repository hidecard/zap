# Zap v2.1.10 Release Notes

**Release date:** 2026-08-21

Zap v2.1.10 is a release-engineering and documentation-quality milestone. It makes the bilingual documentation baseline discoverable and enforceable, and adds repeatable p95 benchmark regression protection without changing the language's deferred trait or broad async scope.

## Highlights

- Added English/Burmese documentation navigation landing pages covering normative specifications, runtime contracts, verification evidence, release policy, and contribution paths.
- Added `scripts/validate_documentation_consistency.sh` with required-file, section-parity, code-fence-parity, stale-version, and README navigation-link checks.
- Added positive and negative documentation-consistency regression coverage through `scripts/test_validate_documentation_consistency.sh`.
- Extended benchmark aggregation with a deterministic `p95_seconds` column and added configurable per-suite warm-up iterations through `ZAP_BENCH_WARMUPS`.
- Added `scripts/check_benchmark_regression.sh` to compare mean and p95 timings against the checked-in `benchmark-results/native-summary.csv` baseline with a configurable threshold.
- Wired documentation and benchmark gates into CI and release preflight, with TSV/log artifact evidence.
- Updated English/Burmese syntax, language-specification, async-boundary, generic-type design, P2 progress, benchmark, README, and changelog documentation to the v2.1.10 release baseline.

## Contract boundaries

This release does not claim a new trait implementation, broad language-level async scheduling syntax, tracing garbage collection, public weak references, or per-run byte accounting. Those remain explicitly deferred roadmap items. The benchmark threshold is a deterministic regression signal, not a promise of identical wall-clock timings across operating systems or hosted runners.

## Verification

The native Rust quality gates passed locally: rustfmt, strict Clippy with `-D warnings`, all-target/all-feature tests with 160 unit tests and 254 core integration tests, and `git diff --check`. GitHub Actions run `32513512535` passed the documentation and benchmark quality job plus Linux x86_64, Windows x86_64, and macOS ARM64 build/test jobs; release workflow run `32513839968` successfully validated, signed, and published the v2.1.10 artifacts.

See the [documentation navigation hub](DOCUMENTATION_NAVIGATION_EN.md), [benchmark harness contract](BENCHMARK_HARNESS_EN.md), [release version policy](RELEASE_VERSION_POLICY_EN.md), and [English language specification](LANGUAGE_SPEC_EN.md) for the maintained contracts.

## References

[1]: DOCUMENTATION_NAVIGATION_EN.md "Zap English documentation navigation"
[2]: BENCHMARK_HARNESS_EN.md "Zap English benchmark harness contract"
[3]: RELEASE_VERSION_POLICY_EN.md "Zap English release version policy"
[4]: LANGUAGE_SPEC_EN.md "Zap English language specification"
