# Source audit and implementation references

Research date: **2026-09-24**. This is a source/documentation review, not a live qualification of any installed account. Product names, quotas, client flags, endpoint shapes, and integration policies must be rechecked against the actual installed versions during implementation. A public repository's ability to read an endpoint is not proof that the endpoint is a supported integration contract.

## Repository references

### 1. T3 Code — execution and capability-driven selection

Repository: <https://github.com/pingdotgg/t3code>

The inspected code-search revision was `e67abcf798f8c4d8458755e3b4dde02c2c1f628b`. Recheck the current default-branch head before copying or porting anything.

- [CodexProvider.ts](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/server/src/provider/Layers/CodexProvider.ts): account/plan labeling and conversion of reported model reasoning options and service tiers into UI descriptors. Do not inherit its product-specific model/default overrides into a generic toolkit.
- [AntigravityProvider.ts](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/server/src/provider/Layers/AntigravityProvider.ts): separates initialization health from session-authenticated metadata and preserves native model IDs/configuration. Useful example of staged discovery rather than claiming installation proves account access.
- [GrokProvider.ts](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/server/src/provider/Layers/GrokProvider.ts) and [GrokAcpSupport.ts](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/server/src/provider/acp/GrokAcpSupport.ts): located through source search; inspect fully before porting. Search excerpts show model-specific effort handling and ACP selection support.
- [ModelPickerContent.tsx](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/web/src/components/chat/ModelPickerContent.tsx): provider-instance keys, per-instance catalogs, favorites, virtualized selection, and disabled/setup handling. It imports `ModelPickerSidebar.tsx`, `ModelListRow.tsx`, and the search/key helpers; inspect those associated files during implementation.
- [ProviderModelPicker.tsx](https://github.com/pingdotgg/t3code/blob/e67abcf798f8c4d8458755e3b4dde02c2c1f628b/apps/web/src/components/chat/ProviderModelPicker.tsx): located through search as the outer picker integration.

Reuse the boundaries, parsers, test cases, and interaction patterns. Do not copy T3's entire server, Effect application graph, project database, terminal UI, or product settings into the toolkit.

### 2. CodeNotch cross-platform implementation — Rust discovery

Repository: <https://github.com/thomaslittle/codenotch-crossplatform>

- [providers/mod.rs](https://github.com/thomaslittle/codenotch-crossplatform/blob/main/src-tauri/src/providers/mod.rs), inspected blob `c48f68c0574ec25d60b26634c7fd86ea9491aa9d`: five provider modules, last-good snapshots, throttling/backoff, and activity handling.
- [providers directory](https://github.com/thomaslittle/codenotch-crossplatform/tree/main/src-tauri/src/providers): inspect the per-provider source and tests before extracting detection, credential-location knowledge, or response parsers.

Important adaptations: replace provider-only cache keys with account/connection-scoped keys; emit partial results instead of waiting for every provider; use bounded incremental log access rather than repeatedly traversing session directories; keep a fresh authentication failure separate from stale quota data. Existing direct endpoint calls require an integration-policy review, not automatic adoption.

### 3. Upstream CodeNotch — broader provider discovery

Repository: <https://github.com/vinzdg/codenotch>

- [README provider table](https://github.com/vinzdg/codenotch/blob/main/README.md), inspected blob `9d6978acb74cab4abe73e33cbcb2fe9ea1a5836e`: documents a much wider provider inventory, including Grok, Antigravity, coding-plan services, and local runtimes. The README's “official” labels are the project's descriptions, not our certification.
- Inspect `Sources/Providers/` and `windows/codenotch/src/` for relevant implementations. Start with `windows/codenotch/src/grok.rs`; verify its current revision and license before reuse.

Treat browser-login integrations, undocumented quota endpoints, and ordinary local-runtime discovery as different trust/support categories.

### 4. Prmptr — exact requested provider-rail UX

Repository: <https://github.com/thomaslittle/prmptr>

The inspected repository-tree commit was `342a4bf2ba5633735ab72d93ded881b4d4bb66ba`.

- [components/model-picker.tsx](https://github.com/thomaslittle/prmptr/blob/342a4bf2ba5633735ab72d93ded881b4d4bb66ba/components/model-picker.tsx), inspected blob `4935c518e11fa580880ce72694685df50b33d602`: `ProviderRailIcon`, provider grouping, favorites, filtering, and retention of the actual provider/subprovider on model rows.
- [THIRD_PARTY_NOTES.md](https://github.com/thomaslittle/prmptr/blob/342a4bf2ba5633735ab72d93ded881b4d4bb66ba/THIRD_PARTY_NOTES.md): product dependency/service notes. Audit the actual license files of any reused code or assets separately.

Preserve the vertical-rail interaction, not application-specific provider enums, localStorage keys, name/version heuristics, or routing assumptions. A grouped visual row must not erase distinct accounts, billing routes, or subproviders. Pre-index rows instead of repeating full catalog filters per model.

## Primary protocol and service documentation

- [Codex App Server](https://developers.openai.com/codex/app-server/) / [current redirected documentation](https://learn.chatgpt.com/docs/app-server): account metadata, model discovery, usage/rate-limit reporting, streaming sessions, and approvals. Model options vary with account/client; inspect returned values. Methods may be version-dependent.
- [ACP initialization](https://agentclientprotocol.com/protocol/v1/initialization): protocol negotiation and advertised capabilities. Missing optional ACP capabilities have protocol-defined unsupported defaults; do not reinterpret them as enabled.
- [ACP session configuration](https://agentclientprotocol.com/protocol/v1/session-config-options): dynamic option descriptors and updates. Prefer `configOptions` where supported; preserve native IDs, ordering, categories, and dependency updates.
- [OpenCode server](https://opencode.ai/docs/server/): an HTTP/OpenAPI server with provider, model, session, and event surfaces. Use this integration rather than scrape the TUI.
- [Claude CLI reference](https://code.claude.com/docs/en/cli-reference): supported programmatic invocation/options; verify installed-version support.
- [Claude legal/compliance](https://code.claude.com/docs/en/legal-and-compliance): distinguishes an end user signing into the unmodified client from taking subscription tokens into a third-party client. Integration mode must be reviewed independently of technical token readability.
- [Antigravity getting started](https://antigravity.google/docs/getting-started?tab=cli): entry point for CLI, authentication, headless, SDK, usage, and model documentation. T3's ACP adapter is a concrete implementation reference, not proof that every installed Antigravity version implements every feature.
- [Gemini CLI authentication](https://geminicli.com/docs/get-started/authentication/): distinct Google, API-key, and cloud authentication routes. Verify current service eligibility and transition notices before claiming a consumer plan works through a specific route.
- [Grok Build](https://docs.x.ai/build/overview): documents headless and ACP integration and custom model routing. A Grok client can therefore be a route to something other than an xAI model.
- [Ollama API](https://docs.ollama.com/api/introduction): local HTTP integration.
- [LM Studio REST API](https://lmstudio.ai/docs/developer/rest): local runtime/model integration.
- [Chrome Native Messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging): installed native host and browser-extension bridge. Pure web code does not acquire OS credential access by importing an SDK.

## Audit requirements for implementation

Record the exact source commit, license, reused symbols, integration mechanism, endpoint classification, supported client versions, platforms actually exercised, field evidence, and live-test date for every adapter. Preserve copyright/notices where code is copied. A source-access problem must be reported honestly; do not recreate unread code and call it a port.

This handoff deliberately does not assert provider plan prices, exact quotas, a permanent list of models, or universal permission to reuse subscription credentials. Those are runtime/account-specific facts, not stable package constants.
