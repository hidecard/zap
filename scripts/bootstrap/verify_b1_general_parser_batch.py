#!/usr/bin/env python3
from pathlib import Path
import json
import os
import subprocess
import sys
import tempfile
import difflib

ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / 'bootstrap' / 'fixtures' / 'parser'
NAMES = [
    'arithmetic', 'compound', 'two_declarations', 'unicode_identifier',
    'multi_digit_number', 'negative_number', 'multiplicative_additive',
    'grouped_expression', 'assignment_statement', 'logical_comparison_matrix',
    'simple_function', 'simple_loop', 'simple_class', 'full_expression',
    'three_declarations', 'nested_calls', 'parenthesized_nested',
    'nested_blocks', 'three_argument_call', 'control_flow', 'mixed_top_level',
    'nested_function_blocks', 'nested_class_method', 'mixed_recursive_sequence',
    'while_simple', 'deep_mixed_blocks', 'four_argument_call',
    'parenthesized_not', 'nested_assignment_block',
]


def run_one(name: str):
    source_path = f'bootstrap/fixtures/parser/{name}.zp'
    runner = None
    try:
        with tempfile.NamedTemporaryFile('w', suffix='.zp', prefix='.zap-b1-', dir=ROOT, delete=False) as handle:
            runner = Path(handle.name)
            handle.write('import "bootstrap/b1/parser.zp"\n')
            handle.write(f'say legacy_parse_result(read_text("{source_path}"), "{source_path}")\n')
        env = os.environ.copy()
        env['PATH'] = str(Path.home() / '.cargo' / 'bin') + ':' + env.get('PATH', '')
        env['RUSTUP_TOOLCHAIN'] = '1.88.0'
        zap_bin = ROOT / 'native' / 'target' / 'release' / 'zap'
        cmd = [str(zap_bin), str(runner)] if zap_bin.exists() else ['cargo', 'run', '--quiet', '--release', '--locked', '--manifest-path', 'native/Cargo.toml', '--', str(runner)]
        proc = subprocess.run(cmd, cwd=ROOT, env=env, text=True, capture_output=True)
        if proc.returncode:
            return False, f'runtime failure: {(proc.stdout + proc.stderr).strip()[-300:]}'
        lines = [line for line in proc.stdout.splitlines() if line.strip()]
        if len(lines) != 1:
            return False, f'framing failure: expected one JSON record, got {len(lines)}'
        actual = json.loads(lines[0])
        expected = json.loads((FIXTURE_DIR / f'{name}.ast.json').read_text())
        if actual != expected:
            if os.environ.get('B1_VERBOSE_DIFF') == '1':
                actual_text = json.dumps(actual, indent=2, sort_keys=True).splitlines()
                expected_text = json.dumps(expected, indent=2, sort_keys=True).splitlines()
                diff = '\\n'.join(difflib.unified_diff(expected_text, actual_text, fromfile='expected', tofile='actual', lineterm=''))
                return False, 'AST mismatch\\n' + diff
            return False, 'AST mismatch'
        return True, ''
    finally:
        if runner:
            runner.unlink(missing_ok=True)


def main():
    failures = []
    for name in NAMES:
        ok, detail = run_one(name)
        if ok:
            print(f'PASS {name}')
        else:
            failures.append((name, detail))
            print(f'FAIL {name}: {detail}')
    print(f'B1 general parser isolated batch: total={len(NAMES)} passed={len(NAMES)-len(failures)} failed={len(failures)}')
    if failures:
        sys.exit(1)


if __name__ == '__main__':
    main()
