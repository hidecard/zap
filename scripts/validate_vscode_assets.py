#!/usr/bin/env python3
"""Validate Zap's checked-in VS Code assets and canonical extension package."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EDITOR_ROOT = ROOT / "editors" / "vscode"
DISTRIBUTION_ROOT = ROOT / "vscode-extension"
CATALOG_PATH = ROOT / "native" / "src" / "stdlib_catalog.rs"
CARGO_PATH = ROOT / "native" / "Cargo.toml"


def fail(message: str) -> None:
    print(f"FAIL\t{message}", file=sys.stderr)
    raise SystemExit(1)


def load_asset_set(root: Path) -> tuple[dict, dict, dict]:
    paths = {
        "grammar": root / "syntaxes" / "zap.tmLanguage.json",
        "configuration": root / "language-configuration.json",
        "manifest": root / "package.json",
    }
    for kind, path in paths.items():
        if not path.is_file():
            fail(f"missing {kind} asset: {path.relative_to(ROOT)}")
    try:
        return tuple(
            json.loads(paths[kind].read_text(encoding="utf-8"))
            for kind in ("grammar", "configuration", "manifest")
        )  # type: ignore[return-value]
    except json.JSONDecodeError as error:
        fail(f"invalid JSON editor asset: {error}")


def main() -> None:
    if not CATALOG_PATH.is_file() or not CARGO_PATH.is_file():
        fail("native catalog or Cargo manifest is missing")

    editor_grammar, editor_configuration, editor_manifest = load_asset_set(EDITOR_ROOT)
    distribution_grammar, distribution_configuration, distribution_manifest = load_asset_set(
        DISTRIBUTION_ROOT
    )

    for label, grammar, configuration, manifest in (
        ("editors/vscode", editor_grammar, editor_configuration, editor_manifest),
        ("vscode-extension", distribution_grammar, distribution_configuration, distribution_manifest),
    ):
        if grammar.get("scopeName") != "source.zap":
            fail(f"{label} grammar scopeName must be source.zap")
        grammars = manifest.get("contributes", {}).get("grammars", [])
        if not grammars or grammars[0].get("scopeName") != "source.zap":
            fail(f"{label} manifest grammar scope does not match source.zap")
        languages = manifest.get("contributes", {}).get("languages", [])
        if not languages or languages[0].get("configuration") != "./language-configuration.json":
            fail(f"{label} manifest does not point to language-configuration.json")
        if "lineComment" not in configuration.get("comments", {}):
            fail(f"{label} language configuration is missing the # line comment")

    cargo = CARGO_PATH.read_text(encoding="utf-8")
    version_match = re.search(r'^version\s*=\s*"([^"]+)"\s*$', cargo, re.MULTILINE)
    if not version_match:
        fail("Cargo version is missing")
    version = version_match.group(1)
    for label, manifest in (
        ("editors/vscode", editor_manifest),
        ("vscode-extension", distribution_manifest),
    ):
        if manifest.get("name") != "zap-language-support":
            fail(f"{label} package name must be zap-language-support")
        if manifest.get("version") != version:
            fail(f"{label} manifest version does not match native Cargo version")

    if distribution_manifest.get("publisher") != "ArkarYan":
        fail("canonical vscode-extension publisher must remain ArkarYan")
    if distribution_manifest.get("main") != "extension.js":
        fail("canonical vscode-extension manifest must load extension.js")
    if distribution_manifest.get("scripts", {}).get("package") != "node scripts/package-extension.js":
        fail("canonical vscode-extension package script is missing")

    if editor_grammar != distribution_grammar:
        fail("editors/vscode grammar has drifted from canonical vscode-extension grammar")
    if editor_configuration != distribution_configuration:
        fail("editors/vscode language configuration has drifted from canonical vscode-extension configuration")

    catalog = CATALOG_PATH.read_text(encoding="utf-8")
    builtins = re.findall(r'stable_builtin!\("([^"]+)"', catalog)
    if not builtins:
        fail("standard-library catalog contains no builtin entries")
    grammar_text = json.dumps(distribution_grammar)
    for builtin in builtins:
        if not re.search(rf"(?<![A-Za-z0-9_]){re.escape(builtin)}(?![A-Za-z0-9_])", grammar_text):
            fail(f"canonical grammar is missing catalog builtin {builtin}")
    for keyword in (
        "let",
        "fn",
        "async",
        "await",
        "if",
        "else",
        "for",
        "while",
        "class",
        "module",
        "import",
        "return",
    ):
        if keyword not in grammar_text:
            fail(f"canonical grammar is missing language keyword {keyword}")

    print(
        f"PASS\tVS Code assets are valid, canonical package metadata is aligned, "
        f"and both trees cover {len(builtins)} catalog builtins"
    )


if __name__ == "__main__":
    main()
