# Local AI Kit

**One integration to discover, inspect, select, and use locally configured AI access.**

A standalone, local-first developer toolkit planned around a modular Rust core, a language-neutral local protocol, TypeScript/Python/Rust interfaces, native-client adapters, and optional reusable provider/model selectors.

> **Current status: planning and implementation handoff only.** No product code, installable SDK, working demo, or live-qualified adapters have been implemented in this repository yet. The 18 implementation tasks remain open. This repository is private; public package names and publication are not authorized or availability-checked.

## Start with Codex

Connect Codex to [this repository](https://github.com/thomaslittle/local-ai-kit) and send it the [repo-linked kickoff prompt](CODEX-KICKOFF.md). No archive upload or previous conversation is required.

Codex should read [AGENTS.md](AGENTS.md), [RESUME.md](docs/handoffs/RESUME.md), and the [full build instructions](CODEX-BUILD-PROMPT.md), then follow the specification and implementation plan. Begin on `feature/initial-implementation` from the current `main` unless that branch already contains work to resume. Never reset an advanced branch.

## Project documents

| Document | Purpose |
| --- | --- |
| [Codex kickoff](CODEX-KICKOFF.md) | Short prompt with direct repository links. |
| [Full Codex build prompt](CODEX-BUILD-PROMPT.md) | Scope, architecture, source reuse, execution rules, and acceptance criteria. |
| [Architecture specification](docs/superpowers/specs/2026-09-24-local-ai-kit-design.md) | Core/runtime design, identities, access routes, evidence, quotas, SDKs, security, performance targets, demo, and selectors. |
| [18-task implementation plan](docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md) | Dependency-ordered work, specific modules/tests, verification, milestones, and release gates. |
| [Source audit](SOURCE-AUDIT.md) | Existing research and exact reference paths; not a live qualification of provider accounts. |
| [Resume checkpoint](docs/handoffs/RESUME.md) | Current status and the next implementation action. |
| [Third-party notices](THIRD_PARTY_NOTICES.md) | Attribution ledger and code-reuse requirements. |

## Planned capabilities

- Discover supported local clients, profiles, configured accounts, and execution environments.
- Inspect evidence-backed authentication, reported plans, route-scoped models, native options/effort controls, capabilities, quota pools, resets, and freshness.
- Communicate through supported installed clients or explicitly configured API routes, including streaming, approvals, cancellation, and native resumption where available.
- Offer embedded Rust, a managed native helper, an optional shared per-user runtime, stdio JSON-RPC, authenticated local HTTP/SSE, and thin SDKs.
- Provide framework-neutral selection logic, optional React components, and a public-SDK-only demo with eight selector designs. The default is the T3/Prmptr-style vertical provider rail.

The package is independent of Maps and image enhancement. Discovery does not prove entitlement or authorize inference. The product must not invent quotas, combine unrelated allowances, export subscription tokens, or silently switch billing routes.

## Implementation sequence

1. **Tasks 1–6:** contracts, platform primitives, consent/transports, discovery, execution, and a complete Codex reference adapter.
2. **Tasks 7–11:** TypeScript SDK/demo baseline, shared ACP, Google/Grok/OpenCode, local runtimes, and additional native-client adapters.
3. **Tasks 12–15:** explicit API routes, shared runtime, Python/other-language integration, reusable selectors, and the full demo showroom.
4. **Tasks 16–18:** generic host examples, adapter contribution kit, platform/security/performance qualification, and package artifacts.

All capabilities above describe the planned product, not already working features. Follow the detailed plan rather than creating placeholder packages or fake success checks.

## Source references

[T3 Code](https://github.com/pingdotgg/t3code) · [CodeNotch cross-platform](https://github.com/thomaslittle/codenotch-crossplatform) · [upstream CodeNotch](https://github.com/vinzdg/codenotch) · [Prmptr](https://github.com/thomaslittle/prmptr)

Recheck upstream versions and licenses when porting a specific adapter. Keep those repositories read-only. See [SOURCE-AUDIT.md](SOURCE-AUDIT.md) for the existing research and the exact selector source.

## Licensing and security

Original repository material uses the [MIT license](LICENSE). Reused third-party code retains its original notices and obligations. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [SECURITY.md](SECURITY.md), and [CONTRIBUTING.md](CONTRIBUTING.md).

The repository bootstrap used the pre-existing planning files; no AI-provider credentials were inspected and no inference was run for this commit. `HANDOFF-MANIFEST.json` records the bootstrap files and SHA-256 checksums; it is a historical handoff manifest, not a build-success report.
