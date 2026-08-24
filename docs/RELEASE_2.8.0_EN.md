# Zap v2.8.0 Release Notes

**Release line:** v2.8.0
**Verified baseline:** Zap v2.7.0 on merged `master`
**Status:** Published Web developer-experience increment

## Summary

Zap v2.8.0 adds a deterministic, read-only `zap web routes` command for inspecting a user-managed Web project’s exported route table without opening a listener. The command supports both human-readable output and `--json` output for editor tooling and scripts.

The release also fixes the Web scaffold so the generated `main.zp` records the actual project name rather than the placeholder `APP_NAME`. The scaffold validator now exercises route inspection in both output modes and verifies that generated project metadata is name-aware.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Route inspection | Execute the exported `routes()` factory and print method, path, handler, and optional scope. | CLI smoke test |
| Route safety | Validate route entry shape, safe absolute paths, method tokens, and handler shape before displaying data. | Shared evaluator contract |
| Server safety | Preserve strict handler resolution inside the live `web_serve` boundary. | Native Web tests |
| Scaffold clarity | Replace the generated `APP_NAME` placeholder with the sanitized project directory name. | Framework starter regression |
| Automation | Add human-readable and JSON route inspection to starter validation. | Framework validator |
| Documentation | Synchronize English/Burmese Web, framework, learner, and CLI references. | Documentation/link gates |

## Usage

```bash
zap new shop
cd shop
zap web check
zap web routes
zap web routes --json
zap dev
```

The normal output is intended for developers reading a project. JSON output is intended for editor integrations and automation. Inspection does not start a server and does not grant a route permission to execute. The development server still performs the stricter handler-resolution check before accepting traffic.

## Compatibility and boundaries

The command is additive and does not change the existing `zap new`, `zap web check`, `zap dev`, or full-document project workflow. User-managed `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/` directories remain ordinary editable Zap modules. Frontend build tools remain optional at build time; the deployed process still needs only the Zap runtime and emitted browser assets.

This release does not claim first-class route syntax, automatic JSON validation schemas, centralized `Result` error middleware, a production async I/O reactor, a provider-neutral production ORM/database platform, cross-file refactoring, incremental compilation, debugger/profiler integration, SSR/template compilation, WebSocket/streaming uploads, a built-in admin UI, or real mobile/AI/IoT provider adapters. These remain separate milestones requiring implementation and platform evidence.

## Verification

The release branch must pass native formatting, the full native test suite, framework starter validation, documentation consistency, Markdown link validation, VS Code asset validation, deployment checks, and clean-tree release preflight before publication. The tagged workflow must also verify Linux x86_64, macOS ARM64, and Windows x86_64 archives, checksums, signatures, provenance, manifest, installer behavior, and published assets.

## References

[1]: ../docs/ZAP_WEB_NATIVE_EN.md
[2]: ../docs/WEB_FRAMEWORK_EN.md
[3]: ../docs/LEARN_ZAP_EN.md
