#!/usr/bin/env python3
"""
Scan Rust source files for occurrences of `crate::` that are not in `use` statements.
Ignores block comments, line comments, and string literals.

Usage:
  python tools/find_crate_usages.py [path]

Example:
  python tools/find_crate_usages.py .
"""

import os
import re
import sys

root = sys.argv[1] if len(sys.argv) > 1 else "."
exclude_dirs = {"target", ".git", "node_modules"}

# Remove /* ... */ blocks
_block_re = re.compile(r"/\*.*?\*/", re.DOTALL)
# Match string literals (including raw and escaped forms). Use DOTALL flag.
# Use double-quoted Python string to avoid backslash/quote escaping issues.
_string_re = re.compile(r"r?\"([^\"\\]|\\.)*\"|'([^'\\]|\\.)*'", re.DOTALL)

matches = []
for dirpath, dirnames, filenames in os.walk(root):
    # prune excluded dirs
    dirnames[:] = [d for d in dirnames if d not in exclude_dirs]
    for fname in filenames:
        if not fname.endswith('.rs'):
            continue
        fpath = os.path.join(dirpath, fname)
        try:
            with open(fpath, 'r', encoding='utf-8') as fh:
                text = fh.read()
        except Exception:
            continue

        # strip block comments first
        stripped = _block_re.sub('', text)
        lines = stripped.splitlines()
        for idx, line in enumerate(lines, start=1):
            # ignore use statements (leading whitespace allowed, optional `pub`)
            if re.match(r"^\s*(pub\s+)?use\b", line):
                continue
            # remove line comments
            code = re.sub(r"//.*", "", line)
            # remove string literals
            code = _string_re.sub('', code)
            if 'crate::' in code:
                matches.append((fpath, idx, line.rstrip()))

if not matches:
    print('No non-use crate:: occurrences found')
    sys.exit(0)

# Print results grouped by file
from collections import defaultdict
by_file = defaultdict(list)
for fpath, lineno, text in matches:
    by_file[fpath].append((lineno, text))

for fpath in sorted(by_file.keys()):
    print(f"{fpath}:")
    for lineno, text in by_file[fpath]:
        print(f"  {lineno}: {text}")
    print()

print(f"Found {len(matches)} occurrence(s) in {len(by_file)} file(s)")
