#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-a11-vm-contract.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b3/vm.zp"
let normal = vm_execution_contract([{"op": "const", "value": 7}, {"op": "print"}, {"op": "halt"}])
let underflow = vm_execution_contract([{"op": "print"}])
let unknown = vm_execution_contract([{"op": "not-an-op"}])
say normal["status"]
say normal["valid"]
say normal["deterministic"]
say normal["first"]["output"][0]
say underflow["status"]
say underflow["error_terminal"]
say underflow["first"]["error"]
say unknown["status"]
say unknown["first"]["error"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_vm_contract true true 7 candidate_vm_contract true stack_underflow candidate_vm_contract unknown_opcode:not-an-op" ]]; then
  echo "unexpected VM contract output: ${lines[*]}" >&2
  exit 1
fi
printf 'A11 candidate VM contract gate passed: deterministic normal execution, terminal stack failure, and deny-by-default opcode rejection\n'
