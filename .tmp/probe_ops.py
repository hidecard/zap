"""Inspect the C backend's supported opcodes and builtin specifications."""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(ROOT, "host", "zap-bootstrap"))

src = open(os.path.join(ROOT, "host", "zap-bootstrap", "c_backend.py"), encoding="utf-8").read()
ops = sorted(set(re.findall(r'op == "([a-z_]+)"', src)))
print("opcode count:", len(ops))
print("opcodes:", ops)

import c_backend  # noqa: E402

print("builtins:", sorted(c_backend._BUILTIN_SPECS))

import compile as seed  # noqa: E402

sample = (
    "let m = {\"a\": 1}\n"
    "let o = some(1)\n"
    "let n = none()\n"
    "let e = error(\"x\")\n"
    "let t = await(1)\n"
    "say json(m)\n"
    "say str(len([1,2]))\n"
    "say option_is_some(o)\n"
    "say option_is_none(n)\n"
    "say option_unwrap_or(n, 5)\n"
    "say is_error(e)\n"
    "let args = argv()\n"
    "say argc()\n"
)
for i, instr in enumerate(seed.compile_program(sample)):
    print(i, instr)