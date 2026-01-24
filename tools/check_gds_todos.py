#!/usr/bin/env python3
"""Enforce GDS TODO format: TODO(owner,YYYY-MM-DD): message

Default behavior mirrors the old `tools/check-gds-todos.sh`:
- If positional paths are provided, check those files.
- Otherwise find changed files under `gds` using `git diff --name-only` (unstaged + staged).
- Skip `gds/doc/*` and markdown files.
- Report lines that contain `TODO` but that do not match `TODO(owner,YYYY-MM-DD):`

Exit code 0 when no violations, 1 when violations are found.
"""
from __future__ import annotations
import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

COMPLIANT_RE = re.compile(r"TODO\([A-Za-z0-9_-]+,[0-9]{4}-[0-9]{2}-[0-9]{2}\):")
IGNORED_GLOB = "gds/doc/"


def git_changed_gds_files() -> list[str]:
    """Return a list of changed (staged + unstaged) files under `gds/` using git."""
    try:
        # unstaged changes
        res1 = subprocess.run(["git", "diff", "--name-only", "--", "gds"], capture_output=True, text=True, check=False)
        # staged
        res2 = subprocess.run(["git", "diff", "--name-only", "--cached", "--", "gds"], capture_output=True, text=True, check=False)
        files = []
        for r in (res1, res2):
            if r.returncode == 0 and r.stdout:
                files.extend([l for l in r.stdout.splitlines() if l.strip()])
        # uniq while preserving order
        seen = set()
        out = []
        for f in files:
            if f not in seen:
                seen.add(f)
                out.append(f)
        return out
    except FileNotFoundError:
        return []


def check_file(path: Path) -> list[tuple[int, str]]:
    """Return list of (lineno, line) for non-compliant TODOs in file."""
    findings: list[tuple[int, str]] = []
    try:
        with path.open("r", encoding="utf-8") as fh:
            for i, line in enumerate(fh, start=1):
                if "TODO" in line and not COMPLIANT_RE.search(line):
                    findings.append((i, line.rstrip('\n')))
    except Exception:
        # If file cannot be read, skip it silently (mirrors previous behaviour)
        return []
    return findings


def main(argv=None) -> int:
    p = argparse.ArgumentParser(description="Check GDS TODO format: TODO(owner,YYYY-MM-DD): message")
    p.add_argument("paths", nargs="*", help="Files or paths to check (default: changed gds files)")
    args = p.parse_args(argv)

    if args.paths:
        files = []
        for pth in args.paths:
            files.append(pth)
    else:
        files = git_changed_gds_files()

    if not files:
        print("No GDS files to check.")
        return 0

    fail = False
    for file in files:
        # simple skip patterns
        if file.startswith(IGNORED_GLOB):
            continue
        if file.lower().endswith('.md'):
            continue
        p = Path(file)
        if not p.exists():
            continue
        findings = check_file(p)
        if findings:
            fail = True
            print(f"Non-compliant TODOs in {file}:")
            for ln, text in findings:
                print(f"{file}:{ln}: {text}")

    if fail:
        print("\nExpected format: TODO(owner,YYYY-MM-DD): message")
        print("Owner can be a handle/team; date should be a real target/reevaluation date.")
        return 1

    print("GDS TODO check passed.")
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
