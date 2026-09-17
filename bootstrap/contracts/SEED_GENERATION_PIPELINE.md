# Zap-Produced Seed Generation Pipeline

## Overview

This document describes the pipeline for generating Zap-produced Rust-free seed binaries for B4 self-hosting certification. The pipeline uses the Python seed compiler and C backend to produce native binaries without invoking Rust/Cargo.

## Pipeline Stages

### Stage 1: Source Compilation to Bytecode

**Input:** Zap source files (`bootstrap/b1/`, `bootstrap/b2/`, `bootstrap/b3/`, `bootstrap/b4/`)
**Tool:** `host/zap-bootstrap/compile.py`
**Output:** Bytecode and metadata

```bash
python3 -c "import json, sys; sys.path.insert(0, 'host/zap-bootstrap'); from compile import compile_program; source = open('<source.zp>', encoding='utf-8-sig').read(); json.dump(compile_program(source), open('<bytecode.json>', 'w', encoding='utf-8'), indent=2, sort_keys=True)"
```

### Stage 2: Bytecode to C Code Generation

**Input:** Bytecode from Stage 1  
**Tool:** `host/zap-bootstrap/c_backend.py`  
**Output:** Self-contained C source file

```python
import json
import sys

sys.path.insert(0, "host/zap-bootstrap")
from c_backend import emit_c

with open("<bytecode.json>", encoding="utf-8") as source:
    program = json.load(source)
emit_c(program, "output.c")
```

### Stage 3: Native Binary Compilation

**Input:** C source file from Stage 2
**Tool:** Platform C compiler (gcc/clang on Linux/macOS, cl.exe on Windows)
**Output:** Native executable

**Linux/macOS:**
```bash
gcc -O2 -o zap output.c
```

**Windows (MSVC):**
```powershell
cl.exe /O2 /Brepro /Fe:zap.exe output.c
```

## Platform-Specific Commands

### Linux x86_64

```bash
# 1. Compile Zap source to bytecode
python3 host/zap-bootstrap/compile.py bootstrap/fixtures/b4/c_backend_seed.zp --output seed_bytecode.json

# 2. Generate C code
python3 -c "
import sys
sys.path.insert(0, 'host/zap-bootstrap')
from c_backend import emit_c
import json
program = json.load(open('seed_bytecode.json'))
emit_c(program, 'seed.c')
"

# 3. Compile to native binary
gcc -O2 -o target/seeds/zap-linux-x86_64 seed.c

# 4. Verify hash
sha256sum target/seeds/zap-linux-x86_64
```

### Windows x86_64

```powershell
# 1. Compile Zap source to bytecode
python3 host/zap-bootstrap/compile.py bootstrap/fixtures/b4/c_backend_seed.zp --output seed_bytecode.json

# 2. Generate C code
python3 -c "
import sys
sys.path.insert(0, 'host/zap-bootstrap')
from c_backend import emit_c
import json
program = json.load(open('seed_bytecode.json'))
emit_c(program, 'seed.c')
"

# 3. Compile to native binary with MSVC
cl.exe /O2 /Brepro /Fe:target\seeds\zap-windows-x86_64.exe seed.c

# 4. Verify hash
Get-FileHash target\seeds\zap-windows-x86_64.exe -Algorithm SHA256
```

### macOS ARM64

```bash
# 1. Compile Zap source to bytecode
python3 host/zap-bootstrap/compile.py bootstrap/fixtures/b4/c_backend_seed.zp --output seed_bytecode.json

# 2. Generate C code
python3 -c "
import sys
sys.path.insert(0, 'host/zap-bootstrap')
from c_backend import emit_c
import json
program = json.load(open('seed_bytecode.json'))
emit_c(program, 'seed.c')
"

# 3. Compile to native binary
clang -O2 -arch arm64 -o target/seeds/zap-macos-arm64 seed.c

# 4. Verify hash
shasum -a 256 target/seeds/zap-macos-arm64
```

## Verification

### Acceptance Testing

Run the C backend acceptance matrix to verify the generated seed:

```bash
# On each platform
B4_C_BACKEND_ACCEPTANCE_REPORT=target/b4-c-backend-acceptance.tsv \
  python3 host/zap-bootstrap/verify_b4_c_backend_acceptance.py
```

### Cross-Platform Determinism

Compare emitted C and stdout hashes across platforms:

```bash
# After all platform reports are generated
B4_C_BACKEND_REPORT_DIR=target/c-backend-reports \
  B4_C_BACKEND_CROSS_PLATFORM_REPORT=target/b4-c-backend-cross-platform.tsv \
  python3 host/zap-bootstrap/verify_b4_c_backend_cross_platform.py
```

### Clean Environment Verification

Verify the seed runs without Rust/Cargo environment variables:

