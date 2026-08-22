#!/usr/bin/env python3
"""Validate the checked-in VS Code assets against Zap's public catalog."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GRAMMAR_PATH = ROOT / "editors" / "vscode" / "syntaxes" / "zap.tmLanguage.json"
CONFIG_PATH = ROOT / "editors" / "vscode" / "language-configuration.json"
MANIFEST_PATH = ROOT / "editors" / "vscode" / "package.json"
CATALOG_PATH = ROOT / "native" / "src" / "stdlib_catalog.rs"
CARGO_PATH = ROOT / "native" / "Cargo.toml"


def fail(message: str) -> None:
    print(f"FAIL\t{message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    for path in (GRAMMAR_PATH, CONFIG_PATH, MANIFEST_PATH, CATALOG_PATH, CARGO_PATH):
        if not path.is_file():
            fail(f"missing editor parity file: {path.relative_to(ROOT)}")

    try:
        grammar = json.loads(GRAMMAR_PATH.read_text(encoding="utf-8"))
        configuration = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
        manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        fail(f"invalid JSON editor asset: {error}")

    if grammar.get("scopeName") != "source.zap":
        fail("grammar scopeName must be source.zap")
    if manifest.get("contributes", {}).get("grammars", [{}])[0].get("scopeName") != "source.zap":
        fail("VS Code manifest grammar scope does not match source.zap")
    if manifest.get("contributes", {}).get("languages", [{}])[0].get("configuration") != "./language-configuration.json":
        fail("VS Code manifest does not point to language-configuration.json")
    if "lineComment" not in configuration.get("comments", {}):
        fail("language configuration is missing the # line comment")

    cargo = CARGO_PATH.read_text(encoding="utf-8")
    version_match = re.search(r'^version\s*=\s*"([^"]+)"\s*$', cargo, re.MULTILINE)
    if not version_match:
        fail("Cargo version is missing")
    if manifest.get("version") != version_match.group(1):
        fail("VS Code manifest version does not match native Cargo version")

    catalog = CATALOG_PATH.read_text(encoding="utf-8")
    builtins = re.findall(r'stable_builtin!\("([^"]+)"', catalog)
    if not builtins:
        fail("standard-library catalog contains no builtin entries")
    grammar_text = GRAMMAR_PATH.read_text(encoding="utf-8")
    for builtin in builtins:
        if not re.search(rf"(?<![A-Za-z0-9_]){re.escape(builtin)}(?![A-Za-z0-9_])", grammar_text):
            fail(f"grammar is missing catalog builtin {builtin}")
    for keyword in ("let", "fn", "async", "await", "if", "else", "for", "while", "class", "module", "import", "return"):
        if keyword not in grammar_text:
            fail(f"grammar is missing language keyword {keyword}")

    print(f"PASS\tVS Code assets are valid and cover {len(builtins)} catalog builtins")


if __name__ == "__main__":
    main()
