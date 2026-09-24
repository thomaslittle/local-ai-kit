# Resume - Local AI Kit

Branch: `feature/initial-implementation`; main unchanged. Resolve current local and remote HEAD before working.

## Current continuation

The real Rust session engine, Codex native execution driver, and permission-scoped execution service are now implemented and tested against synthetic native peers. Existing discovery, Codex metadata, Node/Python SDKs, and headless selectors remain intact. New tests: eight runtime lifecycle tests, six Codex execution tests, and five public-service execution tests. No live inference has been submitted.

Ephemeral Codex threads preserve in-process conversations but do not claim persisted cross-process resumption. Session creation validates account/catalog metadata; only an explicit turn starts inference. A host-owned execution grant controls named workspaces, writes, paid settings, and native request review. RPC cannot set these grants or choose arbitrary executables.

## Active next work

Connect the native event subscriptions and HTTP/SSE bridge, wire the CLI execution factory and explicit browser pairing, and use the existing showcase against live native data. Then continue the remaining adapters, generated types, reusable React packages, packaging, and platform qualification from the original plan. Do not recreate implemented discovery/SDK/UI slices.

## Existing evidence

See `docs/qualification/2026-09-24-codex-metadata.md` for the previous 209-test checkpoint and live non-generative Codex metadata check. See `docs/qualification/2026-09-24-native-execution.md` for this continuation. The full original 18-task release candidate is not yet complete. No merge to main, publication, additional paid coding agents, or unrestricted credential export is authorized.
