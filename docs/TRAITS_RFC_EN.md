# Zap Traits and Composition RFC

**RFC status:** Design-only proposal; no parser or runtime implementation is included.
**Verified baseline:** Zap v2.2.7
**Decision target:** Review for a future post-v2.2 language version; v2.2.0 does not enable traits, interfaces, or new inheritance semantics.
**Audience:** Language designers, runtime maintainers, package authors, and reviewers of future compatibility changes.
**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Learning guide](LEARN_ZAP_EN.md) · [Syntax reference](SYNTAX_GUIDE_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Package guide](PACKAGE_EN.md) · [Burmese RFC](TRAITS_RFC_MM.md)

## Abstract

This RFC proposes a composition-first design for reusable behavior in Zap. The proposal adds named behavioral contracts and explicit composition without replacing the current single-inheritance model in v2.2.0. It defines the conceptual model, surface syntax, method lookup, visibility, diagnostics, migration rules, dispatch choices, rejected alternatives, and compatibility boundaries required before implementation begins.

> **Decision:** Keep `extends` as the current inheritance mechanism, keep traits/interfaces deferred, and do not change parser or evaluator behavior until this RFC has been reviewed and a later version is explicitly approved.

## 1. Problem statement

Zap currently supports classes, methods, and single inheritance through `extends`. That model is useful for substitutable families of objects, but it couples behavior reuse to one nominal parent. A class that needs capabilities from two independent domains must either duplicate methods, create an artificial parent hierarchy, or depend on helper functions that do not participate in method contracts. Those choices make diagnostics, package evolution, and method ownership less explicit.

The proposal addresses composition as a separate design problem. It does not claim that the current runtime already supports traits, interfaces, conflict resolution, or multiple inheritance. The canonical specification currently describes classes and the existing runtime ownership boundary; this RFC is a forward-looking design record, not an implementation contract [1] [2].

## 2. Goals and non-goals

| Area | Goal | Non-goal for this RFC |
|---|---|---|
| Reuse | Compose small, named behavior units into a class | Implement the feature in v2.2.0 |
| Contracts | Define required methods and optional provided methods | Make every function structural by default |
| Lookup | Make precedence and conflict handling deterministic | Permit implicit method-order guessing |
| Visibility | Preserve private/public boundaries across composed units | Expose private implementation details to consumers |
| Diagnostics | Provide stable missing/conflict diagnostics with source spans | Reuse unstable string-only errors |
| Migration | Give `extends` users a mechanical migration path | Remove single inheritance automatically |
| Dispatch | Choose an explicit static/dynamic boundary | Promise a production type system beyond current checks |
| Compatibility | Separate accepted, deprecated, rejected, and future syntax | Introduce parser/runtime changes before review |

## 3. Current baseline

The current Zap baseline uses class declarations, methods, constructors, and single inheritance with `extends`. The current specification owns syntax and runtime semantics, and structured diagnostics must retain severity, stable code, message, and source location where available [1]. The current release line is v2.2.7, and a semantics change requires specification updates, bilingual documentation, conformance fixtures, a changelog entry, and an explicit version decision [1].

The current baseline therefore remains unchanged by this RFC:

```zap
class Animal:
    fn speak(self):
        return "sound"

class Dog extends Animal:
    fn speak(self):
        return "woof"
```

The example continues to mean single inheritance in v2.2.0. No `trait`, `interface`, `with`, or conflict-resolution syntax is accepted until a later approved implementation milestone.

## 4. Terminology

| Term | Proposed meaning |
|---|---|
| Trait | A named set of behavior declarations that may contain required methods, provided methods, and associated visibility metadata. A trait is not itself an instantiable class. |
| Interface | A contract containing required callable signatures and visibility rules. An interface provides no method body in the initial proposal. |
| Composition | Attaching one or more traits or interfaces to a class through explicit syntax. |
| Required method | A method that the composing class must implement before it is concrete. |
| Provided method | A default method body supplied by a trait and eligible for explicit conflict resolution. |
| Conformance | The checker/runtime-visible fact that a class satisfies an interface or trait requirement. |
| Conflict | Two or more composed units provide the same method name and no explicit selection resolves the ambiguity. |
| Linearization | The deterministic ordering used to search class, composed units, and parent methods. |
| Static dispatch | A call resolved from a statically known receiver contract or selected implementation. |
| Dynamic dispatch | A call resolved at runtime from the receiver's concrete class and conformance table. |

