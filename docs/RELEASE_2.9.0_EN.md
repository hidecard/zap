# Zap v2.9.0 Release Notes

**Release line:** v2.9.0
**Verified baseline:** Zap v2.8.0 on merged `master`
**Status:** Published Web framework safety increment

## Summary

Zap v2.9.0 makes Web route validation consistent between inspection, project checks, and the native development server. Duplicate method/path registrations are now rejected before a route table is displayed or a listener accepts traffic.

The same shared route-table validator is used by `zap web check` and `zap web routes`. This turns route conflicts into an early, deterministic project error instead of leaving behavior dependent on declaration order.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Route table | Validate method/path registration uniqueness across the complete exported route list. | Native unit regression |
| `zap web check` | Execute the exported `routes()` factory and reject malformed or conflicting route entries during project validation. | Generated-project smoke test |
| `zap web routes` | Reuse the same route-table validator for text and JSON inspection. | CLI smoke test |
| Live server | Preserve the shared conflict check and strict named-handler resolution before serving. | Native Web contract tests |
| Documentation | Synchronize English/Burmese Web and framework guidance with the executable behavior. | Documentation/link gates |

## Usage

```bash
zap new shop
cd shop
zap web check
zap web routes
zap web routes --json
zap dev
```

A route table may use the same path for different methods, such as `GET /users` and `POST /users`, but the same method/path pair may not be registered more than once. `zap web check` and `zap web routes` report the conflict without starting a network listener. `zap dev` performs the same conflict check and also resolves named handlers before serving.

## Compatibility and boundaries

This is an additive safety improvement for the existing user-managed project structure. It does not introduce hidden app registration or a Django-style `startapp` command. The `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/` directories remain editable Zap modules, and browser build output remains deployable without Node.js as a runtime prerequisite.

This release does not claim first-class route syntax, automatic JSON validation schemas, centralized `Result` error middleware, a production async I/O reactor, provider-neutral ORM/database support, cross-file refactoring, incremental compilation, debugger/profiler integration, SSR/template compilation, WebSocket/streaming uploads, a built-in admin UI, or real mobile/AI/IoT provider adapters. These remain separate milestones requiring implementation and evidence.

## Verification

The release branch must pass native formatting, the full native test suite, framework starter validation, documentation consistency, Markdown link validation, VS Code asset validation, deployment checks, and clean-tree release preflight. The tagged workflow must verify Linux x86_64, macOS ARM64, and Windows x86_64 archives, checksums, signatures, provenance, manifests, installers, and published assets.

## References

[1]: ../docs/ZAP_WEB_NATIVE_EN.md
[2]: ../docs/WEB_FRAMEWORK_EN.md
[3]: ../docs/LEARN_ZAP_EN.md
