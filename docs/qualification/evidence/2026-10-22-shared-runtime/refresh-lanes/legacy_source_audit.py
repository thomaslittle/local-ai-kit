#!/usr/bin/env python3
"""Release-readiness audit for the original shared-service refresh lanes.

This is intentionally a source-contract audit, not a replacement for a future
behavioral scheduler test. It exits non-zero while any required lane is missing.
"""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[5]
service = (root / "crates" / "lai-service" / "src" / "lib.rs").read_text(encoding="utf-8")
cache = (root / "crates" / "lai-service" / "src" / "cache.rs").read_text(encoding="utf-8")
metadata = (root / "crates" / "lai-service" / "src" / "metadata.rs").read_text(encoding="utf-8")
adr = (root / "docs" / "design" / "ADR-001-shared-runtime.md").read_text(encoding="utf-8")

loop_body = service.split("let handle = tokio::spawn(async move {", 1)[-1].split(
    "        });", 1
)[0]
checks = [
    (
        "scheduled inventory reconciliation",
        "refresh_due_inventory" in service
        or "cache.refresh" in loop_body
        or "inventory_reconcile" in loop_body,
    ),
    (
        "separately paced usage/reset lane",
        "refresh_due_usage" in service
        or "usage_lane" in service
        or "usage/reset" in service,
    ),
    (
        "client-interest cadence",
        "client_interest" in service
        or "subscriber_count" in service
        or "interested_clients" in service,
    ),
]

print("refresh-lane audit against original shared-service requirements")
for label, present in checks:
    print(f"  {'PASS' if present else 'MISSING'}: {label}")

print("\nCurrent background loop:")
for number, line in enumerate(service.splitlines(), 1):
    if "tokio::time::interval" in line or "refresh_due_metadata" in line:
        print(f"  lib.rs:{number}: {line.strip()}")

print("\nADR statement:")
for number, line in enumerate(adr.splitlines(), 1):
    if "Provider-supported usage/reset" in line:
        print(f"  ADR-001-shared-runtime.md:{number}: {line.strip()}")

print("\nExisting metadata cache/service tests do not cover these lanes:")
print("  runtime_sync::invalidation_is_debounced_then_the_autonomous_lane_rechecks_it")
print("  (metadata only; no inventory schedule, usage cadence, or interest gate)")

if all(present for _, present in checks):
    print("\nRESULT: PASS")
    sys.exit(0)
print("\nRESULT: FAIL - release-readiness hold; required lanes are absent")
sys.exit(1)
