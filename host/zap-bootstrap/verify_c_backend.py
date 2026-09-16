#!/usr/bin/env python3
"""Rust-free verifier for the Zap C backend.

Compiles real Zap source (a small subset) using the C backend and runs the
resulting native executable, asserting expected output. Requires only Python 3
and a system C compiler (gcc/clang) -- no Rust dependency.
"""
import os
import subprocess
import sys
import tempfile
import shutil

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c_backend import emit_c, compile_c  # noqa: E402
from compile import compile_program  # noqa: E402


_PROGRAMS = [
    # 1. function definition + call + arithmetic
    ("fn add(a, b):\n    return a + b\nlet x = add(2, 3)\nsay x\n", ["5"]),

    # 2. while loop with accumulator
    ("let i = 0\nlet total = 0\nwhile i < 5:\n    total = total + i\n    i = i + 1\nsay total\n", ["10"]),

    # 3. if / else branch
    ("if 2 < 3:\n    say 1\nelse:\n    say 2\n", ["1"]),

    # 4. recursion (factorial) -- exercises nested call frames
    ("fn fact(n):\n    if n == 0:\n        return 1\n    return n * fact(n - 1)\nsay fact(5)\n", ["120"]),

    # 5. string output
    ('say "hi"\n', ["hi"]),

    # 6. list literal + indexing
    ("let xs = [10, 20, 30]\nsay xs[0]\nsay xs[2]\n", ["10", "30"]),

    # 7. len() builtin
    ("let xs = [1, 2, 3]\nsay len(xs)\n", ["3"]),

    # 8. list literal + loop
    ("let values = [1, 2, 3]\nlet i = 0\nwhile i < len(values):\n    say values[i]\n    i = i + 1\n", ["1", "2", "3"]),

    # 9. for loop over list
    ("let values = [1, 2, 3]\nfor x in values:\n    say x\n", ["1", "2", "3"]),

    # 10. for loop with accumulator
    ("let total = 0\nfor x in [1, 2, 3, 4]:\n    total = total + x\nsay total\n", ["10"]),
]


def run_c_backend(source):
    """Compile Zap source to C, then to native executable, and run it."""
    with tempfile.TemporaryDirectory() as tmpdir:
        # Compile to bytecode
        program = compile_program(source)
        
        # Emit C code
        c_path = os.path.join(tmpdir, "program.c")
        emit_c(program, c_path)
        
        # Compile to native executable
        exe_path = os.path.join(tmpdir, "program.exe" if sys.platform == "win32" else "program")
        compile_c(c_path, exe_path)
        
        # Run the executable
        result = subprocess.run(
            [exe_path],
            capture_output=True,
            text=True,
            check=True
        )
        
        # Parse output lines
        output = [line.strip() for line in result.stdout.strip().split("\n") if line.strip()]
        return output


def main():
    import sys
    import os
    if sys.platform == "win32":
        import codecs
        sys.stdout = codecs.getwriter("utf-8")(sys.stdout.buffer, 'strict')
        sys.stderr = codecs.getwriter("utf-8")(sys.stderr.buffer, 'strict')
    
    print("C Backend Verification")
    print("=" * 50)
    
    passed = 0
    failed = 0
    skipped = 0
    
    # Prepare TSV report - use repository root target directory
    repo_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    report_dir = os.path.join(repo_root, "target")
    os.makedirs(report_dir, exist_ok=True)
    report_path = os.path.join(report_dir, "c-backend-verification.tsv")
    
    with open(report_path, "w", encoding="utf-8") as report:
        report.write("schema_version\t1\n")
        report.write("contract_id\tC-BACKEND-VERIFICATION\n")
        report.write("program_id\tstatus\texpected\tactual\terror\n")
        
        for idx, (source, expected) in enumerate(_PROGRAMS, 1):
            print(f"\nProgram {idx}:")
            print(f"Source:\n{source}")
            print(f"Expected: {expected}")
            
            try:
                output = run_c_backend(source)
                print(f"Actual: {output}")
                
                if output == expected:
                    print("PASS")
                    passed += 1
                    report.write(f"program_{idx}\tpass\t{expected}\t{output}\t\n")
                else:
                    print("FAIL - output mismatch")
                    failed += 1
                    report.write(f"program_{idx}\tfail\t{expected}\t{output}\toutput_mismatch\n")
            except Exception as e:
                print(f"FAIL - {e}")
                failed += 1
                report.write(f"program_{idx}\tfail\t{expected}\t\t{str(e)}\n")
        
        report.write(f"summary\t{passed}\t{failed}\t{skipped}\n")
    
    print("\n" + "=" * 50)
    print(f"Results: {passed} pass, {failed} fail, {skipped} skip")
    print(f"Report written to: {report_path}")
    
    if failed > 0:
        sys.exit(1)
    sys.exit(0)


if __name__ == "__main__":
    main()
