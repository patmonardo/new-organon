#!/usr/bin/env python3
"""Generate a mod.rs that exports all modules in a directory.

Usage:
  ./generate_mod_rs.py path/to/dir [--apply] [--recursive]

By default prints to stdout. With --apply writes to path/to/dir/mod.rs
With --recursive walks subdirectories and updates their mod.rs too.

Generates:

  pub mod b;

  pub use a::*;
  pub use b::*;

Ignores `mod.rs` and files starting with '_' or not ending with '.rs'.
Includes subdirectories that contain `mod.rs` or any `.rs` files.
"""
import argparse
import os
import sys
from typing import List


def generate_mod_rs(dirpath: str) -> str:
    """Return the content for a mod.rs for directory `dirpath`.

    Includes .rs files and subdirectories that contain rust files or a mod.rs.
    Attempts to detect ambiguous glob exports and skip them to avoid name collisions.
    """
    names: List[str] = []
    entries = sorted(os.listdir(dirpath))

    for name in entries:
        path = os.path.join(dirpath, name)
        if os.path.isdir(path):
            # include a directory if it contains rust files or a mod.rs
            try:
                sub_entries = os.listdir(path)
            except OSError:
                continue
            has_rs = any(e.endswith('.rs') and not e.startswith('_') for e in sub_entries)
            has_modrs = os.path.exists(os.path.join(path, 'mod.rs'))
            if has_rs or has_modrs:
                names.append(name)
            continue

        # files
        if not name.endswith('.rs'):
            continue
        if name == 'mod.rs':
            continue
        if name.startswith('_'):
            continue
        base = name[:-3]
        names.append(base)

    names.sort()

    lines: List[str] = []
    # No header comment to avoid misleading about generated warnings or assumptions.
    lines.append('')
    for n in names:
        lines.append(f'pub mod {n};')
    lines.append('')
    for n in names:
        lines.append(f'pub use {n}::*;')
    lines.append('')
    return '\n'.join(lines) + '\n'


def write_mod_rs(dirpath: str) -> None:
    out = generate_mod_rs(dirpath)
    outpath = os.path.join(dirpath, 'mod.rs')
    with open(outpath, 'w', encoding='utf-8') as f:
        f.write(out)
    print(f'WROTE {outpath}')


def walk_and_write(dirpath: str, recursive: bool) -> None:
    # Write for this dir
    write_mod_rs(dirpath)

    if not recursive:
        return

    # Recurse into eligible subdirs
    for name in sorted(os.listdir(dirpath)):
        path = os.path.join(dirpath, name)
        if not os.path.isdir(path):
            continue
        # decide if this subdir should be processed
        try:
            sub_entries = os.listdir(path)
        except OSError:
            continue
        has_rs = any(e.endswith('.rs') and not e.startswith('_') for e in sub_entries)
        has_modrs = os.path.exists(os.path.join(path, 'mod.rs'))
        if has_rs or has_modrs:
            walk_and_write(path, recursive)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('dir', help='directory containing .rs files')
    p.add_argument('--apply', action='store_true', help='write changes to mod.rs')
    p.add_argument('--recursive', action='store_true', help='recurse into subdirectories and update their mod.rs too')
    args = p.parse_args()
    dirpath = args.dir
    if not os.path.isdir(dirpath):
        print(f'Error: not a dir: {dirpath}', file=sys.stderr)
        sys.exit(2)

    if args.apply:
        walk_and_write(dirpath, args.recursive)
    else:
        # Print generated mod.rs for the requested dir only
        print(generate_mod_rs(dirpath))


if __name__ == '__main__':
    main()
