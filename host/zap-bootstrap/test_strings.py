import sys
sys.path.insert(0, "host/zap-bootstrap")
from compile import compile_and_run

tests = [
    ('say "hello" + " world"\n', ["hello world"]),
    ('say "a" == "a"\n', [True]),
    ('say "a" == "b"\n', [False]),
    ('let s = "zap"\nsay s + "!"\n', ["zap!"]),
]

for i, (src, expected) in enumerate(tests, 1):
    try:
        result = compile_and_run(src)
        status = "PASS" if result == expected else "FAIL"
        print(f"Test {i}: {status} (expected {expected}, got {result})")
    except Exception as e:
        print(f"Test {i}: ERROR - {e}")
