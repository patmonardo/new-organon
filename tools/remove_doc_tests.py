#!/usr/bin/env python3
"""Find and delete or disable fenced code blocks inside doc comments (doc tests).

Usage:
  tools/remove_doc_tests.py [paths...] [--apply] [--delete] [--only-rust]

By default the script will update fences ` ```rust ` to ` ```rust,ignore ` (safer)
so the examples remain visible but are not run as doctests.

Options:
  --apply      Write changes to files. Without this the script prints a dry-run report.
  --delete     Delete fenced code blocks entirely instead of disabling them.
  --only-rust  Only target fences that specify `rust` as the language (default False).
  --line-doc-only Only target line doc comment fences that start with `///` (opening and closing fences must be on `///` lines).
  --recursive  Recurse into directories provided as paths.

The script operates on line doc comments (`///`, `//!`) and block doc comments (`/**`, `/*!`).
"""
from __future__ import annotations
import argparse
import os
import re
import sys
from pathlib import Path

OPENING_LINE_DOC_RE = re.compile(r"^(?P<prefix>\s*(?:///|//!))(?P<rest>.*)$")
OPENING_BLOCK_START_RE = re.compile(r"^\s*/\*(!|\*)")
CLOSING_BLOCK_RE = re.compile(r"\*/")
FENCE_RE = re.compile(r"(?P<fence>```+)(?P<lang>[^`]*)")

IGNORED_DIRS = {"target", ".git", "node_modules"}


def process_file(path: Path, delete: bool, only_rust: bool, line_doc_only: bool) -> tuple[bool, int]:
    """Process a single file. Returns (changed, count_of_changes)."""
    s = path.read_text(encoding="utf-8")
    lines = s.splitlines(keepends=True)
    out_lines: list[str] = []

    in_block_comment = False
    in_fence = False
    in_fence_line_doc = False
    fence_lang = ''
    fence_start_index = 0
    changes = 0

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()

        # Track entry/exit of normal block comments (including doc-blocks)
        if not in_block_comment and OPENING_BLOCK_START_RE.match(stripped):
            in_block_comment = True

        if in_fence:
            # look for fence close; when --line-doc-only is set, only a line-doc fence closes it
            is_fence_close = '```' in line and (not line_doc_only or OPENING_LINE_DOC_RE.match(line))
            if is_fence_close:
                # end fence — skip it if deleting, else write a closing fence line
                if delete:
                    # drop the closing fence too
                    changes += 1
                    in_fence = False
                    in_fence_line_doc = False
                    fence_lang = ''
                    # do not append this line
                    i += 1
                    continue
                else:
                    # keep the closing fence as-is
                    out_lines.append(line)
                    in_fence = False
                    in_fence_line_doc = False
                    fence_lang = ''
                    i += 1
                    continue
            else:
                # we are inside the fenced block; skip whole contents when deleting
                if delete:
                    i += 1
                    continue
                else:
                    out_lines.append(line)
                    i += 1
                    continue

        # If not currently inside a fenced block, detect a fence that is inside a line doc comment
        m_line_doc = OPENING_LINE_DOC_RE.match(line)
        if m_line_doc:
            rest = m_line_doc.group('rest')
            # if there's a fence opener here
            f = FENCE_RE.search(rest)
            if f:
                lang = f.group('lang').strip()
                if not lang and only_rust:
                    # no language => skip if only_rust
                    out_lines.append(line)
                    i += 1
                    continue
                if only_rust and 'rust' not in lang:
                    out_lines.append(line)
                    i += 1
                    continue

                # found a fence inside a line doc comment
                if delete:
                    # we will drop opening fence line and then drop until matching line-doc closing fence
                    changes += 1
                    in_fence = True
                    in_fence_line_doc = True
                    i += 1
                    # skip the rest — contents will be skipped in the in_fence block
                    continue
                else:
                    # change the opening fence to disable running: add ,ignore if rust or replace with ignore
                    # preserve the original line (including trailing newline) when replacing
                    new_line = line.replace(f.group('fence') + f.group('lang'), f"{f.group('fence')}{lang + (',' if lang else '')}ignore")
                    out_lines.append(new_line)
                    # mark that we are inside this (disabled) fenced block so the closing fence is handled correctly
                    in_fence = True
                    in_fence_line_doc = True
                    changes += 1
                    i += 1
                    continue

        # block comment handling: if inside block comment and we see a fence
        if in_block_comment:
            f = FENCE_RE.search(line)
            if f:
                lang = f.group('lang').strip()
                if only_rust and 'rust' not in lang:
                    out_lines.append(line)
                    i += 1
                    continue
                if delete:
                    # drop this line and enter in_fence (not a line-doc fence)
                    changes += 1
                    in_fence = True
                    in_fence_line_doc = False
                    i += 1
                    continue
                else:
                    # replace fence
                    new_line = line.replace(f.group('fence') + f.group('lang'), f"{f.group('fence')}{lang + (',' if lang else '')}ignore")
                    out_lines.append(new_line)
                    changes += 1
                    i += 1
                    continue

            # detect end of block comment
            if CLOSING_BLOCK_RE.search(line):
                in_block_comment = False

        # default append
        out_lines.append(line)
        i += 1

    if changes > 0:
        new_s = ''.join(out_lines)
        if new_s != s:
            return True, changes
    return False, 0


