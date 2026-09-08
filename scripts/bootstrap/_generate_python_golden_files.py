#!/usr/bin/env python3
"""Generate Python parser golden files for verify_b1_parser_zap_only.sh."""
import json
import os
import subprocess
import sys

PARSER_DIR = "bootstrap/fixtures/parser"
DIAG_DIR = "bootstrap/fixtures/diagnostics"
PYTHON_HOST = "scripts/bootstrap/_run_parser.py"


def run_parser(mode, fixture_path):
    subprocess.run([sys.executable, PYTHON_HOST, mode, fixture_path], check=True)
    with open(".zap-parser-actual.txt", "r", encoding="utf-8") as f:
        return json.load(f)


def generate(mode, fixture_dir, extension):
    generated = []
    for filename in sorted(os.listdir(fixture_dir)):
        if not filename.endswith(".zp"):
            continue
        base = filename[:-3]
        fixture_path = os.path.join(fixture_dir, filename)
        expected_path = os.path.join(fixture_dir, base + extension)

        try:
            actual = run_parser(mode, fixture_path)
        except Exception as e:
            print(f"SKIP {fixture_path}: parser failed: {e}", file=sys.stderr)
            continue

        with open(expected_path, "w", encoding="utf-8") as f:
            json.dump(actual, f, ensure_ascii=False, separators=(",", ":"))
            f.write("\n")
        generated.append(expected_path)
        print(f"Generated {expected_path}")

    return generated


if __name__ == "__main__":
    parser_files = generate("ast", PARSER_DIR, ".python.ast.json")
    diag_files = generate("diagnostics", DIAG_DIR, ".python.json")
    print(f"\nTotal generated: {len(parser_files) + len(diag_files)} files")
