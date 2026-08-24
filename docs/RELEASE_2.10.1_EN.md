# Zap v2.10.1 Release Notes

**Release line:** v2.10.1
**Verified baseline:** Zap v2.10.0 on the latest master before the v2.10.1 tag
**Status:** Published bootstrap parser and diagnostics foundation increment

## Summary

Zap v2.10.1 adds the next auditable increment of the Zap-only bootstrap roadmap. The repository now carries a provisional Zap-written parser candidate for an arithmetic and compound corpus, token-driven delimiter diagnostics backed by the Zap lexer candidate, and a deterministic B2 typed-IR/type-check conformance foundation.

This release remains explicitly **B0**. Rust remains the reference owner for the compiler pipeline, diagnostics, type checking, typed IR, package/build behavior, VM, and platform boundaries. The new Zap artifacts are differential evidence and provisional ownership candidates; they do not claim a fully Zap-only or self-hosted compiler.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Parser corpus | Added canonical compound AST and malformed-input fixtures for maps, lists, postfix indexing, binary operators, conditionals, returns, and calls. | Reference parser differential gate |
| Zap parser candidate | Expanded `bootstrap/b1/parser.zp` from arithmetic-only handling to the owned compound corpus slice. | Byte-for-byte parser candidate gate |
| Diagnostics | Replaced the candidate’s source-substring bracket check with token-stream delimiter scanning for missing and unexpected closing delimiters. | Canonical syntax-diagnostic fixtures |
| Typed IR | Added deterministic annotated typed-IR artifact evidence with schema and `reference_only` markers. | B2 typed-IR reproducibility gate |
| Type checking | Added valid annotation/conditional and incompatible annotation acceptance/rejection fixtures against the native checker. | B2 type-check conformance gate |
| Contracts | Updated bilingual bootstrap contracts, ownership records, current v2.10.1 identities, and Unreleased/release documentation. | Documentation, ownership, and version gates |

## Bootstrap boundary

The parser candidate is intentionally corpus-limited and retains fixture-scoped assumptions. It does not replace the Rust lexer or parser and does not yet cover the complete Zap grammar. The typed-IR artifact and type-check behavior remain native-owned reference contracts. B3 package/build/test-runner and VM/platform checks remain foundation evidence rather than self-hosting claims.

The release therefore must not be described as fully Zap-only, fully self-hosted, or B4. Future stage advancement requires broader owned corpus coverage, independent Zap implementations, byte-for-byte or semantic differential evidence, compatibility decisions for mismatches, and a documented platform-seed boundary.

## Verification

The clean release preflight passed with `passed=202`, `warnings=1`, and `failures=0`. Native formatting, clippy, cargo check, RustSec audit, native tests, host tests, framework checks, bilingual documentation checks, Markdown links, parser/lexer/type-check bootstrap gates, package/build foundations, and VM/platform foundations passed. The single warning is the expected development preflight omission of `RELEASE_TAG`; tagged CI additionally verifies the tag/version match and builds all supported archives.

## References

[1]: ../docs/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
[3]: ../bootstrap/b1/parser.zp
[4]: ../scripts/bootstrap/verify_b1_parser_candidate.sh
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../docs/TYPECHECK_CONFORMANCE_MATRIX_EN.md
