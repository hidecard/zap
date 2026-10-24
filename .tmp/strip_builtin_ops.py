import io
import pathlib

path = pathlib.Path("bootstrap/b4/c_backend.zp")
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)

# Drop the builtin_* opcode handlers: builtins are `call` sites, not opcodes.
out = []
index = 0
removed = 0
while index < len(lines):
    line = lines[index]
    if line.startswith('    if op == "builtin_'):
        index += 2
        removed += 1
        continue
    out.append(line)
    index += 1

path.write_text("".join(out), encoding="utf-8")
print("removed handlers:", removed)
