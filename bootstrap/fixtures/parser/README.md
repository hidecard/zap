# Bootstrap Parser Fixtures

This directory holds the parser-corpus fixtures for the B1 candidate parser.
Every `.zp` source has one of three expected-output companions:

- `<name>.ast.json` — canonical AST produced by the Rust reference
  (`cargo run -- bootstrap ast <name>.zp`)
- `<name>.diagnostics.json` — canonical diagnostics produced by the Rust reference
  (`cargo run -- bootstrap diagnostics <name>.zp`)
- `<name>.invalid_indentation.json` / `<name>.unexpected_indentation.json` —
  indentation-rejection diagnostic produced by the Rust reference

The companion JSON is a **derived output**. Regenerating it requires the Rust
reference runner; see `BASELINE_B0.md` "Scope of frozen artifacts" and
`scripts/bootstrap/capture_parser_fixtures.sh`.

> **Note:** This README covers only the parser corpus (`bootstrap/fixtures/parser/`).
> Companion gap entries for the **lexer** corpus (`bootstrap/fixtures/lexer/`)
> and the **typecheck typed-IR** corpus (`bootstrap/fixtures/typecheck/*.typed-ir.json`)
> are tracked in `bootstrap/BOOTSTRAP_ADVANCEMENT_EVIDENCE.md` under
> "Lexer Corpus Gaps" and "Type Checker Corpus Gaps" and in `contracts/OWNERS.tsv`
> under BOOT-058…BOOT-070. The capture script handles them via
> `--family=lexer|typedir|diagnostics|tokens|parser|all`.

## Status legend

| Status | Meaning |
|---|---|
| ✅ captured | `.ast.json` or `.diagnostics.json` exists and matches the Rust reference |
| ⚠️ pending | `.zp` source exists but the expected-output JSON is missing; tracked by a `provisional-pending-capture` row in `contracts/OWNERS.tsv` (BOOT-051…BOOT-057) |
| 🚫 negative | Source is intentionally rejected by the Rust parser; companion JSON captures the rejection diagnostic |

## Fixture index

