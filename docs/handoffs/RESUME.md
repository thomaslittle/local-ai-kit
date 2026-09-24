# Resume - Local AI Kit

**Branch:** `feature/initial-implementation`. **Main:** unchanged. Resolve the latest exact commit with `git rev-parse HEAD` and the remote; do not resume from a stale chat SHA.

## Working implementation

- Real native service, bounded filesystem discovery, permission-filtered snapshots, revocation, route validation, and native stdio CLI.
- Both Node and Python SDKs communicate with the compiled Rust helper. `python scripts/test_native_bridge.py` is the end-to-end deterministic verification command.
- Codex metadata adapter: launch the installed native app-server, read account/plan, paginate models, preserve native effort/service-tier options, inspect input capabilities, and read rate-limit windows. It sends no thread/turn/generation request.
- `connections.probe` is available in both SDKs. The host must grant probe access separately: embedded policies can scope connection IDs; CLI `--allow-codex-metadata` grants the owning app the Codex provider probe.
- Probe results are cached for 60 seconds with singleflight work; failed probes back off for five seconds. Repeated reads preserve evidence times and revisions. Account switches during a probe invalidate the result. Identity stays masked without separate host permission.
- Billing-sensitive options require billing consent even on subscription routes. Permission-sensitive options fail closed pending a proper approval path.
- Existing headless selector and eight-layout showcase remain. The browser showcase is still synthetic; no native HTTP/SSE hosting has been added in this continuation.

## Verification in this continuation

On Windows x64, Rust 1.97.1 / Node 22.20.0 / Python 3.13.7:

- 107 Rust tests passed after test-first implementation and negative-case fixes.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with all crates inheriting the declared workspace MSRV.
- Six Node/Python-to-real-native-helper integration tests pass.
- The installed Codex CLI 0.156.1 metadata path was exercised through the Python SDK. It returned account/plan evidence, seven native models/options, and one usage window. No prompt was submitted; no tokens or user identity were exported to logs or committed.

See `docs/qualification/2026-09-24-codex-metadata.md` and previous records. The final regression run passed 209 tests: 107 Rust, 24 selector, 32 TypeScript SDK, 18 Python SDK, 22 showcase service/HTTP, and six real native-helper integration tests. Formatting, strict clippy, TypeScript compilation, and native build pass. Browser UI tests and clean-machine package install checks were not rerun. Do not claim the declared MSRV, macOS/Linux behavior, or native generation were live-tested here.

## Next implementation work

1. Implement the permissioned session/turn engine and Codex text execution, streaming, tool approvals, cancellation, and version-aware resumption. The current native adapter deliberately reports generation unavailable. Do not expose a raw unrestricted native-RPC escape hatch.
2. Add native HTTP/SSE and explicit browser pairing so the existing demo can consume live native data through the public SDK.
3. Continue shared ACP, Antigravity/Grok, OpenCode/local runtimes, and the other adapters following the original plan. Only passive inventory exists for non-Codex clients today.
4. Complete generated domain types, finer per-field multi-app grants, authentication-file/watch invalidation, verified shared quota linking, reusable React components, packaging, and platform/performance qualification.

Do not reinterpret completed slices as completion of entire original tasks. In particular the full Codex reference task includes execution, not just metadata.

## Resume safely

Read AGENTS.md and the original spec/18-task plan. Keep reference repositories unchanged. Preserve any local work and refresh remote refs before writing. Workspace: `C:\Users\User\Sites\local-ai-kit\.worktrees\initial-implementation`.

Use synthetic homes/protocol peers for normal tests. Any live generation needs explicit authorization; the preceding non-generative metadata check is not approval to spend inference credits. No package publication, merge to main, account rotation, token export, or unrelated settings changes.
