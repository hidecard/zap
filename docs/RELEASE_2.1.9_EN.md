# Zap v2.1.9 Release Notes

**Release date:** 2026-08-21

Zap v2.1.9 is a focused runtime-safety patch release. It hardens object-field access on the single-threaded `Rc<RefCell>` boundary so borrow conflicts become deterministic structured errors instead of uncontrolled panics.

## Highlights

- Added checked object-field `try_borrow` and `try_borrow_mut` accessors.
- Added stable `BorrowError` diagnostics with compatibility code `ZAP-BORROW-001`, deterministic notes, help text, source-location support, and JSON rendering.
- Made `clear_object_fields()` and `object_field_count()` fallible at the object-field boundary so conflicts are reported rather than panicking.
- Propagated checked borrow failures through recursive JSON conversion, object-field initialization, property assignment, property lookup, and memory validation paths.
- Added regressions for conflicting object borrows, JSON conversion failure propagation, and stable BorrowError metadata.
- Updated the English/Burmese memory model, structured diagnostic model, roadmap, release policy, README onboarding, SECURITY release reference, and type-check conformance baseline.

## Contract boundaries

This patch does not claim a tracing garbage collector, public weak-reference API, process-wide heap telemetry, per-run byte accounting, or automatic reclamation of arbitrary object cycles. Closure `RefCell` ownership, real async scheduling, host I/O isolation, and OS-level sandboxing remain separate roadmap milestones.

## Verification

The native Rust suite, strict formatting and Clippy gates, version consistency validator, positive/negative version regression harness, and cross-platform GitHub Actions matrix are required before publication. The release workflow must validate the tagged source, build Linux x86_64, Windows x86_64, and macOS ARM64 artifacts, verify checksums and signatures, and publish provenance.

See the [English memory model](MEMORY_MODEL_EN.md), [English diagnostic model](DIAGNOSTIC_MODEL_EN.md), and [release version policy](RELEASE_VERSION_POLICY_EN.md) for the normative contracts.
