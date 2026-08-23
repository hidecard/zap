# Zap Time Standard Library

The `time` domain provides deterministic UTC timestamps and integer-millisecond duration helpers. These APIs are available as direct builtins in the current release line.

## API reference

| API | Arguments | Result |
|---|---|---|
| `utc_now()` | None | A map containing `unix_seconds` and `unix_millis`. |
| `duration_parts(milliseconds)` | One integer millisecond duration | A map containing `milliseconds`, `days`, `hours`, `minutes`, `seconds`, and `millis`. |
| `duration_between(end_millis, start_millis)` | Two integer millisecond timestamps | The same duration map for the checked difference `end_millis - start_millis`. |
| `sleep(milliseconds)` | One non-negative integer duration, at most `60_000` | Returns `none` after the bounded wall-clock delay. |

`utc_now()` is based on Unix time in UTC and does not depend on the local timezone. Its millisecond value is consistent with its seconds value: it is at least `unix_seconds * 1000` and less than `(unix_seconds + 1) * 1000`.

`sleep` is a bounded system operation: negative values and values greater than `60_000` milliseconds return deterministic errors before sleeping. It is not a scheduler, reactor, or lazy async continuation.

`duration_parts` preserves the sign of its input. The component fields are truncated toward zero at each unit boundary, so callers can use `milliseconds` when the exact signed value is required. `duration_between` uses checked subtraction and returns a runtime error if the two timestamps would overflow the signed integer range. Duration decomposition also rejects values that cannot be represented safely.

## Example

```zap
let now = utc_now()
say now["unix_seconds"]
say now["unix_millis"]

let started = now["unix_millis"] - 90_061_007
let elapsed = duration_between(now["unix_millis"], started)
say elapsed["days"]
say elapsed["hours"]
say elapsed["minutes"]
say elapsed["seconds"]
say elapsed["millis"]
```

For implementation and regression coverage, see `native/src/evaluator.rs`. The API catalog is maintained in `native/src/stdlib_catalog.rs`.
