# Native service qualification - 2026-09-24

Platform: Windows x64. Rust 1.97.1. Synthetic homes/accounts only.

- Baseline `cargo test --workspace --all-targets`: FAIL, 12 service tests hit committed `unimplemented!` bodies.
- Added boundary suite: FAIL, 4/4 before implementation.
- After implementation: `cargo test --workspace --all-targets`: PASS, 58 tests.
- Covers denied clients, duplicate registrations, caller spoofing, snapshot/model/quota projections, explicit identity grants, revocation, stale-selection validation, billing separation, protocol mismatch, parameter allowlists, redacted source failures, and responsive snapshots during slow scans.
- Concurrent refreshes share one source call. Discovery returns no fixture accounts in production; tests inject a synthetic source explicitly.

Not qualified here: live subscriptions, native provider execution, network enrichment, cross-platform releases, HTTP bridge, SDK-to-native process integration. These remain separate implementation gates.
