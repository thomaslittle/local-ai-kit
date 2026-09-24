# Native bridge qualification - 2026-09-24

- Eight native transport tests were observed failing before implementation, then passed. They exercise split frames, negotiation, correlation, cached-read responsiveness, notification safety, invalid UTF-8/JSON/batches, frame bounds, and domain errors.
- Five executable CLI tests were observed failing against the test-first entry point, then passed. They test actual processes, isolated homes, unchanged dummy credentials, no implicit discovery grant, and invalid command handling.
- `cargo test --workspace --all-targets`: PASS, 71 tests on Windows x64, Rust 1.97.1.
- `python scripts/test_native_bridge.py`: PASS, 3 Node integration tests and 3 Python integration tests. These use the real compiled native binary and Rust service, not a mock peer.
- Node 22.20.0; Python 3.13.7; TypeScript 5.8.3.
- Passive development-machine scan: 21 connections from 9 providers, 0 authenticated accounts, 0 models, 34 ms in one run. No secrets read and no native agent launched by that scan.

The standalone `codex --version`/`app-server --help` read-only commands identify installed Codex CLI 0.156.1 for the next adapter increment. No generation was requested.

Not certified: macOS/Linux executable behavior, hosted CI, quota retrieval, provider generation, HTTP/SSE native hosting, reusable React bindings, release performance budgets.
