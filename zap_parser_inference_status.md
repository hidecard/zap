# Zap parser, inference, and canonical-schema status

Repository: https://github.com/hidecard/zap

This report describes the current local working tree. The Rust implementation remains the differential reference. No commit or GitHub push has been made because complete B2/B3 ownership and full arbitrary-language parity are still unfinished.

## B1 parser differential status

The production `parse(source, source_name)` entry now delegates directly to `parse_general(source, source_name)`. The body of `parse()` contains no fixture/source-shape `len(lines)` or `contains(source, ...)` routing. The general route uses recursive indentation-aware top-level sequence dispatch for declarations, functions, classes, control-flow blocks, and mixed siblings. It also includes safe source-layout rebasing for nested statements and recursive class-body extraction.

The valid Rust-reference differential corpus currently passes **29/29 exact JSON AST comparisons** using `scripts/bootstrap/verify_b1_general_parser_batch.py`. This includes arithmetic, declarations, assignments, nested expressions, functions, classes, loops, nested blocks, mixed top-level sequences, nested functions, nested class methods, and source-span-sensitive fixtures. The runner starts one native process per fixture to avoid the repository's separate single-process multi-output framing/state limitation.

The invalid diagnostic corpus currently passes **10/10 exact JSON comparisons** using `scripts/bootstrap/verify_b1_diagnostics_batch.py`:

| Category | Fixtures | Result |
|---|---:|---:|
| Lexer diagnostics | integer overflow, invalid character, unterminated string | 3/3 |
| Delimiter/header diagnostics | missing closing bracket, unexpected closing bracket, missing function parenthesis | 3/3 |
| Syntax/indentation diagnostics | missing assignment, invalid indentation, unexpected indentation | 3/3 |
| Numeric-literal syntax diagnostic | decimal literal fixture | 1/1 |
| **Total** | **10** | **10/10** |

The B1 acceptance gates for the general parser, multiple class methods, unified `elif`/`try-catch`, token expressions, and the parser candidate all pass. Token expression support covers precedence, unary operators, grouping, list/map literals, nested calls, index suffixes, member suffixes, and named arguments. Top-level continuation headers such as `else`, `elif`, and `catch` are preserved within the owning block chunk instead of being misclassified as sibling statements.

The remaining B1 limitation is breadth rather than the verified corpus: a larger arbitrary valid/invalid corpus is still required to establish complete parity for every Rust AST expression, statement, span, recovery, and multi-diagnostic ordering case.

## B2 generic constraints and alias checking

B2 generic call inference now validates both the constraint declaration and the concrete arguments. A constraint must reference a declared type parameter, and its bound must be a valid base or nested container type. Generic instantiation rejects malformed constraints before substitution and continues to enforce arity and bound compatibility.

Generic alias metadata now validates the alias body recursively. Base types, declared parameters, `list`, `option`, `result`, and nested `map` forms are accepted; malformed wrappers and undeclared type symbols are rejected. The metadata exposes `body_valid`, and alias instantiation refuses invalid bodies or incorrect arity.

Existing generic bounds, generic end-to-end, generic type-declaration, type/generic, and all **33 B2 verifier scripts** pass after these changes. This is an extended foundation, not yet full reference type-checker parity. Still outstanding are multiple-parameter constraint inference in all call forms, explicit generic-call syntax, generic classes and aliases across imports, type-alias environments during AST inference, aliasing/mutation invalidation, short-circuit path sensitivity, and complete loop fixpoint/break/continue dataflow.

## B3 canonical AST schema alignment

The canonical Rust AST serializer defines member expressions as `{kind: "member", member, target}`, index expressions as `{kind: "index", index, target}`, maps as `{kind: "map", entries: [{key, value}]}`, and calls as `{kind: "call", callee, args}`. The B3 typed-IR expression producer now emits canonical `target/member` member fields and structured canonical `elements`/`entries` list/map nodes instead of the previous `object/property` and literal-payload forms.

B3 lowering now consumes canonical member fields. A new `lower_ast_program` bridge accepts canonical parser AST blocks for declarations, literal payload statements, `if`, and `while`, and uses the same jump relocation/patching machinery as typed-IR lowering. The dedicated `verify_b3_canonical_ast_schema.sh` gate passes for member, map, and lowering field alignment. The existing typed-IR expression and bytecode-lowering gates also pass.

B3 is not complete. Canonical AST names and general variable storage are not yet fully represented by the current VM instruction/state model; `for`, `try-catch`, class/function emission, full map/index runtime semantics, and complete statement/opcode coverage still require implementation. The typed-IR producer also retains some source-string parsing paths and therefore is not yet a complete AST-owned emitter.

## Regression evidence

The latest regression run passed the 29-fixture B1 valid batch, 10-fixture diagnostic batch, B1 parser candidate gate, all 33 B2 verifier scripts, the new B3 canonical schema gate, the existing B3 typed-IR/bytecode lowering gate, native unit/all-target tests (**272 passed**), native integration tests (**259 passed**), and `git diff --check`.

## Git status and push decision

Origin was fetched and local `master` was rebased onto the latest remote `origin/master` at commit `2de01a5`, including the remote mutable-loop and short-circuit loop-control work. The working tree remains intentionally uncommitted. Because the complete B2 dataflow/type semantics, complete B3 runtime/schema ownership, and broad Rust-reference differential corpus are not finished, the changes are **not yet ready for the requested final commit and GitHub push**.
