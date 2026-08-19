# Zap P0 Foundation Status

## AST migration status

Zap now has a source-span-aware AST foundation in `native/src/ast.rs`. The parser accepts expressions with precedence, calls, and indexing, together with assignments, typed `let` declarations, `say`, imports, returns, `break`, `continue`, `if/else`, `while`, and `for` blocks. Function and class declarations are now represented as first-class AST nodes.

| Declaration | Supported AST shape | Notes |
|---|---|---|
| `fn add(a: number) -> number:` | `Stmt::Function` | Stores the name, parameter names, optional parameter annotations, optional return annotation, and indented body. |
| `class Child(Parent):` | `Stmt::Class` | Stores the class name, optional single parent name, and indented body. |
| `let total: number = 1` | `Stmt::Declaration` | Stores the variable name, optional annotation, value, and source span. |
| `say value` | `Stmt::Say` | Stores the output expression for later AST execution. |
| `import` / `use` | `Stmt::Import` | Stores the module path and explicit-import mode. |
| `if` / `while` / `for` | Control-flow statement nodes | Uses the same indentation-aware block parser. |

The AST parser enforces four-space indentation, rejects tabs and mixed indentation, requires an indented body after every declaration or control-flow header, and preserves one-based source locations. `run()` now parses every parseable program through the AST boundary. AST-compatible statements, functions, classes, and imports now execute through the native AST boundary. Function and method declarations store an optional `ast_body: Program` directly in the runtime `Function` representation; calls execute that body with the existing checked expression, annotation, flow, and loop-limit logic. Legacy source lines remain available only for legacy-created functions and compatibility fallback. This makes AST parsing and native AST execution the primary program path without removing the safety fallback.

## Runtime safety semantics

Zap integer arithmetic uses checked operations. Addition, subtraction, and multiplication report `OverflowError` when the signed integer range is exceeded. Division and modulo by zero report a runtime error rather than panicking. The `i64::MIN / -1` and `i64::MIN % -1` cases are also rejected as integer overflow.

Sequence indexing is zero-based. A negative numeric index is not treated as Python-style reverse indexing; it is invalid and produces `index out of range`. An index equal to or greater than the sequence length produces the same error. Map indexing uses a text key and reports `key not found` when the key is absent.

## Acceptance status

The native suite currently passes **25 unit tests** and **47 integration tests**. The AST boundary and native AST execution are active for all parseable programs, including function and method bodies. Resource limits, typed diagnostics, and legacy source-line compatibility remain available for legacy-created runtime functions and unsupported fallback paths.

## P0 completion boundary

The final P0 AST body migration is complete for declarations created through the AST parser. Runtime functions and class methods now retain their `Program` body directly and execute without source reconstruction. The remaining legacy line-based representation is intentionally retained as a compatibility format for older/internal declarations. Future work may remove that fallback after a separate compatibility and release cycle.
