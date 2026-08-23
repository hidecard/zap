#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
cd "$ROOT_DIR"
if [[ -f "$HOME/.cargo/env" ]]; then
  source "$HOME/.cargo/env"
fi

python3 - "$ROOT_DIR" <<'PY'
import json
import subprocess
import sys

root = sys.argv[1]
process = subprocess.Popen(
    ["cargo", "run", "--quiet", "--manifest-path", "native/Cargo.toml", "--", "lsp"],
    cwd=root,
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
)


def send(message):
    body = json.dumps(message, separators=(",", ":")).encode("utf-8")
    process.stdin.write(f"Content-Length: {len(body)}\r\n\r\n".encode("ascii") + body)
    process.stdin.flush()


def receive():
    headers = {}
    while True:
        line = process.stdout.readline()
        if not line:
            raise RuntimeError("LSP server closed stdout before a response")
        if line in (b"\r\n", b"\n"):
            break
        name, value = line.decode("ascii").rstrip("\r\n").split(":", 1)
        headers[name.lower()] = value.strip()
    length = int(headers["content-length"])
    body = process.stdout.read(length)
    if len(body) != length:
        raise RuntimeError("LSP response body was truncated")
    return json.loads(body.decode("utf-8"))


def assert_symbol(expected):
    send(
        {
            "jsonrpc": "2.0",
            "id": expected[1],
            "method": "textDocument/documentSymbol",
            "params": {"textDocument": {"uri": uri}},
        }
    )
    response = receive()
    assert [item["name"] for item in response["result"]] == [expected[0]]


try:
    send({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}})
    initialize = receive()
    sync = initialize["result"]["capabilities"]["textDocumentSync"]
    assert sync["change"] == 2 and sync["openClose"] is True

    uri = "file:///protocol-sync.zp"
    send(
        {
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "version": 1,
                    "text": "fn first():\n    return 1\n",
                }
            },
        }
    )
    opened_diagnostics = receive()
    assert opened_diagnostics["method"] == "textDocument/publishDiagnostics"

    # Replace only the function name using a UTF-8-compatible range.
    send(
        {
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {"uri": uri, "version": 2},
                "contentChanges": [
                    {
                        "range": {
                            "start": {"line": 0, "character": 3},
                            "end": {"line": 0, "character": 8},
                        },
                        "text": "second",
                    }
                ],
            },
        }
    )
    changed_diagnostics = receive()
    assert changed_diagnostics["method"] == "textDocument/publishDiagnostics"
    assert changed_diagnostics["params"]["diagnostics"] == []
    assert_symbol(("second", 2))

    # A stale version must not replace the successfully synchronized document.
    send(
        {
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {"uri": uri, "version": 2},
                "contentChanges": [{"text": "fn stale():\n    return 0\n"}],
            },
        }
    )
    assert_symbol(("second", 3))

    # An out-of-range incremental edit must also leave the stored document intact.
    send(
        {
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": {"uri": uri, "version": 4},
                "contentChanges": [
                    {
                        "range": {
                            "start": {"line": 99, "character": 0},
                            "end": {"line": 99, "character": 1},
                        },
                        "text": "fn invalid():\n    return 0\n",
                    }
                ],
            },
        }
    )
    assert_symbol(("second", 4))

    send({"jsonrpc": "2.0", "id": 5, "method": "shutdown", "params": {}})
    assert receive()["result"] is None
    process.stdin.close()
    process.wait(timeout=10)
    if process.returncode != 0:
        raise RuntimeError(f"LSP server exited with {process.returncode}")
except Exception as error:
    process.kill()
    stderr = process.stderr.read().decode("utf-8", errors="replace")
    detail = stderr or repr(error)
    raise RuntimeError(detail) from None

print("LSP protocol synchronization contract passed")
PY
