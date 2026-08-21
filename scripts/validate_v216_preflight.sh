#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 - <<'PY'
from pathlib import Path
text = Path('.github/workflows/ci.yml').read_text()
required = [
    'toolchain: 1.75.0',
    'typecheck_p0_conformance_matrix_tc001_to_tc005',
    'typecheck_p1_conformance_tc006_to_tc008',
    'typecheck_p1_conformance_tc009_conditional_expression',
    'typecheck_p1_conformance_tc010_alias_wrapper_narrowing',
    'lsp_diagnostics_match_cli_type_error_contract',
]
missing = [item for item in required if item not in text]
if missing:
    raise SystemExit(f'missing CI contract entries: {missing}')
print('CI contract entries: passed')
PY
cargo fmt --manifest-path native/Cargo.toml --all -- --check
cargo test --manifest-path native/Cargo.toml --test core typecheck_p0_conformance_matrix_tc001_to_tc005 --all-features -- --exact --nocapture
cargo test --manifest-path native/Cargo.toml --test core typecheck_p1_conformance_tc006_to_tc008 --all-features -- --exact --nocapture
cargo test --manifest-path native/Cargo.toml --test core typecheck_p1_conformance_tc009_conditional_expression --all-features -- --exact --nocapture
cargo test --manifest-path native/Cargo.toml --test core typecheck_p1_conformance_tc010_alias_wrapper_narrowing --all-features -- --exact --nocapture
cargo test --manifest-path native/Cargo.toml --bin zap lsp_diagnostics_match_cli_type_error_contract --all-features -- --nocapture
