import io
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path

import zap


class ZapRuntimeTests(unittest.TestCase):
    def execute(self, source):
        output = io.StringIO()
        with redirect_stdout(output):
            zap.run(source, "<test>")
        return output.getvalue().splitlines()

    def test_basic_output_and_arithmetic(self):
        self.assertEqual(self.execute('value = 2 + 3 * 4\nsay value\n'), ["14"])

    def test_function_map_and_indexing(self):
        source = '''
fn greet(name):
    return "Hi " + name
map user = {"name": "Zap"}
say greet(user["name"])
'''
        self.assertEqual(self.execute(source), ["Hi Zap"])

    def test_while_loop(self):
        source = '''
count = 0
while count < 3:
    say count
    count = count + 1
'''
        self.assertEqual(self.execute(source), ["0", "1", "2"])

    def test_ai_placeholder_is_available(self):
        self.assertIn("[AI placeholder]", self.execute('answer = ai.ask "hello"\nsay answer["text"]\n')[0])

    def test_new_project(self):
        with tempfile.TemporaryDirectory() as tmp:
            target = str(Path(tmp) / "demo")
            old = zap.sys.argv
            try:
                zap.sys.argv = ["zap", "new", target]
                self.assertEqual(zap.main(), 0)
            finally:
                zap.sys.argv = old
            self.assertTrue(Path(target, "main.zp").exists())
            self.assertTrue(Path(target, "README.md").exists())


if __name__ == "__main__":
    unittest.main()
