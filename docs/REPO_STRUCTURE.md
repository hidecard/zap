# Zap Repository Ownership Structure

**Date:** 2026-09-17
**Purpose:** Define clear ownership boundaries so contributors instantly know which code belongs to compiler, runtime, bootstrap, stdlib, tools, or ecosystem.

---

## Proposed Roadmap Structure

```text
zap/
│
├── compiler/          ← Lexer, Parser, AST, Typechecker, IR, Lowering, Codegen
│   ├── lexer/
│   ├── parser/
│   ├── ast/
│   ├── typechecker/
│   ├── ir/
│   ├── lowering/
│   └── codegen/
│
├── runtime/           ← VM, Memory management, Async runtime, Native stdlib bindings
│   ├── vm/
│   ├── memory/
│   ├── async/
│   └── stdlib/
│
├── packages/          ← Package manager, resolver, lockfile, publish logic
│
├── web/               ← Web framework, routing, middleware, serving
│
├── tools/             ← CLI, LSP, Formatter, Linter, Debugger
│   ├── lsp/
│   ├── formatter/
│   ├── linter/
│   └── debugger/
│
├── database/          ← SQLite integration, ORM, migration engine
│
├── tests/
├── examples/
├── docs/
└── bootstrap/         ← B0–B4 verification contracts, fixtures, evidence
```

---

## Current Code → Ownership Mapping

The repository currently organizes implementation code under `native/` (Rust) and `host/` (Python proofs). The mapping below shows where each ownership area lives today and where it maps in the roadmap structure.

### compiler/ (Language Frontend)

