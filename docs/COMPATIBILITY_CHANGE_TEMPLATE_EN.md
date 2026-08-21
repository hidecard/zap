# Compatibility and Deprecation Change Template

Use this template whenever a language, runtime, standard-library, package, or diagnostic behavior changes.

## Change identity

| Field | Value |
|---|---|
| Change ID | `SPEC-NNN` |
| Target release | `vX.Y.Z` |
| Compatibility class | `normative` / `compatibility` / `deprecated` / `rejected` |
| Canonical specification section | `docs/LANGUAGE_SPEC_EN.md#...` and `docs/LANGUAGE_SPEC_MM.md#...` |
| Fixture owner | Repository path plus optional `#fragment` |

## Existing behavior

Describe the previous behavior, including the native and retained legacy result where relevant. State whether the old behavior was documented, accidental, or already classified by the ownership index.

## New behavior

Describe the new normative behavior, accepted inputs, rejected inputs, diagnostics, limits, determinism expectations, and supported platforms. Do not promote legacy acceptance to normative status without an explicit decision.

## Migration and deprecation

For `deprecated` behavior, state the warning or diagnostic code, first release containing the notice, minimum compatibility period, replacement behavior, and removal release decision. For `compatibility` behavior, state why the old behavior remains and which boundary prevents it from silently changing semantics. For `rejected` behavior, provide a fail-closed example.

## Evidence and release gates

List the regression or corpus fixture, ownership-index row, bilingual documentation pair, changelog entry, and verification commands. A change is not release-ready until formatter, strict Clippy, full native tests, focused parity/replay/ownership tests, deployment-policy validation, and target-native CI are green.
