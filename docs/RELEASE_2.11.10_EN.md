# Zap v2.11.10 Release Notes

**Release status:** Maintenance and release-governance documentation release; the bootstrap stage remains B0.

## Summary

Zap v2.11.10 records the repository branch-hygiene and merge policy used after the v2.11.9 release. The audit found no open pull request or clean branch eligible for merge. The remote `fix/json-cycle-guard` branch is associated with closed pull request #1, contains six commits unique relative to the current master line, and is substantially behind current master. Because it represents superseded production-hardening history rather than a clean current delta, it was intentionally retained for continuity and was not blindly merged or deleted.

The release adds a bilingual branch-hygiene record and links it from the documentation navigation and both top-level READMEs. The record defines the ancestry, patch-equivalence, pull-request, and release-history checks required before future branch merging or deletion. Local stale references were pruned; no published release tag was moved, rewritten, or removed.

This release does not claim new compiler semantics or bootstrap ownership. Zap remains **B0**. Rust remains the complete reference compiler and runtime owner, while Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` remains provisional and corpus-limited.

## Changes

| Area | Change | Boundary |
|---|---|---|
| Branch audit | Recorded current branch, PR, ancestry, divergence, and retention findings. | No blind merge or inferred deletion |
| Documentation | Added synchronized English/Burmese branch-hygiene and merge records. | Policy documentation only |
| Navigation | Linked the branch record from both release-operator documentation hubs. | Relative links validated |
| README guidance | Linked the branch record from English and Burmese contribution guidance. | Does not alter branch permissions |
| Cleanup | Pruned local stale references only. | Intentionally retained `origin/fix/json-cycle-guard` |
| Release integrity | Preserved all existing tags and release history. | v2.11.9 and earlier remain immutable |

## Verification contract

The release candidate must pass version consistency, bilingual documentation consistency, Markdown link validation, type-check matrix consistency, specification ownership validation, formatting, native tests, malformed-source safety, bootstrap gates, package/build gates, security audit, and the exact committed release preflight. The public workflow must independently pass source validation, Linux x86_64, macOS ARM64, Windows x86_64, and Publish jobs before artifact verification.

## Deferred scope

Merging or deleting any additional branch without explicit provenance checks, broader branch/loop type inference, generic declarations, nested maps, deeper nested expressions, complete typed-IR ownership, package/build ownership, VM ownership, and B4 self-rebuild acceptance remain deferred. No feature is promoted merely because it appears in a historical branch or documentation record.

## Historical and release policy

Published v2.11.9 and all earlier tags remain immutable. v2.11.10 uses a new annotated tag and must not rewrite prior release history. The current-status pages must identify v2.11.10 as latest only after the public workflow and artifact verification succeed.

## References

[1]: ../docs/BRANCH_HYGIENE_EN.md
[2]: https://github.com/hidecard/zap/pull/1
[3]: ../docs/DOCUMENTATION_NAVIGATION_EN.md
[4]: ../docs/CURRENT_STATUS_EN.md
