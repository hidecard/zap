# Zap v2.7.0 Release Notes

**Release line:** v2.7.0
**Verified baseline:** merged `master` after v2.6.0
**Status:** Published incremental language-server foundation release

## Summary

Zap v2.7.0 adds bounded incremental document synchronization to the native Language Server Protocol implementation. The server now advertises `textDocumentSync.change = 2` and safely applies sequential full-document or range edits while preserving version monotonicity, negotiated UTF-8/UTF-16/UTF-32 character boundaries, and the 32 MiB workspace byte cap.

Malformed, stale, oversized, out-of-range, and unknown-document range edits are rejected without replacing stored text. A maximum of 128 content changes is accepted per notification. Regression tests cover symbol updates, diagnostics, sequential edits, UTF-16 surrogate-pair boundaries, and invalid positions.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| LSP synchronization | Added bounded full and range edit application with deterministic rejection rules. | Native LSP unit tests |
| Position safety | Added UTF-8, UTF-16, and UTF-32 range-to-byte validation at character boundaries. | UTF-16 regression test |
| Resource safety | Enforced a 128-edit notification limit and the existing 32 MiB workspace cap after every edit. | Workspace-boundary tests |
| Documentation | Updated English/Burmese Language Guides and Web-native guides with the current synchronization contract. | Bilingual documentation checks |
| Release integrity | Synchronized v2.7.0 metadata, manifests, specifications, policies, and release notes. | Release/version/preflight gates |

## Compatibility and boundaries

Existing diagnostics, hover, completion, signature help, definitions, document/workspace symbols, formatting, and bounded rename behavior remain available. The change is additive for clients that support range synchronization; clients may continue sending full-document changes.

This release does not claim complete cross-file semantic refactoring, project-wide dependency invalidation, incremental compilation, debugger/profiler integration, a provider-neutral production database platform, a production async I/O reactor, a complete ORM, SSR/template compilation, WebSocket/streaming/upload infrastructure, built-in admin UI, or real mobile/AI/IoT provider adapters. Each remains a separate milestone requiring implementation and platform evidence.

## Verification

The native formatter, LSP tests, full native test suite, host tests, release build, framework starter checks, documentation consistency, Markdown link validation, VS Code parity, deployment checks, and clean-tree release preflight must pass before publication. The tag workflow must also pass Linux x86_64, macOS ARM64, and Windows x86_64 packaging, checksum/signature, provenance, installer, and published-asset verification.

## Upgrade

Download the archive matching the target platform from the v2.7.0 GitHub Release, verify its checksum and detached signature, and confirm the installed binary with `zap --version`. Existing `.zp` projects retain their manifest and lockfile workflow.

## References

[1]: ../docs/ZAP_WEB_NATIVE_EN.md
[2]: ../docs/LEARN_ZAP_EN.md