## 5. Composition versus single inheritance

Composition and inheritance solve different problems and must remain separate in the language model.

| Question | Single inheritance | Proposed composition |
|---|---|---|
| Primary relationship | “Is a specialized form of” | “Has these capabilities” |
| Parent count | At most one nominal parent | Multiple explicit traits/interfaces |
| Reuse unit | Parent class state and methods | Named behavior contract and selected methods |
| State ownership | Parent may contribute instance state | Traits do not implicitly add instance fields in the initial proposal |
| Override rule | Child method overrides inherited method | Class implementation overrides a provided trait method; two unselected providers are a conflict |
| Constructor behavior | Parent constructor rules remain explicit | Traits/interfaces do not run constructors |
| Compatibility | Existing `extends` remains supported | New syntax is gated behind a later version |
| Best use | Object taxonomy and stateful specialization | Cross-cutting capabilities such as printable, iterable, comparable, or serializable behavior |

The proposal intentionally avoids multiple inheritance. A class has at most one class parent and may compose multiple stateless behavior units. This keeps object layout, constructor order, and `super` behavior distinct from capability reuse.

## 6. Proposed surface syntax

The following syntax is illustrative and is not accepted by the v2.2.0 parser.

### 6.1 Trait with a provided method

```zap
trait Printable:
    fn format(self) -> text:
        return "<value>"

class Report with Printable:
    fn format(self) -> text:
        return self.title
```

A class implementation takes precedence over a provided trait method. The class remains responsible for satisfying every required method.

### 6.2 Interface with a required method

```zap
interface Identifiable:
    fn id(self) -> text

class User implements Identifiable:
    fn id(self) -> text:
        return self.name
```

An interface declares a contract but does not provide an implementation. A class that omits `id` cannot be instantiated or passed where `Identifiable` is required, subject to the final static/dynamic dispatch decision.

### 6.3 Explicit conflict selection

```zap
trait JsonView:
    fn render(self) -> text:
        return json(self.data)

trait TableView:
    fn render(self) -> text:
        return join(self.columns, " | ")

class Report with JsonView, TableView:
    use JsonView.render as render
```

The `use Trait.method as name` form is a proposed explicit selection form. If the final syntax uses another spelling, the semantic requirement remains: a conflict must be resolved at the declaration site, not guessed from source order.

## 7. Method lookup and linearization

The initial lookup rule should be deterministic and shallow enough to explain in diagnostics:

1. Search the concrete class for the requested method.
2. Search explicitly selected trait methods declared by the class.
3. Search composed traits in declaration order only when exactly one provider remains.
4. Search the single class parent using the existing `extends` semantics.
5. Report a missing-method diagnostic if no candidate exists.
6. Report a conflict diagnostic if multiple provided candidates remain without an explicit selection.

An explicit class method always wins over a provided trait method. An explicit selection wins over unselected provided methods. A parent class must not silently override a class-level conflict. No implicit diamond linearization or C3-style multiple-parent order is proposed because the design has one class parent and stateless composed units.

### 7.1 Lookup example

```zap
trait Printable:
    fn format(self):
        return "trait"

class Invoice with Printable:
    fn format(self):
        return "invoice"
```

`Invoice.format` resolves to the class method. If `Invoice` removes its method and composes only `Printable`, the provided trait method is selected. If it composes two traits that both provide `format`, the declaration is rejected until an explicit selection is present.

### 7.2 `super` and explicit trait calls

`super` continues to mean the single parent-class path. A trait method must not become an accidental second parent. If a future design permits calling a selected trait implementation, the call must name the trait explicitly, for example `JsonView.render(self)`, and the checker must verify that the class composes that trait. This rule prevents hidden lookup paths and keeps refactoring observable.

## 8. Visibility and ownership

