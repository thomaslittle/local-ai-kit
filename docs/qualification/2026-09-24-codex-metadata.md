# Codex metadata and native integration qualification - 2026-09-24

## Environment

Windows x64; Rust 1.97.1; Node 22.20.0; Python 3.13.7; installed Codex CLI 0.156.1. The workspace declares an inherited Rust MSRV; only the installed toolchain was executed in this checkpoint.

## Test-first evidence

- 12 Codex parser tests: first failed against the stub, then passed. Cover unknown future plans/efforts, identity separation, redaction, input-vs-generation capabilities, missing auth/usage, API-key billing, duplicate native models, and quota alias deduplication.
- Nine synthetic native-process tests: include framing bounds, timeouts, pagination, unavailable usage, refusal of unexpected execution requests, initial account notification handling, and invalidation on later account changes.
- Four Windows executable-resolution tests: native binary, known npm packaging, extensionless npm shim, and rejection of arbitrary shell wrappers.
- Eight native metadata-service tests: separate authorization, projections, revocation, cached concurrency, wrong-account response rejection, stable evidence/revisions, and negative-cache backoff.
- Two core option tests: subscription selection does not authorize paid options; permission-sensitive settings require an approval path.
- Five initial CLI tests plus a probe-advertisement test exercise real executable processes without live accounts.

107 Rust tests passed. Six Node/Python integration tests use the actual compiled helper and native service. Static checks pass with the actual inherited MSRV; broad formatting is mechanical and all native tests were rerun.

## Live metadata check

The Python SDK launched the native Local AI Kit helper, which launched the installed Codex app-server. The result included authenticated account/plan evidence, seven native model records, native effort values that differ by model, one quota pool/window, and masked account identity. The complete check took 1.43 seconds in one run; this is not a percentile or performance guarantee.

No thread, turn, prompt, or generation request was submitted. The toolkit did not inspect token files itself or copy provider credentials. The installed client owned its login. No private identity, raw provider response, or exact personal usage reading was committed.

Initial live probing exposed an expected startup `account/updated` notification before the authoritative account response. A failing fixture pinned this behavior; initial notices are now accepted, while later account changes abort the metadata snapshot.

## Review and limits

Final review is an author self-review, not an independent reviewer. It identified and fixed snapshot revision churn, retry storms after probe failures, billing-sensitive option validation, and startup account-notification handling. Fine-grained per-field multi-app grants, full credential/profile invalidation watchers, process-tree sandboxing for general agent execution, native HTTP/SSE, native generation, full credit normalization, macOS/Linux execution, and release performance budgets remain unqualified.

This is a functional metadata checkpoint, not completion of the full product or 18-task plan.

## Final regression run

The exact worktree contents were checked again before commit:

| Suite | Passed |
| --- | ---: |
| Rust workspace, locked dependencies, all targets | 107 |
| Headless selector | 24 |
| TypeScript SDK | 32 |
| Python SDK | 18 |
| Synthetic showcase service and local HTTP tests | 22 |
| Node/Python SDK to real compiled Rust helper | 6 |

Total: 209 passing tests across these suites, zero failures. `cargo fmt --all --check`, strict workspace clippy, TypeScript compilation, and the native helper build also passed. Browser-interaction tests and clean-machine package installation were not rerun in this continuation; do not count older results as current qualification. Hosted CI was not used as evidence.
