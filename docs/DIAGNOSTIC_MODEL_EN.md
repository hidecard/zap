# Zap Structured Diagnostic Model

Zap exposes one structured diagnostic contract across the CLI project validator and the language server. The contract is designed to make machine-readable errors stable without changing the human-readable message.

## Fields

| Field | Meaning |
| --- | --- |
| `code` | Stable machine-readable identifier such as `ZAP-TYPE-001`; this is the compatibility key for editors and CI. |
| `kind` | Stable user-facing category such as `SyntaxError`, `NameError`, or `TypeError`. |
| `severity` | Current values are `error`; the field is reserved for future warning and information diagnostics. |
| `file` | Source file associated with the diagnostic, when available. |
| `line` / `column` | One-based source position for CLI JSON output. |
| `message` | Normalized user-facing diagnostic message. |
| `notes` | Deterministic follow-up observations that explain the likely cause. |
| `help` | Optional deterministic remediation guidance. |

The CLI JSON mode emits `notes` as an array and `help` as either a string or `null`. LSP diagnostics retain the standard `severity`, `source`, `code`, `range`, and `message` fields and place the additional Zap metadata in the `data` object.

## Stable code registry

| Code | Kind | Meaning |
| --- | --- | --- |
| `ZAP-SYNTAX-001` | `SyntaxError` | Source syntax or parsing failure. |
| `ZAP-NAME-001` | `NameError` | Unknown or undefined name. |
| `ZAP-TYPE-001` | `TypeError` | Value or expression type mismatch. |
| `ZAP-VALUE-001` | `ValueError` | Invalid value or operation. |
| `ZAP-IO-001` | `IOError` | General input/output failure. |
| `ZAP-FILE-001` | `FileNotFound` | Required file does not exist. |
| `ZAP-KEY-001` | `KeyError` | Object or map key is missing. |
| `ZAP-PERM-001` | `PermissionError` | The operation is not permitted. |
| `ZAP-OVERFLOW-001` | `OverflowError` | A bounded numeric or resource operation overflowed. |
| `ZAP-RUNTIME-001` | `Error` | Stable uncaught runtime failure. |
| `ZAP-BORROW-001` | `BorrowError` | Checked object-field or lexical-EnvFrame borrow conflict; the runtime returns an error instead of panicking. |
| `ZAP-MEMORY-001` | `MemoryError` | Run-owned logical byte, object, task, output, or bounded value-lifecycle limit was exceeded. |
| `ZAP-PROJECT-001` | `ProjectError` | Project, manifest, or dependency validation failure. |

Codes are additive compatibility identifiers. A diagnostic kind or message may become more specific in a future release, but an existing code must not be silently reused for a different failure category.

## Compatibility rules

Diagnostic codes, field names, severity values, and message normalization are part of the tooling contract. New fields may be added without removing existing fields. Human-readable rendering may evolve, but CLI JSON and LSP snapshots must remain deterministic and must not include secrets or environment-specific paths unless the source itself contains them. Canonical equality traversals are bounded by `max_value_nodes`, short-circuit previously visited object pairs, and use callable handle identity so cyclic values cannot trigger unbounded recursion.

Type diagnostics currently include the note `Check the expression type and the expected annotation.` and the help text `Use a compatible value or update the type annotation.` Borrow diagnostics use stable guidance for finishing the active object-field or lexical-frame access before attempting a competing read or mutation; the object-field wording remains `Avoid reading and mutating the same object fields at the same time.` with help `Finish the active object-field access before mutating the object.` Canonical `==` and `!=` operations propagate the same `ZAP-BORROW-001` boundary when an object field is already borrowed. Memory diagnostics use deterministic guidance to reduce the value, task, or output admission, or clear cyclic object fields before retrying. Syntax and name diagnostics provide analogous deterministic guidance.

## Verification

The native test suite includes CLI/LSP parity coverage for conditional-expression type errors, including the code, range, severity, notes, help, and normalized message. Run:

```text
cargo test --manifest-path native/Cargo.toml
```