```bash
ZAP_BOOTSTRAP_BIN=target/seeds/zap-linux-x86_64 \
  scripts/bootstrap/verify_b4_clean_environment.sh
```

## Seed Provenance Metadata

Each generated seed should include a `SEED.tsv` metadata file:

```tsv
commit	<git-commit-sha>
platform	<target-triple>
artifact	<binary-name>
built_with	python3-c-backend
rust_free_provenance	true
certification_ready	false
status	zap-produced-seed
bytecode_digest	<sha256-of-bytecode>
c_source_digest	<sha256-of-c-source>
native_digest	<sha256-of-binary>
timestamp	<iso8601-timestamp>
```

## Reproducibility

### Deterministic Build Requirements

1. **Bytecode Determinism:** Same source must produce identical bytecode
2. **C Code Determinism:** Same bytecode must produce identical C code
3. **Native Build Determinism:** Platform-specific flags ensure reproducible binaries
   - Linux/macOS: `-O2` (consistent across gcc/clang)
   - Windows: `/Brepro` (MSVC reproducible builds)

### Verification of Reproducibility

```bash
# Build twice independently
python3 -c "import json, sys; sys.path.insert(0, 'host/zap-bootstrap'); from compile import compile_program; json.dump(compile_program(open('seed.zp', encoding='utf-8-sig').read()), open('seed1.json', 'w', encoding='utf-8'), sort_keys=True)"
python3 -c "import json, sys; sys.path.insert(0, 'host/zap-bootstrap'); from compile import compile_program; json.dump(compile_program(open('seed.zp', encoding='utf-8-sig').read()), open('seed2.json', 'w', encoding='utf-8'), sort_keys=True)"
diff seed1.json seed2.json

python3 -c "import json, sys; sys.path.insert(0, 'host/zap-bootstrap'); from c_backend import emit_c; emit_c(json.load(open('seed1.json', encoding='utf-8')), 'seed1.c')"
python3 -c "import json, sys; sys.path.insert(0, 'host/zap-bootstrap'); from c_backend import emit_c; emit_c(json.load(open('seed2.json', encoding='utf-8')), 'seed2.c')"
diff seed1.c seed2.c

# Compile twice
gcc -O2 -o seed1 seed1.c
gcc -O2 -o seed2 seed2.c
cmp seed1 seed2  # Should be identical
```

## CI Integration

The pipeline is integrated into `.github/workflows/ci.yml`:

1. **c-backend job:** Runs C backend acceptance on each platform
2. **b4-c-backend-cross-platform job:** Aggregates and compares cross-platform reports
3. **b4-platform-evidence job:** Uses platform seeds for B4 verification gates

## Ownership Transition

### Current State
- **Reference owner:** Rust (`native/src`)
- **Production path:** Python seed compiler + C backend
- **Status:** Schema v2 acceptable provenance, not yet Zap-owned

### Target State
- **Reference owner:** Zap (`bootstrap/b1..b4`)
- **Production path:** Zap-owned compiler driver → C backend
- **Status:** Zap-owned, B4 certified

### Migration Path

1. **Audit Python host modules:** Identify production logic vs. proof-only logic
2. **Create Zap-owned equivalents:** Migrate backend lowering, VM execution, package/build
3. **Wire through compiler_driver.zp:** Integrate Zap-owned implementations
4. **Pass B4-FULL rows:** Verify all acceptance rows through Zap-owned path
5. **Update ownership records:** Reflect transition in `bootstrap/contracts/OWNERS.tsv`

## Safety Guarantees

The pipeline ensures:

1. **No Rust invocation:** No `cargo`, `rustc`, or `rustup` in the compilation chain
2. **Deterministic output:** Byte-for-byte reproducible builds
3. **Platform primitives only:** Only python3, gcc/clang, make are used
4. **Clean environment:** Works with Rust/Cargo environment variables removed
5. **Cross-platform parity:** Identical C and stdout hashes across platforms

## Troubleshooting

### C Backend Fails to Compile

**Issue:** Native C compiler not found or incompatible flags
**Solution:** Install platform-specific compiler and verify flags
- Linux: `sudo apt-get install gcc`
- macOS: `xcode-select --install`
- Windows: Visual Studio Build Tools with MSVC

### Hash Mismatch Across Platforms

**Issue:** Different C code or output across platforms
**Solution:** Check for platform-specific conditional code in C backend or fixtures

### Clean Environment Test Fails

**Issue:** Seed requires Rust/Cargo environment variables
**Solution:** Verify seed was generated without Rust dependencies

## References

- B4 Contract: `bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml`
- Seed Production Plan: `docs/SEED_PRODUCTION_PLAN.md`
- C Backend: `host/zap-bootstrap/c_backend.py`
- Seed Compiler: `host/zap-bootstrap/compile.py`
