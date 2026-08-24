# Zap v2.11.13 Release Notes

**Release status:** Published corrective release for the v2.11.12 failed-tag incident; Zap remains B0.

## Summary

Zap v2.11.13 carries forward the provisional, corpus-limited direct `is_option_none` else-body narrowing evidence introduced for one tracked `option<number>` variable and adds the smallest safe cross-platform CI correction. The native web-server regression test no longer half-closes the client write side after sending complete CRLF-terminated request headers; the parser does not require EOF, and avoiding that unnecessary half-close removes the macOS ARM64 local-socket reset behavior observed in CI.

The v2.11.12 annotated tag is preserved as immutable evidence of a release workflow that failed in the macOS ARM64 target-native test, and no v2.11.12 GitHub Release was published. v2.11.13 uses a new tag and was published only after its own complete release workflow and public artifact verification passed.

## Changes

| Area | Change | Boundary |
|---|---|---|
| B2 candidate | Retains direct `is_option_none` else-body narrowing for one tracked `option<number>`. | One direct conditional shape and one indented else body |
| CI reliability | Removes the test client’s unnecessary half-close after complete request headers. | Test-harness portability fix; no runtime ownership claim |
| macOS ARM64 | Full post-fix CI matrix passed the native target tests. | Evidence is tied to the verified commit and workflow run |
| Release incident handling | Preserves immutable v2.11.12 tag/history and uses a new corrective version. | No tag move, deletion, or fabricated release |
| Documentation | Updates active bilingual version metadata and records the failed-tag boundary. | Historical records remain intact |

## Verification contract

The candidate must pass native and candidate B2 verification, malformed-source no-panic safety, matrix parity, specification ownership, Markdown links, VS Code packaging, Cargo formatting/checks, RustSec audit, and the exact committed release preflight. The public workflow must independently pass source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs. Published artifacts must pass checksum, manifest, provenance, and detached-signature verification.

## Incident record

The v2.11.12 tag remains immutable at its original release-preparation commit. Its release workflow passed source validation and Linux/Windows build jobs but failed the macOS ARM64 target-native test `evaluator::tests::native_web_server_handles_requests_and_isolates_handler_errors`, which reported 265 passed and 1 failed. The Publish job was skipped, so v2.11.12 was not a public release. The corrected master commit subsequently passed the full Zap CI matrix, including macOS ARM64.

## Deferred scope

Compound guards, loop mutation, reassignment invalidation, aliases, nested or arbitrary control flow, broader collection/map inference, nested maps, generic declarations, complete typed-IR ownership, package/build ownership, VM ownership, production async reactor ownership, and B4 self-rebuild acceptance remain deferred behind separate design and evidence gates.

## Bootstrap boundary

Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner. The Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` remains provisional and corpus-limited; this release does not claim a fully Zap-only, self-hosted, B1, B2, B3, or B4 compiler.

## References

[1]: RELEASE_ROLLBACK_RUNBOOK_EN.md
[2]: ../bootstrap/contracts/OWNERS.tsv
[3]: ../bootstrap/fixtures/typecheck/else_narrowing.zp
[4]: ../bootstrap/fixtures/typecheck/else_narrowing_incompatible.zp
[5]: ../scripts/bootstrap/verify_b2_typecheck.sh
[6]: ../scripts/bootstrap/verify_b2_typecheck_candidate.sh
[7]: ../native/src/evaluator.rs
