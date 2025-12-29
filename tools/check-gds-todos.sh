#!/usr/bin/env bash

set -euo pipefail

# Enforce GDS TODO format: TODO(owner,YYYY-MM-DD): message
# Defaults to scanning changed GDS files; pass paths to override.

pattern='TODO\([A-Za-z0-9_-]\+,[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}\):'

if command -v git >/dev/null 2>&1; then
  changed_unstaged=$(git diff --name-only -- gds || true)
  changed_staged=$(git diff --name-only --cached -- gds || true)
  files=$(printf '%s\n%s\n' "$changed_unstaged" "$changed_staged" | sort -u)
else
  files=""
fi

if [ "$#" -gt 0 ]; then
  files="$*"
fi

if [ -z "$files" ]; then
  echo "No GDS files to check."
  exit 0
fi

fail=0

for file in $files; do
  case "$file" in
    gds/doc/*) continue ;;
    *.md|*.MD) continue ;;
  esac

  # Skip non-existent files (deleted in diff)
  if [ ! -f "$file" ]; then
    continue
  fi

  hits=$(rg -P "TODO(?!\([A-Za-z0-9_-]+,[0-9]{4}-[0-9]{2}-[0-9]{2}\):)" --no-heading --line-number --color=never "$file" || true)
  if [ -n "$hits" ]; then
    echo "Non-compliant TODOs in $file:"
    echo "$hits"
    fail=1
  fi
done

if [ "$fail" -ne 0 ]; then
  cat <<'EOF'
Expected format: TODO(owner,YYYY-MM-DD): message
Owner can be a handle/team; date should be a real target/reevaluation date.
EOF
  exit 1
fi

echo "GDS TODO check passed."
