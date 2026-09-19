# P0-01 Legacy-to-Native Migration Guidance

## Overview

The native runtime is the canonical implementation. The retained Python runtime is a compatibility reference for older line-based programs. A native-only matrix row is intentional only when its policy and migration note are reviewed together.

## Native-only fixtures

| Fixture | Observed boundary | Migration guidance |
|---|---|---|
| `native-only/arithmetic.zp` | The program uses a modern `let` declaration before arithmetic. The legacy translator rejects that declaration boundary. | Use the native runtime for modern declarations and arithmetic. Legacy programs that must remain portable should use the documented legacy declaration form. |
| `native-only/while_loop.zp` | The loop fixture uses the modern declaration boundary. The legacy translator rejects the program before evaluating the loop. | Move loop-containing programs to native, or retain the legacy declaration form while the compatibility runtime is maintained. |
| `native-only/variables.zp` | Multiple modern declarations and assignment are outside the legacy translator's supported boundary. | Use native for declaration and assignment semantics; keep legacy-only code on the documented compatibility form. |
| `native-only/let_binding.zp` | `let` is intentionally native-only in the retained legacy runtime. | Replace `let` with the legacy form only for temporary compatibility; new code should target native. |
| `native-only/append_function.zp` | The program crosses the modern declaration boundary and then uses the native list `append` builtin. | Use native list operations. Do not treat the legacy runtime as an API-compatible standard library. |
| `native-only/assert_statement.zp` | The program crosses the modern declaration boundary and uses native `assert`. | Use native assertions for validation and tests. Legacy programs need an explicit replacement check. |
| `native-only/join_function.zp` | The program crosses the modern declaration boundary and uses native `join`. | Use the native string/list API or an explicit compatibility helper. |

The inventory may also report `compatibility` rows where both runtimes succeed with different output, and `deprecated` rows where legacy accepts input that native rejects. Those rows are review signals, not permission to silently change semantics. The current inventory is written to `target/legacy-parity-inventory.tsv` by `scripts/inventory_legacy_parity.sh`.

## Migration strategy

1. Identify the owning fixture and policy in `conformance/p0-01/matrix.tsv` or the inventory report.
2. Run the curated gate with `scripts/test_p001_parity.sh` after changing either runtime.
3. For a native-only feature, update the application to the native declaration and builtin boundary.
4. For a compatibility or deprecated row, record the owner, release impact, and migration deadline before promoting it into the curated matrix.
5. Keep diagnostics, output normalization, and migration notes in the same reviewed change.

## CI integration

The curated matrix is a release gate in GitHub Actions. A missing fixture, unknown policy, unexpected exit status, or unapproved common-output drift fails the job and identifies the fixture ID. The inventory command is deterministic and can be attached to release evidence, but it does not replace the reviewed policy matrix.

## Support policy

- `common`: supported in both runtimes with normalized output parity.
- `native-only`: supported by the canonical native runtime; legacy compatibility is not guaranteed.
- `rejected`: intentionally unsupported or malformed input in both runtimes.
- `compatibility`: requires an explicit compatibility decision because successful output differs.
- `deprecated`: legacy-accepted behavior rejected by native; migrate or document the removal decision.
