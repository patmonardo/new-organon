Repository tools

- `find_nested_uses.py` — detects nested `use` statements (depth >= 3) which are hard to maintain and search for.

Usage:

python3 tools/find_nested_uses.py [paths...]

Returns non-zero exit code if violations are found.
