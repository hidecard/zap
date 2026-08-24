# Zap v2.11.1 Release Notes

**Release line:** v2.11.1
**Verified baseline:** Published Zap v2.11.0
**Status:** Bootstrap type-checker and typed-IR candidate increment

## Summary

Zap v2.11.1 adds the first provisional Zap-owned type-checker candidate and a matching candidate-only typed-IR producer for the bootstrap roadmap. The candidate covers annotated number declarations, compatible conditional expressions, and incompatible number annotations, while the typed-IR producer emits the annotated declaration node and compares its owned fields with the native reference artifact.

This release remains explicitly **B0**. Rust remains the reference owner for complete type checking, typed IR, parser semantics, diagnostics, package/build behavior, VM execution, and platform boundaries. The new Zap code is corpus-limited transition evidence and does not claim a fully Zap-only or self-hosted compiler.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Zap type-checker candidate | Added `bootstrap/b2/typecheck.zp` for three B2 fixtures: annotated number, compatible conditional, and incompatible number annotation. | Deterministic candidate gate |
| Type diagnostics | Emits the stable `ZAP-TYPE-001` candidate diagnostic with source location, notes, help, and expected mismatch message. | Candidate acceptance/rejection gate |
| Zap typed-IR candidate | Added `bootstrap/b2/typed_ir.zp` for the annotated declaration fixture with `candidate_only` metadata. | Candidate typed-IR differential gate |
| Reference parity | Compares owned typed-IR node fields with the native reference artifact while keeping native schema ownership explicit. | B2 reference and candidate gates |
| Release contracts | Added CI and release-preflight enforcement, ownership rows, bilingual bootstrap documentation, changelog entries, and v2.11.1 version surfaces. | Repository validation suite |

## Bootstrap boundary

The type-checker candidate does not implement general expression inference, function parameter or return checking, generic/variant narrowing, complete diagnostic parity, or arbitrary source programs. The typed-IR producer is intentionally restricted to one annotated declaration fixture and does not replace the native typed-IR emitter. Both artifacts are provisional and candidate-only.

The release therefore must not be described as fully Zap-only, fully self-hosted, or B4. Broader owned corpus coverage, independent analysis, differential evidence, compatibility decisions, and a documented platform-seed boundary remain necessary for future stage advancement.

## Verification

The clean v2.11.0-based release preflight passed with `passed=204`, `warnings=1`, and `failures=0`. Native formatting, clippy, cargo check, RustSec audit, native and host tests, documentation and link checks, parser/lexer/bootstrap gates, B2 type-checker and typed-IR candidate gates, package/build foundations, and VM/platform foundations passed. The single warning is the expected development preflight omission of `RELEASE_TAG`; tagged CI additionally checks tag/version identity and platform archives.

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/b2/typecheck.zp
[3]: ../bootstrap/b2/typed_ir.zp
[4]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[5]: ../scripts/bootstrap/verify_b2_typed_ir_candidate.sh
