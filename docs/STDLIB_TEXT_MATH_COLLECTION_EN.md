# Zap Standard Library: Text, Math, and Collections

This reference documents the stabilized text, math, and collection helpers available in Zap. The APIs use direct AST evaluation and validate argument count and runtime types before execution.

## Text APIs

| Function | Signature | Returns | Behavior |
|---|---|---|---|
| `len` | `len(value)` | `number` | Returns Unicode character count for text, or element count for a list/map. |
| `str` | `str(value)` | `text` | Converts a Zap value to its display representation. |
| `type` | `type(value)` | `text` | Returns the runtime category: `none`, `bool`, `number`, `text`, `list`, `map`, `object`, `result`, or `option`. |
| `contains` | `contains(text, part)` or `contains(list, value)` | `bool` | Checks text containment or list membership. |
| `is_empty` | `is_empty(value)` | `bool` | Checks whether text, list, or map has no elements. |
| `split` | `split(value, separator)` | `list<text>` | Splits text by a text separator. |
| `join` | `join(values, separator)` | `text` | Joins a list of text values with a separator. |
| `trim` | `trim(value)` | `text` | Removes leading and trailing whitespace. |
| `lower` | `lower(value)` | `text` | Converts text to lowercase. |
| `upper` | `upper(value)` | `text` | Converts text to uppercase. |
| `replace` | `replace(value, from, to)` | `text` | Replaces every occurrence of one text value with another. |
| `starts_with` | `starts_with(value, prefix)` | `bool` | Checks whether text begins with a prefix. |
| `ends_with` | `ends_with(value, suffix)` | `bool` | Checks whether text ends with a suffix. |
| `char_at` | `char_at(value, index)` | `text` | Returns one Unicode scalar value at a zero-based character index. |
| `substring` | `substring(value, start, end)` | `text` | Returns the half-open character range `start <= index < end`; it never slices UTF-8 bytes. |
| `codepoints` | `codepoints(value)` | `list<number>` | Returns each Unicode scalar value as its numeric code point. |

Text operations are Unicode-aware where the operation is character-based. `char_at`, `substring`, `codepoints`, `len`, and `reverse` operate on Unicode scalar values rather than UTF-8 byte offsets. Negative indices and out-of-range `char_at` indices are rejected, while an out-of-range `substring` end produces the available suffix. `join` requires every list element to be text; it does not silently stringify mixed values.

```zap
let source: text = "  Zap Language  "
say trim(source)
say upper(trim(source))
say replace("zap language", "zap", "Zap")
say starts_with("Zap", "Z")
say char_at("က🙂ab", 1)
say substring("က🙂ab", 1, 3)
say codepoints("က🙂")
say join(["web", "ai", "iot"], ", ")
```

## Math APIs

| Function | Signature | Returns | Behavior |
|---|---|---|---|
| `abs` | `abs(value)` | `number` | Returns the absolute value; the minimum signed integer is rejected on overflow. |
| `min` | `min(left, right)` | `number` | Returns the smaller number. |
| `max` | `max(left, right)` | `number` | Returns the larger number. |
| `pow` | `pow(base, exponent)` | `number` | Computes an integer power; the exponent must be non-negative and no greater than `1_000_000`. |
| `sum` | `sum(values)` | `number` | Adds every number in a list using checked integer arithmetic. |
| `range` | `range(end)` or `range(start, end)` | `list<number>` | Creates a half-open integer range: `start <= value < end`. |

Math helpers accept integer `number` values. `pow` rejects negative or over-limit exponents before execution, uses checked exponentiation-by-squaring, and returns a deterministic overflow error instead of wrapping. Other overflow and invalid-exponent cases also produce runtime errors.

```zap
say abs(-42)
say min(8, 3)
say max(8, 3)
say pow(2, 10)
say sum([2, 4, 6])
say range(3)
say range(2, 5)
```

## Collection APIs

| Function | Signature | Returns | Behavior |
|---|---|---|---|
| `keys` | `keys(value)` | `list<text>` | Returns the text keys of a map. |
| `entries` | `entries(value)` | `list<map>` | Returns `{key, value}` records in lexicographic key order. |
| `enumerate` | `enumerate(values)` | `list<map>` | Returns `{index, value}` records with zero-based list indexes. |
| `count` | `count(values, item)` | `number` | Counts values equal to `item` in a list. |
| `reverse` | `reverse(values)` | `list<T>` | Returns a reversed copy; the input list is not mutated. |
| `contains` | `contains(values, item)` | `bool` | Checks list membership using Zap value equality. |
| `is_empty` | `is_empty(values)` | `bool` | Checks whether a list or map is empty. |

```zap
let values: list<number> = [1, 2, 1, 3]
say count(values, 1)
say contains(values, 3)
say reverse(values)

let record = {"name": "Zap", "version": 1}
say keys(record)
say entries(record)
say enumerate(["first", "second"])
say is_empty({})
```

`entries` and `enumerate` are bounded by the runtime iteration limit. They return new lists and do not mutate the input collection; map entry ordering is deterministic so the same input produces the same output across supported platforms.

## Validation and errors

All stabilized helpers reject incorrect argument counts and incompatible runtime values with explicit errors. Examples include `join([1, 2], ",")`, `sum([1, "two"])`, `pow(2, -1)`, and `abs(-9223372036854775808)`. These calls fail rather than silently coercing values or overflowing.

Named arguments are supported for user-defined functions, methods, and closures. Built-in helpers currently use positional arguments and report a clear unsupported named-argument diagnostic when called with named syntax.
