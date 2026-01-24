#!/usr/bin/env python3
"""Remove commented-out test blocks from Rust source files.

This script looks for lines like:
  // #[test]
and removes that line and the subsequent contiguous commented lines (lines starting with //)
usually representing a commented-out test function.

Usage:
  tools/remove_commented_tests.py [paths...] [--apply] [--recursive]

Options:
  --apply      Write changes to files (dry-run by default).
  --recursive  Recurse into directories.

The script prints a report of files and number of commented test blocks found. Use --apply to remove them.
"""
from __future__ import annotations
import argparse
import os
import re
import sys
from pathlib import Path

IGNORED_DIRS = {"target", ".git", "node_modules"}
TEST_ATTR_RE = re.compile(r"^\s*//\s*#\[test\]\s*$")


def process_file(path: Path, apply: bool) -> tuple[bool, int]:
    s = path.read_text(encoding='utf-8')
    lines = s.splitlines(keepends=True)
    out_lines: list[str] = []

    i = 0
    removed_blocks = 0
    changed = False

    while i < len(lines):
        line = lines[i]
        if TEST_ATTR_RE.match(line):
            # start removing this line and any subsequent contiguous comment lines
            removed_blocks += 1
            changed = True
            i += 1
            # remove following lines that start with '//' (possibly with whitespace)
            while i < len(lines) and re.match(r"^\s*//", lines[i]):
                i += 1
            # also remove one trailing blank line if it exists for cleanliness
            if i < len(lines) and lines[i].strip() == "":
                i += 1
            continue
        else:
            out_lines.append(line)
            i += 1

    if not changed:
        return False, 0

    if apply:
        # backup original
        backup = path.with_suffix(path.suffix + '.bak')
        backup.write_text(s, encoding='utf-8')
        path.write_text(''.join(out_lines), encoding='utf-8')
        print(f"WROTE {path} (backup: {backup.name})")

    return True, removed_blocks


def scan_paths(paths: list[Path], recursive: bool, apply: bool) -> int:
    files = []
    for p in paths:
        if p.is_dir():
            if recursive:
                for dirpath, dirnames, filenames in os.walk(p):
                    dirnames[:] = [d for d in dirnames if d not in IGNORED_DIRS]
                    for fname in filenames:
                        if fname.endswith('.rs'):
                            files.append(Path(dirpath) / fname)
            else:
                for fname in os.listdir(p):
                    if fname.endswith('.rs'):
                        files.append(p / fname)
        elif p.is_file():
            files.append(p)
        else:
            print(f"Skipping unknown path: {p}")

    total_removed = 0
    for f in files:
        changed, count = process_file(f, apply)
        if changed:
            print(f"{f}: {count} commented test block(s) {'removed' if apply else 'would be removed'}")
            total_removed += count
    if total_removed == 0:
        print('No commented test blocks found.')
    else:
        print(f'Total commented test blocks: {total_removed}')
    return total_removed


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('paths', nargs='*', default=['.'], help='Files or directories to scan')
    p.add_argument('--apply', action='store_true', help='Write changes to files')
    p.add_argument('--recursive', action='store_true', help='Recurse into directories')
    args = p.parse_args()

    paths = [Path(p) for p in args.paths]
    total = scan_paths(paths, recursive=args.recursive, apply=args.apply)
    if total > 0:
        if not args.apply:
            print('\nDry-run: use --apply to actually remove the commented tests.')
        sys.exit(2)
    else:
        sys.exit(0)
