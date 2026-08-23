# Zap Language Specification

**Purpose:** Canonical normative owner for Zap syntax, typing, runtime behavior, diagnostics, compatibility, and version decisions.
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Learning guide](LEARN_ZAP_EN.md) · [Syntax reference](SYNTAX_GUIDE_EN.md) · [Stdlib reference](STDLIB_INDEX_EN.md) · [Package author guide](PACKAGE_EN.md) · [Runtime state](RUNTIME_STATE_EN.md) · [Deployment boundaries](DEPLOYMENT_EN.md)

**Specification status:** Normative foundation for Zap v2.2.7

This document is the canonical index for language semantics. When an older guide conflicts with this document, the implementation and tests must be brought into alignment with this specification; a compatibility exception must be recorded explicitly rather than inferred from legacy behavior.

## 1. Source and execution model

A Zap program is a UTF-8 source file with the `.zp` extension. The canonical native pipeline is **source → lexer → AST parser → evaluator**. The evaluator executes the parsed AST directly; function and method bodies are not reconstructed from source lines. A source file may be run directly with `zap <file.zp>` or explicitly with `zap run <file.zp>`.

Indentation delimits blocks. Statements include declarations, assignment, expression statements, `say`, `return`, conditionals, loops, functions, classes, and explicit module/import forms. The parser must reject malformed structure with a structured diagnostic rather than panicking.

## 2. Expressions and precedence

Operators bind from strongest to weakest in the following order:

| Level | Operators | Associativity |
|---|---|---|
| 1 | grouping `(...)`, calls, indexing, member access | left-to-right evaluation of the written chain |
| 2 | unary `-`, `not` | right-to-left |
| 3 | exponentiation and multiplicative arithmetic | left-to-right within each level |
| 4 | additive arithmetic and concatenation | left-to-right |
| 5 | comparisons: `<`, `<=`, `>`, `>=`, `==`, `!=` | left-to-right |
| 6 | `and` | left-to-right with short-circuiting |
| 7 | `or` | left-to-right with short-circuiting |

Parentheses are the normative escape hatch for ambiguous intent. Boolean operators short-circuit and must not evaluate an unreachable right-hand operand.

## 3. Values and typing

The core value categories are `text`, `number`, `bool`, `list`, `map`, `object`, `function`, `none`, and typed `result`/`option` forms where exposed by the runtime. Annotations may use primitive names and bounded generic forms such as `list<number>`, `map<text, number>`, `option<text>`, and `result<text>`. The `function` annotation accepts first-class callable values. `any` is an explicit escape hatch and does not imply runtime coercion.

Static checks validate declared annotations, collection element expectations, function arguments, return values, and control-flow narrowing where the implementation has enough information. Runtime checks remain authoritative at dynamic boundaries. A mismatch is reported through the structured diagnostic contract, not as an undocumented Rust panic or unstable string.

## 4. Functions, calls, and closures

A function has a name, ordered parameters, optional annotations, optional default expressions, an optional return annotation, and an AST body. A declared function name evaluates to a first-class callable value; that value can be assigned, passed as an argument, returned, and invoked through any callable expression. Arguments may be positional or named according to the call contract. Defaults are evaluated when the corresponding argument is omitted. Duplicate, unknown, multiply supplied, or non-callable invocations are errors, and arity/type failures use deterministic runtime messages. Closures capture the lexical environment defined by the implementation; ownership and cycle behavior must follow the memory contract. Callable values display as `<callable>` and serialize to the deterministic `{"__zap_variant":"callable"}` marker; the marker is intentionally not deserializable because it does not carry executable code.

## 5. Control flow and modules

`if`/`else`, `while`, and `for` execute in source order. A loop condition is evaluated before every iteration. `return` exits the current function only. Explicit `module` and `import` declarations resolve relative, bounded paths in deterministic source order. Absolute paths, traversal components, malformed entries, and circular module graphs are rejected with stable diagnostics. Legacy `use` imports remain compatibility syntax where documented.

