# Documentation

New here? Start with the [root README](../README.md), then the [prompt-app guide](integration/prompt-app.md) if you are embedding the toolkit in an application.

## Integrating

| Topic | Document |
|---|---|
| API Reference: safe quickstart, lifecycle, permissions, recovery | [api-reference.md](integration/api-reference.md) |
| Method-by-method params/results/errors/examples; wire schemas; HTTP | [methods](reference/methods.md), [schemas](reference/schemas.md), [HTTP routes](reference/http.md) |
| Desktop/prompt apps: select a route and model, grant access, stream, follow up | [prompt-app.md](integration/prompt-app.md) |
| Native helper CLI, SDK bridge, transports | [native-helper.md](integration/native-helper.md), [pairing.md](integration/pairing.md), [shared-broker.md](integration/shared-broker.md), [browser-extension.md](integration/browser-extension.md) |
| Shared runtime lifetime, snapshots, health, notifications | [shared-broker.md](integration/shared-broker.md), [ADR-001](design/ADR-001-shared-runtime.md), [shared-runtime plan](implementation/SHARED-RUNTIME-PLAN.md) |
| Signed/staged runtime updates | [updates.md](integration/updates.md) |
| Installing a built bundle (helper, npm tarballs, wheel) | [distribution.md](integration/distribution.md) |
| Registering Gemini CLI / Antigravity / Grok; billing declarations | [native-profiles.md](integration/native-profiles.md) |
| ChatGPT managed sign-in | [chatgpt-managed-auth.md](integration/chatgpt-managed-auth.md) |
| Generated protocol, OpenAPI, JSON Schema | [generated-api.md](integration/generated-api.md), [protocol-preview.md](integration/protocol-preview.md) |
| Sessions: artifacts, host MCP tools, native resumption | [artifacts.md](integration/artifacts.md), [host-mcp.md](integration/host-mcp.md), [native-session-resumption.md](integration/native-session-resumption.md) |
| Incremental metadata, refresh, local change watching | [incremental-metadata.md](integration/incremental-metadata.md), [local-change-watching.md](integration/local-change-watching.md), [catalog-pages.md](integration/catalog-pages.md) |
| Running the showcase against your own machine | [live-demo.md](integration/live-demo.md) |
| Something is wrong | [troubleshooting.md](troubleshooting.md) |

## Providers and adapters

- [Support matrix](adapters/SUPPORT-MATRIX.md): what each provider supports, with its evidence level.
- Per-provider notes: [Codex](adapters/codex.md), [Claude Code](adapters/claude-code.md), [ACP (Gemini CLI, Antigravity, Grok)](adapters/acp.md), [Grok/Google status](adapters/native-google-grok-status.md), [OpenCode](adapters/opencode.md), [Muse Code](adapters/muse.md), [Cursor](adapters/cursor.md), [HTTP providers](adapters/http-providers.md), [passive inventory](adapters/passive-inventory.md).
- Writing an adapter: [AUTHORING.md](adapters/AUTHORING.md) and [VERSIONING.md](adapters/VERSIONING.md).
- Upstream discovery knowledge: [Tokscale](upstream/tokscale.md). Provider and ChatGPT sign-in knowledge: [T3 Code](upstream/t3code.md).

## Design and security

- [Architecture specification](design/architecture.md) (original design; see the support matrix for current status).
- [ADR-001: one durable, permissioned local AI runtime](design/ADR-001-shared-runtime.md).
- [Threat model](security/threat-model.md), [shared-runtime security review](security/SHARED-RUNTIME-REVIEW.md), and the [security policy](../SECURITY.md).
- [Source audit](design/source-audit.md) and [third-party notices](../THIRD_PARTY_NOTICES.md).
- [Historical implementation plan](design/implementation-plan.md).

## Status, evidence, and history

- [Changelog](../CHANGELOG.md): release-by-release summary.
- [Development log](implementation/DEVELOPMENT-LOG.md) and [implementation ledger](implementation/LEDGER.md): what was built and how it was verified.
- [Qualification records](qualification/) and [performance baseline](performance/BASELINE.md): dated platform and benchmark evidence. These describe specific commits and machines, not current guarantees.

## Contributing

[CONTRIBUTING.md](../CONTRIBUTING.md), [AGENTS.md](../AGENTS.md) (for AI coding agents), and the [Code of Conduct](../CODE_OF_CONDUCT.md).