| Roadmap Area | Current Location | Description |
|---|---|---|
| compiler/lexer | `native/src/lexer.rs` | Tokenizer: source → tokens |
| compiler/parser | `native/src/parser.rs` | Tokens → AST |
| compiler/ast | `native/src/ast.rs` | AST node definitions |
| compiler/typechecker | `native/src/evaluator.rs` (partial) | Type checking, inference, flow analysis |
| compiler/ir | `native/src/bytecode.rs` | Typed IR / bytecode |
| compiler/lowering | `native/src/evaluator.rs` (partial) | AST → typed IR lowering |
| compiler/codegen | `native/src/bytecode.rs` (partial) | IR → native codegen |
| compiler/lexical diagnostics | `native/src/diagnostics.rs` | Diagnostic model |
| **host/zap-lexer-host/** | `host/zap-lexer-host/lexer.py` | Python host proof for lexer (B1 no-Rust evidence) |
| **host/zap-parser-host/** | `host/zap-parser-host/parser.py` | Python host proof for parser (B1 no-Rust evidence) |

### runtime/ (Language Backend)

| Roadmap Area | Current Location | Description |
|---|---|---|
| runtime/vm | `native/src/evaluator.rs` | Bytecode interpreter, value model |
| runtime/memory | `native/src/value.rs` | Memory management, ownership, GC |
| runtime/async | `native/src/async_runtime.rs` | Task scheduler, async boundaries |
| runtime/stdlib | `native/src/stdlib*.rs` | Standard library implementations |
| runtime/state | `native/src/runtime_state.rs` | Runtime state management |
| runtime/database | `native/src/database.rs` | SQLite and DB interfaces |
| host/zap-vm-host/ | `host/zap-vm-host/` | Python host proof for VM |

### tools/ (Developer Tools)

| Roadmap Area | Current Location | Description |
|---|---|---|
| tools/cli | `native/src/cli.rs` | CLI command dispatcher |
| tools/lsp | `native/src/lsp.rs` | Language Server Protocol |
| tools/formatter | `native/src/diagnostics.rs` (partial) + scripts | Source formatter |
| tools/linter | `native/src/diagnostics.rs` (partial) + scripts | Lint rules |
| tools/debugger | `native/src/evaluator.rs` (debug hooks) | Debugger infrastructure |
| vscode-extension/ | `vscode-extension/` | VS Code extension |

### bootstrap/ (Verification Infrastructure)

| Area | Location |
|---|---|
| bootstrap/contracts/ | `bootstrap/contracts/` — B4 contract, ownership, rule index |
| bootstrap/b1/ | `bootstrap/b1/` — B1 lexer/parser evidence |
| bootstrap/b2/ | `bootstrap/b2/` — B2 typed-IR evidence |
| bootstrap/b3/ | `bootstrap/b3/` — B3 package evidence |
| bootstrap/b4/ | `bootstrap/b4/` — B4 self-hosting evidence |
| bootstrap/fixtures/ | `bootstrap/fixtures/` — test fixtures (lexer, parser, stdlib) |
| bootstrap/evidence/ | `bootstrap/evidence/` — certification evidence |
| bootstrap/docs/ | `bootstrap/docs/` — bootstrap design docs |

### database/

| Area | Location |
|---|---|
| database/ | `native/src/database.rs` + `frameworks/web/database/` — SQLite, ORM, migrations |

### packages/

| Area | Location |
|---|---|
| packages/ | `native/src/project.rs` + `native/src/registry.rs` — package management logic |

### web/

| Area | Location |
|---|---|
| web/ | `frameworks/web/` — Web framework, routing, serving |
| frameworks/ai/ | `frameworks/ai/` — ⚠️ DO NOT DEVELOP until self-hosting complete |
| frameworks/mobile/ | `frameworks/mobile/` — ⚠️ DO NOT DEVELOP until self-hosting complete |
| frameworks/iot/ | `frameworks/iot/` — ⚠️ DO NOT DEVELOP until self-hosting complete |

### tests/

| Area | Location |
|---|---|
| tests/ | `tests/` — Zap-level smoke tests |
| native/tests/ | `native/tests/` — Rust unit/integration tests |
| corpus/ | `corpus/` — property/fuzz corpus |
| conformance/ | `conformance/` — conformance test suite |

### examples/

| Area | Location |
|---|---|
| examples/ | `examples/` — example programs (hello, collections, error_handling, etc.) |

### docs/

| Area | Location |
|---|---|
| docs/ | `docs/` — all documentation (EN/MM bilingual) |

### scripts/

| Area | Location |
|---|---|
| scripts/ | `scripts/` — build, test, verify, release automation |

---

## Ownership Rules

1. **Compiler code** (`compiler/`) owns language syntax, types, and IR. It must NOT depend on runtime implementation details.
2. **Runtime code** (`runtime/`) owns execution, memory, and async. It must NOT depend on compiler internals.
3. **Bootstrap code** (`bootstrap/`) is verification-only. It must NOT contain production implementation.
4. **Tools code** (`tools/`) depends on compiler and runtime APIs only, not on each other.
5. **Standard library** (`runtime/stdlib/`) is the bridge between runtime and compiler — it must be platform-safe and documented.
6. **Packages/Registry** (`packages/`) depends on runtime and filesystem, not on compiler internals.
7. **Frameworks** (`web/`, `database/`) depend on runtime + stdlib, not on compiler internals.
8. **No circular dependencies** between ownership boundaries.

---

## Status

| Area | Status | Notes |
|---|---|---|
| compiler/ | **Implementation exists** in `native/src/` | Physical reorganization pending; ownership documented here |
| runtime/ | **Implementation exists** in `native/src/` | Physical reorganization pending; ownership documented here |
| tools/ | **Implementation exists** in `native/src/` + `vscode-extension/` | Physical reorganization pending; ownership documented here |
| bootstrap/ | **Complete** | `bootstrap/` directory already exists |
| database/ | **Implementation exists** in `native/src/database.rs` | Physical reorganization pending |
| packages/ | **Implementation exists** in `native/src/project.rs` + `registry.rs` | Physical reorganization pending |
| web/ | **Implementation exists** in `frameworks/web/` | Physical reorganization pending |
| tests/ | **Complete** | `tests/` directory exists |
| examples/ | **Complete** | `examples/` directory exists |
| docs/ | **Complete** | `docs/` directory exists |
