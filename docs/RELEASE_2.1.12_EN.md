# Zap v2.1.12 Release Notes

**Release date:** 2026-08-22

## Release summary

Zap v2.1.12 makes canonical AST execution normative for normal source programs and local modules. The release preserves current language behavior while removing the normal-program fallback to the legacy line interpreter and retaining line execution only for an explicit compatibility boundary.

## Highlights

The AST parser now represents exported bindings and functions directly in `Stmt::Declaration` and `Stmt::Function` nodes. Local module files are parsed and executed through the same AST executor as the main program, and export markers are preserved for explicit imports. The parser also owns the current `?` Result/Option propagation syntax, prefix `not` expressions, and empty-class declarations used by existing fixtures.

The evaluator no longer reconstructs source lines from a parsed AST when selecting the normal execution path. `ast_program_compatible` is an explicit compatibility predicate that covers all syntax currently produced by `parse_program`. The legacy line interpreter remains available only for older/internal `Function` records with `body: Vec<String>` and no `ast_body: Program`; no new syntax should be added to that path.

The milestone adds AST parser, export, canonical module-import, syntax-failure, `?` propagation, boolean-prefix, empty-class, and inherited-field-visibility regressions. English/Burmese AST foundation status, roadmap, README, and documentation-consistency coverage were updated together.

## Compatibility and deferred scope

Normal source programs that fail AST parsing now return a syntax diagnostic instead of being interpreted by the legacy line path. This makes the AST parser/evaluator boundary normative for parser-owned source. Existing source behavior covered by the parser remains compatible, including Result/Option propagation, OOP visibility, and local module exports.

The legacy line executor remains compatibility-only for pre-AST or test-created line-bodied functions. Removing that representation requires a separately documented breaking compatibility decision after legacy fixtures and migration guidance are reviewed. This release does not introduce first-class callable values, parent-linked environment frames, cumulative memory budgets, broad async syntax, or new traits/interfaces semantics.

## Verification

The milestone passed Rust 1.75.0 formatting, strict Clippy with `-D warnings`, the full native all-target/all-feature suite, documentation consistency validation with 82 checks, its regression harness, and `git diff --check`. The native integration suite reports 254 passing tests, and the focused AST/export/module regressions pass.

## Upgrade guidance

Users can upgrade by downloading the archive for their operating system and architecture from the [v2.1.12 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.12). Verify the published checksum and signature before installation. No source-language migration is required for parser-owned programs; code depending on undocumented legacy line-interpreter fallback should be reviewed as compatibility-sensitive.

## Documentation

Read the [English AST foundation status](P0_FOUNDATION_STATUS_EN.md), [Burmese AST foundation status](P0_FOUNDATION_STATUS_MM.md), [English runtime-state contract](RUNTIME_STATE_EN.md), [Burmese runtime-state contract](RUNTIME_STATE_MM.md), [English documentation navigation](DOCUMENTATION_NAVIGATION_EN.md), and [Burmese documentation navigation](DOCUMENTATION_NAVIGATION_MM.md). Remaining memory, async, conformance, specification, tooling, benchmark, registry-edge-case, and traits work is tracked in the bilingual TODO registers and next-step plans.
