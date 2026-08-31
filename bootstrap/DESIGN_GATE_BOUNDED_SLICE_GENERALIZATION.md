# Design/Evidence Gate: Bounded-Slice Type Inference

## Current Bounded Slice (B1/B2-safe)

### What Currently Works
The current B1 parser and B2 type-checker candidate support bounded inference:

1. **List element inference with numeric literal indices:**
   - `list<number>` indexed with `0`, `1`, `2`, etc. returns `number`
   - `list<text>` indexed with numeric literals returns `text`
   - Incompatible assignments produce stable diagnostics (BOOT-022)

2. **Map element inference with text literal keys:**
   - `map<text,number>` indexed with `"score"`, `"name"`, etc. returns `number`
   - Basic key-type checking for text literal keys
   - Incompatible assignments produce stable diagnostics (v2.11.8)

3. **Simple collection literal typing:**
   - `[1, 2, 3]` infers as `list<number>`
   - `["a", "b"]` infers as `list<text>`
   - `{"score": 7}` infers as `map<text,number>`

4. **Basic generic declarations (bounded):**
   - `fn identity<T>(value: T) -> T` accepted in parser
   - Generic parameter validation rejects constraints at parser boundary
   - Only unconstrained type parameters `T`, `U`, `K`, `V` allowed

### Current Limitations
- Only numeric literal indices for lists (not arbitrary expressions)
- Only text literal keys for maps (not arbitrary expressions)
- No nested collection inference (e.g., `list<list<number>>`)
- No user-defined generic class declarations
- No generic function instantiations in type checker
- No compound collection expressions

## Blocked Generalizations (Separate Evidence/Design Gate Required)

The following features MUST NOT be generalized without a separate design/evidence gate:

### 1. Compound Guard Narrowing
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Simple boolean guards only  
**Required evidence:**
- [ ] Rust reference parity for compound guard behavior
- [ ] Diagnostic fixture batch for compound guard acceptance/rejection
- [ ] Type soundness proof for compound narrowing
- [ ] Performance benchmark for guard evaluation

### 2. Loop Mutation and Narrowing
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** No loop-variable type narrowing  
**Required evidence:**
- [ ] Rust reference parity for loop mutation behavior
- [ ] Diagnostic fixture batch for loop narrowing
- [ ] Fixpoint convergence proof
- [ ] Edge-case corpus for while/for loops

### 3. Reassignment Invalidation
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Simple reassignment without invalidation tracking  
**Required evidence:**
- [ ] Rust reference parity for reassignment behavior
- [ ] Diagnostic fixture batch for incompatible reassignment
- [ ] Scope-exit restoration proof
- [ ] Cross-scope mutation corpus

### 4. General Control-Flow Narrowing
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Branch narrowing exists but bounded  
**Required evidence:**
- [ ] Rust reference parity for control-flow narrowing
- [ ] Diagnostic fixture batch for all branch/loop/else narrowing
- [ ] Merged environment soundness proof
- [ ] Short-circuit and else-flow corpus

### 5. Multiple Option Variables
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Single option variable handling  
**Required evidence:**
- [ ] Rust reference parity for option behavior
- [ ] Diagnostic fixture batch for multiple option interactions
- [ ] Type unification proof for option composition

### 6. Aliases
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Type alias parsing only, no expansion  
**Required evidence:**
- [ ] Rust reference parity for alias behavior
- [ ] Diagnostic fixture batch for alias resolution
- [ ] Expansion soundness proof
- [ ] Cross-module alias corpus

### 7. Arbitrary Predicates
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Literal predicate types only  
**Required evidence:**
- [ ] Rust reference parity for predicate inference
- [ ] Diagnostic fixture batch for predicate acceptance/rejection
- [ ] Predicate evaluation soundness proof

### 8. General Else-Flow Analysis
**Status:** BLOCKED from bounded-slice generalization  
**Current behavior:** Simple else-branch typing  
**Required evidence:**
- [ ] Rust reference parity for else-flow behavior
- [ ] Diagnostic fixture batch for else-branch narrowing
- [ ] Environment merge soundness proof for else paths

## Staged Gate Requirements

Before any blocked generalization can proceed, ALL of the following must be satisfied:

### Gate 1: Rust Reference Parity
- [ ] Collect Rust reference behavior for the specific feature
- [ ] Create differential fixtures for each acceptance and rejection case
- [ ] Verify diagnostic parity between Rust reference and Zap candidate
- [ ] Document any intentional deviations with compatibility record

### Gate 2: Corpus Coverage
- [ ] Create valid corpus fixtures demonstrating the feature
- [ ] Create invalid corpus fixtures demonstrating error cases
- [ ] Ensure edge cases are covered
- [ ] Add malformed recovery tests for parser/type-checker errors

### Gate 3: Type System Soundness
- [ ] Prove that the generalization maintains type safety
- [ ] Verify no unsound type coercions are introduced
- [ ] Check that instantiation preserves type guarantees
- [ ] Validate that inference doesn't create cycles

### Gate 4: Performance Characteristics
- [ ] Benchmark inference performance for the feature
- [ ] Measure memory usage for complex cases
- [ ] Compare against Rust reference performance
- [ ] Identify any performance regressions

### Gate 5: Diagnostic Quality
- [ ] Ensure error messages are clear and actionable
- [ ] Verify error locations are precise
- [ ] Test multi-diagnostic scenarios
- [ ] Validate error recovery behavior

## Approval Required

Before proceeding with any blocked generalization:

- [ ] Technical lead approval of design document
- [ ] Security review of type system changes
- [ ] Performance team sign-off on benchmarks
- [ ] Documentation team review of impact
- [ ] Project manager approval of timeline

## Version Impact

Any blocked generalization would be a significant language feature addition and should be treated as a minor version bump (e.g., v2.10.1 -> v2.11.0) due to:

- New language syntax or semantics
- Expanded type system capabilities
- Potential breaking changes in error messages
- New library functions or behaviors

## Dependencies

This gate depends on:
- Completion of current B1 parser milestone
- Stable B2 type checker foundation for bounded features
- B3 typed-IR ownership
- B4 acceptance evidence before replacing Rust reference

## Related Work

- BOOT-022: Bounded list-element inference
- v2.11.8: Bounded map inference with text literal keys
- DESIGN_GATE_GENERAL_COLLECTION_INFERENCE.md: Proposed staged generalization
- Generic function design document: Preliminary generic system design

## References

- Current Zap type system specification
- Rust generic/control-flow system documentation
- Hindley-Milner type system (theoretical foundation)
- Bootstrap contract (BOOTSTRAP_CONTRACT_EN.md)
