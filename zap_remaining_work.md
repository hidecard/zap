# Zap — Remaining Work and Verification Status

> **Snapshot:** `master` at `5cfeb58` (`ci: make lexer ownership contract CRLF-safe`), checked on 2026-09-08. This document separates **verified current results** from **roadmap claims**. A gate is marked complete only when the current checkout passes the corresponding command.

## Executive summary

Zap’s developer toolchain, native runtime build, release-version validation, B1 lexer ownership contract, B1 parser candidate gate, and B2 typed-IR candidate gate are now passing locally from the current checkout. The earlier token-native indentation crash and the CI release-binary path problem have been addressed. The lexer ownership contract also now handles the repository’s CRLF-formatted `OWNERS.tsv` correctly.

The latest GitHub CI run for `5cfeb58` is still **in progress**, so the repository must not yet be called CI-green. The remaining product work is broader than bounded validation: complete parser and type-checker ownership, complete typed-IR production, Zap-owned package/build/VM execution, and a clean two-stage self-rebuild are still required before `self_hosted = true`.

## Current verified status

### PASS — verified locally from `5cfeb58`

- `make doctor` passes with Rust/Cargo `1.88.0`, `cargo-audit`, the pinned toolchain, and native runtime `zap 2.11.18` available.
- Native tests pass: the latest local run reports **259 passed, 0 failed** for the integration/native test group; the previously recorded native test groups also passed with zero failures.
- `scripts/test_validate_release_version.sh` passes after using a portable built-binary resolver. The regression suite reports **27 checks passed** and observes `zap --version = 2.11.18`.
- `scripts/validate_release_version.sh` passes its current repository consistency checks.
- `scripts/validate_markdown_links.py` passes with **796 links checked**.
- `scripts/bootstrap/verify_b1_lexer.sh` passes the token and diagnostic corpus. Diagnostic fixtures may print a raw `cmp` difference before normalized comparison; the normalized comparison passes and the script exits successfully.
- `scripts/bootstrap/verify_b1_lexer_contract.sh` passes differential corpus, ownership, B1 milestone, and no-Rust-boundary checks.
- `scripts/bootstrap/verify_b1_token_native_indentation.sh` passes; the earlier `ProjectError: index out of range` is no longer reproducible.
- `scripts/bootstrap/verify_b1_boundary_fixtures.sh` passes when the platform binary path is available.
- `scripts/bootstrap/verify_b1_parser_candidate.sh` passes arithmetic AST, compound AST, and delimiter-diagnostic cases with portable temporary paths.
- `scripts/bootstrap/verify_b2_typecheck.sh` passes.
- `scripts/bootstrap/verify_b2_recursive_alias.sh` passes, including `ZAP-TYPE-011` coverage.
- `scripts/bootstrap/verify_b2_typed_ir_candidate.sh` passes its annotated declaration and bounded generic-identity differential checks.
- `scripts/verify_section_a_next50.sh` passes the checked-in focused inference/scope/loop/call-graph cases.

### CI status — not final yet

