# Zap Bootstrap Contract

**Status:** Bootstrap foundation for Zap v2.9.0

## Purpose

This contract defines how the current Rust implementation and the future Zap implementation cooperate during self-hosting. The Rust implementation is **B0**, the reference compiler/runtime. A Zap implementation becomes **B1** only when it produces the same canonical token stream, AST, diagnostics, and accepted/rejected decisions for the owned fixture set.

## Bootstrap stages

| Stage | Implementation | Required evidence |
|---|---|---|
| B0 | Current native Rust parser/type checker/evaluator | Existing conformance and corpus tests remain green |
| B1 | Zap lexer/parser and AST producer | Token/AST JSON matches B0 fixtures |
| B2 | Zap type checker | Typecheck decisions and diagnostic JSON match B0 |
| B3 | Zap pure standard library and typed IR bridge | Compiler can run without network or ambient environment |
| B4 | Self-rebuild | B3 can build the compiler sources and reproduce its own artifacts |

## Canonical artifacts

Every bootstrap compiler must expose deterministic artifacts for the same UTF-8 source:

1. `tokens.json` — token kind, normalized value, and source span.
2. `ast.json` — AST node kind, fields, and source span.
3. `diagnostics.json` — severity, stable code, message, location, notes, and help.
4. `typecheck.json` — accept/reject decision and inferred/declared type information.
5. `manifest.json` — compiler version, language-spec version, stdlib version, and schema versions.

Artifacts must not contain memory addresses, timestamps, random identifiers, host paths, or environment-dependent ordering. Map/object serialization order is lexical or specification-defined.

## Ownership boundaries

The lexer owns UTF-8 tokenization and source spans. The parser owns AST construction and precedence. The type checker owns static type decisions. The evaluator owns runtime behavior. The standard library owns pure text, collection, math, JSON, option, and result helpers. Filesystem, process, network, and Web operations belong to an explicit host capability boundary and are not part of the pure bootstrap compiler.

## Differential test rule

For every fixture, B0 and the candidate bootstrap compiler must be invoked with the same source bytes and the same schema version. A fixture passes only when the canonical JSON artifacts match after permitted normalization. A mismatch is a compiler bug or an explicitly recorded compatibility decision; it must not be hidden by changing the fixture without a specification update.

## Reproducibility rule

A bootstrap command must work from a clean checkout with no network access and no ambient project-specific environment variables. The command must record the input source hash, compiler hash, schema versions, and output artifact hashes. B4 is not complete until two consecutive clean runs produce identical artifact hashes.

## Initial owned fixtures

The first fixture set lives under `bootstrap/fixtures/`:

- `lexer/hello.zp` — names, text, numbers, and `say`.
- `parser/precedence.zp` — grouping, arithmetic, comparison, and boolean precedence.
- `typecheck/list_number.zp` — a valid typed collection.
- `typecheck/type_error.zp` — a deterministic rejected annotation.
- `stdlib/pure_values.zp` — pure collection/text helpers without host access.

The fixture set must grow with each migrated syntax or standard-library feature. Existing `conformance/` and `corpus/` suites remain the broader compatibility source.

## Acceptance gate

B1 work may be merged only when the bootstrap validator passes, the existing native suite passes on the pinned toolchain, the English/Burmese contract pair is synchronized, and the change records the stage, schema, fixture owners, and known limitations.
