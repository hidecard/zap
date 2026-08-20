# Zap Type Narrowing

Zap supports branch-local narrowing for `option<T>` and `result<T>` values. A successful guard changes the static type only inside the guarded block; after the block, the original wrapper type is restored.

## Guarded payload access

Use `is_some`, `is_ok`, or `is_err` to prove the payload shape before passing a value to a function that expects the payload type.

```zap
fn use_number(value: number):
    say value

let maybe: option<number> = some(7)
let result: result<number> = ok(9)

if is_some(maybe):
    use_number(maybe)

if is_ok(result):
    use_number(result)
```

`is_err(result)` narrows the value to the error payload type when the result annotation carries an error type.

## Boolean conjunctions

With `and`, each safe guard contributes a narrowing to the same branch.

```zap
let maybe: option<number> = some(7)
let result: result<number> = ok(9)

if is_some(maybe) and is_ok(result):
    let first: number = maybe
    let second: number = result
```

This behavior is intentionally branch-local. A guard that cannot be proven safe does not silently narrow a value.

## Safe disjunctions

A disjunction is narrowed only when every alternative establishes the same variable and payload type. This avoids unsoundly narrowing a value when one side of an `or` expression provides a different fact.

```zap
let maybe: option<number> = some(7)

if is_some(maybe) or is_some(maybe):
    let value: number = maybe
```

When alternatives do not establish the same fact, keep the wrapper type or use separate branches.

## Alias variables

Aliases retain the inferred `option<T>` or `result<T>` type and can be narrowed independently.

```zap
let original: option<number> = some(7)
let alias = original

if is_some(alias):
    let value: number = alias
```

Narrowing an alias does not mutate the static type of the original variable.

## `else` branches and restoration

The successful branch receives the payload type. The `else` branch retains the negative information that the success condition was not established, so the value remains an option/result wrapper rather than becoming a payload automatically.

```zap
let maybe: option<number> = some(7)

if is_some(maybe):
    let value: number = maybe
else:
    let still_wrapped: option<number> = maybe
```

After the conditional, `maybe` is again `option<number>` in both paths. Passing it directly to a function that requires `number` is rejected by `zap check` unless it is guarded or explicitly unwrapped.

## Scope and diagnostics

Narrowing applies to nested statements whose indentation belongs to the guarded block. It does not leak into sibling statements or code after the conditional. The checker reports the expected and actual types, including wrapper types such as `option<number>` and `result<number>`, when a narrowed value is used outside its valid scope.

## Current boundary

The current implementation covers direct predicate guards, boolean `and`/`or` combinations with safe common facts, inferred aliases, and branch restoration. More advanced flow facts such as arbitrary user-defined predicates, mutation-sensitive alias analysis, and complex loop invariants remain future static-checker work.
