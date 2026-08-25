# Zap Type-Checking and Conformance Acceptance Matrix

**Status:** Baseline for the PDF-driven follow-up roadmap  
**Verified baseline:** published v2.11.14
**Scope:** Static checking, control-flow narrowing, diagnostics, and conformance fixtures

This document defines the acceptance boundary for the next type-system workstream. It deliberately separates behavior that is already implemented from work that still requires a design decision or implementation. Its active metadata is published v2.11.14; the failed v2.11.12 tag remains historical evidence, and this release does not reopen the completed async runtime, registry, or release-engineering work.

## Current baseline

| Area | Current behavior | Status | Evidence target |
|---|---|---:|---|
| Primitive annotations | `text`, `number`, `bool`, `list`, `map`, `object`, and `none` annotations are parsed and checked where supported | Implemented baseline | Parser/evaluator tests and `zap check` |
| Function argument count | Static diagnostics report missing or extra arguments | Implemented baseline | Human and JSON diagnostics |
| Literal argument mismatch | Literal arguments are checked against known parameter annotations | Implemented baseline | `zap check --json` |
| Return annotation | Return expressions are checked against a function return annotation | Implemented baseline | Function annotation tests |
| Result/Option payloads | `result<T>` and `option<T>` payload annotations validate `ok`, `err`, and `some` payloads | Implemented baseline | Type annotation tests |
| JSON diagnostics | Diagnostics expose `kind`, `message`, `error`, `file`, `line`, and `column` fields | Implemented baseline | CLI/LSP fixtures |
| Simple narrowing | Branch-local narrowing for supported `option<T>` and `result<T>` guards | Implemented baseline | `TYPE_NARROWING_EN.md` |
| Complex narrowing | Nested boolean expressions, loops, reassignment, and incompatible aliases | Implemented baseline | TC-001–TC-006 and TC-010 fixtures; advanced inference remains deferred |
| Complex inference | Nested calls, collection elements, and control-flow expressions | Implemented baseline | TC-007–TC-009 fixtures; advanced generic inference remains deferred |
| Generic design | Generic list/map/function syntax and inference contract | Implemented baseline | `TYPECHECK_GENERIC_DESIGN_EN.md`; bounded `identity<T>`/`same<T>` declaration evidence is recorded separately, while constraints and advanced inference remain deferred |

## Current conformance evidence

