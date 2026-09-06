#!/usr/bin/env python3
"""Generate missing parser fixture golden files using the Python parser host."""
import json
import os
import subprocess
import sys

PARSER_DIR = "bootstrap/fixtures/parser"
PYTHON_HOST = "host/zap-parser-host/parser.py"

def run_parser(mode, fixture_path):
    result = subprocess.run(
        ["python3", PYTHON_HOST, mode, fixture_path],
        capture_output=True,
    )
    raw = result.stdout
    if raw.startswith(b"\xff\xfe"):
        raw = raw[2:]
    if raw.startswith(b"\xef\xbb\xbf"):
        raw = raw[3:]
    return json.loads(raw)

def main():
    generated = []
    skipped = []
    
    for filename in sorted(os.listdir(PARSER_DIR)):
        if not filename.endswith(".zp"):
            continue
        
        base = filename[:-3]
        fixture_path = os.path.join(PARSER_DIR, filename)
        
        ast_path = os.path.join(PARSER_DIR, base + ".ast.json")
        diag_path = os.path.join(PARSER_DIR, base + ".diagnostics.json")
        json_path = os.path.join(PARSER_DIR, base + ".json")
        
        has_ast = os.path.exists(ast_path)
        has_diag = os.path.exists(diag_path)
        has_json = os.path.exists(json_path)
        
        if has_ast and (has_diag or has_json):
            skipped.append(f"{filename}: already has golden files")
            continue
        
        try:
            output = run_parser("ast", fixture_path)
        except Exception as e:
            print(f"SKIP {filename}: parser failed: {e}", file=sys.stderr)
            skipped.append(f"{filename}: parser failed")
            continue
        
        if "ast" in output:
            if not has_ast:
                with open(ast_path, "w", encoding="utf-8") as f:
                    json.dump(output, f, ensure_ascii=False, separators=(",", ":"))
                    f.write("\n")
                generated.append(ast_path)
            if not has_diag and not has_json:
                diag_output = {"diagnostics": [], "kind": "zap.diagnostics", "schema_version": 1, "source_name": output.get("source_name", fixture_path)}
                diag_path_to_write = diag_path if not has_json else json_path
                if not has_json:
                    with open(diag_path, "w", encoding="utf-8") as f:
                        json.dump(diag_output, f, ensure_ascii=False, separators=(",", ":"))
                        f.write("\n")
                        generated.append(diag_path)
        elif "diagnostics" in output:
            if not has_diag and not has_json:
                target = diag_path if not has_json else json_path
                with open(target, "w", encoding="utf-8") as f:
                    json.dump(output, f, ensure_ascii=False, separators=(",", ":"))
                    f.write("\n")
                generated.append(target)
            if not has_ast:
                ast_path_to_write = ast_path
                # No AST for invalid input
        else:
            print(f"SKIP {filename}: unexpected output format", file=sys.stderr)
            skipped.append(f"{filename}: unexpected output")
            continue
    
    print(f"Generated {len(generated)} golden files")
    for path in generated:
        print(f"  {path}")
    
    if skipped:
        print(f"\nSkipped {len(skipped)} fixtures:")
        for msg in skipped:
            print(f"  {msg}")

if __name__ == "__main__":
    main()
