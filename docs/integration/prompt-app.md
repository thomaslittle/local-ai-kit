# Integrating a prompt-driven desktop app

For an app such as crntly, use Local AI Kit to discover access, choose a route and model, and stream a text response. Keep scene specifications, layout validation, asset handling, and rendering in the application.

## Host and renderer

For Tauri, embed `lai-kit` in the Rust host and use the optional React selectors in the renderer. For a Node/Electron host, use the TypeScript SDK with the bundled helper over stdio. A normal hosted website cannot discover a visitor's installed clients without an explicitly paired local helper or browser extension.

The host registers endpoint/native-client configuration and grants. The renderer receives normalized public data and narrow application commands. Do not accept executable paths, arbitrary workspace paths, credentials, or unrestricted grant objects from renderer content.

## User flow

1. On the user's Connect action, enable passive discovery and scan. Imports and initial rendering stay inert.
2. Run explicit metadata checks on selected connections. `readCatalogSnapshot` and `createSnapshotStore` assemble paged catalogs. Authentication, billing, freshness, and capabilities remain separate facts.
3. Present the provider, connection, model, and its native options. Retain the complete `Selection`: `routeId`, `modelId`, `revision`, and `options`. Model names alone are not an execution identity.
4. Validate `generation.text` and the exact selection through the service. Local Ollama/LM Studio routes do not require an invented account login. Unknown billing is blocked until the host supplies a justified route classification; allowing metered access does not turn unknown billing into verified access.
5. Ask for the app's required workspace and execution scope, then register the host grant. For text-only scene specifications, use read-only mode and keep native tools disabled. Do not opt into an installed agent's native tool policy merely to generate JSON.
6. Create a session. Start one turn with a unique request ID only after the user submits their prompt. Creating a session never sends a prompt.
7. Consume the event stream, filtered by session and turn ID. Subscribe with a saved cursor before starting, or use the native `events.read` cursor/tail cursor to replay the same turn. Handle `EVENT_GAP` explicitly. An ambiguous start failure must not trigger an automatic retry, new session, or provider fallback.
8. Treat `final` status as the result. Cancellation uses the returned turn ID. Close the session and application client on shutdown; this only releases that application's resources.

Use the [embedded Ollama integration test](../../crates/lai-kit/tests/local_execution.rs) as a concrete host example. It exercises discovery, metadata, selection validation, denied execution before grants, session creation, streaming, and cleanup through the public embedded API against a loopback fixture. The [Tauri guide](../../examples/tauri/README.md) covers the host boundary. The live showroom's default launcher grants metadata only; its disabled execution controls are expected.

## Scene output and continuation

Require your own versioned scene JSON schema and validate the completed text before previewing or saving it. The current common turn input is text; do not infer toolkit image input or structured-output support merely because a provider advertises those model capabilities. Display unsupported options honestly. Provider output must not become executable HTML or host commands automatically.

Keep an open session for follow-up edits. Native suspend/resume is available only when the session reports `nativeResumption: true`; currently this is the shared ACP path when its peer advertises `loadSession`. Keep the opaque token in app memory and never render/log it. Tokens are owner-scoped, single-use, and invalid after revocation or helper restart. Failed resumption never becomes a fresh conversation. Other providers require a fresh session after restart; transcript import is a separate application feature.

## Readiness boundary

This is an unpublished developer alpha. Deterministic adapter and package tests establish mechanics, not a guarantee that every installed version or user's subscription accepts inference. Live metadata checks do not qualify generation. Before shipping crntly integration, qualify the app's actual host lifecycle, scene validation, cancellation, and the specific provider versions you intend to support. Platform and live-provider evidence is recorded in [release qualification](../qualification/RELEASE-CANDIDATE.md).
