# Zap v2.11.2 Release Notes

**Release line:** v2.11.2
**Verified baseline:** Published Zap v2.11.1
**Status:** Bootstrap function type-checking corpus increment

## Summary

Zap v2.11.2 expands the provisional Zap-owned type-checker candidate beyond simple declarations and conditionals. The candidate now covers one annotated function, parameter/return propagation for that corpus slice, a compatible numeric call, and a deterministic incompatible function-call diagnostic. The native reference checker gate covers the same acceptance and rejection cases.

This release remains explicitly **B0**. Rust remains the reference owner for complete type checking, typed IR, parser semantics, diagnostics, package/build behavior, VM execution, and platform boundaries. The new Zap code is corpus-limited transition evidence and does not claim a fully Zap-only or self-hosted compiler.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Function corpus | Added valid and invalid fixtures for an annotated `number` parameter, return propagation, and a numeric call. | Native B2 conformance gate |
| Call diagnostics | Added candidate handling for an incompatible text argument at the call site with stable `ZAP-TYPE-001`, line, column, notes, help, and message fields. | Zap-owned B2 candidate gate |
| Candidate determinism | Extended the candidate runner to execute all five B2 cases twice and require byte-identical JSON output. | Deterministic candidate gate |
| Documentation | Updated bilingual bootstrap contracts, ownership records, Unreleased/release notes, and current v2.11.2 version surfaces. | Documentation, version, and ownership gates |

## Bootstrap boundary

The type-checker candidate still does not implement general expression inference, multiple parameters, default arguments, function return annotations, generic/variant narrowing, control-flow facts, complete diagnostics, or arbitrary source programs. The typed-IR producer remains candidate-only and restricted to the annotated declaration fixture; it does not yet emit the new function corpus.

The release therefore must not be described as fully Zap-only, fully self-hosted, or B4. Broader owned corpus coverage, independent analysis, differential evidence, compatibility decisions, and a documented platform-seed boundary remain necessary for future stage advancement.

## Verification

The clean release preflight passed with `passed=204`, `warnings=1`, and `failures=0` before the v2.11.2 version bump. The focused post-change gates pass for the expanded native and Zap-owned B2 function corpus, existing parser/lexer gates, B0 artifacts, B3 package/build foundations, VM/platform foundations, specification ownership, documentation consistency, and formatting. The single preflight warning is the expected development omission of `RELEASE_TAG`; tagged CI checks tag/version identity and platform archives.

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../scripts/bootstrap/verify_b2_typecheck.sh
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../bootstrap/fixtures/typecheck/function.zp
[6]: ../bootstrap/fixtures/typecheck/function_incompatible.zp
