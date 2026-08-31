# Rust-independence roadmap

## Status

Zap is currently at bootstrap stage **B0**.  The Rust native compiler and
runtime remain the authoritative implementation for complete language
semantics, diagnostics, package/build behaviour, and supported release
artifacts.  No release or documentation may describe Zap as fully self-hosted
until acceptance gates A1 through A13 in
[the self-hosting contract](COMPILER_SELF_HOSTING_A_ACCEPTANCE_EN.md) pass.

There is now a separately verified, Rust-free seed path:

```text
Zap source (supported seed subset)
  -> host/zap-bootstrap/compile.py
  -> bytecode
  -> host/zap-vm-host/run.py
  -> output
```

`scripts/bootstrap/verify_non_rust_seed_pipeline.sh` runs this path with the
usual Rust toolchain variables removed.  It covers arithmetic, branches,
loops, calls, recursion, closures, classes/methods, and caught raises in small
fixtures.  It is evidence for a non-Rust execution path only; the seed hosts
are Python implementations and are not the final Zap-owned compiler/runtime.

## Ownership plan

| Phase | Deliverable | Acceptance boundary |
|---|---|---|
| 1. Independent seed | Keep the source-to-bytecode-to-VM seed gate green in CI. | No Cargo/rustc/rustup process is needed for supported seed fixtures. |
| 2. Canonical front end | Make the Zap B1 lexer/parser consume arbitrary source rather than fixture shapes; add valid, malformed, Unicode, indentation, span, and diagnostic differential corpora. | A7/A8 candidate/reference parity is measured and deterministic. |
| 3. Type ownership | Complete B2 inference, aliases, generics, flow/mutation invalidation, and diagnostics from parser AST. | A1--A6 acceptance matrices pass, including negative cases. |
| 4. Compiler ownership | Replace source-string routing with a canonical AST to typed-IR producer, then lower every supported AST form. | A9 artifacts are deterministic and semantically match the reference. |
| 5. Runtime ownership | Execute produced bytecode in the Zap-owned VM with calls, closures, classes, collections, errors, limits, and package/build behaviour. | A10/A11 differential, limit, and security gates pass. |
| 6. Self rebuild | Build the documented seed with Zap, rebuild the compiler with that compiler, and prove byte-identical/reproducible artifacts on release targets. | A12/A13 pass on Linux x86_64, macOS ARM64, and Windows x86_64. |

## Working rules

- Every increment needs paired positive and negative fixtures, deterministic
  replay, and an explicit candidate/reference comparison.
- A green seed-pipeline test never transfers ownership from Rust by itself.
- Remove a Rust stage only after its replacement has passed its gate; do not
  silently swap a reference implementation.
- The final bootstrap toolchain must be Zap-owned.  Python is allowed only as
  a temporary seed host and test harness until the corresponding Zap component
  is executable independently.

## Commands

```bash
make bootstrap-non-rust-test
# or
bash scripts/bootstrap/verify_non_rust_seed_pipeline.sh
```

See also the [Burmese roadmap](RUST_INDEPENDENCE_ROADMAP_MM.md) and the
[self-hosting acceptance contract](COMPILER_SELF_HOSTING_A_ACCEPTANCE_EN.md).
