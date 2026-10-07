# Zap — Remaining Work and Verification Status

> **Snapshot:** `master` at `258c04d` (`fix(ci): correct Zap C backend smoke fixture`), checked on 2026-10-03. This document separates **verified current results** from **roadmap claims**. A gate is marked complete only when the current checkout passes the corresponding command.

## Executive summary

Zap’s developer toolchain, native runtime build, release-version validation, B1 lexer ownership contract, B1 parser candidate gate, and B2 typed-IR candidate gate are now passing locally from the current checkout. The earlier token-native indentation crash and the CI release-binary path problem have been addressed. The lexer ownership contract also now handles the repository’s CRLF-formatted `OWNERS.tsv` correctly.

The latest GitHub CI run for `8057d1e` is **green** across Rust quality, native builds, C backend Linux/Windows/macOS jobs, cross-platform comparison, and B4 platform evidence. Local B1/B2/B3/B4 regression gates also pass, including parser ownership, while-else typed-IR preservation, package/build resolution, and supported-subset rebuild. The remaining product work is broader than bounded validation: complete parser and type-checker ownership, complete typed-IR production, native-independent runtime coverage, and a clean two-stage self-rebuild are still required before `self_hosted = true`.

## Current verified status

### PASS — verified locally from `1e123a8`

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
- `scripts/bootstrap/verify_zap_c_backend_structure.sh` passes after correcting its scalar fixture to use a numeric constant; the gate now also prints compiler diagnostics on failure.
- `scripts/bootstrap/aggregate_b1_parser_gates.sh` passes all 22 discovered B1 gates with `PASS: 22`, `FAIL: 0`, and `SKIP: 0`; direct-execution and failure propagation behavior are verified locally.
- `while ... else` is now accepted by the Zap-owned general parser as a `while` node with `else_branch`; the token-native indentation, full-language corpus, and parser-candidate gates pass locally with 6 diagnostics and no unsupported valid-syntax fixture.
- The follow-up arbitrary-block gate expectation was updated to the same supported contract after CI exposed the stale rejection assertion; local B1 aggregate remains `22/22` with zero failures and zero skips.
- Typed-IR ownership now preserves direct `while` `else_branch` blocks in both structural and inferred IR, validates the nested branch recursively, and keeps the reference projection branch-aware. Top-level parser rebasing and AST lowering also retain the branch; local B1 full-language/arbitrary-block, B2 owned typed-IR, B4 ownership, and B1 aggregate gates pass.
- The canonical B4 AST lowering path had a runtime-only gap: `while ... else` was retained in AST/typed-IR but its `else` instructions were dropped by `ast_control.zp`. The lowering now emits the branch on natural termination and skips it on `break`; the regression gate covers both semantics, with B4 full acceptance `18/18` and three-stage Rust-free replay passing.

### CI status — green validation baseline

