#!/usr/bin/env python3
"""
Replace PAGE_SIZE / PAGE_SHIFT / PAGE_MASK constants with PageUtil::PAGE_SIZE_32KB equivalents
and add `use crate::collections::PageUtil;` if missing.

Usage:
  python3 tools/replace_page_constants.py [--dry-run] paths...

Defaults to processing .rs files under provided paths.
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import List


PAT_PAGE_SIZE = re.compile(r'^(?P<indent>\s*)const\s+PAGE_SIZE\s*:\s*usize\s*=\s*4096\s*;\s*(?P<comment>//.*)?$')
PAT_PAGE_SHIFT = re.compile(r'^(?P<indent>\s*)const\s+PAGE_SHIFT\s*:\s*usize\s*=\s*12\s*;\s*(?P<comment>//.*)?$')
PAT_PAGE_MASK = re.compile(r'^(?P<indent>\s*)const\s+PAGE_MASK\s*:\s*usize\s*=\s*PAGE_SIZE\s*-\s*1\s*;\s*(?P<comment>//.*)?$')


def find_rs_files(paths: List[Path]) -> List[Path]:
    files: List[Path] = []
    for p in paths:
        if p.is_file() and p.suffix == '.rs':
            files.append(p)
        else:
            for f in p.rglob('*.rs'):
                if f.is_file():
                    files.append(f)
    return files


def ensure_import(lines: List[str]) -> (List[str], bool):
    """Ensure `use crate::collections::PageUtil;` is present after other `use` imports or at top; return (lines, changed)."""
    import_stmt = 'use crate::collections::PageUtil;\n'
    for line in lines:
        if 'use crate::collections::PageUtil' in line:
            return lines, False

    # Find last use statement to insert after
    last_use_idx = -1
    for i, line in enumerate(lines[:50]):  # only check early lines
        if line.strip().startswith('use '):
            last_use_idx = i

    if last_use_idx >= 0:
        lines.insert(last_use_idx + 1, import_stmt)
    else:
        # insert after module doc or at top
        insert_at = 0
        if lines and lines[0].startswith('//!'):
            insert_at = 1
        lines.insert(insert_at, import_stmt)

    return lines, True


def process_file(path: Path) -> (bool, List[str]):
    txt = path.read_text(encoding='utf-8')
    lines = txt.replace('\r\n', '\n').splitlines(keepends=True)
    changed = False
    changes = []

    for i, line in enumerate(lines):
        s = line.rstrip('\n')
        m = PAT_PAGE_SIZE.match(s)
        if m:
            indent = m.group('indent') or ''
            comment = m.group('comment') or ''
            new = f"{indent}const PAGE_SIZE: usize = PageUtil::PAGE_SIZE_32KB;{(' ' + comment) if comment else ''}\n"
            changes.append(f"{path}: line {i+1}: PAGE_SIZE -> PageUtil::PAGE_SIZE_32KB")
            lines[i] = new
            changed = True
            continue

        m = PAT_PAGE_SHIFT.match(s)
        if m:
            indent = m.group('indent') or ''
            comment = m.group('comment') or ''
            new = f"{indent}const PAGE_SHIFT: usize = 15;{(' ' + comment) if comment else ''}\n"
            changes.append(f"{path}: line {i+1}: PAGE_SHIFT -> 15")
            lines[i] = new
            changed = True
            continue

        m = PAT_PAGE_MASK.match(s)
        if m:
            indent = m.group('indent') or ''
            comment = m.group('comment') or ''
            new = f"{indent}const PAGE_MASK: usize = PAGE_SIZE - 1;{(' ' + comment) if comment else ''}\n"
            # This one may not need change, but keep for completeness
            # If original spacing varied, normalize it
            if line != new:
                changes.append(f"{path}: line {i+1}: PAGE_MASK normalized")
                lines[i] = new
                changed = True

    import_changed = False
    if changed:
        lines, import_changed = ensure_import(lines)
        if import_changed:
            changes.insert(0, f"{path}: inserted `use crate::collections::PageUtil;`")

    if changed:
        new_txt = ''.join(lines)
        return True, changes
    return False, []


def main(argv: List[str]) -> int:
    parser = argparse.ArgumentParser(description='Replace PAGE_* constants with PageUtil usages')
    parser.add_argument('paths', nargs='+', help='Files or directories to process')
    parser.add_argument('--dry-run', action='store_true', help='Do not modify files; only print what would change')
    args = parser.parse_args(argv)

    paths = [Path(p) for p in args.paths]
    files = find_rs_files(paths)
    total = 0

    for f in files:
        try:
            changed, changes = process_file(f)
        except Exception as e:
            print(f"Skipping {f}: {e}")
            continue

        if changed:
            total += 1
            print(f"Would modify {f}:" if args.dry_run else f"Modifying {f}:")
            for c in changes:
                print('  ' + c)
            if not args.dry_run:
                # Actually write changes (re-run process to get modified text)
                # Simpler to re-run process_file to get lines and write them here
                txt = f.read_text(encoding='utf-8')
                lines = txt.replace('\r\n', '\n').splitlines(keepends=True)
                # Apply replacements (same logic as process_file but mutating)
                for i, line in enumerate(lines):
                    s = line.rstrip('\n')
                    if PAT_PAGE_SIZE.match(s):
                        indent = PAT_PAGE_SIZE.match(s).group('indent') or ''
                        comment = PAT_PAGE_SIZE.match(s).group('comment') or ''
                        lines[i] = f"{indent}const PAGE_SIZE: usize = PageUtil::PAGE_SIZE_32KB;{(' ' + comment) if comment else ''}\n"
                    elif PAT_PAGE_SHIFT.match(s):
                        indent = PAT_PAGE_SHIFT.match(s).group('indent') or ''
                        comment = PAT_PAGE_SHIFT.match(s).group('comment') or ''
                        lines[i] = f"{indent}const PAGE_SHIFT: usize = 15;{(' ' + comment) if comment else ''}\n"
                    elif PAT_PAGE_MASK.match(s):
                        indent = PAT_PAGE_MASK.match(s).group('indent') or ''
                        comment = PAT_PAGE_MASK.match(s).group('comment') or ''
                        lines[i] = f"{indent}const PAGE_MASK: usize = PAGE_SIZE - 1;{(' ' + comment) if comment else ''}\n"

                lines, import_changed = ensure_import(lines)
                f.write_text(''.join(lines), encoding='utf-8')

    print(f"Scanned {len(files)} .rs files; {total} file(s) changed.")
    return 0


if __name__ == '__main__':
    raise SystemExit(main(sys.argv[1:]))
