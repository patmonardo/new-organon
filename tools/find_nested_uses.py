#!/usr/bin/env python3
"""Report nested `use` statements (depth >= 3) across the repo.

This script finds `use` statements like:
  use crate::a::b::c::X;
  use super::x::y::z;
  use foo::bar::baz::qux;

These nested imports are hard to maintain; prefer importing the specific module path you want
(e.g., `use crate::algo::astar::computation;` rather than `use crate::algo::computation::*;`).

Exit with a non-zero status if any nested uses are found.
"""
from __future__ import annotations
import argparse
import os
import re
import sys
from pathlib import Path

NESTED_USE_RE = re.compile(r"^\s*(?:pub\s+)?use\s+(?:crate|self|super|[A-Za-z0-9_]+)::(?:[A-Za-z0-9_]+::){2,}[A-Za-z0-9_]+.*;")
IGNORED_DIRS = {"target", ".git", "node_modules"}


def should_ignore(path: Path) -> bool:
    for p in path.parts:
        if p in IGNORED_DIRS:
            return True
    return False


def scan(root: Path) -> list[tuple[Path,int,str]]:
    findings = []
    for dirpath, dirnames, filenames in os.walk(root):
        # prune ignored dirs
        dirnames[:] = [d for d in dirnames if d not in IGNORED_DIRS]
        for fname in filenames:
            if not fname.endswith('.rs'):
                continue
            fpath = Path(dirpath) / fname
            try:
                with fpath.open('r', encoding='utf-8') as f:
                    for i, line in enumerate(f, start=1):
                        if NESTED_USE_RE.match(line):
                            findings.append((fpath, i, line.rstrip('\n')))
            except Exception:
                # ignore unreadable files
                continue
    return findings


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description='Find nested use statements')
    p.add_argument('paths', nargs='*', help='Paths to scan (default: repo root)')
    args = p.parse_args(argv)

    roots = [Path(p) for p in args.paths] if args.paths else [Path('.')]
    findings = []
    for r in roots:
        if r.is_file() and r.suffix == '.rs':
            with r.open('r', encoding='utf-8') as f:
                for i, line in enumerate(f, start=1):
                    if NESTED_USE_RE.match(line):
                        findings.append((r, i, line.rstrip('\n')))
        elif r.is_dir():
            findings.extend(scan(r))

    if not findings:
        print('No nested `use` statements found.')
        return 0

    print('Nested `use` statements detected (depth >=3):')
    for path, ln, line in findings:
        print(f'  {path}:{ln}: {line}')

    print('\nGuidance: prefer flatter imports or import the specific submodule; consider adding an alias if needed.')
    return 2


if __name__ == '__main__':
    raise SystemExit(main())