The proposal preserves the existing distinction between public and private members. A composed unit may expose public methods as part of its contract, but its private helpers remain available only inside that unit unless the final specification explicitly grants friend access. Composition must not turn a private helper into a public class method merely because a public method calls it.

| Member | Declared in | Visible to composing class | Visible to external caller |
|---|---|---:|---:|
| Public required method | Interface/trait contract | Yes | Yes when the receiver contract permits it |
| Public provided method | Trait | Yes | Yes when composed and exported |
| Private trait helper | Trait | No by default | No |
| Class-private method | Class | Yes inside class rules | No |
| Parent protected/public method | Parent class | According to existing inheritance rules | According to existing inheritance rules |

The final implementation must define whether a trait can declare fields. This RFC recommends **no implicit instance fields** in v1 of the feature. State should remain owned by the class, while a trait may require accessor methods such as `name(self)` or `data(self)`.

## 9. Missing and conflicting implementation diagnostics

Diagnostics are part of the language contract, not an afterthought. The implementation must define stable codes before parser/runtime work begins.

| Condition | Proposed code | Required diagnostic content |
|---|---|---|
| Missing required method | `ZAP-TRAIT-001` | Composing class, required method, declaring trait/interface, source span, and suggested implementation signature |
| Conflicting provided methods | `ZAP-TRAIT-002` | Class, method, all provider names/spans, and explicit-selection hint |
| Invalid trait/interface target | `ZAP-TRAIT-003` | Target name, expected contract kind, source span, and available declarations |
| Private member access | `ZAP-TRAIT-004` | Member, declaring unit, caller context, and visibility explanation |
| Invalid explicit selection | `ZAP-TRAIT-005` | Selected unit/method, composition declaration, and valid candidates |
| Unsupported feature version | `ZAP-TRAIT-006` | Syntax, current version, first supported version if approved, and migration hint |

Diagnostics must be emitted consistently by CLI checks, runtime boundaries, and LSP consumers. Each code requires English/Burmese documentation and a durable conformance fixture before the feature is enabled.

## 10. Migration from inheritance

Migration must be opt-in and preserve behavior before it improves structure. The recommended order is:

1. Keep the existing parent class and add tests for the behavior that will become a capability.
2. Extract stateless methods into a trait without changing method names or visibility.
3. Replace inherited calls with explicit class-owned state accessors where necessary.
4. Add `with TraitName` only after the trait contract and conflict checks pass.
5. Keep `extends Parent` when the relationship is still a genuine subtype relationship.
6. Remove duplicated methods only after parity fixtures pass.

### 10.1 Example migration

Before:

```zap
class PrintableReport extends Report:
    fn format(self):
        return json(self.data)
```

After the feature is approved:

```zap
trait JsonPrintable:
    fn format(self):
        return json(self.data)

class PrintableReport extends Report with JsonPrintable:
    pass
```

The migration is not source-compatible in v2.2.0 because `trait`, `with`, and `pass` in this form are only proposed. A later implementation must provide a versioned migration tool or clear diagnostics rather than silently accepting the syntax on an older runtime.

## 11. Static versus dynamic dispatch

This RFC recommends a **hybrid boundary**:

| Call site | Preferred dispatch | Reason |
|---|---|---|
| Statically checked interface-typed parameter | Static conformance check, then direct selected method | Early missing/conflict diagnostics and predictable hot path |
| Concrete class method call | Existing class/parent lookup | Preserves current behavior and compatibility |
| Value annotated `any` or dynamically loaded | Dynamic dispatch through a conformance table | Retains Zap's dynamic boundary without pretending it is statically known |
| LSP completion/hover | Contract metadata from the canonical catalog/AST | Editor behavior must remain parser-owned and deterministic |

Static dispatch here means contract validation, not a promise of whole-program compilation. Dynamic dispatch must preserve stable `NameError`, `TypeError`, or trait-specific diagnostics at the current runtime boundary. The proposal does not require v2.2.0 to add a new type-system phase.

## 12. Package and version compatibility

Traits and interfaces affect public APIs, method lookup, diagnostics, package metadata, and editor tooling. An approved implementation must therefore update the language specification, the bilingual syntax guide, package-author guidance, standard-library stability records, LSP metadata, conformance ownership, and changelog together.