## 6. Runtime ownership and asynchronous boundaries

Object fields use the documented single-threaded `Rc<RefCell>` ownership model. Cyclic object graphs require an explicit cycle-breaking operation before the owning graph is discarded. The runtime is not thread-safe by default; this boundary is intentional.

The current async executor is deterministic and poll-budgeted. Language `async fn` calls schedule their completed values through the caller's `RuntimeState` and return a context-owned `ScheduledFuture`; `await` and `task_join` drive the executor before consuming the result, while `task_is_ready` observes readiness without polling. The runtime also provides joinable tasks, cancellation-aware joins, timeout propagation, and typed task failures. It is not a production I/O reactor. Blocking calls, socket readiness, worker scheduling, shutdown, and forced cancellation of foreign blocking work require the separate production boundary contract in `ASYNC_BOUNDARIES_EN.md`.

## 7. Diagnostics and compatibility

Every user-facing diagnostic must preserve severity, stable code, message, source location where available, notes, and help. CLI and LSP consumers share the same semantic diagnostic fields. Compatibility behavior must be labeled as one of: **normative**, **compatibility**, **deprecated**, or **rejected**. A behavior cannot become normative solely because an old fixture happens to accept it.

The current release line is v2.2.7. A semantics change requires a specification update, bilingual documentation parity, conformance tests, changelog entry, and an explicit version decision. Release artifacts must continue to pass the pinned Rust toolchain, formatting, strict Clippy, native tests, provenance, and signature gates. Future changes must use the bilingual [`COMPATIBILITY_CHANGE_TEMPLATE_EN.md`](COMPATIBILITY_CHANGE_TEMPLATE_EN.md) and [`COMPATIBILITY_CHANGE_TEMPLATE_MM.md`](COMPATIBILITY_CHANGE_TEMPLATE_MM.md) records.

## 8. Conformance ownership

The parser owns syntax and AST construction. The evaluator owns runtime expression and statement behavior. The diagnostics module owns the stable error contract. The registry module owns package transport, authentication, checksums, signatures, and cache policy. CI owns enforcement of the repository's declared gates. No subsystem may silently redefine another subsystem's contract.

## Specification ownership index

The machine-readable rule-to-section-to-fixture map is [`SPEC_OWNERSHIP_INDEX.tsv`](SPEC_OWNERSHIP_INDEX.tsv), with the field and migration contract documented in [`SPEC_OWNERSHIP_EN.md`](SPEC_OWNERSHIP_EN.md) and [`SPEC_OWNERSHIP_MM.md`](SPEC_OWNERSHIP_MM.md). CI validates that every indexed English section, Burmese section, and fixture owner exists; the validator also enforces unique rule IDs and required domain coverage.

## Related normative contracts

The following documents provide detailed subcontracts and must remain bilingual with this specification:

| Contract | English | Burmese |
|---|---|---|
| Diagnostics | `DIAGNOSTIC_MODEL_EN.md` | `DIAGNOSTIC_MODEL_MM.md` |
| Memory | `MEMORY_MODEL_EN.md` | `MEMORY_MODEL_MM.md` |
| Async boundary | `ASYNC_BOUNDARIES_EN.md` | `ASYNC_BOUNDARIES_MM.md` |
| Syntax reference | `SYNTAX_GUIDE_EN.md` | `SYNTAX_GUIDE_MM.md` |
| Standard library | `STDLIB_TEXT_MATH_COLLECTION_EN.md` | `STDLIB_TEXT_MATH_COLLECTION_MM.md` |

**Current limitation:** This is the canonical semantic foundation and navigation point. The expanded ownership index now covers 36 stable rules, including post-review LSP, standard-library determinism, memory-budget, registry-transport, benchmark-provenance, and release-version contracts. Remaining work is to migrate every other fragmented rule into this document or an explicitly linked normative subcontract, add parser/evaluator conformance fixtures for each rule, and record version ownership for unresolved legacy behavior.
