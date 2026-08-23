# P0 AST Foundation Status

**Verified against Zap v2.2.7.**

## AST migration status

Zap has a source-span-aware AST foundation in `native/src/ast.rs`. The parser accepts expressions with precedence, calls, indexing, conditional expressions, and `await`, together with assignments, typed `let` declarations, `say`, imports, returns, `break`, `continue`, `if/else`, `while`, `for`, `try/catch`, `raise`, modules, functions, classes, and exported bindings/functions.

| Declaration | Supported AST shape | Notes |
|---|---|---|
| `fn add(a: number) -> number:` | `Stmt::Function` | Stores the name, parameters, optional annotations, return annotation, body, visibility, async flag, and export flag. |
| `class Child(Parent):` | `Stmt::Class` | Stores the class name, optional single parent name, and indented body. |
| `let total: number = 1` | `Stmt::Declaration` | Stores the variable name, optional annotation, value, export flag, and source span. |
| `say value` | `Stmt::Say` | Stores the output expression for native AST execution. |
| `import` / `use` | `Stmt::Import` | Stores the module path, alias, and explicit-import mode. |
| `if` / `while` / `for` / `try` | Control-flow statement nodes | Uses the same indentation-aware block parser and native flow propagation. |

The AST parser enforces four-space indentation, rejects tabs and mixed indentation, requires an indented body after every declaration or control-flow header, and preserves one-based source locations. `run()` now parses every normal source program through the AST boundary and returns a syntax diagnostic on parse failure; it no longer falls back to the line interpreter. Local module files are also parsed and executed through the AST boundary, including export markers used by explicit imports.

## Compatibility-only legacy boundary

The line interpreter remains an internal compatibility boundary for `Function` records created by older or test-only paths that contain `body: Vec<String>` without an `ast_body: Program`. New source programs and newly parsed functions must not depend on this representation. No new syntax is added to the line interpreter. Compatibility behavior is retained for the current release line and may be removed only in a separately documented breaking release after legacy fixtures and migration guidance are reviewed.

> **Policy:** Native AST execution is normative for parser-owned source. Line-based execution is compatibility-only and is not a normal-program fallback.

## Runtime safety semantics

Zap integer arithmetic uses checked operations. Addition, subtraction, and multiplication report `OverflowError` when the signed integer range is exceeded. Division and modulo by zero report a runtime error rather than panicking. The `i64::MIN / -1` and `i64::MIN % -1` cases are also rejected as integer overflow.

Sequence indexing is zero-based. A negative numeric index is invalid and produces `index out of range`. An index equal to or greater than the sequence length produces the same error. Map indexing uses a text key and reports `key not found` when the key is absent.

## Acceptance status

The native suite currently passes **232 unit tests** and **256 core integration tests**. The AST boundary and native AST execution are active for all parser-owned programs, including function, method, export, and local-module bodies. Resource limits, typed diagnostics, and legacy source-line compatibility remain available only for legacy-created runtime functions and explicit compatibility tests.

## P0 completion boundary

The canonical AST execution slice is complete for normal programs and local modules. Runtime functions and class methods created by the AST parser retain their `Program` body directly and execute without source reconstruction. Their lexical closures retain parent-linked live binding cells, so an outer reassignment remains visible to a returned closure, sibling closures share the same captured binding, inner `let` declarations shadow without mutating the outer binding, and recursive calls preserve their own call frames. The remaining line-based representation is intentionally retained as a compatibility format for older/internal declarations. Its removal is a future breaking compatibility decision, not an implicit behavior change.
