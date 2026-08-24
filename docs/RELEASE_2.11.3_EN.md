# Zap v2.11.3 Release Notes

**Release line:** v2.11.3
**Verified baseline:** Published Zap v2.11.1 plus the validated v2.11.2 roadmap checkpoint
**Status:** Bootstrap function type-checking corpus and release-pipeline resilience increment

## Summary

Zap v2.11.3 expands the provisional Zap-owned type-checker candidate with one annotated function, parameter/return propagation for that corpus slice, a compatible numeric call, and a deterministic incompatible function-call diagnostic. It also makes the multi-platform release workflow retry target-native tests once after a failed first attempt, while remaining fail-closed if the repeated test fails.

This release remains explicitly **B0**. Rust remains the reference owner for complete type checking, typed IR, parser semantics, diagnostics, package/build behavior, VM execution, and platform boundaries. The new Zap code is corpus-limited transition evidence and does not claim a fully Zap-only or self-hosted compiler.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Function corpus | Added valid and invalid fixtures for an annotated `number` parameter, return propagation, and a numeric call. | Native B2 conformance gate |
| Call diagnostics | Added candidate handling for an incompatible text argument at the call site with stable `ZAP-TYPE-001`, location, notes, help, and message fields. | Zap-owned B2 candidate gate |
| Candidate determinism | Extended the candidate runner to execute five B2 cases twice and require byte-identical JSON output. | Deterministic candidate gate |
| Release resilience | Added one retry to the target-native test step after a first-attempt failure; the second failure still stops the release. | Release workflow review and tagged CI |
| Documentation | Updated bilingual contracts, ownership records, changelogs, release notes, and current v2.11.3 version surfaces. | Documentation, version, and ownership gates |

## Bootstrap boundary

The type-checker candidate still does not implement general expression inference, multiple parameters, default arguments, function return annotations, generic/variant narrowing, control-flow facts, complete diagnostics, or arbitrary source programs. The typed-IR producer remains candidate-only and restricted to the annotated declaration fixture; it does not yet emit the function corpus.

The release therefore must not be described as fully Zap-only, fully self-hosted, or B4. Broader owned corpus coverage, independent analysis, differential evidence, compatibility decisions, and a documented platform-seed boundary remain necessary for future stage advancement.

## Verification

The clean v2.11.2-based preflight passed with `passed=204`, `warnings=1`, and `failures=0` before the release-pipeline retry change. The expanded native and Zap-owned B2 function gates, existing parser/lexer gates, B0 artifacts, B3 package/build foundations, VM/platform foundations, documentation consistency, ownership, and formatting checks passed. A previous v2.11.2 tagged run had one isolated macOS ARM64 target-native test failure and therefore did not publish; its tag was not rewritten. v2.11.3 includes the narrow retry hardening and is the next release candidate.

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../scripts/bootstrap/verify_b2_typecheck.sh
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../.github/workflows/release.yml
[6]: ../bootstrap/fixtures/typecheck/function.zp
[7]: ../bootstrap/fixtures/typecheck/function_incompatible.zp
