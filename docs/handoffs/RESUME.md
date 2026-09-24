# Resume - Local AI Kit

Branch: `feature/initial-implementation`; main unchanged. Resolve current local and remote HEAD before working.

## Current continuation

The real Rust session engine, Codex native execution driver, and permission-scoped execution service are now implemented and tested against synthetic native peers. Existing discovery, Codex metadata, Node/Python SDKs, and headless selectors remain intact. New tests: eight runtime lifecycle tests, six Codex execution tests, and five public-service execution tests. No live inference has been submitted.

Ephemeral Codex threads preserve in-process conversations but do not claim persisted cross-process resumption. Session creation validates account/catalog metadata; only an explicit turn starts inference. A host-owned execution grant controls named workspaces, writes, paid settings, and native request review. RPC cannot set these grants or choose arbitrary executables.

## Active next work

Connect the native event subscriptions and HTTP/SSE bridge, wire the CLI execution factory and explicit browser pairing, and use the existing showcase against live native data. Then continue the remaining adapters, generated types, reusable React packages, packaging, and platform qualification from the original plan. Do not recreate implemented discovery/SDK/UI slices.

## Existing evidence

See `docs/qualification/2026-09-24-codex-metadata.md` for the previous 209-test checkpoint and live non-generative Codex metadata check. See `docs/qualification/2026-09-24-native-execution.md` for this continuation. The full original 18-task release candidate is not yet complete. No merge to main, publication, additional paid coding agents, or unrestricted credential export is authorized.

## Native HTTP and live demo increment

The CLI now supports loopback HTTP, explicit single-use pairing, a native-served demo, and separately authorized Codex execution in a named workspace. Stdio event subscriptions and native SSE use the real session engine. Node/Python receive typed event-gap errors without losing unrelated RPC requests. The actual browser displayed real Codex models using a non-generative metadata check. See `docs/integration/live-demo.md` and `docs/qualification/2026-09-24-native-http-demo.md`.

Next: expand shared native adapters, strengthen process-tree cleanup and session freshness handling, generate SDK/domain contracts and API documentation, complete reusable React controls, and qualify packaged artifacts. The full release candidate remains incomplete; do not restart the already connected SDK/native/browser chain.

## Additional native provider
Claude Code now has native control metadata and text-only structured execution, alongside Codex. Use repeatable `--allow-metadata PROVIDER` and `--allow-execution PROVIDER` flags. See `docs/adapters/claude-code.md`. No live inference was submitted. Shared ACP, HTTP providers/gateways and the remaining release/SDK/UI requirements are next.

## Gateway route boundary
Pending multi-provider gateway metadata is integrated with route-scoped model IDs, vendor identity, and billing. Extra routes remain execution-disabled unless an executor is registered. The gateway regression and full Windows native baseline pass. Next: shared ACP engine and native Google/Grok adapters.

## Shared ACP core
ACP parsing and process/runtime integration are now implemented. Nine catalog tests and eight synthetic-child execution tests pass, alongside the full Windows workspace. Grok native initialization/session metadata was checked without inference. Next: wire provider-specific launch/account/usage evidence, then OpenCode/local APIs, SDK generated contracts, reusable React components, and release qualification. See docs/adapters/acp.md.

## Bounded HTTP foundation
Implemented the pending HTTP client and verified all 189 Windows native tests. Grok metadata is wired; native execution remains gated separately. Next: registered local/API endpoints, provider-specific execution, generated SDK contracts, React controls, and packaged release checks.

## HTTP provider routes
Seven HTTP protocol families now implement catalog normalization and text sessions through the existing runtime. The CLI accepts explicit endpoint files and provider grants; local runtimes use known loopback defaults only after probing permission. All 207 Windows Rust tests passed before the current checkpoint. API input capability assertions are explicitly host-declared when the model catalog lacks evidence. Next: OpenCode and native Google/Grok execution, generated contracts, React packages, host examples, packaging and qualification. See docs/adapters/http-providers.md.

## OpenCode gateway
Implemented distinct upstream catalogs/native variants and permission-verified text sessions through local HTTP/SSE. Config supports kind opencode with explicit providerBilling declarations when the gateway omits billing evidence. Session cleanup never deletes user-owned conversations. Next: remaining native clients, generated contracts, reusable React controls, host packaging and platform qualification.

## Canonical SDK and API generation
Rust owns request/response maps as well as domain records. Deterministic generation now creates TypeScript/Python types, Python client stubs, OpenAPI and the JSON-RPC manifest; drift verification is implemented. Runtime health and adapter coverage distinguish implementation from user grants. Next: reusable React controls, remaining native clients and host integration/packaging qualification.

## Optional reusable React packages
Added side-effect-free immutable SnapshotStore, React external-store bindings, and eight reusable styled selector variants. Model lists virtualize 5,000 entries and preserve route identity; sensitive options require a host callback and stale approvals are rejected. Node SDK 39, selector 24, React/components 13 tests pass. See packages/react, packages/ui-react, examples/react, and the React qualification report. Remaining: native client coverage/host transports, clean-package/platform and security/performance qualification.
