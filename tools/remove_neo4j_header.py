#!/usr/bin/env python3
"""
Remove the Neo4j GPL header from the top of files.

Usage:
  python3 tools/remove_neo4j_header.py [--dry-run] [--extensions .rs .java .py ...] paths...

By default this walks the given paths recursively and edits files in-place.
Use --dry-run to only print which files would be changed.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import List


HEADER = """// Copyright (c) 2025 Rust-GDS Contributors
//
"""


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")


def has_header_at_top(text: str) -> bool:
    # Normalize line endings for comparison
    norm = text.replace("\r\n", "\n").lstrip()
    return norm.startswith(HEADER)


def remove_header(text: str) -> str:
    norm = text.replace("\r\n", "\n")
    stripped = norm.lstrip()
    if not stripped.startswith(HEADER):
        return text
    # Remove the header and a single following blank line if present
    after = stripped[len(HEADER):]
    if after.startswith("\n"):
        after = after[1:]
    # Preserve original leading whitespace/newlines count by removing only the header occurrence
    # Find where the stripped text starts within original to preserve any leading whitespace style
    idx = norm.find(stripped)
    return norm[:idx] + after


def walk_paths(paths: List[Path], exts: List[str]) -> List[Path]:
    files: List[Path] = []
    for p in paths:
        if p.is_file():
            if not exts or p.suffix in exts:
                files.append(p)
        else:
            for f in p.rglob("*"):
                if f.is_file():
                    if not exts or f.suffix in exts:
                        files.append(f)
    return files


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(description="Remove Neo4j GPL header from files")
    parser.add_argument("paths", nargs="+", help="Files or directories to process")
    parser.add_argument("--dry-run", action="store_true", help="Do not modify files; just print what would change")
    parser.add_argument("--extensions", nargs="*", default=[".rs", ".java", ".py", ".ts", ".js", ".md"],
                        help="File extensions to include (default common code files)")

    args = parser.parse_args(argv)

    paths = [Path(p) for p in args.paths]
    exts = args.extensions

    files = walk_paths(paths, exts)
    changed = []

    for f in files:
        try:
            txt = read_text(f)
        except Exception as e:
            print(f"Skipping {f}: cannot read ({e})")
            continue

        if has_header_at_top(txt):
            new = remove_header(txt)
            if new != txt:
                changed.append(f)
                if args.dry_run:
                    print(f"Would remove header: {f}")
                else:
                    write_text(f, new)
                    print(f"Removed header: {f}")

    print(f"Processed {len(files)} files; modified {len(changed)} files.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