TC-006 now has permanent loop-boundary fixtures proving that a guarded `while` body can use the narrowed payload and that the original wrapper type is restored after the loop. Native else-branch narrowing covers the explicit `is_option_none(value)` guard: the true branch retains the option wrapper and the else branch receives the payload type. The v2.11.12 tag attempt added candidate-side native and Zap fixtures for that same direct shape, limited to one tracked `option<number>` variable, one indented else body, and paired acceptance/rejection diagnostics; its macOS ARM64 release workflow failed before publication, and the published v2.11.13 corrective release carries the evidence with the validated cross-platform test-harness fix. The published v2.11.14 release adds a direct bool-literal annotation pair: `true`/`false` assigned to `bool` is accepted, while a direct numeric literal assigned to `bool` is rejected. It also adds a direct none-literal annotation pair: `none` assigned to `none` is accepted, while a direct boolean literal assigned to `none` is rejected. The published v2.11.14 release also adds a direct list-literal annotation pair: the exact `[1, 2]` literal is accepted as `list<number>`, while assigning that literal to `text` is rejected. The published v2.11.15 release adds a direct map-literal annotation pair limited to the exact `{"score": 7}` literal inferred as `map<text,number>`, while assigning that literal to `text` is rejected. The published v2.11.16 release adds a direct option-constructor annotation pair limited to the exact `some(1)` expression inferred as `option<number>`, while assigning that expression to `text` is rejected. The v2.11.17 preparation adds an A2 expression matrix limited to exact `1 + 2` as `number`, `"a" + "b"` as `text`, `1 < 2` as `bool`, `true and false` as `bool`, `ok(1)` as `result<number>`, `[1 + 1, 2]` as `list<number>`, and `{"score": 1 + 2}` as `map<text,number>`, with only Rust-confirmed incompatible pairs. The current A3 checkpoint adds Rust-backed `identity<T>` and `same<T>` declaration fixtures covering AST metadata, inferred identity calls, multiple-parameter substitution, structural `option<T>` wrapper substitution, conflicting substitutions, generic return checking, and runtime substitution; broader constraints, explicit generic arguments, generic classes/aliases, and complete declaration coverage remain deferred. The comparison expression remains compatibility-accepted in broader annotations, so no invented negative case is included. TC-009 now has permanent positive and negative `zap check --json` fixtures for compatible branches, incompatible branch result types, and non-boolean conditions. The v2.11.8 release carries the bounded `map<text,number>` element indexed by a text literal, including the paired incompatible assignment diagnostic. The v2.11.9 release carries the direct `is_some` guard narrowing a tracked `option<number>` inside one indented `if` body, including the paired incompatible payload assignment diagnostic. The v2.11.11 release adds a bounded direct `is_some` guard inside one indented `while` body and verifies that the original `option<number>` wrapper is restored after the loop through a paired incompatible assignment fixture. Its L3 regression asserts that an incompatible conditional expression preserves the JSON `ok`, `kind`, `file`, `line`, `column`, `message`, and `error` fields. TC-010 now has permanent fixtures proving that `option<T>` and `result<T>` wrapper identity survives alias assignment and that reassignment invalidates a narrowed alias fact. TC-012 is now recorded as an implemented baseline for `list<T>`, `map<K, V>`, `option<T>`, and `result<T>` annotations, with malformed generic forms rejected and user-defined generic declarations and advanced inference explicitly deferred. These fixtures establish L2 behavior and the TC-009 L3 diagnostic contract. The L4 regression `lsp_diagnostics_match_cli_type_error_contract` now verifies that LSP diagnostics reuse the shared static checker, preserve the `TypeError` code, convert the same source location to the LSP zero-based range, and expose the normalized message. Lint diagnostics remain available through the same source-diagnostic bridge.

## Acceptance levels

A feature may be promoted from **proposed** to **implemented** only when its syntax, positive behavior, negative behavior, diagnostic shape, and bilingual documentation are all covered. Runtime behavior alone is not sufficient for a static-checking release gate.

| Level | Meaning | Required evidence |
|---|---|---|
| L0 | Not specified | No implementation work should begin |
| L1 | Syntax/design accepted | Specification and parser rejection/acceptance cases |
| L2 | Static behavior implemented | Positive and negative `zap check` fixtures |
| L3 | Diagnostic contract stable | JSON schema, location, error kind, and message assertions |
| L4 | Conformance-ready | Runtime agreement, formatter/LSP agreement, bilingual docs, and CI gate |

## Conformance scenario matrix

| ID | Scenario | Expected static result | Diagnostic requirement | Priority |
|---|---|---|---|---:|
| TC-001 | `if is_some(value)` narrows `option<number>` inside the branch | Accept branch-local numeric use | No diagnostic for valid use; wrapper restored after branch | P0 |
| TC-002 | `if is_some(value) and value > 0` combines a guard and numeric comparison | Accept only when both facts hold | Stable location on the unsafe comparison | P0 |
| TC-003 | `if is_some(a) or is_some(a)` repeats the same safe fact | Accept deterministic same-variable narrowing | No duplicate or contradictory diagnostic | P0 |
| TC-004 | `if is_some(a) or is_some(b)` uses different variables | Do not unsafely narrow either variable | TypeError at the use requiring an unproven payload | P0 |
| TC-005 | A narrowed variable is reassigned inside the branch | Invalidate the prior narrowing after reassignment | Diagnostic must identify the post-assignment use if unsafe | P0 |
| TC-006 | A loop mutates a narrowed variable | Recompute facts at loop boundaries | No stale narrowing may escape the loop | P1 |
| TC-007 | A nested function call returns an annotated value | Propagate the return type through the call | Stable mismatch location at call or assignment site | P0 |
| TC-008 | A collection element is used against an annotation | Infer/check the element type where statically known | JSON diagnostic includes element expression location | P1 |
| TC-009 | A control-flow expression returns incompatible branch types | Reject incompatible expression result | TypeError includes both branch context and location | P1 |
| TC-010 | An alias carries `option<T>` or `result<T>` through multiple branches | Preserve wrapper identity and narrowing facts | No unsound alias widening | P1 |
| TC-011 | Unknown annotation or malformed generic annotation | Reject during parse/check | `kind=TypeError` or syntax diagnostic with exact span | Implemented baseline |
| TC-012 | Generic syntax such as `list<number>` is used | Accept supported generic annotation forms; reject malformed or unsupported forms | Unknown generic forms remain rejected with a type diagnostic | Implemented baseline |

