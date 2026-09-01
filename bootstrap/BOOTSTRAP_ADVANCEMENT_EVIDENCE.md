# Bootstrap Stage Advancement Evidence

## Current Stage: B0 (with B1 lexer/parser candidates)

### B1 Lexer Evidence
- **Owner:** `bootstrap/b1/lexer.zp`
- **Status:** Provisional
- **Fixtures:** `bootstrap/fixtures/lexer/*.zp`, `bootstrap/fixtures/lexer/*.tokens.json`
- **Verification:** `scripts/bootstrap/verify_b1_lexer.sh`, `scripts/bootstrap/verify_b1_lexer_contract.sh`
- **Token schema:** Version 1, stable
- **Coverage:** Basic tokens, operators, unicode identifiers, numeric literals

### B1 Parser Evidence
- **Owner:** `bootstrap/b1/parser.zp`
- **Status:** Provisional
- **Fixtures:** `bootstrap/fixtures/parser/*.zp`, `bootstrap/fixtures/parser/*.ast.json`
- **Verification:** `scripts/bootstrap/verify_b1_parser_candidate.sh`, `scripts/bootstrap/verify_b1_token_native_indentation.sh`
- **AST schema:** Version 1, stable
- **Coverage:** Arithmetic, comparisons, logical operators, declarations, assignments, functions, loops, classes, traits, interfaces, generics (bounded), control flow, nested blocks, token-native indentation

### B2 Type Checker Evidence
- **Owner:** `bootstrap/b2/typecheck.zp`
- **Status:** Provisional
- **Fixtures:** `bootstrap/fixtures/typecheck/*.zp`, `bootstrap/fixtures/typecheck/*.incompatible.zp`
- **Verification:** `scripts/bootstrap/verify_b2_typecheck_candidate.sh`, `scripts/bootstrap/verify_b2_typecheck.sh`
- **Coverage:** Basic type inference, annotations, branch narrowing, loop narrowing, else narrowing, collection types, generic declarations (bounded)

### B2 Typed IR Evidence
- **Owner:** `bootstrap/b2/typed_ir.zp`
- **Status:** Provisional
- **Fixtures:** `bootstrap/fixtures/typecheck/*.typed-ir.json`
- **Verification:** `scripts/bootstrap/verify_b2_typed_ir_candidate.sh`
- **Coverage:** Expression nodes, operators, literals, declarations, assignments

## Corpus Parity Gaps

### Parser Corpus Gaps
- [ ] `arbitrary_complex_call.zp` - missing expected AST fixture (`arbitrary_complex_call.ast.json`)
- [ ] `arbitrary_deep_nesting.zp` - missing expected AST fixture (`arbitrary_deep_nesting.ast.json`)
- [ ] `arbitrary_nested_expressions.zp` - missing expected AST fixture (`arbitrary_nested_expressions.ast.json`)
- [ ] `malformed_recovery.zp` - missing expected diagnostic fixture (`malformed_recovery.diagnostics.json`)
- [ ] `multi_diagnostic.zp` - missing expected diagnostic fixture (`multi_diagnostic.diagnostics.json`)
- [ ] `numeric_literals.zp` - has `numeric_literals.diagnostics.json` but no expected AST fixture (`numeric_literals.ast.json`)
- [ ] `span_coverage.zp` - missing expected AST fixture (`span_coverage.ast.json`)

#### Blocker: golden fixtures require Rust reference runner
The six missing expected-output JSON fixtures must be produced by `cargo run --manifest-path native/Cargo.toml -- bootstrap ast|diagnostics <path>` (see `native/src/cli.rs:1283-1307` and `native/src/bootstrap.rs:54,167`). As of 2026-09-01, the sandboxed environment cannot run the reference: `~/.rustup/toolchains` is empty, the sandbox network is unreachable (cannot sync the pinned 1.88.0 toolchain), and no prebuilt `target/release/zap` binary exists. Synthesizing the JSON by hand is forbidden by `contracts/BOOTSTRAP_CONTRACT_EN.md:38-39` (differential-gate rule) and `BASELINE_B0.md:64` (freeze rules). Resolution requires either an offline-available Rust toolchain or a prebuilt `zap` binary in `target/release/`.

### Type Checker Corpus Gaps
- [ ] Generic nested option/list substitution
- [ ] Generic declaration scope external
- [ ] Generic declaration scope parameter
- [ ] Imported generic body boundary
- [ ] Compound bounds
- [ ] Explicit generic call deferred
- [ ] Generic class alias deferred