The latest GitHub run is [Zap CI run #34186734282](https://github.com/hidecard/zap/actions/runs/34186734282) for commit `5cfeb58`. At the latest check it is **in progress** in `Rust quality checks`, currently running `Verify run_zap() portability refactor smoke`, with no recorded failure yet. Because the run has not completed, the repository is **not certified CI-green**.

## Completed immediate blockers

- [x] Repair the B1 lexer contract ownership assertion to match the canonical `OWNERS.tsv` status.
- [x] Make the lexer ownership assertion CRLF-safe by trimming trailing carriage returns from parsed fields.
- [x] Repair the CI release-version regression test so it discovers the built native binary on Linux, macOS, and Windows instead of hard-coding `bin/zap.exe`.
- [x] Repair the parser-candidate gate’s hard-coded `D:/zap` temporary paths and use portable `mktemp` paths.
- [x] Make parser and typed-IR candidate normalization accept the intended multi-record JSON output format.
- [x] Re-run local doctor, native build/tests, release-version checks, Markdown links, lexer contract, parser candidate, and typed-IR candidate validation.

## Remaining P0 work — required before declaring the validation baseline green

- [ ] Wait for CI run `34186734282` to complete and inspect every failed job or step.
- [ ] If CI exposes additional portability failures in `run_zap()` smoke coverage, fix them and rerun the complete workflow.
- [ ] Repair the B1 aggregate runner so it executes every supported gate format directly rather than silently classifying real gates as `SKIP`.
- [ ] Make aggregate runners return a non-zero exit code whenever any child gate fails.
- [ ] Refresh consolidated B1/B2/B3/B4 evidence from a clean checkout after CI is green.

## Remaining P1 work — complete parser and analysis ownership

- [ ] Complete arbitrary-program parser ownership for all required valid and invalid grammar, nested function/class/module forms, complete block metadata, and token-native handling without bounded-corpus assumptions.
- [ ] Complete the B1 diagnostic parity matrix for code, line, column, message, severity, and source-name fields.
- [ ] Replace bounded/provisional type inference with a complete AST-driven flow environment covering arbitrary expressions, nested collections, generic calls, imported bodies, loop mutation, reassignment invalidation, and call cycles.
- [ ] Make the typed-IR producer consume the complete parser AST directly, including all statement/expression kinds, source spans, generic substitutions, and deterministic serialization/readback.
- [ ] Expand the differential corpus for valid and invalid programs and wire every new fixture into CI.

## Remaining P2 work — runtime, packaging, and self-hosting

- [ ] Complete Zap-owned package/build/lock/offline-policy behavior and dependency resolution across transitive, duplicate, cycle, and cross-version cases.
- [ ] Complete native-independent bytecode/VM semantics for arbitrary-arity calls, closures, functions, classes, member/index mutation, exceptions, and error propagation.
- [ ] Re-run and certify the B4 Rust-free acceptance rows from the current commit. Existing certification artifacts are evidence to verify, not a substitute for a current green run.
- [ ] Complete platform-seed reproducibility and byte-for-byte or canonical second-stage self-rebuild.
- [ ] Run the self-build from a clean checkout without the Rust reference compiler, hidden local binaries, temporary symlinks, or manually generated expected outputs.
- [ ] Keep `self_hosted = false` until the platform-seed self-rebuild acceptance gate passes and its evidence is committed.

## Recent progress now reflected

The default-parameter parser path is portable, the previous token-native indentation crash is fixed, the lexer ownership contract is now CRLF-safe, and the CI release-version test no longer assumes a Windows executable name on Linux. Parser boundary fixtures, recursive type-alias diagnostics, B2 typecheck coverage, typed-IR golden normalization, Python-based JSON checking, compatibility matrices, security regression tests, and release gates have been added or expanded.

## Recommended execution order

First wait for and resolve any remaining failures in CI run `34186734282`. Then make the aggregate runner truthful and regenerate consolidated evidence from a clean checkout. After the validation baseline is genuinely green, continue with complete parser/AST ownership, then general type inference and typed-IR ownership. Only after those are stable should Zap-owned package/build/VM execution and the B4 platform-seed self-rebuild be certified.

## References

- Latest fix commit: https://github.com/hidecard/zap/commit/5cfeb5804b5e36d8fbcfbc1d6ad1205542c20280
- Latest CI run: https://github.com/hidecard/zap/actions/runs/34186734282
- B1 lexer ownership contract: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_lexer_contract.sh
- B1 lexer gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_lexer.sh
- B1 parser candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_parser_candidate.sh
- B2 typed-IR candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b2_typed_ir_candidate.sh
- Release version regression test: https://github.com/hidecard/zap/blob/master/scripts/test_validate_release_version.sh
- B1/B2/B3/B4 execution queue: https://github.com/hidecard/zap/blob/master/SECTION_A_NEXT10_QUEUE.md
