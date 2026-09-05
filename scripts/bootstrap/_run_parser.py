#!/usr/bin/env python3
"""Run a parser mode and write output to a file."""
import json
import subprocess
import sys

PYTHON_HOST = "host/zap-parser-host/parser.py"
OUTPUT_FILE = ".zap-parser-actual.txt"

mode = sys.argv[1]
fixture = sys.argv[2]

result = subprocess.run(
    ["python3", PYTHON_HOST, mode, fixture],
    capture_output=True,
)
raw = result.stdout
# Strip UTF-16 LE BOM if present (PowerShell adds it on Windows)
if raw.startswith(b"\xff\xfe"):
    raw = raw[2:]
# Also handle UTF-8 BOM
if raw.startswith(b"\xef\xbb\xbf"):
    raw = raw[3:]

try:
    data = json.loads(raw)
except json.JSONDecodeError as e:
    print(f"INVALID_JSON: {e}", file=sys.stderr)
    sys.exit(1)

with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
    json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
    f.write("\n")