### Known Bounded-Slice Limitations (Not Gaps, Design Boundaries)
1. String-based `parse_expression` handles single operators per expression
2. Generic parameter validation rejects constraints at parser boundary
3. Type checker does not instantiate generic functions
4. No compound guard narrowing
5. No loop mutation tracking
6. No reassignment invalidation
7. No general control-flow narrowing
8. No alias expansion
9. No arbitrary predicate inference
10. No general else-flow analysis
11. Multiple option variables not handled (single option variable only)

### B1 Parser Fixes Applied (2026-08-31)
1. **Multiple same-precedence operators:** `parse_expression` now correctly handles chains like `a or b or c` and `1 + 2 + 3` by recursing on the remaining text after the first operator occurrence instead of only parsing `parts[1]`.
2. **Arbitrary call arity:** `parse_call_arguments` now supports any number of arguments via a loop, removing the previous hard limit of 4.
3. **Chained `super().` member access:** `parse_postfix` now correctly handles `super().foo.bar()` by building a proper member-access chain rooted at the `super()` call node.

## Typed-IR Ownership Evidence

### Current Ownership
- Typed IR producer: `bootstrap/b2/typed_ir.zp`
- Typed IR consumer: `bootstrap/b2/typecheck.zp` (imports typed_ir.zp)
- Native reference: `native/src/bootstrap.rs`

### IR Schema Version: 1

### Produced IR Node Types
- `literal` (number, text, bool, none)
- `binary` (add, subtract, multiply, divide, remainder, less, less_equal, greater, greater_equal, equal, not_equal, and, or)
- `unary` (not, negate)
- `call` (with positional and named arguments)
- `member`
- `index`
- `declaration` (with annotation, inferred_type)
- `assignment`
- `conditional` (if-else)
- `propagate` (try expression)

### Missing IR Node Types
- `for` loop
- `while` loop
- `try_catch`
- `function` definition
- `class` definition
- `trait` / `interface`
- `list` literal
- `map` literal
- `option` constructor (`some`, `none`)
- `result` constructor (`ok`, `error`)
- `await`
- `raise`

## B4 Acceptance Evidence Requirements

Before advancing beyond B0, the following evidence must be complete:

### 1. Differential Parity
- [ ] B1 lexer produces identical token stream to Rust reference for owned corpus
- [ ] B1 parser produces identical AST to Rust reference for owned corpus
- [ ] B2 type checker produces identical acceptance/rejection to Rust reference for owned corpus
- [ ] B2 typed IR produces identical structure to Rust reference for owned corpus

### 2. Determinism
- [ ] Repeated runs produce identical token hashes
- [ ] Repeated runs produce identical AST hashes
- [ ] Repeated runs produce identical diagnostic hashes
- [ ] Repeated runs produce identical typed-IR hashes

### 3. Resource Limits
- [ ] Maximum source size enforced
- [ ] Maximum recursion depth enforced
- [ ] Maximum loop iterations configurable
- [ ] Maximum collection size enforced
- [ ] File I/O size limits enforced

### 4. Error Handling
- [ ] No panic/unchecked unwrap on malformed input
- [ ] All errors produce structured diagnostics
- [ ] Error recovery produces valid AST where possible
- [ ] Multi-diagnostic output is stable

### 5. Security
- [ ] Path traversal rejected
- [ ] Symlink escape rejected
- [ ] Absolute paths rejected
- [ ] Dependency confusion protection active

## Next Staged Gate: B1 Parser Completeness

### Gate Criteria
1. All parser fixtures produce correct AST or diagnostic
2. Token-native path produces identical structure to source-line path
3. No panic on malformed input
4. Indentation validation rejects invalid programs
5. Generic syntax parsing bounded and stable

### Evidence to Collect
- [ ] Run `verify_b1_parser_candidate.sh` on Rust reference
- [ ] Run `verify_b1_token_native_indentation.sh` on Rust reference
- [ ] Capture differential mismatches (if any)
- [ ] Document intentional deviations

## Next Staged Gate: B2 Type Checker Completeness

### Gate Criteria
1. All typecheck fixtures produce correct acceptance/rejection
2. Branch narrowing works for option types
3. Loop narrowing works for bounded cases
4. Else narrowing works for bounded cases
5. Generic parameter validation rejects constraints

### Evidence to Collect
- [ ] Run `verify_b2_typecheck_candidate.sh` on Rust reference
- [ ] Run `verify_b2_complete_inference_10.sh` on Rust reference
- [ ] Capture differential mismatches (if any)
- [ ] Document intentional deviations
