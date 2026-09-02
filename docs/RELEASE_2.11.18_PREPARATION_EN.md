# Zap v2.11.18 Release Preparation

This document tracks the current preparation work for the v2.11.18 release
line. It complements the canonical release-version policy in
[`RELEASE_VERSION_POLICY_EN.md`](RELEASE_VERSION_POLICY_EN.md) and the
[English current-status index](CURRENT_STATUS_EN.md).

## Status

The v2.11.17 line is the latest published release. The v2.11.18
preparation is in progress; no v2.11.18 tag exists yet and no public
GitHub Release has been published.

## Required release surfaces (v2.11.18)

| Surface | Required contract |
|---|---|
| `native/Cargo.toml` | Bump `version` from `2.11.17` to `2.11.18` |
| `native/Cargo.lock` | Update `zap-native` package version to `2.11.18` |
| `CHANGELOG.md`, `CHANGELOG_EN.md`, `CHANGELOG_MM.md` | Mention `2.11.18` |
| `README.md`, `README_MM.md` | Update current release line and archive filenames |
| `SECURITY.md` | Update `Latest v2.11.x` and integrity URL |
| `docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md`, `docs/TYPECHECK_CONFORMANCE_MATRIX_MM.md` | Mention `2.11.18` |
| `vscode-extension/package.json` | Bump `version` to `2.11.18` |
| `docs/RELEASE_2.11.18_EN.md`, `docs/RELEASE_2.11.18_MM.md` | Bilingual release notes for v2.11.18 |
| `docs/CURRENT_STATUS_EN.md`, `docs/CURRENT_STATUS_MM.md` | Record the v2.11.18 prep step |

`scripts/validate_release_version.sh` enforces every surface above in a
single TSV report. `scripts/release_preflight.sh` adds a witness check
for the B4 rust-free full-language contract (status label and acceptance
row counts) so the contract is recorded in the preflight log even when
its acceptance rows are still `provisional` or `not-certified`.

## Preflight workflow

`.github/workflows/prepare-v2.11.18.yml` runs a CI-side v2.11.18
preflight on every `master` push that touches the preflight scripts or
the B4 contract, and is also triggerable via `workflow_dispatch`. It
sets `EXPECTED_VERSION=2.11.18`, `RELEASE_TAG=v2.11.18`, and
`ZAP_SKIP_RELEASE_NOTES=1` so the preflight exercises every other
required surface even before the bilingual release notes have been
authored. The `release.yml` workflow re-runs the same preflight on the
real `v2.11.18` tag with `RUN_CARGO_AUDIT=1`,
`SKIP_DEPLOYMENT_VALIDATION=0`, and without the `ZAP_SKIP_RELEASE_NOTES`
override.

## Cross-platform baseline

`scripts/benchmark_b2_typed_ir.sh` is wired into the v2.11.18
preflight (P1-09). It writes a per-`(target_triple, suite)` baseline
row to `benchmark-results/b2-typed-ir.baseline.tsv` and a M2-BENCH-01
compatible provenance sidecar. The release preflight now requires the
aggregator (`scripts/aggregate_b2_typed_ir.sh`) to produce a
deterministic per-suite summary CSV, and verifies the baseline table
has at least one row per suite. The cross-platform build matrix in
`ci.yml` also uploads `zap-b2-typed-ir-baseline-<sha>` so per-target
execution evidence accumulates per release.

## B4 contract witness

The B4 rust-free full-language contract remains `not-certified` until
every acceptance row passes on every supported target. The release
preflight records the contract status label (`not-certified` /
`provisional` / `certified`) and counts of `provisional` and
`not-certified` rows in the B4 acceptance TSV. While the contract is
not certified, the preflight emits a `WARN` and continues; the
release is published regardless. The contract is only moved to
`certified` after every acceptance row passes on every supported
target, with a follow-up doc/PR that records the change.

## Acceptance gates for v2.11.18

| Gate | Status |
|---|---|
| Version consistency across Cargo, lockfile, CLI, changelogs, READMEs, security, conformance matrix, vscode-extension, and bilingual release notes | `scripts/validate_release_version.sh` (CI) |
| B2 typed-IR cross-platform baseline | `scripts/test_b2_typed_ir_benchmark.sh` + `scripts/test_aggregate_b2_typed_ir.sh` (CI) |
| Documentation consistency, ownership, parity, fixed-seed replay, bounded replay, async boundary, platform archive, registry corpus, benchmark regression/provenance, stdlib policy, B0/B1/B2/B3 contracts, LSP/VS Code parity | `scripts/release_preflight.sh` (CI dry-run + tag run) |
| B4 contract witness | `scripts/release_preflight.sh` `check_b4_contract_witness` (records status, requires no certification) |

The v2.11.18 tag will not be pushed until `release.yml` passes
end-to-end on the tag commit, including the preflight, the
cross-platform native build, the immutable-tag publish job, and the
post-publish verification of the downloaded assets.
