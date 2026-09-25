# Implementation support matrix

This table reports shipped operations, not a guarantee about every account or native-client version. Unknown account fields are never manufactured. "Fixture-tested" means an actual synthetic child process or local protocol server was exercised; it is not paid live inference qualification.

| Integration | Discovery | Account/plan/usage evidence | Model/options | Text execution | Important limits |
|---|---|---|---|---|---|
| Codex | Passive native inventory | Native account/plan and reported usage | Native catalog and effort/tier descriptors | Implemented | Prior installed-client metadata check; no new live inference; in-process conversations, no persisted resumption. |
| Claude Code | Passive inventory | Native control metadata where reported | Native descriptors | Implemented, text-only | Tools are not silently enabled; plan/usage availability depends on client fields. |
| Grok | Passive inventory + explicit native profile | Authenticated ACP session when accepted; plan/quota unknown unless reported | Native ACP catalog/options | Implemented; fixture-tested | Metadata-only Windows qualification on Grok 1.0.41 returned Grok 4.7 + reasoning effort; no live prompt was sent. |
| Gemini CLI | Passive inventory + explicit native profile | Authenticated ACP evidence when accepted; plan/quota remain unknown unless reported | Native ACP catalog/options | Implemented; fixture-tested | Existing-login reuse only; no compatible client was available for live qualification on the Windows PC. Separate from Gemini API. |
| Antigravity | Passive inventory + explicit ACP runtime profile | Authenticated ACP evidence when accepted; plan/quota remain unknown unless reported | Native ACP catalog/options | Implemented; fixture-tested | Requires `agy_acp_server` + `localharness_external`; the IDE launcher is intentionally rejected. |
| Cursor | Passive inventory | Immutable cached plan/identity, always stale/unverified | Not reported | Unsupported | No token/cookie reuse or private usage endpoint. Generic `agent` binaries are not attributed to Cursor. |
| OpenCode | Passive/configured local endpoint | Connected upstream routes; billing declaration stays user-supplied | Native vendor/model IDs, variants, context | Implemented | Owns its sessions, denies tools, streams/cancels/closes only its work. |
| Ollama | Passive/configured local endpoint | Local-runtime reachability, not a paid plan | Native model/details evidence | Implemented | Local capability fields must actually be reported. |
| LM Studio | Passive/configured local endpoint | Local-runtime reachability | Native model metadata | Implemented | Embedding models are not treated as chat; absent quotas stay unknown. |
| OpenAI API | Explicit endpoint/key reference | API route, no subscription transfer | Catalog plus explicit capability evidence | Implemented | Metered permission is separate from subscription consent. |
| Anthropic API | Explicit endpoint/key reference | API route | Reported catalog/options | Implemented | No Claude subscription token exchange. |
| Gemini API | Explicit endpoint/key reference | API route | Reported generation methods/catalog | Implemented | Not Antigravity/Gemini CLI subscription access. |
| xAI API | Explicit endpoint/key reference | API route | Reported catalog/evidence | Implemented | Not an automatic Grok subscription route. |
| Compatible API | Explicit endpoint | User-configured API/local route | Reported or explicitly supplied metadata | Implemented | No model-name heuristics or silent fallback. |

## Cross-cutting features

Windows native helper, stdio, HTTP/SSE pairing, Windows named-pipe broker, Unix socket broker, Native Messaging framing, TypeScript/Python SDKs, embedded Rust facade, JSON Schema/OpenAPI, eight reusable React selectors, and the demo are implemented. Scoped memory artifacts, revocation, expiry, request limits, process/session cleanup, and native result download are implemented.

No package registry publication, OS signing/notarization, universal credential migration, third-party token replay, hosted account connector, or durable cross-process resumption is claimed. Live generation across all providers is not qualified. The C# HTTP example is compiled and exercised against the Windows native service. Tauri remains a source-level integration guide, not a separately compiled application.

Windows and Ubuntu native runs are recorded in qualification reports. macOS needs an actual working runner; a workflow definition is not test evidence. Read [native Google/Grok status](native-google-grok-status.md), [Cursor cached metadata](cursor.md), and the [qualification directory](../qualification/) before treating a provider as available in your application.
