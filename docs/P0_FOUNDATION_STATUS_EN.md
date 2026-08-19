# Zap P0 Foundation Status

## AST migration status

Zap now has a source-span-aware AST foundation in `native/src/ast.rs`. The parser accepts expressions with precedence, calls, and indexing, together with assignments, returns, `break`, `continue`, `if/else`, `while`, and `for` blocks. Function and class declarations are now represented as first-class AST nodes.

| Declaration | Supported AST shape | Notes |
|---|---|---|
| `fn add(a: number) -> number:` | `Stmt::Function` | Stores the name, parameter names, optional parameter annotations, optional return annotation, and indented body. |
| `class Child(Parent):` | `Stmt::Class` | Stores the class name, optional single parent name, and indented body. |
| `if` / `while` / `for` | Control-flow statement nodes | Uses the same indentation-aware block parser. |

The AST parser enforces four-space indentation, rejects tabs and mixed indentation, requires an indented body after every declaration or control-flow header, and preserves one-based source locations. The legacy evaluator remains available for compatibility while the evaluator migration proceeds incrementally.

## Runtime safety semantics

Zap integer arithmetic uses checked operations. Addition, subtraction, and multiplication report `OverflowError` when the signed integer range is exceeded. Division and modulo by zero report a runtime error rather than panicking. The `i64::MIN / -1` and `i64::MIN % -1` cases are also rejected as integer overflow.

Sequence indexing is zero-based. A negative numeric index is not treated as Python-style reverse indexing; it is invalid and produces `index out of range`. An index equal to or greater than the sequence length produces the same error. Map indexing uses a text key and reports `key not found` when the key is absent.

## Acceptance status

The native suite currently passes all AST unit tests and integration tests after the declaration-node extension. The P0 work still includes the incremental replacement of line-based evaluation with AST evaluation, while the parser and safety boundaries are now ready for that migration.

## Next implementation boundary

The next safe milestone is to lower `Stmt::Function` and `Stmt::Class` into evaluator declarations without changing existing `.zp` behavior. That migration must preserve resource limits, typed diagnostics, module path restrictions, and cross-platform file handling.
