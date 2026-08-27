#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-trait-table.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b2/typecheck.zp"
let result = from_json(check("trait Printable:\n    fn render(self) -> text:\n        return \"printable\"\nclass Report with Printable:\n    fn render(self) -> text:\n        return \"report\"\ntrait TableView:\n    fn render(self) -> text:\n        return \"table\"\nclass Selected with Printable, TableView:\n    use Printable.render as render\ntrait NeedsId:\n    fn id(self) -> text\nclass Missing with NeedsId:\n    fn marker(self):\n        return 0", "table.zp"))
say result["ok"]
say len(result["implementations"])
say result["implementations"][0]["methods"][0]["status"]
say result["implementations"][1]["methods"][0]["status"]
say result["implementations"][3]["methods"][0]["status"]
say result["diagnostics"][0]["code"]
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["false", "4", "class_override", "explicit_selection", "missing", "ZAP-TRAIT-001"]:
    raise SystemExit(f"unexpected implementation table output: {lines!r}")
PY
printf 'B2 trait implementation-table gate passed: override, selection, missing obligation, and diagnostic\n'
