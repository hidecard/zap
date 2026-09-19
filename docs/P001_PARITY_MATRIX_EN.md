# P0-01-A Native/Legacy Parity Matrix

## Scope and ownership

This matrix makes native Zap behavior executable against the retained Python reference runtime. The versioned source of truth is [`conformance/p0-01/matrix.tsv`](../conformance/p0-01/matrix.tsv), and the durable source fixtures live beside it. The native runtime is the canonical implementation; the legacy runtime is a compatibility reference, not a second normative specification.

## Policy classes

| Policy | Required native result | Required legacy result | Meaning |
|---|---|---|---|
| `common` | Exit `0` and normalized stdout digest matches legacy | Exit `0` and matching normalized stdout digest | Behavior remains compatible across both runtimes |
| `native-only` | Exit `0` | Non-zero exit | The native language contract intentionally exceeds the retained legacy translator; the case requires migration documentation rather than silent drift |
| `rejected` | Non-zero exit | Non-zero exit | Malformed or unsupported input must fail closed in both implementations |
| `compatibility` | Observed in inventory | Observed in inventory | Both runtimes succeed but user-visible output differs; requires an explicit compatibility decision |
| `deprecated` | Observed in inventory | Observed in inventory | Legacy accepts behavior that native rejects; requires an explicit migration or removal decision |

The executable matrix currently gates `common`, `native-only`, and `rejected` rows. The inventory command records all five observed classes so compatibility and deprecated behavior cannot disappear from review.

The matrix does not compare raw error wording because the runtimes have different diagnostic surfaces. It compares exit status for rejection and SHA-256 digests of normalized stdout for successful common behavior. Normalization removes blank lines and converts CRLF to LF; it does not erase user-visible output content.

## Versioned cases

| Fixture ID | Policy | Fixture | Rationale |
|---|---|---|---|
| `P001-COMMON-HELLO` | `common` | `common/hello.zp` | Stable `say` output |
| `P001-COMMON-CONDITIONAL` | `common` | `common/conditional.zp` | Basic indentation and conditional execution |
| `P001-COMMON-FUNCTION` | `common` | `common/function_body.zp` | Function declaration, return, call, and numeric output |
| `P001-NATIVE-ARITHMETIC` | `native-only` | `native-only/arithmetic.zp` | Modern declaration boundary with arithmetic |
| `P001-NATIVE-WHILE-LOOP` | `native-only` | `native-only/while_loop.zp` | Modern declaration boundary with a loop |
| `P001-NATIVE-VARIABLES` | `native-only` | `native-only/variables.zp` | Modern declaration and assignment boundary |
| `P001-NATIVE-LET` | `native-only` | `native-only/let_binding.zp` | Native declaration semantics are not translated by the retained legacy runtime |
| `P001-NATIVE-APPEND` | `native-only` | `native-only/append_function.zp` | Native list builtin behind the modern declaration boundary |
| `P001-NATIVE-ASSERT` | `native-only` | `native-only/assert_statement.zp` | Native assertion builtin behind the modern declaration boundary |
| `P001-NATIVE-JOIN` | `native-only` | `native-only/join_function.zp` | Native string builtin behind the modern declaration boundary |
| `P001-REJECT-GROUP` | `rejected` | `rejected/unclosed_group.zp` | Unterminated expression delimiter |
| `P001-REJECT-STRING` | `rejected` | `rejected/unterminated_string.zp` | Unterminated text literal |
| `P001-REJECT-INVALID-INDENTATION` | `rejected` | `rejected/invalid_indentation.zp` | Invalid block indentation |

## Executable gate and inventory

Run the curated matrix locally with:

```text
ZAP_PARITY_REPORT=target/p001-parity-report.tsv scripts/test_p001_parity.sh
```

Run the tracked-fixture inventory with:

```text
ZAP_LEGACY_PARITY_INVENTORY=target/legacy-parity-inventory.tsv scripts/inventory_legacy_parity.sh
```

The runner builds or selects the native binary when needed, invokes both engines with the same fixture, applies documented normalization, and writes a deterministic tab-separated report containing fixture ID, policy, exit statuses, output digests, decision, and classification. The inventory uses only tracked `.zp` sources under the fixture, conformance, example, test, corpus, and framework trees. It runs each source in an isolated temporary working directory so file-writing fixtures do not modify the repository.

GitHub Actions runs the curated gate in the Rust quality job and uploads `target/p001-parity-report.tsv` as a commit-named artifact. CI therefore carries an executable parity report rather than relying on a prose comparison. The inventory report is suitable for release evidence and review; it is not a second normative specification.

## Migration rule

A new native behavior must first receive a matrix row and one of the three executable policy classes. A `common` mismatch is a parity regression and must be fixed or explicitly reclassified in a reviewed matrix change. A `native-only` row must link to bilingual migration guidance and remain intentional. A `rejected` row must continue to reject without panic. `compatibility` and `deprecated` inventory rows require an owner and a release decision before they can be promoted into the curated matrix. No fixture may depend on network access, wall-clock time, host-specific absolute paths, or secret values.

The retained legacy line-based representation remains a compatibility format for older/internal declarations. This matrix does not authorize broad syntax expansion, traits implementation, or removal of the fallback. Those changes require a separate compatibility decision and release note.
