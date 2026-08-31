# Design/Evidence Gate: General Collection/Map Inference

## Current Bounded Slice (B0-safe)

### What Currently Works
The current B0 bootstrap implementation supports bounded collection inference:

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

### Current Limitations
- Only numeric literal indices for lists (not arbitrary expressions)
- Only text literal keys for maps (not arbitrary expressions)
- No nested collection inference (e.g., `list<list<number>>`)
- No user-defined generic declarations
- No generic function instantiations
- No compound collection expressions

## Proposed Generalization Scope

### 1. Arbitrary Index Expressions
**Goal:** Support any expression as collection index, not just literals

**Examples:**
```zap
let values: list<number> = [1, 2, 3]
let i: number = 0
let first: number = values[i]  # Currently unsupported

let data: map<text,number> = {"score": 7}
let key: text = "score"
let value: number = data[key]  # Currently unsupported
```

**Design Considerations:**
- Need to infer type of arbitrary index expression
- Must validate that inferred index type matches collection key type
- Error messages should point to index expression location
- Need to handle potentially complex index expressions

### 2. Deeper Nested Inference
**Goal:** Support nested collections and deep indexing

**Examples:**
```zap
let matrix: list<list<number>> = [[1, 2], [3, 4]]
let cell: number = matrix[0][1]  # Currently unsupported

let nested: map<text,map<text,number>> = {"outer": {"inner": 5}}
let value: number = nested["outer"]["inner"]  # Currently unsupported
```

**Design Considerations:**
- Need to track type through multiple indexing operations
- Each level must be validated independently
- Error location must be precise for each indexing operation
- Performance considerations for deep nesting

### 3. User-Defined Generic Declarations
**Goal:** Allow users to define their own generic functions and classes

**Examples:**
```zap
fn identity<T>(value: T) -> T:
    return value

let num: number = identity<number>(5)
let txt: text = identity<text>("hello")

class Container<T>:
    fn new(value: T) -> Container<T>:
        # implementation
```

**Design Considerations:**
- Need generic parameter parsing and validation
- Type substitution during instantiation
- Generic constraint checking (bounds)
- Generic method resolution
- Interaction with existing type inference

### 4. Compound Collection Expressions
**Goal:** Support complex expressions involving collections

**Examples:**
```zap
let result: list<number> = map([1, 2, 3], fn(x: number) -> number: return x * 2)
let filtered: list<number> = filter(values, fn(x: number) -> bool: return x > 0)
let flattened: list<number> = concat([[1, 2], [3, 4]])
```

**Design Considerations:**
- Higher-order function type inference
- Closure type capture
- Generic function specialization
- Performance for collection operations

## Required Evidence

### 1. Rust Reference Parity
- [ ] Collect Rust reference behavior for all proposed features
- [ ] Create differential fixtures for each generalization
- [ ] Verify diagnostic parity between Rust reference and Zap candidate
- [ ] Document any intentional deviations

### 2. Corpus Coverage
- [ ] Create valid corpus fixtures demonstrating each feature
- [ ] Create invalid corpus fixtures demonstrating error cases
- [ ] Ensure edge cases are covered (empty collections, null indices, etc.)
- [ ] Add malformed recovery tests for parser errors

### 3. Type System Soundness
- [ ] Prove that generalized inference maintains type safety
- [ ] Verify no unsound type coercions are introduced
- [ ] Check that generic instantiation preserves type guarantees
- [ ] Validate that nested inference doesn't create cycles

### 4. Performance Characteristics
- [ ] Benchmark collection inference performance
- [ ] Measure memory usage for complex nested types
- [ ] Compare against Rust reference performance
- [ ] Identify any performance regressions

### 5. Diagnostic Quality
- [ ] Ensure error messages are clear and actionable
- [ ] Verify error locations are precise
- [ ] Test multi-diagnostic scenarios
- [ ] Validate error recovery behavior

## Design Alternatives

### Alternative 1: Incremental Generalization
**Approach:** Implement one generalization at a time with separate gates

**Pros:**
- Lower risk per increment
- Easier to debug issues
- Can pause at any point
- Clear progress tracking

**Cons:**
- Slower overall progress
- More gate overhead
- Potential for incomplete intermediate states

### Alternative 2: Holistic Generalization
**Approach:** Implement all generalizations together as a coherent system

**Pros:**
- Faster overall delivery
- More coherent design
- Fewer intermediate states
- Better integration testing

**Cons:**
- Higher risk
- Harder to debug
- All-or-nothing delivery
- Complex dependencies

