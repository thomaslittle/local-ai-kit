#!/usr/bin/env python3
"""Exercise the real refresh scheduler with synthetic adapters and paused clocks.

The interrupted source-only audit and its failing output are retained beside
this runner as legacy_source_audit.py and audit-output.txt. No source-string
check can qualify the scheduler.
"""
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parents[5]
command = ["cargo", "test", "--locked", "-p", "lai-service", "--lib", "--test", "refresh_lanes"]
print("Behavioral refresh-lane qualification: " + " ".join(command), flush=True)
raise SystemExit(subprocess.run(command, cwd=root).returncode)
