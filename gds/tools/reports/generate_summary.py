#!/usr/bin/env python3
"""Simple report generator for gds config inventory CSV.

Reads `config_inventory.csv` and prints a small summary to stdout.
"""
import csv
from collections import Counter
from pathlib import Path

csv_path = Path(__file__).parent / 'config_inventory.csv'
if not csv_path.exists():
    print(f"No inventory found at {csv_path}. Run the scanner to generate it.")
    raise SystemExit(1)

rows = list(csv.DictReader(csv_path.open()))
print(f"Configs found: {len(rows)}")

by_file = Counter(r['path'] for r in rows)
by_validate = Counter(r['has_validate'] for r in rows)
by_builder = Counter(r['has_Builder'] for r in rows)
by_default = Counter(r['derive_Default'] for r in rows)

print('\nTop 10 files by config count:')
for f, c in by_file.most_common(10):
    print(f" - {f}: {c}")

print('\nValidation support:')
for k, v in sorted(by_validate.items(), key=lambda x:-int(x[0]=="True")):
    print(f" - has_validate={k}: {v}")

print('\nBuilder support:')
for k, v in by_builder.items():
    print(f" - has_Builder={k}: {v}")

print('\nDerives Default:')
for k, v in by_default.items():
    print(f" - derive_Default={k}: {v}")