| `.zp` source | Expected output | Status | Notes |
|---|---|---|---|
| `arbitrary_complex_call.zp` | `arbitrary_complex_call.ast.json` | ⚠️ pending | BOOT-051 |
| `arbitrary_deep_indentation.zp` | *(none — accepted by token-native path)* | 🚫 negative | Used by `verify_b1_token_native_indentation.sh` |
| `arbitrary_deep_nesting.zp` | `arbitrary_deep_nesting.ast.json` | ⚠️ pending | BOOT-052 |
| `arbitrary_nested_blocks_complex.zp` | *(none — accepted by token-native path)* | ✅ captured (runtime) | Used by `verify_b1_token_native_indentation.sh` |
| `arbitrary_nested_expressions.zp` | `arbitrary_nested_expressions.ast.json` | ⚠️ pending | BOOT-053 |
| `arithmetic.zp` | `arithmetic.ast.json` | ✅ captured | BOOT-015 |
| `assignment_statement.zp` | `assignment_statement.ast.json` | ✅ captured | |
| `compound.zp` | `compound.ast.json` | ✅ captured | BOOT-013 |
| `control_flow.zp` | `control_flow.ast.json` | ✅ captured | |
| `deep_mixed_blocks.zp` | `deep_mixed_blocks.ast.json` | ✅ captured | |
| `four_argument_call.zp` | `four_argument_call.ast.json` | ✅ captured | |
| `full_expression.zp` | `full_expression.ast.json` | ✅ captured | |
| `grouped_expression.zp` | `grouped_expression.ast.json` | ✅ captured | |
| `invalid_indentation.zp` | `invalid_indentation.json` | ✅ captured | |
| `invalid_indentation_jump.zp` | `invalid_indentation_jump.zp`-rejection diagnostic | ✅ captured (runtime) | Used by `verify_b1_token_native_indentation.sh`; expected message `"unexpected indentation"` |
| `logical_comparison_matrix.zp` | `logical_comparison_matrix.ast.json` | ✅ captured | |
| `malformed_recovery.zp` | `malformed_recovery.diagnostics.json` | ⚠️ pending | BOOT-054 |
| `mixed_recursive_sequence.zp` | `mixed_recursive_sequence.ast.json` | ✅ captured | |
| `mixed_top_level_statements.zp` | *(none — accepted by token-native path)* | ✅ captured (runtime) | Used by `verify_b1_token_native_indentation.sh` |
| `mixed_top_level.zp` | `mixed_top_level.ast.json` | ✅ captured | |
| `multi_diagnostic.zp` | `multi_diagnostic.diagnostics.json` | ⚠️ pending | BOOT-055 |
| `multi_digit_number.zp` | `multi_digit_number.ast.json` | ✅ captured | |
| `multiplicative_additive.zp` | `multiplicative_additive.ast.json` | ✅ captured | |
| `negative_number.zp` | `negative_number.ast.json` | ✅ captured | |
| `nested_assignment_block.zp` | `nested_assignment_block.ast.json` | ✅ captured | |
| `nested_blocks.zp` | `nested_blocks.ast.json` | ✅ captured | |
| `nested_calls.zp` | `nested_calls.ast.json` | ✅ captured | |
| `nested_class_method.zp` | `nested_class_method.ast.json` | ✅ captured | |
| `nested_function_blocks.zp` | `nested_function_blocks.ast.json` | ✅ captured | |
| `numeric_literals.zp` | `numeric_literals.ast.json` + `numeric_literals.diagnostics.json` | ⚠️ partial (BOOT-056) | AST pending; diagnostics captured |
| `parenthesized_nested.zp` | `parenthesized_nested.ast.json` | ✅ captured | |
| `parenthesized_not.zp` | `parenthesized_not.ast.json` | ✅ captured | |
| `simple_class.zp` | `simple_class.ast.json` | ✅ captured | |
| `simple_function.zp` | `simple_function.ast.json` | ✅ captured | |
| `simple_loop.zp` | `simple_loop.ast.json` | ✅ captured | |
| `span_coverage.zp` | `span_coverage.ast.json` | ⚠️ pending | BOOT-057 |
| `three_argument_call.zp` | `three_argument_call.ast.json` | ✅ captured | |
| `three_declarations.zp` | `three_declarations.ast.json` | ✅ captured | |
| `two_declarations.zp` | `two_declarations.ast.json` | ✅ captured | |
| `unexpected_indentation.zp` | `unexpected_indentation.json` | ✅ captured | |
| `unicode_identifier.zp` | `unicode_identifier.ast.json` | ✅ captured | BOOT-003 |
| `while_else_syntax.zp` | *(none — rejected by parser)* | 🚫 negative | Used by `verify_b1_token_native_indentation.sh`; expected message `"unsupported 'while ... else' syntax"` |
| `while_simple.zp` | `while_simple.ast.json` | ✅ captured | |
| `while_without_else.zp` | *(none — accepted by token-native path)* | ✅ captured (runtime) | Used by `verify_b1_token_native_indentation.sh` |

## Coverage summary

- Total `.zp` sources: **43**
- AST/diagnostics JSON captured: **30**
- ⚠️ Pending capture: **7** (BOOT-051…BOOT-057)
- Negative fixtures (rejected by parser; companion JSON captures rejection): **6**
- Runtime-only (accepted by token-native path; no companion JSON expected): **5**

## How to close the pending captures

```sh
bash scripts/bootstrap/capture_parser_fixtures.sh
```

This script must be run on a host where the Rust toolchain is available
(`~/.rustup/toolchains` populated or `target/release/zap` prebuilt). The sandbox
used for plan authoring does not satisfy this requirement; see
`bootstrap/BOOTSTRAP_ADVANCEMENT_EVIDENCE.md` "Blocker: golden fixtures require
Rust reference runner".