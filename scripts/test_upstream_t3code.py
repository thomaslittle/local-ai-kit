import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "upstream_t3code", ROOT / "scripts" / "upstream_t3code.py"
)
sync = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(sync)


class UpstreamT3CodeTests(unittest.TestCase):
    def test_pin_records_every_daily_area(self):
        self.assertEqual(sync.validate_offline(), 0)

    def test_newest_nightly_uses_the_build_day_then_sequence(self):
        tags = [
            "v0.0.45-nightly.20261002.100",
            "v0.0.46-nightly.20261015.1",
            "v0.0.46-nightly.20261015.3158",
            "v0.0.99-nightly.20261001.9",
            "v0.0.46",
        ]
        self.assertEqual(sync.newest_nightly(tags), "v0.0.46-nightly.20261015.3158")
        self.assertIsNone(sync.nightly_sort_key("v0.0.45"))

    def test_empty_diff_is_not_a_provider_change(self):
        self.assertFalse(sync.diff_is_relevant(""))
        self.assertFalse(sync.diff_is_relevant("\n"))
        self.assertTrue(sync.diff_is_relevant("apps/server/src/provider/CodexChatGptAuth.ts\n"))

    def test_report_names_the_four_watched_areas(self):
        report = sync.format_report(
            "a" * 40,
            "b" * 40,
            "diff --git a/apps/server/src/provider/CodexChatGptAuth.ts\n",
            "v0.0.46-nightly.20261015.3158",
            "c" * 40,
            "d" * 40,
            "",
        )
        for marker in (
            "local model providers",
            "cloud CLI providers",
            "installed CLI detection",
            "ChatGPT OAuth",
        ):
            self.assertIn(marker, report)
        self.assertIn("No watched-path changes.", report)
        self.assertIn("CodexChatGptAuth.ts", report)


if __name__ == "__main__":
    unittest.main()