The latest GitHub run is [Zap CI run #37190863622](https://github.com/hidecard/zap/actions/runs/37190863622) for commit `8057d1e`. It completed **successfully**: all 11 jobs passed, including `Rust quality checks`, three native build jobs, three C backend jobs, cross-platform comparison, and three B4 platform-evidence jobs. CI is green for the current validation baseline; full self-hosting certification remains open.

## Completed immediate blockers

- [x] Repair the B1 lexer contract ownership assertion to match the canonical `OWNERS.tsv` status.
- [x] Make the lexer ownership assertion CRLF-safe by trimming trailing carriage returns from parsed fields.
- [x] Repair the CI release-version regression test so it discovers the built native binary on Linux, macOS, and Windows instead of hard-coding `bin/zap.exe`.
- [x] Repair the parser-candidate gate’s hard-coded `D:/zap` temporary paths and use portable `mktemp` paths.
- [x] Make parser and typed-IR candidate normalization accept the intended multi-record JSON output format.
- [x] Re-run local doctor, native build/tests, release-version checks, Markdown links, lexer contract, parser candidate, and typed-IR candidate validation.
- [x] Restore executable permission for `verify_b1_parser_zap_only.sh` and use explicit `bash` invocation in the CI workflow.
- [x] Correct the Zap C backend structure smoke fixture and verify the gate locally.
- [x] Verify the B1 aggregate runner executes all 22 gates and returns non-zero on child failure.

## Remaining P0 work — required before declaring the validation baseline green

- [x] Commit and push the C backend smoke-fixture fix, then wait for and verify the successor CI run.
- [ ] If CI exposes additional portability failures in `run_zap()` smoke coverage, fix them and rerun the complete workflow.
- [x] Refresh the current local B1/B2/B3/B4 acceptance evidence after CI is green; the clean checkout is at `8057d1e` and the targeted B3/B4 package/rebuild gates pass.

## Remaining P1 work — complete parser and analysis ownership

- [ ] Complete arbitrary-program parser ownership for all remaining valid and invalid grammar, nested function/class/module forms, complete block metadata, and token-native handling without bounded-corpus assumptions. The former `while ... else` parser/typed-IR boundary gap is closed; broader loop control semantics still require dedicated end-to-end coverage.
- [ ] Complete the B1 diagnostic parity matrix for code, line, column, message, severity, and source-name fields.
- [ ] Replace bounded/provisional type inference with a complete AST-driven flow environment covering arbitrary expressions, nested collections, generic calls, imported bodies, loop mutation, reassignment invalidation, and call cycles.
- [ ] Make the typed-IR producer consume the complete parser AST directly, including all statement/expression kinds, source spans, generic substitutions, and deterministic serialization/readback.
- [ ] Expand the differential corpus for valid and invalid programs and wire every new fixture into CI.

## Remaining P2 work — runtime, packaging, and self-hosting

- [x] Verify the current Zap-owned package/build/lock/offline-policy behavior and dependency resolution across transitive, duplicate, cycle, and cross-version cases; B3 foundation/build-plan/dependency/registry gates and B4 owned-package/user-command gates pass locally.
- [ ] Complete native-independent bytecode/VM semantics for arbitrary-arity calls, closures, functions, classes, member/index mutation, exceptions, and error propagation.
- [ ] Re-run and certify the B4 Rust-free acceptance rows from the current commit. Existing certification artifacts are evidence to verify, not a substitute for a current green run.
- [ ] Complete platform-seed reproducibility and byte-for-byte or canonical second-stage self-rebuild.
- [ ] Run the self-build from a clean checkout without the Rust reference compiler, hidden local binaries, temporary symlinks, or manually generated expected outputs.
- [ ] Keep `self_hosted = false` until the platform-seed self-rebuild acceptance gate passes and its evidence is committed.

## Recent progress now reflected

The default-parameter parser path is portable, the previous token-native indentation crash is fixed, the lexer ownership contract is now CRLF-safe, and the CI release-version test no longer assumes a Windows executable name on Linux. Parser boundary fixtures, recursive type-alias diagnostics, B2 typecheck coverage, typed-IR golden normalization, Python-based JSON checking, compatibility matrices, security regression tests, and release gates have been added or expanded.

The latest runtime audit fixed the class and closure ownership gaps: class methods were checked without a receiver type, constructors were reported as unknown functions, class method lowering used the outer function depth, and nested closure bodies lost their enclosing environment. Class constructors/method receivers are now typechecked, class methods lower with the correct depth, stable seed artifacts always expose an `error` field, closure bodies receive outer bindings plus parameters, `set target = value` is parsed as field assignment, nested member lookup returns a stable result, and empty/nested class diagnostics remain fail-closed. B2 complete inference/typed-IR/member gates and B4 functions, classes, closures, fields, mutable closures, and canonical AST closure gates pass locally (`10/10`).

## Recommended execution order

The validation baseline and CI are green. Next push the closure/field fixes and verify the successor CI run, then continue with index mutation, exceptions/generators, complete-language diagnostics, and seed provenance. The B1 aggregate remains `22/22`, B2 inference gates pass, B4 full acceptance is `18/18`, and Rust-free three-stage replay passes; B4 certification remains blocked by complete ownership and seed provenance evidence.

## References

- Latest fix commit: https://github.com/hidecard/zap/commit/5e3ccae
- Latest CI run: https://github.com/hidecard/zap/actions/runs/37260637327
- B1 lexer ownership contract: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_lexer_contract.sh
- B1 lexer gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_lexer.sh
- B1 parser candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_parser_candidate.sh
- B2 typed-IR candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b2_typed_ir_candidate.sh
- Release version regression test: https://github.com/hidecard/zap/blob/master/scripts/test_validate_release_version.sh
- B1/B2/B3/B4 execution queue: https://github.com/hidecard/zap/blob/master/SECTION_A_NEXT10_QUEUE.md