| Change | Compatibility classification |
|---|---|
| Existing `class` and single `extends` behavior | Normative and unchanged in v2.2.0 |
| New `trait`/`interface`/`with` syntax before approval | Rejected by the v2.2.0 parser |
| Approved additive syntax in a later minor release | New feature, with explicit capability/version metadata |
| Changing existing method lookup | Breaking semantics; requires a major-version decision or compatibility layer |
| Removing or changing `extends` | Breaking change; requires migration plan and explicit major-version decision |
| Changing conflict diagnostics after enablement | Compatibility-sensitive; requires stable code and migration notes |

The standard-library stability policy requires public APIs to record stability, introduction release, deprecation, platform, limits, timeout/error, and determinism metadata [3]. A future trait-backed stdlib API must update those records before release.

## 13. Rejected alternatives

### 13.1 Multiple inheritance

Rejected for the initial design because it complicates object layout, constructor order, `super` semantics, diamond lookup, and diagnostics. One nominal parent plus stateless composition gives the language a smaller semantic surface.

### 13.2 Implicit structural typing for every object

Rejected because it would change the meaning of existing annotations and make accidental conformance difficult to diagnose. Structural checks may be considered later for explicit interfaces, but they must be opt-in.

### 13.3 Trait methods selected by source order

Rejected because reordering declarations would silently change behavior. Conflicts must be explicit and reviewable.

### 13.4 Traits that silently add fields

Rejected for the initial version because field ownership, initialization order, serialization, and memory lifecycle would become implicit. Accessor requirements are easier to audit.

### 13.5 Runtime-only conflict errors

Rejected because missing/conflicting implementations should be reported as early as the available information permits, with the same structured contract for CLI and LSP.

### 13.6 Implementing traits before the RFC review

Rejected by project policy. The parser and runtime must not gain broad language syntax until the conformance, specification ownership, and bilingual documentation gates are complete.

## 14. Implementation gates after approval

This RFC is complete as a design milestone only when the following gates are satisfied before implementation begins:

| Gate | Required evidence |
|---|---|
| Specification | Canonical English/Burmese rule sections with ownership IDs |
| Syntax | Parser acceptance/rejection fixtures for every proposed form |
| Lookup | Class, parent, single-provider, conflict, and explicit-selection tests |
| Visibility | Public/private and external-access fixtures |
| Diagnostics | Stable `ZAP-TRAIT-*` codes, JSON fields, CLI/LSP parity tests |
| Migration | Before/after examples and compatibility/deprecation notes |
| Dispatch | Static/dynamic boundary tests and documented limitations |
| Packages | Trait/interface metadata rules for manifest and registry consumers |
| Tooling | Completion, hover, definition, rename, and formatting parity fixtures |
| Platforms | Native Linux, Windows, and macOS behavior where the feature touches runtime state |
| Release | Changelog, bilingual docs, version decision, and full quality gates |

No implementation commit should be accepted by this RFC alone. A later implementation milestone must cite this document and update the specification ownership index.

## 15. Explicit version decision

**Decision for v2.2.0:** Traits, interfaces, composition syntax, new conflict-resolution syntax, and related parser/runtime behavior remain **deferred**. The v2.2.0 release may ship this RFC as a reviewed design record, but it must not advertise the proposed syntax as supported.

**Future decision point:** A later release proposal may enable an additive subset only after this RFC is reviewed, the diagnostics and lookup rules are frozen, the bilingual contract is updated, and conformance fixtures pass on supported targets. Any change to existing inheritance semantics requires a separate compatibility and major-version decision.

## References

[1]: LANGUAGE_SPEC_EN.md — Zap canonical language specification and ownership boundaries.
[2]: MEMORY_MODEL_EN.md — Zap ownership and single-threaded object-field boundaries.
[3]: STDLIB_POLICY_EN.md — Zap public API stability and release policy.
[4]: COMPATIBILITY_CHANGE_TEMPLATE_EN.md — Required compatibility/deprecation change record.
[5]: SPEC_OWNERSHIP_EN.md — Rule-to-section-to-fixture ownership contract.
