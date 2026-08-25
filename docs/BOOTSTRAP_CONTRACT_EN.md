# Zap Bootstrap and Self-Hosting Contract

**Status:** B0 reference baseline for published Zap v2.11.16

Zap’s self-hosting roadmap is staged. The current release remains a **Rust reference/native implementation**; it is not yet a fully Zap-only compiler. The normative stage contract, independent version identities, and machine-readable ownership records are maintained under [`bootstrap/contracts`](../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md).

## Current B0 boundary

The reference pipeline is:

```text
Zap source -> Rust lexer -> AST parser -> evaluator/runtime
```

Rust/Cargo is therefore still required to build the current compiler. The operating-system loader and explicitly documented platform boundary are accepted as infrastructure boundaries, while other language runtimes and frameworks are not required by the current Zap compiler path.

## B1 candidate status

A first Zap-owned lexer candidate is now checked in at [`bootstrap/b1/lexer.zp`](../bootstrap/b1/lexer.zp). It covers the current identifier, number, text, comment, whitespace, operator, delimiter, Unicode, and fail-closed diagnostic paths needed by the initial owned corpus. [`scripts/bootstrap/verify_b1_lexer.sh`](../scripts/bootstrap/verify_b1_lexer.sh) executes the candidate and compares its output with the B0 token/diagnostic artifacts.

This is a **corpus-limited B1 foundation**, not a completed B1 compiler. The candidate is not yet the reference owner, does not replace the Rust lexer, and must expand through differential fixtures before the repository can advance the bootstrap stage claim.

The native reference parser now has an additive differential corpus at [`bootstrap/fixtures/parser/compound.zp`](../bootstrap/fixtures/parser/compound.zp), with a canonical AST snapshot and a syntax-rejection diagnostic fixture. [`scripts/bootstrap/verify_b1_parser.sh`](../scripts/bootstrap/verify_b1_parser.sh) checks byte-for-byte reproducibility and reference output. This is parser contract evidence only; no full Zap-owned parser is claimed.

A first Zap-written parser candidate is now checked in at [`bootstrap/b1/parser.zp`](../bootstrap/b1/parser.zp). It intentionally owns the arithmetic declaration fixture and the compound corpus covering maps, lists, postfix indexing, binary operators, a function with a conditional/returns, and a call, plus one missing-bracket rejection path. [`scripts/bootstrap/verify_b1_parser_candidate.sh`](../scripts/bootstrap/verify_b1_parser_candidate.sh) compares these outputs byte-for-byte with B0 artifacts. This is a **provisional, corpus-limited candidate**: it does not replace the Rust parser, still contains fixture-scoped parsing assumptions, and does not advance the repository beyond B0.

## B2 conformance foundation

The repository now has a reference-only B2 conformance gate at [`scripts/bootstrap/verify_b2_typecheck.sh`](../scripts/bootstrap/verify_b2_typecheck.sh). It compares the annotated typed-IR artifact byte-for-byte across repeated native runs and checks native type-check acceptance for annotated and conditional expressions plus rejection for incompatible annotation, function-call, collection-element, and bounded map-element cases. The typed-IR artifact remains explicitly `reference_only`; this is a conformance foundation, not a Zap-owned type checker.

