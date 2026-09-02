#!/usr/bin/env python3
from pathlib import Path
import json
import os
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
LEX_FIXTURES = ['integer_overflow', 'invalid_character', 'unterminated_string']
PARSE_FIXTURES = ['missing_closing_bracket', 'unexpected_closing_bracket', 'missing_assignment', 'missing_function_paren']
PARSER_DIR_FIXTURES = [
    ('invalid_indentation', 'invalid_indentation.json'),
    ('unexpected_indentation', 'unexpected_indentation.json'),
    ('numeric_literals', 'numeric_literals.diagnostics.json'),
]


def run_one(name: str, lexer_only: bool, rel_dir: str = 'bootstrap/fixtures/diagnostics', expected_name: str | None = None):
    source_path = f'{rel_dir}/{name}.zp'
    expected_path = ROOT / rel_dir / (expected_name or f'{name}.json')
    runner = None
    try:
        with tempfile.NamedTemporaryFile('w', suffix='.zp', prefix='.zap-b1-diag-', dir=ROOT, delete=False) as handle:
            runner = Path(handle.name)
            handle.write('import "bootstrap/b1/parser.zp"\n')
            handle.write('import "bootstrap/b1/lexer.zp"\n')
            handle.write(f'let source = read_text("{source_path}")\n')
            if lexer_only:
                handle.write(f'say lex(source, "{source_path}")\n')
            else:
                handle.write(f'let stream = from_json(lex(source, "{source_path}"))\n')
                handle.write(f'say parse_or_diagnostics(source, stream["tokens"], "{source_path}")\n')
        env = os.environ.copy()
        env['PATH'] = str(Path.home() / '.cargo' / 'bin') + ':' + env.get('PATH', '')
        env['RUSTUP_TOOLCHAIN'] = '1.88.0'
        zap_bin = ROOT / 'native' / 'target' / 'release' / 'zap'
        cmd = [str(zap_bin), str(runner)] if zap_bin.exists() else ['cargo', 'run', '--quiet', '--release', '--locked', '--manifest-path', 'native/Cargo.toml', '--', str(runner)]
        proc = subprocess.run(cmd, cwd=ROOT, env=env, text=True, capture_output=True)
        if proc.returncode:
            return False, f'runtime failure: {(proc.stdout + proc.stderr).strip()[-400:]}'
        lines = [line for line in proc.stdout.splitlines() if line.strip()]
        if len(lines) != 1:
            return False, f'framing failure: expected one JSON record, got {len(lines)}'
        actual = json.loads(lines[0])
        expected = json.loads(expected_path.read_text())
        if actual != expected:
            return False, 'diagnostic mismatch'
        return True, ''
    finally:
        if runner:
            runner.unlink(missing_ok=True)


def main():
    failures = []
    for name in LEX_FIXTURES:
        ok, detail = run_one(name, True)
        print(('PASS ' if ok else 'FAIL ') + name + ('' if ok else ': ' + detail))
        if not ok:
            failures.append(name)
    for name in PARSE_FIXTURES:
        ok, detail = run_one(name, False)
        print(('PASS ' if ok else 'FAIL ') + name + ('' if ok else ': ' + detail))
        if not ok:
            failures.append(name)
    for name, expected_name in PARSER_DIR_FIXTURES:
        ok, detail = run_one(name, False, 'bootstrap/fixtures/parser', expected_name)
        print(('PASS ' if ok else 'FAIL ') + name + ('' if ok else ': ' + detail))
        if not ok:
            failures.append(name)
    total = len(LEX_FIXTURES) + len(PARSE_FIXTURES) + len(PARSER_DIR_FIXTURES)
    print(f'B1 diagnostics isolated batch: total={total} passed={total-len(failures)} failed={len(failures)}')
    if failures:
        sys.exit(1)


if __name__ == '__main__':
    main()
