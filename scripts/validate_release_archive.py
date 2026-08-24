#!/usr/bin/env python3
"""Validate local documentation links in a standalone Zap release archive."""

from __future__ import annotations

import posixpath
import re
import sys
import tarfile
import zipfile
from pathlib import PurePosixPath

LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def read_archive(path: str) -> tuple[set[str], dict[str, str]]:
    if path.endswith((".tar.gz", ".tgz")):
        with tarfile.open(path, "r:gz") as archive:
            names = {name.rstrip("/") for name in archive.getnames()}
            texts: dict[str, str] = {}
            for name in names:
                if name.endswith(".md"):
                    member = archive.extractfile(name)
                    if member is not None:
                        texts[name] = member.read().decode("utf-8")
            return names, texts
    if path.endswith(".zip"):
        with zipfile.ZipFile(path) as archive:
            names = {name.rstrip("/") for name in archive.namelist()}
            texts = {
                name: archive.read(name).decode("utf-8")
                for name in names
                if name.endswith(".md")
            }
            return names, texts
    raise SystemExit(f"unsupported archive format: {path}")


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: validate_release_archive.py ARCHIVE", file=sys.stderr)
        return 2
    names, texts = read_archive(sys.argv[1])
    errors: list[str] = []
    for name, text in texts.items():
        # README is the public entrypoint; other docs may intentionally point to source-only paths.
        if PurePosixPath(name).name != "README.md":
            continue
        for target in LINK_RE.findall(text):
            target = target.split("#", 1)[0].split("?", 1)[0].strip()
            if not target or target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            if target.startswith("/"):
                continue
            resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
            if resolved not in names:
                errors.append(f"{name}: missing local target {target} -> {resolved}")
    if errors:
        print("release archive documentation validation failed:", file=sys.stderr)
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"release archive documentation validation passed: {sys.argv[1]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
