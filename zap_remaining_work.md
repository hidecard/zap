# Zap — Remaining Work and Verification Status

> **Snapshot:** `master` at `0d2c9c7` (`fix(ci): replace jq with Python wrapper and normalize typed-IR golden files`), checked on 2026-09-06. This document separates **verified current results** from **repository roadmap claims**. A gate is marked complete here only when the current checkout was run successfully.

## Executive summary

Zap has made substantial progress in the B1 parser, B2 type checking, typed-IR validation, compatibility documentation, and CI tooling. The developer toolchain is ready, the native runtime builds, and the native tests pass. The previous token-native indentation `index out of range` problem is resolved.

The remaining immediate work is now concentrated in **validation infrastructure and output contracts**, not the earlier indentation crash. The current checkout still has three P0 validation issues: a shell syntax error in the standalone lexer gate, an unset `ZAP_BIN` in the parser-candidate gate, and multi-document JSON output in the typed-IR candidate validator. Full/general parser ownership, complete AST-driven type inference, production typed-IR ownership, native-independent VM execution, and B4 self-rebuild remain roadmap work until their acceptance gates are rerun and certified from this checkout.

## Current verified status

### PASS — verified in the current checkout

- `make doctor` passes. Rust/Cargo `1.88.0`, `cargo-audit 0.22.0`, the pinned toolchain, and native runtime `zap 2.11.18` are available.
- Native tests pass: **271 unit/all-target tests and 259 integration tests**, with zero failures in the latest run.
- `verify_b1_token_native_indentation.sh` passes. The earlier `ProjectError: index out of range` is no longer reproducible.
- `verify_b1_boundary_fixtures.sh` passes when the platform binary path is available.
- `verify_b2_typecheck.sh` passes.
- `verify_b2_recursive_alias.sh` passes, including `ZAP-TYPE-011` coverage.
- `verify_section_a_next50.sh` passes.
- B1 aggregate execution reports **15 pass, 1 fail, and 4 skip**; this is not a full green result because the parser-candidate gate still fails and some scripts are skipped by the aggregate extractor.

### FAIL — immediate blockers

- **B1 lexer gate:** `scripts/bootstrap/verify_b1_lexer.sh` fails at line 161 with `syntax error near unexpected token 'done'`. The aggregate runner can incorrectly report the lexer as passing because it extracts a generated runner instead of executing the source gate directly. The source script must be repaired and run directly.
- **B1 parser-candidate gate:** `scripts/bootstrap/verify_b1_parser_candidate.sh` fails at line 363 with `ZAP_BIN: unbound variable`. The script needs a portable default binary lookup or an explicit environment contract.
- **B2 typed-IR candidate gate:** `scripts/bootstrap/verify_b2_typed_ir_candidate.sh` fails with `JSONDecodeError: Extra data: line 2 column 1`. The producer/validator contract must define whether output is one JSON document, JSON Lines, or a framed multi-result stream, then the fixture and validator must agree.

### PARTIAL — aggregate/tooling issues

- The B1 aggregate runner still skips gates whose runner format does not match its heredoc extractor. It must support direct execution for external-runner and unquoted-heredoc scripts instead of silently classifying them as `SKIP`.
- Aggregate failure handling must return a non-zero exit code whenever `FAIL > 0`; otherwise CI can display a misleading success status.
- Binary discovery must be portable across Linux, macOS, and Windows. Prefer `ZAP_BIN`, then `native/target/release/zap`, then `.exe`, with an explicit error if none exists.
- After the three P0 fixes, rerun the complete B1/B2/B3/B4 validation chain and refresh the evidence files.

## Priority Todo list

### P0 — fix before claiming a green validation baseline

- [ ] Repair the shell syntax error in `scripts/bootstrap/verify_b1_lexer.sh` and run the source script directly.
- [ ] Give `verify_b1_parser_candidate.sh` a portable `ZAP_BIN` default and rerun it directly.
- [ ] Define and enforce the typed-IR output framing contract; fix `verify_b2_typed_ir_candidate.sh` so its JSON parser accepts exactly the intended format.
- [ ] Repair aggregate gate discovery so no real gate is silently skipped.
- [ ] Make aggregate runners return non-zero when any gate fails.
- [ ] Re-run `make doctor`, native tests, B1 differential/candidate/boundary gates, B2 typecheck/typed-IR gates, and the consolidated CI validation after the fixes.

### P1 — complete parser and analysis ownership

- [ ] Complete arbitrary-program parser ownership: all valid and invalid grammar, nested function/class/module forms, complete block metadata, and token-native handling without bounded corpus assumptions.
- [ ] Complete the B1 diagnostic parity matrix for code, line, column, message, severity, and source-name fields.
- [ ] Replace bounded/provisional type inference with a complete AST-driven flow environment covering arbitrary expressions, nested collections, generic calls, imported bodies, loop mutation, reassignment invalidation, and call cycles.
- [ ] Make the typed-IR producer consume the complete parser AST directly, including all statement/expression kinds, source spans, generic substitutions, and deterministic serialization/readback.
- [ ] Expand the differential corpus for both valid and invalid programs and keep every new fixture wired into CI.

### P2 — runtime, packaging, and self-hosting

- [ ] Complete Zap-owned package/build/lock/offline-policy behavior and dependency resolution across transitive, duplicate, cycle, and cross-version cases.
- [ ] Complete native-independent bytecode/VM semantics for arbitrary-arity calls, closures, functions, classes, member/index mutation, exceptions, and error propagation.
- [ ] Re-run and certify the B4 Rust-free acceptance rows from the current commit. Existing certification artifacts are evidence to verify, not a substitute for a current green run.
- [ ] Complete platform-seed reproducibility and byte-for-byte second-stage self-rebuild.
- [ ] Keep `self_hosted = false` until the platform-seed self-rebuild acceptance gate passes from a clean environment.

## Recent progress now reflected

- The default-parameter parser change in `parse_parameter_list` is present in the latest history.
- The token-native indentation crash is fixed and its dedicated gate passes.
- Parser boundary fixtures, recursive type-alias diagnostics, B2 typecheck coverage, typed-IR golden normalization, Python-based JSON checking, compatibility matrices, and example programs were added or expanded.
- CI no longer depends on `jq` for the updated JSON checks.

## Recommended execution order

First repair the three P0 validation blockers and the aggregate runner contract. Then rerun the complete evidence chain from a clean checkout. After the validation baseline is genuinely green, continue with complete parser/AST ownership, then type inference and typed-IR ownership, and only after those are stable proceed to native-independent package/VM execution and B4 self-rebuild.

## References

- Latest commit: https://github.com/hidecard/zap/commit/0d2c9c7
- B1 lexer gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_lexer.sh
- B1 parser candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b1_parser_candidate.sh
- B2 typed-IR candidate gate: https://github.com/hidecard/zap/blob/master/scripts/bootstrap/verify_b2_typed_ir_candidate.sh
- B1/B2/B3/B4 execution queue: https://github.com/hidecard/zap/blob/master/SECTION_A_NEXT10_QUEUE.md
