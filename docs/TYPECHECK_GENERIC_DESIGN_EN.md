# Zap Generic Type Syntax Decision Record

**Decision:** TC-012 generic type syntax is accepted as an implemented baseline for the current v2.2.7 type-checking contract. No new parser or runtime change is required for this design gate.

## Scope

Zap accepts nested generic annotations using angle brackets. The supported forms are `list<T>`, `map<K, V>`, `option<T>`, and `result<T>`, where each type argument must itself be an allowed primitive, wrapper, collection, or `any` annotation.

The existing parser splits nested type arguments deterministically and rejects empty arguments, unbalanced delimiters, unsupported generic bases, and incorrect arity. `map<K, V>` is intentionally limited to two arguments, and its key type is currently restricted to `text` or `any` by the annotation-matching contract.

| Form | Status | Current contract |
|---|---|---|
| `list<T>` | Implemented baseline | Exactly one recursively valid type argument. |
| `map<K, V>` | Implemented baseline | Exactly two recursively valid arguments; key type must be `text` or `any` when matched against a concrete value. |
| `option<T>` | Implemented baseline | Exactly one recursively valid payload type; `option<any>` remains compatible with a concrete option payload. |
| `result<T>` | Implemented baseline | Exactly one recursively valid payload type. |
| User-defined generic declarations | Deferred | No generic class, function, or type-parameter declaration syntax is introduced in v2.2.7. |
| Generic inference from unannotated expressions | Deferred | Inference remains conservative and must not manufacture a generic type from insufficient evidence. |

## Syntax and validation rules

The grammar decision for v2.2.7 is intentionally small:

```text
Type        := Primitive | "list<" Type ">"
             | "map<" Type "," Type ">"
             | "option<" Type ">"
             | "result<" Type ">"
```

Whitespace around nested arguments is accepted after trimming. Generic forms must be closed with a matching `>`. The checker reports an unknown type annotation for malformed or unsupported forms instead of silently widening them to `any`.

> A generic annotation is a type contract, not a request to infer missing information. When the checker cannot establish compatibility, it must reject the program or retain the conservative `any` boundary already defined by the surrounding expression rules.

## Compatibility and rollout

This decision preserves the existing `option<T>` and `result<T>` semantics used by branch narrowing and alias invalidation. It also formalizes the collection forms already exercised by the native test suite. The v2.2.7 release gate therefore records TC-012 as an implemented baseline rather than adding a duplicate experimental parser path.

Future work may add generic function parameters, user-defined generic declarations, variance rules, and stronger collection-element inference. Those features require a separate design record because they would affect declaration parsing, symbol binding, call-site inference, diagnostics, and LSP synchronization.

## Conformance evidence

The native suite covers valid nested collection and variant annotations, incompatible generic assignments, malformed forms such as `list<>`, and nested generic matching. These tests are the required TC-012 non-regression boundary for v2.2.7. Generic declaration syntax and advanced inference remain explicitly deferred and must not be inferred from this baseline.

## Acceptance decision

TC-012 is **implemented baseline** for v2.2.7. The next generic milestone is a separate design and implementation phase for generic declarations and inference; it is not part of the current release gate.

**Author:** Manus AI
**Version:** v2.2.7 design gate
**Status:** Accepted
