#!/usr/bin/env python3
"""CI verifier for the non-Rust Zap VM host.

This exercises `run.py` (a faithful Python re-implementation of
`bootstrap/b3/vm.zp`) against representative bytecode programs and asserts
their outputs. It requires only Python 3 -- no Rust toolchain -- so the
Zap execution layer can be verified independently of the native reference
interpreter. This is the Rust-independence execution gate (roadmap step 2).
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run import run  # noqa: E402


def _programs():
    return [
        # 1. literal + say
        ([{"op": "const", "value": 7},
          {"op": "store", "name": "x"},
          {"op": "load", "name": "x"},
          {"op": "print"},
          {"op": "halt"}],
         [7]),

        # 2. function call + return
        ([{"op": "function_def", "name": "add", "params": ["a", "b"], "entry": 1, "end": 5},
          {"op": "load", "name": "a"},
          {"op": "load", "name": "b"},
          {"op": "add"},
          {"op": "return_value"},
          {"op": "const", "value": 2},
          {"op": "const", "value": 3},
          {"op": "call", "name": "add", "argc": 2},
          {"op": "print"},
          {"op": "halt"}],
         [5]),

        # 3. branch via jump_if_false
        ([{"op": "const", "value": 1},
          {"op": "store", "name": "n"},
          {"op": "load", "name": "n"},
          {"op": "const", "value": 0},
          {"op": "greater"},
          {"op": "jump_if_false", "target": 8},
          {"op": "const", "value": 100},
          {"op": "print"},
          {"op": "halt"}],
         [100]),

        # 4. class: constructor field + method dispatch
        ([{"op": "class_def", "name": "Point", "base": None, "entry": 1, "end": 10},
          {"op": "function_def", "name": "__init__", "owner": "Point", "params": ["self", "x"], "entry": 2, "end": 6},
          {"op": "load", "name": "self"},
          {"op": "load", "name": "x"},
          {"op": "field_store", "receiver": "self", "field": "x"},
          {"op": "return_none"},
          {"op": "function_def", "name": "get", "owner": "Point", "params": ["self"], "entry": 7, "end": 10},
          {"op": "load", "name": "self"},
          {"op": "field_load", "receiver": "self", "field": "x"},
          {"op": "return_value"},
          {"op": "const", "value": 5},
          {"op": "call", "name": "Point", "argc": 1},
          {"op": "store", "name": "p"},
          {"op": "load", "name": "p"},
          {"op": "method_call", "receiver": "p", "method": "get", "argc": 0},
          {"op": "print"},
          {"op": "halt"}],
         [5]),

        # 5. try/raise caught by handler
        ([{"op": "try_begin", "binding": "err", "target": 6},
          {"op": "const", "value": "boom"},
          {"op": "raise"},
          {"op": "const", "value": 999},
          {"op": "print"},
          {"op": "jump", "target": 9},
          {"op": "const", "value": 1},
          {"op": "print"},
          {"op": "try_end"},
          {"op": "halt"}],
         [1]),

        # 6. closure capturing an enclosing variable
        ([{"op": "function_def", "name": "make_adder", "params": ["n"], "entry": 1, "end": 8},
          {"op": "function_def", "name": "add", "binding": "add", "captures": ["n"], "params": ["x"], "entry": 2, "end": 6},
          {"op": "load", "name": "x"},
          {"op": "load", "name": "n"},
          {"op": "add"},
          {"op": "return_value"},
          {"op": "load", "name": "add"},
          {"op": "return_value"},
          {"op": "const", "value": 10},
          {"op": "call", "name": "make_adder", "argc": 1},
          {"op": "store", "name": "f"},
          {"op": "load", "name": "f"},
          {"op": "const", "value": 5},
          {"op": "call", "name": "f", "argc": 1},
          {"op": "print"},
          {"op": "halt"}],
         [15]),

        # 7. loop (while + break pattern) summing 1..5
        ([{"op": "const", "value": 1},
          {"op": "store", "name": "i"},
          {"op": "const", "value": 0},
          {"op": "store", "name": "total"},
          {"op": "load", "name": "i"},
          {"op": "const", "value": 5},
          {"op": "greater"},
          {"op": "jump_if_false", "target": 9},
          {"op": "jump", "target": 18},
          {"op": "load", "name": "total"},
          {"op": "load", "name": "i"},
          {"op": "add"},
          {"op": "store", "name": "total"},
          {"op": "load", "name": "i"},
          {"op": "const", "value": 1},
          {"op": "add"},
          {"op": "store", "name": "i"},
          {"op": "jump", "target": 4},
          {"op": "load", "name": "total"},
          {"op": "print"},
          {"op": "halt"}],
         [15]),
    ]


def main():
    failures = 0
    for index, (program, expected) in enumerate(_programs()):
        state = run(program)
        if state["error"] is not None:
            print("FAIL program %d: vm error %s" % (index + 1, state["error"]))
            failures += 1
            continue
        if state["output"] != expected:
            print("FAIL program %d: expected %s got %s"
                  % (index + 1, expected, state["output"]))
            failures += 1
            continue
    if failures:
        print("non-rust-vm-host verification FAILED: %d case(s) failed" % failures)
        return 1
    print("non-rust-vm-host verification passed: %d programs (arithmetic, "
          "function call, branch, loop, class+method, try/raise, closure) execute "
          "without the Rust VM" % len(_programs()))
    return 0


if __name__ == "__main__":
    sys.exit(main())
