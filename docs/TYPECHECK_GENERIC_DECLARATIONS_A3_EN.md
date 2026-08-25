# A3 User-Defined Generic Declarations — Design and Acceptance Record

**Status:** Design gate; A3 is not complete and Zap remains B0. Rust remains the reference compiler, type checker, runtime, and diagnostic owner.

## Purpose

This record defines the smallest safe A3 implementation target for user-defined generic declarations. Existing `list<T>`, `map<K, V>`, `option<T>`, and `result<T>` annotations are not generic declarations and do not satisfy this gate. A declaration feature must be implemented in the Rust reference first, then reproduced by the bootstrap candidate through differential evidence.

## Initial syntax decision

The initial declaration form is:

```zap
fn identity<T>(value: T) -> T:
    return value
```

The declaration type-parameter list is placed between the function name and parameter list. A type parameter is an ASCII identifier beginning with an uppercase letter; duplicate parameters, empty lists, malformed brackets, and undeclared type parameters are rejected. The first implementation slice permits type parameters in parameter and return annotations and does not add explicit call-site type arguments.

The first accepted semantics are:

| Case | Required result |
|---|---|
| `identity<number>(1)` by inferred call type | Accept and infer `number` |
| `identity<text>("x")` by inferred call type | Accept and infer `text` |
| `same<T>(left: T, right: T)` with two equal static types | Accept one consistent substitution |
| `same<T>(1, "x")` | Reject with a stable incompatible-substitution diagnostic |
| `identity<T>` returning a value incompatible with the substituted return type | Reject at the return location |

The angle-bracket examples in the call column describe the inferred type, not an explicit call syntax; explicit generic call arguments are deferred until a separate grammar decision.

## Substitution and safety rules

At each call, the checker collects a substitution for every declared type parameter from annotated arguments and verifies that repeated uses agree. A missing or conflicting substitution is an error, not an implicit `any`. Substitution is structural for the supported primitive and wrapper annotations, and it must preserve the existing nested-annotation arity and map-key restrictions. Recursive substitution depth is capped at 32; excessive depth fails closed with a stable type diagnostic. Runtime calls must use the same substitution result as static checking rather than accepting a generic annotation as an unknown concrete type.

## Required evidence for the initial slice

The initial A3 acceptance checkpoint requires Rust parser acceptance and rejection fixtures, canonical AST output that records the declared type parameters, native static-check acceptance and rejection fixtures for numeric/text substitutions, multiple type parameters, structural `option<T>`/`result<T>` wrappers, conflicting substitutions, and generic arity, deterministic repeated runs, runtime return checking after substitution, stable JSON diagnostics, bootstrap candidate differential output, malformed-source no-panic coverage, bilingual documentation, and a provisional ownership record. Parser, static checker, evaluator, typed-IR, LSP, and bootstrap candidate changes must be tested together.

## Current bounded parser-evidence checkpoint

The current provisional checkpoint adds three Rust-reference-backed malformed-header fixtures: `generic_empty_params.zp` rejects `fn empty<>` with `generic type-parameter list cannot be empty`; `generic_duplicate_params.zp` rejects a repeated `T` with `duplicate generic type parameter: T`; and `generic_invalid_param.zp` rejects lowercase `t` with `invalid generic type parameter 't'`. Each produces the stable `ZAP-SYNTAX-001` / `SyntaxError` diagnostic at line 1, column 1, and the candidate differential verifier reproduces these exact corpus cases. The same checkpoint now includes `generic_list_wrapper.zp`, where `list_keep<T>(list<T>) -> list<T>` is accepted and executed by the Rust reference, plus `generic_list_wrapper_incompatible.zp`, where assigning the result to `text` yields the stable `variable 'wrong' expects text, got list<number>` diagnostic at line 4, column 1. This is bounded structural list evidence only; it does not establish arbitrary malformed-generic parsing, complete A3, candidate ownership, B4, or self-hosting. The Rust parser now also has a focused nesting-aware signature splitter for the accepted `map<K, V>` parameter shape: `generic_map_wrapper.zp` accepts and executes `map_keep<K, V>(map<K, V>) -> map<K, V>`, while `generic_map_wrapper_incompatible.zp` produces `variable 'wrong' expects text, got map<text,number>` at line 4, column 1. The candidate mirrors only these exact fixtures. The next bounded checkpoint adds `generic_cross_module_library.zp`, which exports `identity<T>`, and importing main fixtures: `generic_cross_module.zp` is accepted and executed, while `generic_cross_module_incompatible.zp` is rejected at line 3, column 1 with `variable 'wrong' expects text, got number`. Rust project checking now recursively collects exported function signatures from explicitly imported modules and applies the existing generic call substitution checks to the importing main module; the candidate recognizes only this exact imported `identity(…)` corpus shape. Imported function-body checking, namespace collision policy, aliases, non-generic cross-module inference, and full module-wide typed-IR/LSP propagation remain open. Constraint probing is also explicitly deferred: Rust rejects `fn bounded<T: number>(…)` with `invalid generic type parameter 'T: number'`, rejects `fn bounded<T extends number>(…)` with `invalid generic type parameter 'T extends number'`, and rejects the `where` form with `unknown return type annotation 'T where T: number'`; the candidate reproduces these exact three diagnostics only as a deferred corpus record. No trait-bound or constraint semantics are implemented or claimed. The explicit-call probe `identity<number>(1)` is currently accepted by the Rust project-check path but fails at runtime with `undefined variable: number`; this records that explicit generic call syntax is not implemented as a language feature and must not be inferred from the explanatory notation used elsewhere. The candidate mirrors only the static acceptance of this exact deferred fixture.

## Explicitly deferred scope

This design does not claim complete A3. Constraints and trait bounds, generic classes and aliases, explicit generic call arguments, higher-kinded forms, variance, overload resolution, cross-module instantiation, full collection inference, closure capture semantics, and complete typed-IR/LSP generic metadata remain deferred until separately accepted. A3 cannot be marked complete from the `identity<T>` slice alone; the ordered A3 gate remains open until the required declaration, scope, constraint, arity, substitution, recursion, diagnostic, runtime, and cross-platform evidence passes.

## Ownership and release rule

Rust remains authoritative until the A3 gate passes. The bootstrap candidate may implement only a corpus-limited mirror after the reference behavior is frozen. No B1/B2/B3/B4 or self-hosting claim follows from this design record, and no new release is cut merely because this design gate is documented.

**Author:** Manus AI
**Baseline:** Zap v2.11.16
