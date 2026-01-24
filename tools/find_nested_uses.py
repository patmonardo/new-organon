#!/usr/bin/env python3
"""Find `use` statements that split module paths using braces by default.

This script primarily detects lines like:
  use foo::{bar::S1, baf::S2};
  use crate::a::{b::{X, Y}, c::Z};

These 'split' uses can make it harder to maintain module paths — prefer importing
the submodule instead, or use explicit aliases. The script still supports
`--segment` to search for path segments (e.g., `computation`) and `--deep` to
run the depth-based (>=3) detector if you want.

By default the script ignores `std`, `crate::core`, and `crate::types` unless
`--include-core`/`--include-types` are passed.
"""
from __future__ import annotations
import argparse
import os
import re
import sys
from pathlib import Path

# Match nested `use` statements like `use crate::a::b::c::X;` or `use super::x::y::z;`.
# By default we ignore `std::` imports. Use `--ignore-prefix` to add additional prefixes
# to ignore (e.g., `crate::core`, `crate::types`). Convenience flags `--ignore-core`
# and `--ignore-types` are provided.
# detect split-use patterns: e.g., `use foo::{bar::S1, baz::S2};`
SPLIT_USE_RE = re.compile(r"^\s*(?:pub\s+)?use\s+([A-Za-z0-9_:]+)::\{[^}]*::[^}]*\}.*;")
# fallback deep-use regex (same as previous nested detector when requested)
DEEP_USE_RE = re.compile(r"^\s*(?:pub\s+)?use\s+([A-Za-z0-9_]+)(?:::(?P<second>[A-Za-z0-9_]+))?::(?:[A-Za-z0-9_]+::){2,}[A-Za-z0-9_]+.*;")
IGNORED_DIRS = {"target", ".git", "node_modules"}


def should_ignore(path: Path) -> bool:
    for p in path.parts:
        if p in IGNORED_DIRS:
            return True
    return False


def _collect_full_use_statement(lines: list[str], start_idx: int) -> tuple[str, int]:
    """Collect a `use` statement possibly spanning multiple lines.

    Returns (combined_text, end_index).
    The end_index is the last index consumed (0-based).
    """
    text = lines[start_idx]
    brace_balance = text.count('{') - text.count('}')
    idx = start_idx
    # keep appending until we see a semicolon and balanced braces
    while (';' not in text) or brace_balance > 0:
        idx += 1
        if idx >= len(lines):
            break
        nxt = lines[idx]
        text += nxt
        brace_balance += nxt.count('{') - nxt.count('}')
    return (text, idx)


def scan(root: Path, ignore_prefixes: set[str]) -> list[tuple[Path,int,str]]:
    """Scan for nested `use` statements (depth >= 3) and return matches."""
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
                        m = NESTED_USE_RE.match(line)
                        if not m:
                            continue
                        # derive the prefix to check against ignore list robustly
                        # extract the path portion between `use` and `;` and split into segments
                        path_part = line.split('use', 1)[1].split(';', 1)[0].strip()
                        segments = [s for s in path_part.split('::') if s]
                        if not segments:
                            continue
                        if segments[0] == 'crate' and len(segments) >= 2:
                            prefix = f"crate::{segments[1]}"
                        else:
                            prefix = segments[0]
                        # skip if prefix matches any ignored prefix
                        if any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                            continue
                        findings.append((fpath, i, line.rstrip('\n')))
            except Exception:
                # ignore unreadable files
                continue
    return findings


