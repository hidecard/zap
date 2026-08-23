# Zap v2.4.0 Release Notes

**Release date:** 2026-08-23  
**Release line:** v2.4.0  
**Theme:** A complete learning path and a cleaner project entry point.

## Highlights

Zap v2.4.0 makes the language easier to learn and the repository easier to navigate. The bilingual learner material is now a complete **Zap Language Guide** that begins with installation and the first `.zp` file, then progresses through values, types, control flow, functions, closures, classes, modules, Result/Option, diagnostics, standard-library operations, testing, packages, Web development, databases, async boundaries, LSP usage, runtime safety, and advanced practice.

The root README is now a focused landing page rather than a duplicate implementation dossier. It provides installation, the one-command project generator, the essential CLI workflow, documentation links, frontend integration guidance, development commands, and explicit stable/deferred boundaries.

## One-command project workflow

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

The generated project contains `zap.toml`, `zap.lock`, `main.zp`, `web.zp`, `server.zp`, `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/`. The directories remain ordinary user-managed files. Zap does not introduce a Django-style `startapp` command or a hidden app registry.

## Documentation cleanup

The stale duplicate `docs/PACKAGES.md` note was removed in favor of the maintained bilingual package guides, `docs/PACKAGE_EN.md` and `docs/PACKAGE.md`. Historical release notes, mandatory contract documents, framework READMEs, and security/release evidence remain available because they are part of the project’s traceability or release surface.

The English and Burmese documentation hubs now promote the Language Guide as the first learning entry point and point current work to the v2.4.0 release notes, language specification, and release-version policy.

## Validation

The release candidate is expected to pass the pinned Rust formatting and test suite, Framework starter validation, documentation consistency, release-version consistency, VS Code asset parity, LSP semantic parity, native and host checks, security checks, and the cross-platform Linux, Windows, and macOS ARM64 build workflow.

## Boundaries

This release does not claim a complete ORM, provider-neutral production migration platform, user-defined trait syntax, a production asynchronous I/O reactor, cross-file semantic rename, a template compiler, or a hidden application registry. These areas remain explicitly deferred and are documented in the language specification and related contracts.

## Upgrade guidance

Install the v2.4.0 standalone executable for the platform and architecture that match your machine. Existing `.zp` projects continue to use their current manifest and lockfile workflow. Run `zap check`, `zap test`, and `zap build --locked` after changing the executable or project dependencies.
