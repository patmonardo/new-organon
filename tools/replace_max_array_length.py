#!/usr/bin/env python3
"""
Replace occurrences of the explicit MAX_ARRAY_LENGTH constant with PageUtil::MAX_ARRAY_LENGTH.

Usage:
  python3 tools/replace_max_array_length.py [--dry-run] [--extensions .rs] paths...

Example:
  python3 tools/replace_max_array_length.py --dry-run .
  python3 tools/replace_max_array_length.py src/collections/backends/huge --extensions .rs
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import List


PATTERN = re.compile(r'^(?P<indent>\s*)const\s+MAX_ARRAY_LENGTH\s*:\s*usize\s*=\s*(?P<rhs>[^;]+);\s*(?P<comment>//.*)?$')


def find_files(paths: List[Path], exts: List[str]) -> List[Path]:
    files: List[Path] = []
    for p in paths:
        if p.is_file():
            if not exts or p.suffix in exts:
                files.append(p)
        else:
            for f in p.rglob("*"):
                if f.is_file() and (not exts or f.suffix in exts):
                    files.append(f)
    return files


def process_text(text: str) -> (str, List[str]):
    """Return new_text and list of replacement summaries."""
    lines = text.replace("\r\n", "\n").splitlines(keepends=True)
    changed = []
    for i, line in enumerate(lines):
        m = PATTERN.match(line.rstrip('\n'))
        if m:
            indent = m.group('indent') or ''
            comment = m.group('comment') or ''
            new_line = f"{indent}const MAX_ARRAY_LENGTH: usize = PageUtil::MAX_ARRAY_LENGTH;{(' ' + comment) if comment else ''}\n"
            changed.append(f"line {i+1}: {line.strip()} -> {new_line.strip()}")
            lines[i] = new_line
    return ''.join(lines), changed


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(description="Replace MAX_ARRAY_LENGTH constant with PageUtil::MAX_ARRAY_LENGTH")
    parser.add_argument('paths', nargs='+', help='Files or directories to process')
    parser.add_argument('--dry-run', action='store_true', help='Do not modify files; just print what would change')
    parser.add_argument('--extensions', nargs='*', default=['.rs'], help='File extensions to include (default: .rs)')

    args = parser.parse_args(argv)
    paths = [Path(p) for p in args.paths]
    exts = args.extensions

    files = find_files(paths, exts)
    total_changed = 0

    for f in files:
        try:
            txt = f.read_text(encoding='utf-8')
        except Exception as e:
            print(f"Skipping {f}: cannot read ({e})")
            continue

        new_txt, changes = process_text(txt)
        if changes:
            total_changed += 1
            if args.dry_run:
                print(f"Would modify {f}:")
                for c in changes:
                    print(f"  {c}")
            else:
                f.write_text(new_txt, encoding='utf-8')
                print(f"Modified {f} ({len(changes)} changes)")

    print(f"Scanned {len(files)} files; modified {total_changed} files.")
    return 0


if __name__ == '__main__':
    raise SystemExit(main(sys.argv[1:]))
