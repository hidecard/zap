#!/usr/bin/env python3
"""Validate repository-relative Markdown links in tracked Markdown files."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\n]+)\)")


def tracked_markdown_files() -> list[Path]:
    result = subprocess.run(
        ["git", "ls-files", "--", "*.md", "*.markdown"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return [ROOT / line for line in result.stdout.splitlines() if line]


def is_external(target: str) -> bool:
    parsed = urlsplit(target)
    return bool(parsed.scheme or parsed.netloc) or target.startswith(("#", "//"))


def clean_target(raw: str) -> str:
    target = raw.strip()
    if target.startswith("<") and ">" in target:
        target = target[1 : target.index(">")]
    else:
        # Markdown permits an optional title after the destination. The
        # repository links use no spaces in destinations, so this safely
        # separates the common title form without altering valid paths.
        target = target.split(maxsplit=1)[0]
    parsed = urlsplit(target)
    return unquote(parsed.path)


def main() -> int:
    failures: list[str] = []
    checked = 0
    for markdown in tracked_markdown_files():
        text = markdown.read_text(encoding="utf-8")
        in_fence = False
        for line_number, line in enumerate(text.splitlines(), start=1):
            if line.strip().startswith("```"):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            for match in LINK_RE.finditer(line):
                raw = match.group(1).strip()
                if not raw or is_external(raw):
                    continue
                target = clean_target(raw)
                if not target:
                    continue
                checked += 1
                resolved = (markdown.parent / target).resolve()
                try:
                    resolved.relative_to(ROOT.resolve())
                except ValueError:
                    failures.append(
                        f"{markdown.relative_to(ROOT)}:{line_number}: link escapes repository: {raw}"
                    )
                    continue
                if not resolved.exists():
                    failures.append(
                        f"{markdown.relative_to(ROOT)}:{line_number}: missing target: {raw}"
                    )

    for failure in failures:
        print(f"FAIL {failure}")
    if failures:
        print(f"markdown link validation failed: {len(failures)} issue(s); checked={checked}", file=sys.stderr)
        return 1
    print(f"markdown link validation passed: checked={checked}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