def scan_for_segments(root: Path, segments_to_find: set[str], ignore_prefixes: set[str]) -> list[tuple[Path,int,str,str]]:
    """Scan for `use` statements that contain any of the provided segments.

    Returns tuples of (path, line_no, line, matched_segment).
    """
    findings = []
    if not segments_to_find:
        return findings
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in IGNORED_DIRS]
        for fname in filenames:
            if not fname.endswith('.rs'):
                continue
            fpath = Path(dirpath) / fname
            try:
                with fpath.open('r', encoding='utf-8') as f:
                    lines = f.readlines()
                i = 0
                while i < len(lines):
                    line = lines[i]
                    if 'use' not in line:
                        i += 1
                        continue
                    text, end_idx = _collect_full_use_statement(lines, i)
                    path_part = text.split('use', 1)[1].split(';', 1)[0].strip()
                    segments = [s for s in path_part.split('::') if s]
                    if not segments:
                        i = end_idx + 1
                        continue
                    if segments[0] == 'crate' and len(segments) >= 2:
                        prefix = f"crate::{segments[1]}"
                    else:
                        prefix = segments[0]
                    if any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                        i = end_idx + 1
                        continue
                    for seg in segments_to_find:
                        if f"::{seg}::" in path_part or path_part.endswith(f"::{seg}") or f"::{seg}," in path_part:
                            findings.append((fpath, i+1, text.rstrip('\n'), seg))
                            break
                    i = end_idx + 1
            except Exception:
                continue
    return findings


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description='Find nested use statements')
    p.add_argument('paths', nargs='*', help='Paths to scan (default: repo root)')
    p.add_argument('--ignore-prefix', action='append', default=[],
                   help='Prefix to ignore (can be passed multiple times), e.g. "crate::core"')
    p.add_argument('--include-core', action='store_true', help='Include `crate::core` in scanning (default: ignored)')
    p.add_argument('--include-types', action='store_true', help='Include `crate::types` in scanning (default: ignored)')
    p.add_argument('--segment', action='append', default=[],
                   help='Report `use` lines that contain this path segment (repeatable). Example: --segment computation')
    p.add_argument('--only-segment', action='store_true', help='Only report segment matches (skip depth-based nested reporting)')
    p.add_argument('--only-deep', action='store_true', help='Only report depth-based nested uses (skip segment reporting)')
    p.add_argument('--deep', action='store_true', help='Also run deep (depth >=3) detection in addition to default split-use detection')
    p.add_argument('--split', action='store_true', help='When segments are provided, print nested and segment matches separately')
    args = p.parse_args(argv)

    # build ignore list: always ignore std, and ignore crate::core and crate::types by default
    ignore_prefixes = set(args.ignore_prefix)
    ignore_prefixes.update({'std', 'crate::core', 'crate::types'})
    # allow opt-in via --include-* flags
    if args.include_core:
        ignore_prefixes.discard('crate::core')
    if args.include_types:
        ignore_prefixes.discard('crate::types')

    roots = [Path(p) for p in args.paths] if args.paths else [Path('.')]
    split_findings = []
    nested_findings = []
    segment_findings = []

    for r in roots:
        if r.is_file() and r.suffix == '.rs':
            # read all lines and iterate indices so we can collect multi-line use statements
            with r.open('r', encoding='utf-8') as f:
                lines = f.readlines()
            idx = 0
            while idx < len(lines):
                line = lines[idx]
                # split-use detection (default)
                if not args.only_deep:
                    if SPLIT_USE_RE.search(line) or ("use" in line and "{" in line):
                        text, end_idx = _collect_full_use_statement(lines, idx)
                        path_part = text.split('use', 1)[1].split(';', 1)[0].strip()
                        segments = [s for s in path_part.split('::') if s]
                        if segments:
                            if segments[0] == 'crate' and len(segments) >= 2:
                                prefix = f"crate::{segments[1]}"
                            else:
                                prefix = segments[0]
                            if not any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                                split_findings.append((r, idx+1, text.rstrip('\n')))
                        idx = end_idx + 1
                        continue
                # deep-use detection (when requested)
                if args.only_deep or args.segment or args.deep:
                    if DEEP_USE_RE.match(line):
                        text, end_idx = _collect_full_use_statement(lines, idx)
                        path_part = text.split('use', 1)[1].split(';', 1)[0].strip()
                        segments = [s for s in path_part.split('::') if s]
                        if segments:
                            if segments[0] == 'crate' and len(segments) >= 2:
                                prefix = f"crate::{segments[1]}"
                            else:
                                prefix = segments[0]
                            if not any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                                nested_findings.append((r, idx+1, text.rstrip('\n')))
                        idx = end_idx + 1
                        continue
                # segment detection for single-file path
                if args.segment and 'use' in line:
                    text, end_idx = _collect_full_use_statement(lines, idx)
                    path_part = text.split('use', 1)[1].split(';', 1)[0].strip()
                    if path_part:
                        segments = [s for s in path_part.split('::') if s]
                        if segments:
                            if segments[0] == 'crate' and len(segments) >= 2:
                                prefix = f"crate::{segments[1]}"
                            else:
                                prefix = segments[0]
                            if not any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                                for seg in args.segment:
                                    if f"::{seg}::" in path_part or path_part.endswith(f"::{seg}") or f"::{seg}," in path_part:
                                        segment_findings.append((r, idx+1, text.rstrip('\n'), seg))
                    idx = end_idx + 1
                    continue
                idx += 1
        elif r.is_dir():
            # use directory helpers
            if not args.only_deep:
                # scan for split uses in dir
                for dirpath, dirnames, filenames in os.walk(r):
                    dirnames[:] = [d for d in dirnames if d not in IGNORED_DIRS]
                    for fname in filenames:
                        if not fname.endswith('.rs'):
                            continue
                        fpath = Path(dirpath) / fname
                        try:
                            with fpath.open('r', encoding='utf-8') as f:
                                for i, line in enumerate(f, start=1):
                                    if SPLIT_USE_RE.match(line):
                                        path_part = line.split('use', 1)[1].split(';', 1)[0].strip()
                                        segments = [s for s in path_part.split('::') if s]
                                        if segments:
                                            if segments[0] == 'crate' and len(segments) >= 2:
                                                prefix = f"crate::{segments[1]}"
                                            else:
                                                prefix = segments[0]
                                            if not any(prefix == ip or prefix.startswith(f"{ip}::") for ip in ignore_prefixes):
                                                split_findings.append((fpath, i, line.rstrip('\n')))
                        except Exception:
                            continue
            if args.only_deep or args.segment or args.deep:
                nested_findings.extend(scan(r, ignore_prefixes))
            if args.segment:
                segment_findings.extend(scan_for_segments(r, set(args.segment), ignore_prefixes))

    printed = False

    if split_findings and not args.only_deep:
        print('Split `use` statements detected (braced modules that split paths):')
        for path, ln, line in split_findings:
            print(f'  {path}:{ln}: {line}')
        printed = True

    if (nested_findings and (args.only_deep or args.deep)):
        print('\nLong `use` statements detected (depth >=3):')
        for path, ln, line in nested_findings:
            print(f'  {path}:{ln}: {line}')
        printed = True

    if args.segment:
        if segment_findings:
            print('\n`use` statements containing requested segments:')
            for path, ln, line, seg in segment_findings:
                print(f'  {path}:{ln}: ({seg}) {line}')
            printed = True
        else:
            print('\nNo segment matches found.')

    if not printed:
        print('No split or long `use` statements found (with requested filters).')

    print(f"\nIgnored prefixes: {sorted(ignore_prefixes)}")
    if args.segment:
        print(f"Segments searched: {sorted(set(args.segment))}")
    return 2 if printed else 0

    if not findings:
        print('No nested `use` statements found.')
        return 0

    print('Nested `use` statements detected (depth >=3):')
    for path, ln, line in findings:
        print(f'  {path}:{ln}: {line}')

    print('\nGuidance: prefer flatter imports or import the specific submodule; consider adding an alias if needed.')
    print(f"\nIgnored prefixes: {sorted(ignore_prefixes)}")
    return 2


if __name__ == '__main__':
    raise SystemExit(main())
