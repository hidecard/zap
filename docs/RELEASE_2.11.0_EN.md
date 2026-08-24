# Zap v2.11.0 Release Notes

**Release line:** v2.11.0
**Verified baseline:** Published Zap v2.10.1
**Status:** Published deterministic Web route-explanation increment

## Summary

Zap v2.11.0 adds `zap explain route <path>`, an inspect-only native Web command that loads and validates a user-managed Zap Web project, reuses the native route matcher, and reports which declared routes match a concrete request path. It does not open a listener or execute a handler.

The command is bounded and automation-friendly. It accepts a safe absolute path up to 2,048 bytes, strips a query string before matching, reports `:parameter` and final `*wildcard` extraction, preserves declaration order, and supports JSON output. The output intentionally explains path candidates rather than pretending to trace middleware, authorization, or business execution.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| CLI surface | Added `zap explain route <path> [directory] [--json]` and help coverage. | Native CLI unit tests |
| Matching semantics | Reused the same exact, `:parameter`, and final `*wildcard` matcher used by the native Web server. | Native route matcher tests and generated-project smoke test |
| Safety bounds | Rejects relative paths, traversal-shaped paths, empty interior segments, and paths over 2,048 bytes before project loading. | Native CLI unit tests |
| Human output | Shows project validity, path, candidate count, declaration index, method, pattern, handler, and extracted parameters. | Generated-project smoke test |
| JSON output | Emits `project`, normalized `path`, and ordered `matches` records for tooling. | Generated-project smoke test |
| Documentation | Updated bilingual framework, native Web, learner, P2 status, and command-validation documentation. | Documentation, link, policy, and framework gates |

## Usage

```bash
zap explain route /users/42
zap explain route /assets/chunks/app.js ./shop --json
```

A concrete path may match more than one declaration when a specific route and a catch-all SPA fallback both apply. The command lists all path candidates in declaration order. At request time, the native server still chooses the first matching declaration for the requested method; a path match with no matching method becomes `405`, while no path match becomes `404`.

Example JSON shape:

```json
{"project":"valid Zap Web project: shop 0.1.0 (main: main.zp)","path":"/users/42","matches":[{"index":5,"method":"GET","path":"/users/:id","handler":"get_user","params":{"id":"42"}}]}
```

## Compatibility and boundaries

This is an additive CLI capability. Existing `zap web routes`, `zap web check`, `zap dev`, direct response maps, Result-aware handlers, and user-managed project directories remain unchanged. The command does not modify project files, start a network listener, invoke a handler, inspect middleware execution, perform authorization, or infer business behavior.

The route explanation is not a production request tracer or a full route compiler. The native Web server remains a bounded single-threaded development/reference server. Graceful shutdown, concurrent production serving, cancellation/backpressure, readiness, TLS/edge integration, observability, provider-neutral database support, and the other documented production milestones remain separate work.

## Verification

The increment passed the native formatting and test suite, release build, framework starter validation with 201 checks, standard-library policy validation, bilingual documentation consistency with 174 checks, Markdown link validation with 763 links, VS Code asset validation, whitespace checks, and the full clean-tree release preflight with 199 checks passed, 1 warning, and 0 failures.

## References

[1]: ../docs/WEB_FRAMEWORK_EN.md
[2]: ../docs/ZAP_WEB_NATIVE_EN.md
[3]: ../docs/LEARN_ZAP_EN.md
