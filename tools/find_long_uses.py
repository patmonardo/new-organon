#!/usr/bin/env python3
"""Report long/deep `use` statements (depth >= 3) across the repo.

This is the behavior previously provided by `find_nested_uses.py` when run
without the split-use focus. It detects uses like:
  use crate::a::b::c::X;

Defaults: ignores `std`, `crate::core`, `crate::types` (opt-in via flags).
"""
from __future__ import annotations
import argparse
import os
import re
import sys
from pathlib import Path

# Depth-based nested `use` detection (>=3 path segments after the prefix)
NESTED_USE_RE = re.compile(r"^\s*(?:pub\s+)?use\s+([A-Za-z0-9_]+)(?:::(?P<second>[A-Za-z0-9_]+))?::(?:[A-Za-z0-9_]+::){2,}[A-Za-z0-9_]+.*;")
IGNORED_DIRS = {"target", ".git", "node_modules"}


def should_ignore(path: Path) -> bool:
    for p in path.parts:
        if p in IGNORED_DIRS:
            return True
    return False


def scan(root: Path, ignore_prefixes: set[str]) -> list[tuple[Path,int,str]]:
    findings = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in IGNORED_DIRS]
        for fname in filenames:
            if not fname.endswith('.rs'):
                continue
            fpath = Path(dirpath) / fname
            try:
                with fpath.open('r', encoding='utf-8') as f:
                    for i, line in enumerate(f, start=1):
                        m = NESTED_USE_RE.match(line)
                        if not m:
                            continue
                        # derive prefix robustly
                        path_part = line.split('use', 1)[1].split(';', 1)[0].strip()
                        segments = [s for s in path_part.split('::') if s]
                        if not segments:
                            continue
                        if segments[0] == 'crate' and len(segments) >= 2:
                            prefix = f"crate::{segments[1]}"
                        else:
                            prefix = segments[0]
                        if any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                            continue
                        findings.append((fpath, i, line.rstrip('\n')))
            except Exception:
                continue
    return findings


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description='Find long/deep use statements (depth >=3)')
    p.add_argument('paths', nargs='*', help='Paths to scan (default: repo root)')
    p.add_argument('--ignore-prefix', action='append', default=[],
                   help='Prefix to ignore (can be passed multiple times), e.g. "crate::core"')
    p.add_argument('--include-core', action='store_true', help='Include `crate::core` in scanning (default: ignored)')
    p.add_argument('--include-types', action='store_true', help='Include `crate::types` in scanning (default: ignored)')
    p.add_argument('--segment', action='append', default=[],
                   help='Report `use` lines that contain this path segment (repeatable). Example: --segment computation')
    p.add_argument('--only-segment', action='store_true', help='Only report segment matches (skip depth-based nested reporting)')
    p.add_argument('--only-deep', action='store_true', help='Only report depth-based nested uses (skip segment reporting)')
    args = p.parse_args(argv)

    ignore_prefixes = set(args.ignore_prefix)
    ignore_prefixes.update({'std', 'crate::core', 'crate::types'})
    if args.include_core:
        ignore_prefixes.discard('crate::core')
    if args.include_types:
        ignore_prefixes.discard('crate::types')

    roots = [Path(p) for p in args.paths] if args.paths else [Path('.')]
    nested_findings = []
    segment_findings = []
    for r in roots:
        if r.is_file() and r.suffix == '.rs':
            with r.open('r', encoding='utf-8') as f:
                for i, line in enumerate(f, start=1):
                    m = NESTED_USE_RE.match(line)
                    if m:
                        path_part = line.split('use', 1)[1].split(';', 1)[0].strip()
                        segments = [s for s in path_part.split('::') if s]
                        if not segments:
                            continue
                        if segments[0] == 'crate' and len(segments) >= 2:
                            prefix = f"crate::{segments[1]}"
                        else:
                            prefix = segments[0]
                        if not any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                            nested_findings.append((r, i, line.rstrip('\n')))
                    if args.segment:
                        for seg in args.segment:
                            path_part = line.split('use', 1)[1].split(';', 1)[0].strip() if 'use' in line else ''
                            if not path_part:
                                continue
                            if any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                                continue
                            if f"::{seg}::" in path_part or path_part.endswith(f"::{seg}"):
                                segment_findings.append((r, i, line.rstrip('\n'), seg))
        elif r.is_dir():
            nested_findings.extend(scan(r, ignore_prefixes))
            if args.segment:
                # delegate to helper similar to original
                from tools.find_nested_uses import scan_for_segments
                segment_findings.extend(scan_for_segments(r, set(args.segment), ignore_prefixes))

    if nested_findings:
        print('Long `use` statements detected (depth >=3):')
        for path, ln, line in nested_findings:
            print(f'  {path}:{ln}: {line}')
    elif not args.only_segment:
        print('No long `use` statements found.')

    if args.segment:
        if segment_findings:
            print('\n`use` statements containing requested segments:')
            for path, ln, line, seg in segment_findings:
                print(f'  {path}:{ln}: ({seg}) {line}')
        else:
            print('\nNo segment matches found.')

    print(f"\nIgnored prefixes: {sorted(ignore_prefixes)}")
    if args.segment:
        print(f"Segments searched: {sorted(set(args.segment))}")
    return 2 if nested_findings or segment_findings else 0


if __name__ == '__main__':
    raise SystemExit(main())
