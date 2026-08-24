# Zap Branch Hygiene and Merge Record

**Audit baseline:** v2.11.9
**Repository:** [github.com/hidecard/zap](https://github.com/hidecard/zap)
**Decision:** No branch was merged or deleted in this maintenance cycle.

## Audit result

The integrated `master` branch is the active release baseline. At the audit point, local `master` and `origin/master` were aligned, and there were no open pull requests eligible for merge. The remote contained one additional branch, `fix/json-cycle-guard`.

That branch is associated with [closed pull request #1](https://github.com/hidecard/zap/pull/1), is not merged, and contains six commits that are unique relative to the current master line. It is also substantially behind current master. Its changes represent superseded production-hardening work rather than a clean, reviewable delta for the current release. A blind merge would risk reintroducing obsolete history and conflicting with later integrated changes.

The branch is intentionally retained for continuity and auditability. It must not be deleted without explicit authorization and a separate review proving that its historical reference is no longer required. Local stale references were pruned with `git fetch --prune origin`; no release tag was removed, moved, or rewritten.

## Operating policy

| Situation | Required action |
|---|---|
| Open branch with reviewable changes and a passing merge path | Review, validate, then merge through the normal pull-request path. |
| Closed branch whose changes are already superseded | Do not blindly merge; retain only when its historical reference is intentional. |
| Branch with unclear provenance or a large divergent delta | Preserve it and document the reason; do not delete or merge by inference. |
| Published release tag | Treat it as immutable; use a new tag for every subsequent release. |

This record is maintenance evidence, not a claim that every historical branch has been semantically re-audited. Future branch cleanup requires the same ancestry, patch-equivalence, pull-request, and release-history checks.

## Bootstrap boundary

Branch hygiene does not change Zap’s maturity claim. Zap remains **B0**. Rust remains the complete/reference compiler and runtime owner, while Zap lexer, parser, type-checker, and typed-IR work under `bootstrap/` remains provisional and corpus-limited.

## References

[1]: https://github.com/hidecard/zap/pull/1
[2]: https://github.com/hidecard/zap/tree/master
[3]: ../docs/CURRENT_STATUS_EN.md
