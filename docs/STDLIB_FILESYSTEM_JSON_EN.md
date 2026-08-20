# Zap Standard Library: Filesystem and JSON

This guide documents the stabilized filesystem and JSON APIs available through Zap's native runtime. These functions are available from direct AST calls and use structured arguments rather than source-text reconstruction.

## Filesystem APIs

| Function | Signature | Returns | Behavior |
|---|---|---|---|
| `read_text` | `read_text(path: text)` | `text` | Reads a UTF-8 text file. |
| `write_text` | `write_text(path: text, content: text)` | `none` | Writes or replaces a UTF-8 text file. |
| `read_lines` | `read_lines(path: text)` | `list<text>` | Reads a file as lines without trailing line separators. |
| `write_lines` | `write_lines(path: text, lines: list<text>)` | `none` | Writes text lines using the platform's normal newline handling. |
| `exists` | `exists(path: text)` | `bool` | Reports whether a path exists. |
| `path_join` | `path_join(first: text, second: text, ...)` | `text` | Joins path components using the host platform's path rules. |
| `basename` | `basename(path: text)` | `text` | Returns the final path component. |
| `dirname` | `dirname(path: text)` | `text` | Returns the parent path. |

All filesystem functions validate their argument count and types. Read and write operations return a runtime error when the path cannot be accessed or the content cannot be decoded or written. User source files and file reads are bounded by the runtime's configured safety limits; callers should process large data in smaller chunks.

```zap
let path: text = path_join("data", "users.txt")
write_lines(path, ["alice", "bob"])
let users: list<text> = read_lines(path)
if exists(path):
    say basename(path)
```

## JSON APIs

| Function | Signature | Returns | Behavior |
|---|---|---|---|
| `json` | `json(value)` | `text` | Encodes a Zap value as JSON text. |
| `from_json` | `from_json(source: text)` | `any` | Parses JSON text into a Zap value. |

JSON conversion rules are deterministic. `none` becomes JSON `null`; booleans, numbers, text, lists, and maps become their corresponding JSON values. Zap `option` and `result` values use tagged objects so their variant information is preserved during a round trip.

```zap
let source: text = json([1, 2, 3])
let values = from_json(source)
say values[1]

let record = from_json("{\"name\":\"Zap\",\"version\":1}")
say record["name"]
```

`json` accepts exactly one argument. `from_json` accepts exactly one text argument. Malformed JSON, unsupported numeric values, unknown Zap variant tags, and values outside Zap's integer range produce clear runtime errors. JSON input and output are bounded by an **8 MiB** safety limit; oversized payloads are rejected instead of being processed without bounds.

## Error examples

```zap
// Type error: from_json requires text.
let value = from_json(42)

// Parse error: the source is not valid JSON.
let broken = from_json("{invalid}")
```

These APIs are designed to be portable across supported platforms. Path separators are supplied by the host runtime; applications should use `path_join` instead of manually concatenating separators.
