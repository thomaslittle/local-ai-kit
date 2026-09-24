# Native host and shared broker qualification

Work branch: feature/initial-implementation. Windows x64; Rust 1.97.1. No provider inference was used.

- Native workspace: 240 tests pass, including actual binary Native Messaging frames, independent origin validation, partial/oversized messages, named-pipe grants, one-use credentials, duplicate listener rejection, two-client isolation, and parent/child lifetime separation.
- Strict all-workspace Clippy passed before the final documentation/help-only edit.
- Node SDK -> bridge -> real Windows broker -> Rust discovery and Python async SDK -> same broker both passed. Node closing its bridge left another client connected.
- JS SDK full suite: 45 tests pass; selector core: 24. Six new browser-Port transport tests cover handshake/data, safe errors, disconnection, size limits/cancellation, same-tick subscription events, and malformed replies.
- Four native-host preparation tests pass. Existing installation directories are preserved, origins are exact, and installed payloads have checksummed receipts.
- Generic extension built successfully with esbuild. No actual browser profile/registry registration, extension-store publication, or live model generation was performed.

Unix socket implementation uses permission mode 0600. Windows uses an owner-only security descriptor and local-only named pipes. Unix execution must be recorded on its own runner; Windows success is not a Unix qualification.

Ruling: CLI broker stdin/stdout is a trusted host control channel and intentionally returns an expiring application capability token. It is not a provider token, must not be logged, and never travels in process arguments. Native Messaging stdout contains only framed protocol messages.
