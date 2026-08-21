# Zap Structured Diagnostic Model

Zap exposes one structured diagnostic contract across the CLI project validator and the language server. The contract is designed to make machine-readable errors stable without changing the human-readable message.

## Fields

| Field | Meaning |
| --- | --- |
| `kind` / `code` | Stable category such as `SyntaxError`, `NameError`, or `TypeError`. |
| `severity` | Current values are `error`; the field is reserved for future warning and information diagnostics. |
| `file` | Source file associated with the diagnostic, when available. |
| `line` / `column` | One-based source position for CLI JSON output. |
| `message` | Normalized user-facing diagnostic message. |
| `notes` | Deterministic follow-up observations that explain the likely cause. |
| `help` | Optional deterministic remediation guidance. |

The CLI JSON mode emits `notes` as an array and `help` as either a string or `null`. LSP diagnostics retain the standard `severity`, `source`, `code`, `range`, and `message` fields and place the additional Zap metadata in the `data` object.

## Compatibility rules

Diagnostic codes, field names, severity values, and message normalization are part of the tooling contract. New fields may be added without removing existing fields. Human-readable rendering may evolve, but CLI JSON and LSP snapshots must remain deterministic and must not include secrets or environment-specific paths unless the source itself contains them.

Type diagnostics currently include the note `Check the expression type and the expected annotation.` and the help text `Use a compatible value or update the type annotation.` Syntax and name diagnostics provide analogous deterministic guidance.

## Verification

The native test suite includes CLI/LSP parity coverage for conditional-expression type errors, including the code, range, severity, notes, help, and normalized message. Run:

```text
cargo test --manifest-path native/Cargo.toml
```