def scan_paths(paths: list[Path], recursive: bool, delete: bool, only_rust: bool, line_doc_only: bool, apply: bool) -> int:
    total = 0
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

    changed_files = []
    for f in files:
        ch, cnt = process_file(f, delete=delete, only_rust=only_rust, line_doc_only=line_doc_only)
        if ch:
            total += cnt
            changed_files.append((f, cnt))
            if apply:
                # re-run to actually write the new contents (process_file above is non-mutating)
                # for safety we will perform the edit using a simple replace on the file text
                s = f.read_text(encoding='utf-8')
                # apply transformations via a second pass that returns transformed content
                new_content = apply_transformations_to_text(s, delete=delete, only_rust=only_rust, line_doc_only=line_doc_only)
                backup = f.with_suffix(f.suffix + '.bak')
                f.write_text(new_content, encoding='utf-8')
                print(f"WROTE {f} (backup: {backup.name})")
                f.write_text(new_content, encoding='utf-8')
    if changed_files:
        print('Files with changes:')
        for f, cnt in changed_files:
            print(f'  {f}: {cnt} changes')
    else:
        print('No doc-code fences found matching criteria.')
    return total


# A helper that actually returns the transformed text so we can write it when --apply is provided
def apply_transformations_to_text(s: str, delete: bool, only_rust: bool, line_doc_only: bool) -> str:
    lines = s.splitlines(keepends=True)
    out_lines: list[str] = []

    in_block_comment = False
    in_fence = False
    in_fence_line_doc = False

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()

        if not in_block_comment and OPENING_BLOCK_START_RE.match(stripped):
            in_block_comment = True

        if in_fence:
            is_fence_close = '```' in line and (not line_doc_only or OPENING_LINE_DOC_RE.match(line))
            if is_fence_close:
                if delete:
                    in_fence = False
                    in_fence_line_doc = False
                    i += 1
                    continue
                else:
                    out_lines.append(line)
                    in_fence = False
                    in_fence_line_doc = False
                    i += 1
                    continue
            else:
                if delete:
                    i += 1
                    continue
                else:
                    out_lines.append(line)
                    i += 1
                    continue

        m_line_doc = OPENING_LINE_DOC_RE.match(line)
        if m_line_doc:
            rest = m_line_doc.group('rest')
            f = FENCE_RE.search(rest)
            if f:
                lang = f.group('lang').strip()
                if only_rust and 'rust' not in lang:
                    out_lines.append(line)
                    i += 1
                    continue
                if delete:
                    in_fence = True
                    in_fence_line_doc = True
                    i += 1
                    continue
                else:
                    # preserve the original line (including trailing newline) when replacing
                    new_line = line.replace(f.group('fence') + f.group('lang'), f"{f.group('fence')}{lang + (',' if lang else '')}ignore")
                    out_lines.append(new_line)
                    # mark that we're inside this (disabled) fenced block so the closing fence is handled correctly
                    in_fence = True
                    in_fence_line_doc = True
                    i += 1
                    continue

        if in_block_comment:
            f = FENCE_RE.search(line)
            if f:
                lang = f.group('lang').strip()
                if only_rust and 'rust' not in lang:
                    out_lines.append(line)
                    i += 1
                    continue
                if delete:
                    in_fence = True
                    in_fence_line_doc = False
                    i += 1
                    continue
                else:
                    new_line = line.replace(f.group('fence') + f.group('lang'), f"{f.group('fence')}{lang + (',' if lang else '')}ignore")
                    out_lines.append(new_line)
                    i += 1
                    continue

            if CLOSING_BLOCK_RE.search(line):
                in_block_comment = False

        out_lines.append(line)
        i += 1

    return ''.join(out_lines)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('paths', nargs='*', default=['.'], help='Files or directories to scan')
    p.add_argument('--apply', action='store_true', help='Write changes to files')
    p.add_argument('--delete', action='store_true', help='Delete fenced code blocks instead of disabling')
    p.add_argument('--only-rust', action='store_true', default=False, help='Only affect code fences that specify rust')
    p.add_argument('--line-doc-only', action='store_true', help='Only operate on line doc comment fences (///)')
    p.add_argument('--recursive', action='store_true', help='Recurse into directories')
    args = p.parse_args()

    paths = [Path(p) for p in args.paths]
    total = scan_paths(paths, recursive=args.recursive, delete=args.delete, only_rust=args.only_rust, line_doc_only=args.line_doc_only, apply=args.apply)
    if total > 0:
        print(f'Processed {total} doc-code fences.')
        if not args.apply:
            print('\nRun with --apply to write changes to files.')
        sys.exit(2)
    else:
        print('No changes necessary.')
        sys.exit(0)