A first provisional Zap-owned type-checker candidate is now recorded at [`bootstrap/b2/typecheck.zp`](../bootstrap/b2/typecheck.zp), with [`scripts/bootstrap/verify_b2_typecheck_candidate.sh`](../scripts/bootstrap/verify_b2_typecheck_candidate.sh) enforcing deterministic acceptance for annotated and conditional fixtures, one annotated function with a return, stable incompatible-number, incompatible-call, and negative collection-element diagnostics, a bounded nested-list index inference slice, a bounded map-element index inference slice, a bounded branch-local option-narrowing slice, and a bounded loop-body narrowing/loop-boundary restoration slice, and a bounded direct `is_option_none` else-body narrowing slice, plus bounded direct bool-literal, none-literal, exact map-literal, and exact collection-expression annotation slices. The nested slice accepts `list<list<number>>` indexing and rejects assigning its numeric result to `text` through paired fixtures. The map slice is limited to a tracked `map<text,number>` variable indexed by a text literal, with a paired rejection when its numeric result is assigned to `text`. The branch slice is limited to a tracked `option<number>` variable narrowed by a direct `is_some` guard within one indented `if` body, with a paired rejection when the numeric payload is assigned to `text`. The loop slice is limited to a tracked `option<number>` variable narrowed by a direct `is_some` guard within one indented `while` body, with a paired positive body use and a post-loop rejection proving that the original option wrapper is restored. The else-body slice is limited to a tracked `option<number>` variable under a direct `if is_option_none(name): ... else:` shape: the true body retains the wrapper and one indented else body receives the numeric payload, with a paired rejection when that payload is assigned to `text`. The bool slice is limited to direct `true`/`false` literals assigned to a `bool` annotation, with a paired rejection for a direct numeric literal assigned to `bool`. The none slice is limited to a direct `none` literal assigned to a `none` annotation, with a paired rejection for a direct boolean literal assigned to `none`. The list-literal slice is limited to the exact `[1, 2]` literal inferred as `list<number>`, with a paired rejection when the same direct literal is assigned to `text`. The map-literal slice is limited to the exact `{"score": 7}` literal inferred as `map<text,number>`, with a paired rejection when the same direct literal is assigned to `text`. The option-constructor slice is limited to the exact `some(1)` expression inferred as `option<number>`, with a paired rejection when the same direct expression is assigned to `text`. It does not claim general option/result constructor inference. The A2 expression matrix additionally covers exact `1 + 2` as `number`, exact `"a" + "b"` as `text`, exact `1 < 2` as `bool`, exact `true and false` as `bool`, exact `ok(1)` as `result<number>`, exact `[1 + 1, 2]` as `list<number>`, and exact `{"score": 1 + 2}` as `map<text,number>`, with Rust-confirmed incompatible pairs where the reference rejects them. The comparison expression remains compatibility-accepted in broader annotations and is therefore not given an invented negative case. A matching candidate-only typed-IR producer is recorded at [`bootstrap/b2/typed_ir.zp`](../bootstrap/b2/typed_ir.zp), with [`scripts/bootstrap/verify_b2_typed_ir_candidate.sh`](../scripts/bootstrap/verify_b2_typed_ir_candidate.sh) comparing its owned node fields with the native reference artifact. Both are intentionally corpus-limited: they do not implement general expression inference, generic/variant narrowing, complete function checking, or complete diagnostic parity. Native Rust remains the reference owner and the bootstrap stage remains B0.

## B3 foundation status

The repository now has a reference-only B3 foundation gate at [`scripts/bootstrap/verify_b3_foundations.sh`](../scripts/bootstrap/verify_b3_foundations.sh). It validates the catalog determinism taxonomy, generates a canonical dependency-free manifest lockfile, checks lockfile reproducibility, runs an offline locked build, and executes a Zap test fixture. These checks demonstrate existing package/build/test-runner behavior; they do not claim that the compiler pipeline is already Zap-owned.

## Reference VM and platform-seed status

The first isolated bytecode VM foundation is implemented in [`native/src/bytecode.rs`](../native/src/bytecode.rs). `zap bootstrap vm-demo` executes a bounded arithmetic program and emits the canonical [`bootstrap/fixtures/bytecode/vm_demo.json`](../bootstrap/fixtures/bytecode/vm_demo.json) artifact. The VM rejects unsupported schema versions, malformed stack shapes, missing halts, arithmetic failures, and step-budget exhaustion without panicking.

The platform boundary remains documented rather than self-hosted. The compiler core has no network or process capability; console, bounded file access, memory, and optional clock behavior remain explicit seed responsibilities. [`scripts/bootstrap/verify_vm_platform.sh`](../scripts/bootstrap/verify_vm_platform.sh) checks this boundary and the deterministic VM smoke artifact.

