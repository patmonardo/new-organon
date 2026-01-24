#!/usr/bin/env python3
"""Expand `pub use <mod>::*;` into explicit `pub use <mod>::{A, B, C};` where safe.

Usage:
  ./expand_pub_uses.py path/to/dir_or_file [--apply] [--recursive] [--max-items N]

- By default prints proposed changes to stdout (preview).
- With --apply writes changes in-place and makes a .bak file next to modified files.
- --recursive will process subdirectories containing Rust files.
- Will skip modules with nested wildcard re-exports to avoid ambiguity.
- NOTE: By default this tool **excludes** `mod.rs` files (they often contain code and hand-maintained re-exports).
  Use `--include-mods` to override and include `mod.rs` files deliberately.

Heuristic parsing only — review changes and run cargo check after applying.
"""
import argparse
import os
import re
import sys
from typing import List, Optional, Tuple

ITEM_RE = re.compile(r'pub\s*(?:\([^)]*\))?\s*(struct|enum|trait|type|fn|const|static|mod)\s+([A-Za-z0-9_]+)')
PUB_USE_WILDCARD_RE = re.compile(r'^(?P<indent>\s*)pub\s+use\s+(?P<path>[A-Za-z0-9_:]+)::\*\s*;\s*$')
PUB_USE_LIST_RE = re.compile(r'pub\s+use\s+[A-Za-z0-9_:]+::\{')
PUB_USE_SIMPLE_RE = re.compile(r'^(?P<indent>\s*)pub\s+use\s+(?P<path>[A-Za-z0-9_:]+)\s*;\s*$')


def find_rust_files(dirpath: str) -> List[str]:
    files = []
    for name in sorted(os.listdir(dirpath)):
        path = os.path.join(dirpath, name)
        if os.path.isdir(path):
            continue
        if name.endswith('.rs'):
            files.append(path)
    return files


def gather_public_items(module_path: str) -> Tuple[Optional[List[str]], Optional[str]]:
    """Return (list_of_items, reason_to_skip). If skip, items is None and reason provided."""
    if not os.path.exists(module_path):
        return None, f'module not found: {module_path}'
    try:
        with open(module_path, 'r', encoding='utf-8') as f:
            text = f.read()
    except OSError as e:
        return None, f'ioerror: {e}'

    # If module itself contains wildcard pub use, we skip to avoid infinite/ambiguous expansion
    if re.search(r'pub\s+use\s+[A-Za-z0-9_:]+::\*\s*;', text):
        return None, 'contains nested wildcard pub use'

    items = set()
    for m in ITEM_RE.finditer(text):
        items.add(m.group(2))

    # Also pick up explicit pub use lists inside the module
    for m in re.finditer(r'pub\s+use\s+[A-Za-z0-9_:]+::\{([^}]*)\}\s*;', text):
        inner = m.group(1)
        for name in [p.strip().split(' as ')[0] for p in inner.split(',') if p.strip()]:
            items.add(name)

    if not items:
        return None, 'no public items found'

    return sorted(items), None


def resolve_module(base_dir: str, module_path: str) -> Optional[str]:
    """Resolve a module path like 'foo' or 'foo::bar' relative to base_dir to a file path.
    Returns first candidate that exists.
    """
    parts = module_path.split('::')
    # First try file.rs in base_dir
    cur = base_dir
    for i, part in enumerate(parts):
        file_candidate = os.path.join(cur, f'{part}.rs')
        dir_candidate = os.path.join(cur, part, 'mod.rs')
        if i == len(parts) - 1:
            # last part: check both
            if os.path.exists(file_candidate):
                return file_candidate
            if os.path.exists(dir_candidate):
                return dir_candidate
        else:
            # must descend into subdir
            cur = os.path.join(cur, part)
            if not os.path.isdir(cur):
                return None
    return None


def process_file(path: str, apply: bool, max_items: int) -> Optional[str]:
    changed = False
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    new_lines = []
    for ln in lines:
        m = PUB_USE_WILDCARD_RE.match(ln)
        if not m:
            new_lines.append(ln)
            continue
        indent = m.group('indent')
        mod_path = m.group('path')

        # resolve module relative to this file's dir
        base_dir = os.path.dirname(path)
        module_file = resolve_module(base_dir, mod_path)
        if not module_file:
            reason = f'could not resolve module {mod_path}'
            new_lines.append(ln)
            continue
        items, reason = gather_public_items(module_file)
        if items is None:
            new_lines.append(ln)
            continue
        if len(items) > max_items:
            new_lines.append(ln)
            continue

        # produce replacement
        items_sorted = sorted(items)
        items_str = ', '.join(items_sorted)
        replacement = f'{indent}pub use {mod_path}::{{{items_str}}};\n'
        new_lines.append(replacement)
        changed = True

    if changed and apply:
        bak_path = path + '.bak'
        with open(bak_path, 'w', encoding='utf-8') as f:
            f.write(''.join(lines))
        with open(path, 'w', encoding='utf-8') as f:
            f.write(''.join(new_lines))
        return f'WROTE {path} (bak: {bak_path})'
    if changed:
        return f'WOULD CHANGE {path}'
    return None


def walk_and_process(target: str, apply: bool, recursive: bool, max_items: int, include_mods: bool) -> List[str]:
    results = []
    if os.path.isdir(target):
        for root, dirs, files in os.walk(target):
            # if not recursive, only process the top dir
            if not recursive and root != target:
                continue
            for name in sorted(files):
                if not name.endswith('.rs'):
                    continue
                # Skip `mod.rs` by default to avoid touching files that commonly contain code
                if name == 'mod.rs' and not include_mods:
                    continue
                path = os.path.join(root, name)
                res = process_file(path, apply, max_items)
                if res:
                    results.append(res)
    elif os.path.isfile(target) and target.endswith('.rs'):
        # If a single file path is given and it's a mod.rs, respect include_mods flag
        if os.path.basename(target) == 'mod.rs' and not include_mods:
            print(f'Skipping mod.rs (use --include-mods to include): {target}')
            return results
        res = process_file(target, apply, max_items)
        if res:
            results.append(res)
    else:
        print(f'Not a rust file or dir: {target}', file=sys.stderr)
    return results


def main():
    p = argparse.ArgumentParser()
    p.add_argument('target', help='directory or .rs file to process')
    p.add_argument('--apply', action='store_true', help='write changes')
    p.add_argument('--recursive', action='store_true', help='recurse into subdirs')
    p.add_argument('--max-items', type=int, default=50, help='skip modules with more public items')
    p.add_argument('--include-mods', action='store_true', help='include `mod.rs` files (disabled by default)')
    args = p.parse_args()

    results = walk_and_process(args.target, args.apply, args.recursive, args.max_items, args.include_mods)
    if not results:
        print('No changes proposed or applied.')
        return
    for r in results:
        print(r)

if __name__ == '__main__':
    main()
