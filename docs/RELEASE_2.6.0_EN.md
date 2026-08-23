# Zap v2.6.0 Release Notes

**Release line:** v2.6.0
**Verified baseline:** merged `master` after v2.5.0
**Status:** Published incremental Web observability and integrity release

## Summary

Zap v2.6.0 adds a bounded public `/metrics` endpoint to the `zap-host` adapter and records executable evidence for the new observability contract. The endpoint emits low-cardinality process counters for total requests, 5xx responses, and in-flight requests without paths, identities, request IDs, or user-controlled labels.

The one-command user-managed `zap new <directory>` workflow remains unchanged. The Web adapter continues to provide bounded request policy, request IDs, timeouts, rate-limit ordering, readiness, graceful drain, database-pool admission guards, and explicit authentication/authorization seams. The release strengthens the documentation and quickstart examples for these boundaries.

## Implemented changes

| Area | Change | Evidence |
|---|---|---|
| Observability | Added public `GET /metrics` with Prometheus-style text output and bounded metric names. | Host unit and HTTP contract tests |
| Security boundary | Metrics output excludes user-controlled labels and request-identifying data. | Low-cardinality renderer test |
| Web documentation | Updated English/Burmese host guide and quickstart with endpoint, route table, and curl example. | Bilingual documentation review |
| Release integrity | Synchronized v2.6.0 metadata, manifests, specifications, policies, and current baselines. | Release/version/documentation gates |

## Compatibility and boundaries

The new endpoint is additive. Existing health, readiness, API, authentication, rate-limit, request-ID, and graceful-shutdown contracts remain unchanged. `/metrics` is intended for local and controlled host-adapter monitoring; deployments must still protect management endpoints according to their network policy.

This release does not claim a complete ORM, provider-neutral production database platform, production async I/O reactor, user-defined traits or generic declarations, cross-file semantic rename, SSR/template compiler, WebSocket/streaming/upload stack, built-in admin UI, or real mobile/AI/IoT provider adapters. Those remain separate milestones requiring implementation and platform evidence.

## Verification

The focused host adapter format, strict Clippy, and test suite pass, including the new metrics regression. The full native and host release gates, framework starter checks, documentation consistency, Markdown link validation, VS Code parity, LSP parity, and clean-tree release preflight must pass before publication.

## Upgrade

Download the archive matching the target platform from the v2.6.0 GitHub Release, verify its checksum and detached signature, and confirm the binary with `zap --version`. Existing Zap projects retain their manifest and lockfile workflow.

## References

[1]: ../docs/ZAP_HOST_EN.md
[2]: ../docs/ZAP_HOST_QUICKSTART_EN.md