## Canonical inspection commands

The native CLI now exposes read-only B0 inspection commands:

```text
zap bootstrap status
zap bootstrap tokens <file.zp>
zap bootstrap ast <file.zp>
zap bootstrap typed-ir <file.zp>
zap bootstrap diagnostics <file.zp>
```

The first batch freezes representative token, AST, reference-only typed-IR, diagnostic, metadata, platform-boundary, and standard-library fixtures under [`bootstrap/fixtures`](../bootstrap/fixtures). Run [`scripts/bootstrap/verify_b0_artifacts.sh`](../scripts/bootstrap/verify_b0_artifacts.sh) to rebuild those artifacts and compare them byte-for-byte with the committed corpus.

## Stage policy

| Stage | Meaning | Allowed release claim |
|---|---|---|
| B0 | Rust owns reference behavior and fixtures | Rust reference/native implementation |
| B1 | Zap lexer/parser reproduces B0 artifacts | Zap bootstrap compiler foundation |
| B2 | Zap diagnostics/type checker reproduces B0 acceptance and rejection | Zap bootstrap compiler foundation |
| B3 | Zap stdlib, typed IR, package resolver, and test runner operate offline and deterministically | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler rebuilds itself from the documented platform seed | Fully Zap-only self-hosted compiler |

No release may use the B4 wording before the B4 bootstrap checks pass. Future semantic or artifact changes require bilingual contract updates, fixture changes, ownership records, compatibility decisions where applicable, and regression evidence.

## Next gate

The active implementation gate remains staged parity expansion: the Zap-owned lexer and parser candidates must expand their owned corpora and compare output with the Rust reference for valid, Unicode, malformed, overflow, and determinism cases. The B2 typed-IR/type-check conformance foundation, the first provisional Zap-owned checker candidate, and its candidate-only typed-IR producer are now enforced, but complete type checking and typed IR remain native-owned. The v2.11.7 increment added one provisional list-element inference path and one negative collection-element diagnostic fixture; the v2.11.8 increment added a bounded map-element path with paired valid and incompatible fixtures; the v2.11.9 increment added a bounded direct-`is_some` branch-local narrowing path with paired valid and incompatible fixtures, without changing ownership; the v2.11.11 increment added direct-`is_some` narrowing inside one indented `while` body and verified post-loop wrapper restoration with a paired incompatible fixture; the v2.11.12 tag attempt added direct `is_option_none` else-body narrowing for one tracked `option<number>` variable with paired valid and incompatible fixtures, but its macOS ARM64 release workflow failed before publication; the published v2.11.13 corrective release carries that evidence together with the validated cross-platform test-harness fix; the published v2.11.14 release adds bounded direct bool-literal, none-literal, and exact `[1, 2]` list-literal annotation cases with paired acceptance and rejection fixtures; the published v2.11.15 release adds a bounded exact `{"score": 7}` map-literal annotation case with paired acceptance and rejection fixtures; the published v2.11.16 release adds a bounded exact direct `some(1)` option-constructor annotation case with paired acceptance and rejection fixtures; the v2.11.17 preparation records the A2 exact-expression matrix, including exact list/map arithmetic expressions, as a separately evidenced B0-safe increment; the current A3 checkpoint adds a Rust-backed generic `identity<T>`/`same<T>` declaration slice with AST metadata, inferred identity calls, multiple-parameter substitution, structural `option<T>` and `result<T>` wrapper substitution, generic arity diagnostics, conflict diagnostics, and runtime substitution checks. Broader collection inference, arbitrary nested expressions, nested maps, compound guards, loop mutation, reassignment invalidation, aliases, generic constraints, explicit generic call arguments, generic classes and aliases, and complete user-defined generic declaration support remain future work behind separate evidence and design gates. VM work and native-backend work must remain behind those gates rather than being claimed prematurely.
