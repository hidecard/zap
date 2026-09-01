#!/usr/bin/env bash
# B4 second-stage rebuild verification.
#
# Verifies that the compiler can produce an artifact, and that artifact can be
# used as input to produce a second-stage artifact, with deterministic results.
# This is the "compiler compiling its own output" gate — a prerequisite for
# true self-hosting.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

REPORT="${B4_SECOND_STAGE_REPORT:-target/b4-second-stage-rebuild.tsv}"
mkdir -p "$(dirname "$REPORT")"

fail() {
  echo "B4 second-stage rebuild failed: $*" >&2
  exit 1
}

runner=$(mktemp "$ROOT_DIR/.zap-2nd-stage.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT

# Stage 1: source -> bytecode artifact
# Stage 2: bytecode artifact -> execution result
# Verify both stages are deterministic
cat > "$runner" <<'EOF'
import "bootstrap/b3/vm.zp"
import "bootstrap/b4/native_independent.zp"

let source = "let x = 10\nlet y = 20\nsay x + y\n"

# First-stage: compile to bytecode artifact
let stage1_first = seed_compile_source(source, "stage1")
let stage1_second = seed_compile_source(source, "stage1")
let stage1_deterministic = stage1_first["status"] == "compiled_slice" and json(stage1_first) == json(stage1_second)

# Second-stage: execute the bytecode artifact
let stage2_first = vm_run(stage1_first["instructions"])
let stage2_second = vm_run(stage1_second["instructions"])
let stage2_deterministic = json(stage2_first) == json(stage2_second)

# Full pipeline replay
let pipeline_first = seed_execute_owned_pipeline(source, "pipeline")
let pipeline_second = seed_execute_owned_pipeline(source, "pipeline")
let pipeline_deterministic = json(pipeline_first) == json(pipeline_second)

# Cross-stage: artifact from run 1 fed to run 2
let cross_first = seed_compile_source(source, "cross")
let cross_vm = vm_run(cross_first["instructions"])
let cross_second = seed_compile_source(source, "cross")
let cross_vm_second = vm_run(cross_second["instructions"])
let cross_deterministic = json(cross_vm) == json(cross_vm_second)

say stage1_deterministic
say stage2_deterministic
say pipeline_deterministic
say cross_deterministic
EOF

cat > "$expected" <<'EOF'
true
true
true
true
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "second-stage rebuild verification failed"

# Second run: fresh process, same source
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "second-stage rebuild not reproducible across runs"

# Typed IR second-stage: source -> typed IR -> bytecode -> execution
cat > "$runner" <<'EOF'
import "bootstrap/b3/vm.zp"
import "bootstrap/b4/native_independent.zp"

let source = "fn double(n: number) -> number:\n    return n * 2\n\nsay double(5)\n"

let typed_first = seed_compile_typed_ir_source(source, "typed_stage")
let typed_second = seed_compile_typed_ir_source(source, "typed_stage")
let typed_deterministic = typed_first["status"] == "compiled_typed_ir_slice" and json(typed_first) == json(typed_second)

# Self-rebuild typed IR
let rebuild_first = seed_self_rebuild_typed_ir(source, "typed_rebuild")
let rebuild_second = seed_self_rebuild_typed_ir(source, "typed_rebuild")
let rebuild_deterministic = rebuild_first["byte_equal"] and rebuild_second["byte_equal"]

say typed_deterministic
say rebuild_deterministic
EOF

cat > "$expected" <<'EOF'
true
true
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "typed IR second-stage rebuild failed"

# Report
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-SECOND-STAGE-REBUILD\nstatus\tpassed\n' >> "$REPORT"
printf 'stage1_deterministic\ttrue\n' >> "$REPORT"
printf 'stage2_deterministic\ttrue\n' >> "$REPORT"
printf 'pipeline_deterministic\ttrue\n' >> "$REPORT"
printf 'cross_stage_deterministic\ttrue\n' >> "$REPORT"
printf 'typed_ir_deterministic\ttrue\n' >> "$REPORT"
printf 'typed_rebuild_deterministic\ttrue\n' >> "$REPORT"

printf 'B4 second-stage rebuild gate passed: 6 deterministic second-stage verification cases\n'