### Alternative 3: Staged Generalization
**Approach:** Group related generalizations and implement in stages

**Pros:**
- Balanced risk and speed
- Logical grouping of features
- Clear stage boundaries
- Manageable complexity

**Cons:**
- Requires careful stage planning
- Potential for stage coupling
- More complex than incremental

## Recommended Approach

**Recommendation: Alternative 3 (Staged Generalization)**

### Stage 1: Arbitrary Index Expressions
- Focus on expanding from literal to arbitrary indices
- Relatively low risk, high value
- Clear success criteria
- Minimal interaction with other features

### Stage 2: Deeper Nested Inference
- Build on Stage 1 foundation
- Natural extension of indexing
- Can leverage existing type infrastructure
- Moderate complexity

### Stage 3: User-Defined Generics
- Requires separate design phase
- Most complex feature
- Foundation for future language features
- Highest risk, highest value

### Stage 4: Compound Collection Expressions
- Depends on Stages 1-3
- Library feature rather than language feature
- Can be implemented in user code initially
- Lower priority than core language features

## Success Criteria

### Stage 1 Success Criteria
- [ ] All arbitrary index expression fixtures pass
- [ ] Rust reference parity achieved
- [ ] No performance regression
- [ ] Clear error messages for type mismatches

### Stage 2 Success Criteria
- [ ] All nested collection fixtures pass
- [ ] Deep nesting works correctly
- [ ] Error messages identify correct nesting level
- [ ] Performance acceptable for common cases

### Stage 3 Success Criteria
- [ ] Generic function declarations work
- [ ] Generic class declarations work
- [ ] Type substitution correct
- [ ] Generic constraints enforced
- [ ] Rust reference parity for generics

### Stage 4 Success Criteria
- [ ] Higher-order functions work with collections
- [ ] Closure type inference correct
- [ ] Performance acceptable
- [ ] Documentation complete

## Risk Assessment

### High Risks
1. **Generic system complexity:** May introduce subtle type system bugs
2. **Performance regression:** Complex inference may be slow
3. **Diagnostic quality:** Complex types may produce confusing errors

### Medium Risks
1. **Nested inference:** May create type inference cycles
2. **Arbitrary indices:** May introduce unsound coercions
3. **Integration with existing features:** May break current functionality

### Low Risks
1. **Parser changes:** Relatively well-understood area
2. **AST extensions:** Clear pattern to follow
3. **Test infrastructure:** Already exists and works well

## Mitigation Strategies

1. **Extensive differential testing:** Compare against Rust reference at every step
2. **Performance monitoring:** Benchmark after each stage
3. **Incremental validation:** Don't proceed until previous stage is solid
4. **Documentation first:** Write specs before implementation
5. **Code review:** All changes require thorough review
6. **Rollback capability:** Keep previous working version accessible

## Next Steps

1. **Stage 1 Preparation**
   - Create design document for arbitrary index expressions
   - Write differential fixtures
   - Set up performance benchmarks
   - Define success criteria

2. **Stage 1 Implementation**
   - Implement arbitrary index expression inference
   - Add type checking for index expressions
   - Update error handling
   - Run full test suite

3. **Stage 1 Validation**
   - Verify Rust reference parity
   - Check performance
   - Validate diagnostic quality
   - Document any deviations

4. **Stage Decision**
   - If Stage 1 successful, proceed to Stage 2
   - If issues found, fix before proceeding
   - If fundamental problems, reconsider approach

## Approval Required

Before proceeding with any generalization:

- [ ] Technical lead approval of design document
- [ ] Security review of type system changes
- [ ] Performance team sign-off on benchmarks
- [ ] Documentation team review of impact
- [ ] Project manager approval of timeline

## Version Impact

This generalization would be a significant language feature addition and should be treated as a minor version bump (e.g., v0.9.0 → v0.10.0) due to:

- New language syntax (generic declarations)
- Expanded type system capabilities
- Potential breaking changes in error messages
- New library functions for compound operations

## Dependencies

This work depends on:

- Completion of current B0 bootstrap checkpoints
- Stable B2 type checker foundation
- B3 typed-IR ownership
- B4 acceptance evidence

## Related Work

- BOOT-022: Bounded list-element inference
- v2.11.8: Bounded map inference with text literal keys
- A2 exact-expression matrix: Foundation for expression typing
- Generic function design document: Preliminary generic system design

## References

- Current Zap type system specification
- Rust generic system documentation
- TypeScript generic type system (for comparison)
- Hindley-Milner type system (theoretical foundation)