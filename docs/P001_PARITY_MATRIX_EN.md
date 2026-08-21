# P0-01-A Native/Legacy Parity Matrix

## Scope and ownership

This matrix makes native Zap behavior executable against the retained Python reference runtime. The versioned source of truth is [`conformance/p0-01/matrix.tsv`](../conformance/p0-01/matrix.tsv), and the durable source fixtures live beside it. The native runtime is the canonical implementation; the legacy runtime is a compatibility reference, not a second normative specification.

## Policy classes

| Policy | Required native result | Required legacy result | Meaning |
|---|---|---|---|
| `common` | Exit `0` and normalized stdout digest matches legacy | Exit `0` and matching normalized stdout digest | Behavior remains compatible across both runtimes |
| `native-only` | Exit `0` | Non-zero exit | The native language contract intentionally exceeds the retained legacy translator; the case requires migration documentation rather than silent drift |
| `rejected` | Non-zero exit | Non-zero exit | Malformed or unsupported input must fail closed in both implementations |

The matrix does not compare raw error wording because the runtimes have different diagnostic surfaces. It compares exit status for rejection and SHA-256 digests of normalized stdout for successful common behavior. Normalization removes blank lines and converts CRLF to LF; it does not erase user-visible output content.

## Versioned cases

| Fixture ID | Policy | Fixture | Rationale |
|---|---|---|---|
| `P001-COMMON-HELLO` | `common` | `common/hello.zp` | Stable `say` output |
| `P001-COMMON-CONDITIONAL` | `common` | `common/conditional.zp` | Basic indentation and conditional execution |
| `P001-COMMON-FUNCTION` | `common` | `common/function_body.zp` | Function declaration, return, call, and numeric output |
| `P001-NATIVE-LET` | `native-only` | `native-only/let_binding.zp` | Native declaration semantics are not translated by the retained legacy runtime |
| `P001-REJECT-GROUP` | `rejected` | `rejected/unclosed_group.zp` | Unterminated expression delimiter |
| `P001-REJECT-STRING` | `rejected` | `rejected/unterminated_string.zp` | Unterminated text literal |

## Executable gate

Run the matrix locally with:

```text
ZAP_PARITY_REPORT=target/p001-parity-report.tsv scripts/test_p001_parity.sh
```

The runner builds the debug native binary when needed, invokes both engines with the same fixture, applies the documented normalization, and writes a deterministic tab-separated report containing fixture ID, policy, exit statuses, output digests, and the decision. An unapproved status or digest difference fails the command and identifies the owning fixture.

GitHub Actions runs this gate in the Rust quality job and uploads `target/p001-parity-report.tsv` as a commit-named artifact. CI therefore carries an executable parity report rather than relying on a prose comparison.

## Migration rule

A new native behavior must first receive a matrix row and one of the three policy classes. A `common` mismatch is a parity regression and must be fixed or explicitly reclassified in a reviewed matrix change. A `native-only` row must link to bilingual migration guidance and remain intentional. A `rejected` row must continue to reject without panic. No fixture may depend on network access, wall-clock time, host-specific absolute paths, or secret values.

The retained legacy line-based representation remains a compatibility format for older/internal declarations. This matrix does not authorize broad syntax expansion, traits implementation, or removal of the fallback. Those changes require a separate compatibility decision and release note.
