#!/usr/bin/env python3
"""Rust-free verifier for the Zap bootstrap compiler.

Compiles real Zap source (a small subset) and runs it on the non-Rust VM host,
asserting expected output. Requires only Python 3 -- no Rust toolchain. This is
the end-to-end single-language demonstration: source -> bytecode -> execution
with no Rust dependency.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from compile import compile_and_run  # noqa: E402


_PROGRAMS = [
    # 1. function definition + call + arithmetic
    ("fn add(a, b):\n    return a + b\nlet x = add(2, 3)\nsay x\n", [5]),

    # 2. while loop with accumulator
    ("let i = 0\nlet total = 0\nwhile i < 5:\n    total = total + i\n    i = i + 1\nsay total\n", [10]),

    # 3. if / else branch
    ("if 2 < 3:\n    say 1\nelse:\n    say 2\n", [1]),

    # 4. recursion (factorial) -- exercises nested call frames
    ("fn fact(n):\n    if n == 0:\n        return 1\n    return n * fact(n - 1)\nsay fact(5)\n", [120]),

    # 5. string output
    ('say "hi"\n', ["hi"]),

    # 6. list literal + indexing
    ("let xs = [10, 20, 30]\nsay xs[0]\nsay xs[2]\n", [10, 30]),

    # 7. len() builtin
    ("let xs = [1, 2, 3]\nsay len(xs)\n", [3]),

    # 8. list literal + loop
    ("let values = [1, 2, 3]\nlet i = 0\nwhile i < len(values):\n    say values[i]\n    i = i + 1\n", [1, 2, 3]),

    # 9. for loop over list
    ("let values = [1, 2, 3]\nfor x in values:\n    say x\n", [1, 2, 3]),

    # 10. for loop with accumulator
    ("let total = 0\nfor x in [1, 2, 3, 4]:\n    total = total + x\nsay total\n", [10]),

    # 11. string concatenation
    ('say "hello" + " world"\n', ["hello world"]),

    # 12. string equality
    ('say "a" == "a"\n', [True]),
    ('say "a" == "b"\n', [False]),

    # 13. string variable + concatenation
    ('let s = "zap"\nsay s + "!"\n', ["zap!"]),

    # 14. nested function calls
    ("fn double(n):\n    return n * 2\nfn quad(n):\n    return double(double(n))\nsay quad(3)\n", [12]),

    # 15. while loop with nested block
    ("let i = 0\nwhile i < 3:\n    let x = i\n    say x\n    i = i + 1\n", [0, 1, 2]),

    # 16. for loop with list of strings
    ('let words = ["a", "b", "c"]\nfor w in words:\n    say w\n', ["a", "b", "c"]),

    # 17. mixed types in list
    ("let xs = [1, 2, 3]\nsay xs[1]\n", [2]),

    # 18. nested while with break-like accumulation
    ("let i = 0\nlet total = 0\nwhile i < 4:\n    let j = 0\n    while j < 2:\n        total = total + 1\n        j = j + 1\n    i = i + 1\nsay total\n", [8]),
]


def main():
    failures = 0
    for index, (source, expected) in enumerate(_PROGRAMS):
        try:
            output = compile_and_run(source)
        except Exception as exc:  # noqa: BLE001
            print("FAIL program %d: exception %s" % (index + 1, exc))
            failures += 1
            continue
        if output != expected:
            print("FAIL program %d: expected %s got %s"
                  % (index + 1, expected, output))
            failures += 1
    if failures:
        print("non-rust-bootstrap-compiler verification FAILED: %d case(s)" % failures)
        return 1
    print("non-rust-bootstrap-compiler verification passed: %d Zap source programs "
          "compiled and executed without the Rust VM" % len(_PROGRAMS))
    return 0


if __name__ == "__main__":
    sys.exit(main())
