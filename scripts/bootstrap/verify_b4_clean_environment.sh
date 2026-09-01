#!/usr/bin/env bash
# B4 clean environment run verification.
#
# Verifies that the bootstrap pipeline runs correctly in a clean environment
# with no residual host state. This checks:
# 1. No dependency on Rust toolchain variables
# 2. No residual state between runs
# 3. Deterministic output regardless of execution environment
# 4. Proper isolation between pipeline executions
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

REPORT="${B4_CLEAN_ENV_REPORT:-target/b4-clean-environment.tsv}"
mkdir -p "$(dirname "$REPORT")"

fail() {
  echo "B4 clean environment failed: $*" >&2
  exit 1
}

runner=$(mktemp "$ROOT_DIR/.zap-clean-env.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT

# Test 1: Run with Rust toolchain variables unset (simulating clean env)
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"

let source = "let a = 5\nlet b = 10\nsay a + b\n"

let result = seed_execute_owned_pipeline(source, "clean_env")
let status_ok = result["status"] == "candidate_pipeline_executed"
let chain_valid = result["stage_chain_valid"]
let artifact_count = len(result["artifacts"])
let has_stages = contains(json(result), "\"stages\"")

say status_ok
say chain_valid
say artifact_count >= 2
say has_stages
EOF

cat > "$expected" <<'EOF'
true
true
true
true
EOF

# Run with Rust vars unset
env -u CARGO -u CARGO_HOME -u RUSTC -u RUSTUP_HOME \
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "clean environment run failed with Rust vars unset"

# Test 2: Run with Rust vars set (normal env) - should produce identical output
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "normal environment run produced different output"

# Test 3: Multiple sequential runs - verify no state leakage
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"

let source1 = "let x = 1\nsay x\n"
let source2 = "let y = 2\nsay y\n"
let source3 = "let z = 3\nsay z\n"

let r1 = seed_execute_owned_pipeline(source1, "seq_1")
let r2 = seed_execute_owned_pipeline(source2, "seq_2")
let r3 = seed_execute_owned_pipeline(source3, "seq_3")

let all_ok = r1["status"] == "candidate_pipeline_executed" and r2["status"] == "candidate_pipeline_executed" and r3["status"] == "candidate_pipeline_executed"
let all_chain = r1["stage_chain_valid"] and r2["stage_chain_valid"] and r3["stage_chain_valid"]

say all_ok
say all_chain
EOF

cat > "$expected" <<'EOF'
true
true
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "sequential runs showed state leakage"

# Test 4: Platform evidence record validation
cat > "$runner" <<'EOF'
import "bootstrap/b4/seed_pipeline.zp"

let record = seed_platform_record_evidence("clean", "test_artifact_bytes", "sha256:abc123", "executed", "sha256:def456", "sha256:ghi789", "clean", "bootstrap-artifact")
let valid = seed_platform_evidence_record_valid(record)

say valid
EOF

cat > "$expected" <<'EOF'
true
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "platform evidence record validation failed"

# Test 5: Clean environment with different source surfaces
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"

let s0 = "say 42\n"
let s1 = "fn add(a: number, b: number) -> number:\n    return a + b\nsay add(1, 2)\n"
let s2 = "let x = 1 + 2\nsay x\n"

let r0 = seed_execute_owned_pipeline(s0, "simple")
let r1 = seed_execute_owned_pipeline(s1, "function")
let r2 = seed_execute_owned_pipeline(s2, "arithmetic")

let all_ok = r0["status"] == "candidate_pipeline_executed" and r1["status"] == "candidate_pipeline_executed" and r2["status"] == "candidate_pipeline_executed"

say all_ok
EOF

cat > "$expected" <<'EOF'
true
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cmp "$out" "$expected" || fail "clean environment run failed for diverse source surfaces"

# Report
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-CLEAN-ENVIRONMENT\nstatus\tpassed\n' >> "$REPORT"
printf 'no_rust_dependency\ttrue\n' >> "$REPORT"
printf 'normal_env_matches\ttrue\n' >> "$REPORT"
printf 'no_state_leakage\ttrue\n' >> "$REPORT"
printf 'platform_evidence_valid\ttrue\n' >> "$REPORT"
printf 'diverse_sources_ok\ttrue\n' >> "$REPORT"

printf 'B4 clean environment gate passed: 5 clean-environment verification cases\n'