## Diagnostic contract

Every new static-checking failure must preserve the structured diagnostic boundary:

```json
{
  "kind": "TypeError",
  "message": "...",
  "error": "...",
  "file": "main.zp",
  "line": 1,
  "column": 1
}
```

The exact wording may evolve, but `kind`, source location, and a user-actionable message are required. Internal Rust errors must not leak into the public JSON contract. Human-readable diagnostics and JSON diagnostics must describe the same failure.

## Implementation order

The implementation order is complete for TC-001 through TC-012 at the documented baseline boundary. The v2.11.7 candidate adds a bounded nested-list slice: paired fixtures cover `list<list<number>>` indexing and rejection when its numeric result is assigned to `text`. The v2.11.8 release adds a bounded map-element slice limited to a tracked `map<text,number>` variable and a text-literal key, with paired valid and incompatible fixtures. The v2.11.9 release adds a bounded direct-`is_some` branch-local narrowing slice limited to a tracked `option<number>` variable and one indented `if` body, with paired valid and incompatible fixtures. The v2.11.11 release adds a bounded direct-`is_some` loop-body slice and checks wrapper restoration after the loop with a paired incompatible fixture. The v2.11.12 tag attempt added a bounded direct `is_option_none` else-body slice limited to one tracked `option<number>` variable and one indented else body, with paired valid and incompatible fixtures, but its macOS ARM64 release workflow failed before publication; the published v2.11.13 corrective release carries that evidence with the validated cross-platform test-harness fix, and the published v2.11.14 release adds a bounded direct bool-literal annotation pair with stable acceptance/rejection diagnostics, a bounded direct none-literal annotation pair with stable acceptance/rejection diagnostics, and a bounded direct `[1, 2]` list-literal annotation pair with stable acceptance/rejection diagnostics; the published v2.11.15 release adds a bounded exact `{"score": 7}` map-literal annotation pair with stable acceptance/rejection diagnostics; the published v2.11.16 release adds a bounded exact `some(1)` option-constructor annotation pair with stable acceptance/rejection diagnostics; the v2.11.17 preparation adds a bounded A2 expression matrix for exact arithmetic, text addition, boolean logic, comparison, result construction, list arithmetic, and map arithmetic with stable Rust-confirmed diagnostics. The current A3 checkpoint adds bounded generic declaration and substitution evidence for identity, multiple parameters, and `option<T>` wrappers. The v2.11.17 line must continue with separately evidenced gates rather than generalize these slices. Future work should extend broader collection inference, nested maps, compound guards, loop mutation, reassignment invalidation, aliases, arbitrary nested expressions, generic constraints, explicit generic arguments, generic classes/aliases, and complete user-defined generic declarations only under a new design record and release gate. Generic syntax must not be expanded merely to satisfy a fixture.

## Do-not-duplicate boundary

The following work is intentionally outside this matrix because it is already implemented and validated in v2.1-D/E: structured task APIs, cancellation and deadlines, threaded file/TCP/process adapters, registry authentication/service deployment, release preflight, artifact manifests, signing, provenance, and post-publish release verification.

## Definition of done for this workstream

The v2.3.0 type-checking baseline is complete for TC-001 through TC-012 at the supported syntax boundary: P0 rows have L3 evidence, TC-006/TC-009/TC-010 have L3 evidence, diagnostics have stable file/line/column locations, accepted programs agree with runtime behavior, negative fixtures fail for their intended reasons, LSP consumes the shared diagnostic vocabulary, and the English/Burmese documentation pair is synchronized. Advanced generic declarations and inference remain outside this release boundary.
