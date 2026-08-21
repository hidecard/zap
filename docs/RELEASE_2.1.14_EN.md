# Zap v2.1.14 Release Notes

**Verified version:** v2.1.14
**Release date:** 2026-08-21

## Summary

Zap v2.1.14 is the superseding release for the v2.1.13 tag after a Windows-only line-helper compatibility regression was detected by cross-platform CI. The release preserves the explicit workspace and LSP state migration while restoring the historical behavior required by absolute-path `read_lines` and `write_lines` programs.

## Runtime-state changes

Workspace confinement remains owned by the per-run `RuntimeState`, and the LSP document map remains owned by an explicit per-session `LspState`. The native evaluator records one canonical workspace root and reuses it across nested AST execution, module loading, and context-aware filesystem operations. LSP responses read from the server-owned document map, and independent `LspState` instances cannot observe one another's open documents.

## Cross-platform compatibility fix

The compatibility-only line-helper behavior for absolute-path `read_lines` and `write_lines` is preserved. This keeps existing Windows, Linux, and macOS programs stable while retaining context-aware confinement for the migrated filesystem operations. The failed v2.1.13 Windows CI run was diagnosed, fixed in a focused commit, and superseded by this release version rather than reusing the failed tag.

## Compatibility boundary

Canonical AST execution remains normative for parser-owned programs and local modules. The legacy line interpreter remains an explicit compatibility-only path for older line-bodied function records. First-class callable values, parent-linked `EnvFrame` binding cells, cumulative memory budgets, broad language async syntax, and traits/interfaces semantics remain deferred.

## Verification

The release candidate passed Rust formatting, strict Clippy with `-D warnings`, the full native all-target/all-feature suite with 254 integration tests, workspace and LSP isolation regressions, documentation consistency validation, the documentation regression harness, release preflight, and `git diff --check`. The corrected commit was pushed to `master` before the v2.1.14 tag was created.

## References

* [Runtime-state contract](RUNTIME_STATE_EN.md)
* [AST foundation status](P0_FOUNDATION_STATUS_EN.md)
* [Documentation navigation](DOCUMENTATION_NAVIGATION_EN.md)
* [Full changelog](../CHANGELOG_EN.md)
