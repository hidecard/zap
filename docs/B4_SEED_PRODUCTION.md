# B4 Seed Production Requirements

## Overview

B4 self-hosting certification requires a verified prebuilt `ZAP_BOOTSTRAP_BIN` that can compile the Zap compiler source code without Rust/Cargo. This document describes the requirements and process for producing such a seed.

## Seed Requirements

A verified `ZAP_BOOTSTRAP_BIN` must satisfy:

1. **Executable**: The binary must be executable on the target platform (Linux x86_64, macOS ARM64, or Windows x86_64)
2. **Version response**: Must respond to `--version` with a valid version string
3. **Driver contract**: Must report `driver_contract_status() = "owned"` when queried via `bootstrap/b4/compiler_driver.zp`
4. **Pipeline execution**: Must successfully execute `driver_execute_owned_pipeline` on simple source
5. **Determinism**: Must produce byte-identical output across fresh processes
6. **Module resolution**: Must resolve modules via `driver_resolve_modules`

## Production Process

### Stage 1: Build from Rust Reference

```bash
# Build the native binary from the Rust reference implementation
cargo build --release --locked --manifest-path native/Cargo.toml

# Verify the binary
./native/target/release/zap --version
./native/target/release/zap -c 'import "bootstrap/b4/compiler_driver.zp"; say driver_contract_status()'
```

### Stage 2: Bootstrap Seed Validation

```bash
# Run seed preflight validation
ZAP_BOOTSTRAP_BIN=./native/target/release/zap bash scripts/bootstrap/verify_b4_seed_preflight.sh

# Run driver-owned pipeline validation
bash scripts/bootstrap/verify_b4_driver_owned_pipeline.sh
bash scripts/bootstrap/verify_b4_driver_source_to_vm.sh
```

### Stage 3: Cross-Platform Production

The seed must be produced on each target platform:
- Linux x86_64: Build on Linux x86_64
- macOS ARM64: Build on macOS ARM64
- Windows x86_64: Build on Windows x86_64

Each platform must produce identical behavior for the same source input.

### Stage 4: Self-Rebuild Validation

```bash
# Set the verified seed
export ZAP_BOOTSTRAP_BIN=/path/to/verified/seed

# Run two-stage rebuild
bash scripts/bootstrap/verify_b4_second_stage_rebuild.sh

# Run three-stage rebuild
bash scripts/bootstrap/verify_b4_self_rebuild_ast_36.sh

# Run clean environment validation
bash scripts/bootstrap/verify_b4_clean_environment.sh
```

## Verification Gates

Once a seed is produced, the following gates must pass:

| Gate | Script | Requirement |
|------|--------|-------------|
| Seed preflight | `verify_b4_seed_preflight.sh` | Version, contract status, pipeline, determinism, modules |
| Byte determinism | `verify_b4_byte_determinism.sh` | Byte-for-byte identical artifacts |
| Second-stage rebuild | `verify_b4_second_stage_rebuild.sh` | Two-stage rebuild produces identical output |
| Clean environment | `verify_b4_clean_environment.sh` | No Rust toolchain required |
| Driver-owned pipeline | `verify_b4_driver_owned_pipeline.sh` | Driver functions work through seed |
| Driver source-to-VM | `verify_b4_driver_source_to_vm.sh` | Source-to-VM through driver |

## Certification Criteria

B4 certification requires:
1. Verified seed produced on all three platforms
2. All verification gates pass on each platform
3. Artifact manifests are byte-identical across platforms
4. No Rust/Cargo fallback in any gate
5. Independent re-run of `verify_b4_evidence.sh` passes

## Current Blocker

The current checkout does not have a verified `ZAP_BOOTSTRAP_BIN`. All seed-dependent gates are blocked until a seed is produced and validated through `verify_b4_seed_preflight.sh`.
