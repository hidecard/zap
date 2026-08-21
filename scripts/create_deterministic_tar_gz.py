#!/usr/bin/env python3
"""Create a deterministic gzip-compressed tar archive.

The helper intentionally avoids platform-specific tar flags so release jobs on
Linux and macOS produce the same archive metadata contract.
"""
from __future__ import annotations

import argparse
import gzip
import os
import tarfile
from pathlib import Path


def add_tree(archive: tarfile.TarFile, root: Path, archive_root: str) -> None:
    directories = [root, *sorted((path for path in root.rglob("*") if path.is_dir()), key=lambda p: p.as_posix())]
    for directory in directories:
        relative = directory.relative_to(root).as_posix()
        name = archive_root if not relative or relative == "." else f"{archive_root}/{relative}"
        info = archive.gettarinfo(str(directory), arcname=name)
        info.mtime = 0
        info.uid = 0
        info.gid = 0
        info.uname = ""
        info.gname = ""
        archive.addfile(info)

    files = sorted((path for path in root.rglob("*") if path.is_file()), key=lambda p: p.as_posix())
    for file_path in files:
        relative = file_path.relative_to(root).as_posix()
        name = f"{archive_root}/{relative}"
        info = archive.gettarinfo(str(file_path), arcname=name)
        info.mtime = 0
        info.uid = 0
        info.gid = 0
        info.uname = ""
        info.gname = ""
        with file_path.open("rb") as source:
            archive.addfile(info, source)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_root", type=Path)
    parser.add_argument("destination", type=Path)
    parser.add_argument("--archive-root", default="zap")
    args = parser.parse_args()

    source_root = args.source_root.resolve()
    if not source_root.is_dir():
        parser.error(f"source root is not a directory: {source_root}")
    args.destination.parent.mkdir(parents=True, exist_ok=True)
    with args.destination.open("wb") as output:
        with gzip.GzipFile(filename="", mode="wb", fileobj=output, mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as archive:
                add_tree(archive, source_root, args.archive_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
