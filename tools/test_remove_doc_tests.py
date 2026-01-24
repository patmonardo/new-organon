import sys
import unittest
from pathlib import Path

# Ensure repo root on path so "tools" is importable during test run
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools.remove_doc_tests import FENCE_RE, apply_transformations_to_text


class TestDocFenceHandling(unittest.TestCase):
    def test_fence_re_captures_backticks(self):
        cases = [
            ('```', '```'),
            ('````', '````'),
            ('```rust', '```'),
            ('````rust', '````'),
            ('/// ```', '```'),
            ('/// ````', '````'),
            ('``` rust', '```'),
        ]
        for s, expected in cases:
            m = FENCE_RE.search(s)
            self.assertIsNotNone(m, msg=f"FENCE_RE did not match: {s!r}")
            self.assertEqual(m.group('fence'), expected)

    def test_disable_preserves_backticks_line_doc(self):
        src = '/// ```rust\n/// let x = 1;\n/// ```\n'
        out = apply_transformations_to_text(src, delete=False, only_rust=False, line_doc_only=True)
        # opening fence should be transformed to include ,ignore and preserve backtick count
        self.assertIn('/// ```rust,ignore\n', out)
        # closing fence should remain
        self.assertIn('/// ```\n', out)

        src2 = '/// ````rust\n/// code\n/// ````\n'
        out2 = apply_transformations_to_text(src2, delete=False, only_rust=False, line_doc_only=True)
        self.assertIn('/// ````rust,ignore\n', out2)
        self.assertIn('/// ````\n', out2)

    def test_delete_line_doc_block(self):
        src = '/// ```rust\n/// let x = 1;\n/// println!("hi");\n/// ```\n'
        out = apply_transformations_to_text(src, delete=True, only_rust=False, line_doc_only=True)
        # entire doc block should be removed
        self.assertNotIn('let x = 1', out)
        self.assertNotIn('```', out)


if __name__ == '__main__':
    unittest.main()
