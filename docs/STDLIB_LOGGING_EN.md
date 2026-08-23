# Zap Structured Logging Standard Library

The `logging` domain provides deterministic structured-record builders for machine-readable events. The runtime validates each event and returns data; it does not write directly to stdout or stderr. Applications therefore choose their own output sink.

## `log_record(level, message, fields)`

Returns a map with `level`, `message`, and `fields` keys. The level must be `trace`, `debug`, `info`, `warn`, or `error`. The message must be non-empty text, and `fields` must be a map containing JSON-compatible Zap values.

```zap
let event = log_record("info", "server started", {"port": 8080, "mode": "dev"})
say event["level"]
say event["fields"]["port"]
```

## `log_json(level, message, fields)`

Returns one JSON line containing the same logical record. Top-level keys are emitted in canonical order: `fields`, `level`, then `message`. Field names are sorted alphabetically, making output independent of hash-map iteration order.

```zap
let line = log_json("warn", "slow request", {"path": "/health", "duration_ms": 250})
say line
```

## Safety limits and errors

| Item | Limit |
|---|---:|
| Message size | 8 KiB |
| Number of fields | 64 |
| Field-name size | 256 bytes |
| Encoded JSON output | 64 KiB |

Invalid levels, empty messages, non-map fields, oversized messages, oversized field names, and more than 64 fields return stable runtime errors. JSON output is bounded; oversized data is not silently truncated.

Structured logging does not add timestamps implicitly. Applications that need event time can explicitly combine `utc_now()` with the fields map.

```zap
let current = utc_now()
let event = log_record("debug", "poll completed", {
    "unix_millis": current["unix_millis"],
    "items": 12
})
say log_json(event["level"], event["message"], event["fields"])
```

> **Determinism guarantee:** `log_json` sorts field names and applies fixed size limits. It never depends on hash-map iteration order and never wraps or truncates oversized data.

## Related APIs

Use `utc_now()` for explicit UTC event timestamps, `json()` for general JSON serialization, and `atomic_write()` when persisting a complete log snapshot safely to a file.

[Back to the standard-library index](STDLIB_INDEX_EN.md)

[Back to the v2.1 roadmap](V2.1_ROADMAP_EN.md)

---

Author: **Zap project maintainers**
_Last updated: 2026-08-21._

## References

[1]: ../native/src/evaluator.rs "Zap evaluator structured logging implementation and tests"
[2]: ../native/src/stdlib_catalog.rs "Zap public standard-library catalog"

[1] [2]

This guide documents the Zap v2.1-C structured logging slice.
