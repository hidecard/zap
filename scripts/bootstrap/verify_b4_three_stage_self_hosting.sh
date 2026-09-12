#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 three-stage self-hosting failed: $*" >&2; exit 1; }
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "prebuilt Zap seed required"
REPORT="${B4_THREE_STAGE_REPORT:-target/b4-three-stage-self-hosting.tsv}"
mkdir -p "$(dirname "$REPORT")"
runner=$(mktemp "$ROOT_DIR/.zap-b4-three-stage.XXXXXX.zp")
out_a=$(mktemp)
out_b=$(mktemp)
trap 'rm -f "$runner" "$out_a" "$out_b"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let source = "fn double(n: number) -> number:\n    return n * 2\n\nlet value = double(5)\nsay value\n"
let first = driver_execute_owned_pipeline(source, "three_stage.zp")
let second = driver_execute_owned_pipeline(source, "three_stage.zp")
let stages = first["stages"]
let stage_shape = len(stages) == 3 and stages[0]["input_kind"] == "source" and stages[0]["output_kind"] == "typed_ir" and stages[1]["input_kind"] == "typed_ir" and stages[1]["output_kind"] == "bytecode" and stages[2]["input_kind"] == "bytecode" and stages[2]["output_kind"] == "execution"
let artifact_shape = len(first["artifacts"]) >= 2 and contains(json(first), "typed_ir") and contains(json(first), "bytecode")
let replay = json(first) == json(second)
say first["stage_chain_valid"]
say stage_shape
say artifact_shape
say replay
EOF

# Run in a minimal environment: the seed itself must not need Cargo/Rust.
minimal_env=(env -i PATH="$PATH" HOME="$HOME" RUSTC= RUSTUP_HOME= CARGO= CARGO_HOME=)
"${minimal_env[@]}" "$SEED" "$(basename "$runner")" > "$out_a"
"${minimal_env[@]}" "$SEED" "$(basename "$runner")" > "$out_b"
cmp "$out_a" "$out_b" || fail "three-stage artifact changed across fresh processes"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out_a")
expected=(true true true true)
[[ "${lines[*]}" == "${expected[*]}" ]] || fail "invalid three-stage output: ${lines[*]}"

seed_sha=$(sha256sum "$SEED" | awk '{print $1}')
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-THREE-STAGE-SELF-HOSTING\nstatus\tpassed\n' >> "$REPORT"
printf 'seed_path\t%s\nseed_sha256\t%s\n' "$SEED" "$seed_sha" >> "$REPORT"
printf 'seed_runtime\tzap-prebuilt\nrust_environment\tunavailable\nstage_count\t3\nstage_chain_valid\ttrue\nartifact_replay_deterministic\ttrue\n' >> "$REPORT"
printf 'B4 three-stage self-hosting gate passed: Rust-free runtime, source->typed-IR->bytecode->execution chain, and fresh-process replay verified\n'
