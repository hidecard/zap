# Zap v2.5.0 Release Notes

**Release line:** v2.5.0

**Theme:** Documentation integrity and safer project operations

## Summary

Zap v2.5.0 consolidates the v2.4.0 learning and Web direction into a more reliable user-facing surface. The release keeps the one-command, user-managed `zap new <directory>` workflow, adds repository-wide Markdown link validation to CI and release preflight, repairs stale operational metadata, and makes the boundary between a development/reference Web slice and a production deployment explicit.

## What changed

| Area | Change |
|---|---|
| Documentation integrity | Added a repository-wide relative Markdown link validator and connected it to CI and release preflight. The validator checks tracked Markdown files, ignores external URLs, rejects links escaping the repository, and reports missing targets with file and line numbers. |
| Canonical learning path | Replaced the stale Burmese-only `docs/LANGUAGE_GUIDE.md` course with a compatibility alias to the maintained bilingual Language Guides. New projects should use `zap new`, while `zap init` remains documented only as a compatibility command. |
| Usage and operations | Rewrote the bilingual usage guides and synchronized deployment, production-operations, RustSec, standard-library, ecosystem, and progress metadata with the v2.5.0 development line. Old roadmap records are now labeled historical where appropriate. |
| Provenance | Added bilingual post-v2.4.0 remediation/provenance records so the immutable v2.4.0 tag is not confused with later master corrections. |
| User-managed Web | Preserved the explicit `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/` project layout without introducing a hidden app registry or Django-style `startapp`. |

## Important boundary

This release does not claim a complete ORM, provider-neutral production migration platform, user-defined traits or generic declarations, production async I/O reactor, cross-file semantic rename, SSR/template compiler, WebSocket/streaming/upload stack, built-in admin UI, or real mobile/AI/IoT provider adapters. Those remain separately scoped milestones requiring language contracts, security evidence, and platform tests.

## Verification

The release branch must pass the pinned Rust formatting and test gates, strict Clippy, framework starter validation, documentation consistency and bilingual parity, repository-wide Markdown link validation, release-version checks, VS Code asset validation, LSP semantic parity, registry deployment checks, and the complete release preflight before tagging.

## Upgrade guidance

Users installing v2.5.0 should download the archive matching their platform and architecture, verify its checksum and signature, and run `zap --version`. Existing `.zp` projects remain compatible with their manifest and lockfile workflow. New Web projects should use:

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

For detailed learning, read the [English Language Guide](LEARN_ZAP_EN.md) or [Burmese Language Guide](LEARN_ZAP_MM.md). For release provenance, read the [post-v2.4.0 remediation record](POST_V2.4.0_REMEDIATION_EN.md).
