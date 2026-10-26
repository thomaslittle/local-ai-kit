# Changelog

All notable changes to Local AI Kit are recorded here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions are developer alphas (`0.1.0-alpha.N`) and are not yet semver-stable: the protocol, SDKs, and helper flags can change between alphas. Each [GitHub release](https://github.com/thomaslittle/local-ai-kit/releases) carries its own manifest, SHA-256 checksums, SBOM, and retained notices. Release binaries are unsigned; a checksum detects corruption but is not a publisher signature. Verification details for each build are in the [development log](docs/implementation/DEVELOPMENT-LOG.md).

## [Unreleased]

## [0.1.0-alpha.19] - 2026-10-26

### Changed

- Grok metadata probes repeat a logged-out `grok models` result once. A definite recheck replaces the first result. An unknown recheck stays logged out. Reimplemented from T3 Code [a2157ece](https://github.com/pingdotgg/t3code/commit/a2157eced708fd25fab7b616f5a68cb65312eaed).
- A native Codex binary is launchable only with an adjacent non-empty `codex-code-mode-host` executable. Shebang scripts, version-manager shims, and Windows non-`.exe` launchers stay exempt. Reimplemented from [348888a7](https://github.com/pingdotgg/t3code/commit/348888a767433464dd47e723ec6509dc5a8282db).
- Known ChatGPT plan codes keep the raw `planType` and use the current display labels (Pro 200, Pro Lite 100, Pro Max 500, Business Premium, Business, Enterprise, Edu). Reimplemented from [33f1fe5e](https://github.com/pingdotgg/t3code/commit/33f1fe5ebe53fdd8d18b6a5d63dfd29c336ba5a9).
- Codex `config/read` `service_tier` selects an advertised choice. `fast` aliases `priority`. The probe does not add a tier. Reimplemented from [3b844dab](https://github.com/pingdotgg/t3code/commit/3b844dab5ced7277ce4a2cf0e5e11d0f04e62241).
- Codex `individualLimit.remainingPercent` is reported as a "Monthly credit limit" window. Reimplemented from [4c18faf6](https://github.com/pingdotgg/t3code/commit/4c18faf62cd1685b0243dcacc1a94e333c2b93ff).
- Claude monthly spend prefers an enabled budget when currency and exponent match, then enabled extra usage. Reimplemented from [fd8dd516](https://github.com/pingdotgg/t3code/commit/fd8dd51654f855dbf4e23c457701f5088c59b689).
- The T3 Code pin moved to `f0dd86f16482595c6cfffe0ac64dc12bffb634f9` (`main`), nightly `v0.1.3-nightly.20261026.3717`, stable `v0.1.2`. Ollama, LM Studio, llama.cpp, ChatGPT OAuth protocol changes, credential-file Grok billing, Muse home spawning, and T3 release CI were reviewed and not adopted.

## [0.1.0-alpha.18] - 2026-10-22

### Added

- Independent passive inventory, slow metadata and faster supported-usage refresh scheduling, with active/idle cadence, fair bounded admission, shared probe/backoff state and owned cancellation. Provider reset times request observations; they never reset counters locally.
- Shared per-user runtime with an explicit independent lifetime, owner-only token descriptor, race-free single-instance binding, idle shutdown, and on-demand SDK startup. Consumers can share one discovery/metadata/usage/session runtime without a parent-owned process dying with the first app.
- Broker-owned SQLite persistence (`lai-store`) with an atomically committed ready snapshot, monotonic revisions, epoch-bound bounded journal, sanitized metadata, issues, and notification delivery state. Warm startup hydrates the snapshot as stale and reverifies it.
- Inventory change events (`events.read` with `scope: "inventory"`), journal cursor/gap recovery, TypeScript inventory/live-store helpers, and Python inventory subscription.
- Persistent actionable health (`Issue`, `HealthSummary`), issue acknowledge/snooze, projected issue permissions, and one notification coordinator with quiet hours, privacy, reminders, and durable deduplication.
- Native Rust control center (`local-ai-kit-control`) over the public authenticated API: overview, accounts, models, usage, issues, apps, diagnostics, refresh, probe, revoke, and redacted export.
- `lai-update`: Ed25519 publisher verification over canonical JSON, platform/protocol/version/checksum checks, staged installation, session lease, atomic activation, and rollback. The release build emits an unsigned manifest; signing and publication remain approval gates.
- `apps.list` / `apps.revoke` administrative methods for active-connection revocation, and `STORAGE_UNAVAILABLE` for typed storage failures.
- T3 upstream coverage for shell resolution paths, classification of provider vs orchestration noise, missing-watched-path detection, unwatched provider-shaped candidates, durable reports, report-only CI mode, and explicit PR-permission remediation.

### Changed

- Managed ChatGPT identity validation requires issuer, audience, expiry and subject claims; jsonwebtoken 10.3.0 uses AWS-LC, and the CSS build parsers use patched versions. Signed synthetic tokens cover missing and incorrectly typed standard claims.
- Native verification harnesses accept an explicit helper path for isolated Cargo targets. CI disables dev/test symbols to bound generated output.
- Inventory events now expose only scoped resource IDs or an anonymous resynchronization hint. Late probe failures have the same generation guards as successful reports, and stale quota warnings survive timer expiry until authoritative recovery.
- `serve --ipc` remains parent-owned by default; `--independent` is a separate, explicit lifetime. The SDK does not silently start a full scanner when a configured broker is unreachable.
- Metadata invalidation now publishes a stale transition, advances the revision, and schedules an autonomous debounced re-read. A failed provider does not erase last-known observations.
- Host MCP sessions can opt into the installed Codex or Claude Code native tools with `permissionMode: "native_policy"` plus the separately granted `--allow-native-policy`. `--review-native-requests` remains an independent once-only approval grant; the read-only host-tools-only default is unchanged.

## [0.1.0-alpha.17] - 2026-10-16

### Changed

- Grok metadata probes set `GROK_DISABLE_AUTOUPDATER=1` so a health check does not start Grok's updater. Text sessions leave that variable unset. Reimplemented from T3 Code [2a7bd167](https://github.com/pingdotgg/t3code/commit/2a7bd1673c7bdad4304b7771944e73c29ecd3c01).
- The T3 Code pin moved to `cfafb4bb54d244e4612f5681aad3832ea64cb0fb` (`main`), nightly `v0.1.1-nightly.20261016.3203`, stable `v0.1.0`. Codex's new 20-second status probe already matched the local metadata probe. Registry downloads, Codex and Antigravity installs, and the Cursor SDK were reviewed and not adopted.

## [0.1.0-alpha.16] - 2026-10-15

### Added

- Passive Pi inventory (`pi` on `PATH`, `PI_CODING_AGENT_DIR` or `~/.pi/agent`). Execution stays unsupported. OpenCode's installer binary at `~/.opencode/bin/opencode` is found when it is not on `PATH`.
- ChatGPT managed sign-in follows the pinned T3 Code rules for `earliest_refresh_at`, RS256 and ES256 identity tokens, and `prompt=consent` after a sign-in that omitted plan sharing.
- [UPSTREAM_T3CODE.md](UPSTREAM_T3CODE.md) records the T3 Code commit, nightly tag, and stable tag. The daily upstream watch diffs local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth.

### Changed

- Prepared the repository for public release: rewrote the README, contributing guide, security policy, and agent guide; added a code of conduct, issue and pull-request templates, and dependency update configuration; moved historical planning material under `docs/design/`.
- Package READMEs and docs no longer describe the project as a planning stub.

### Fixed

- macOS: discovery log-tail tests now use a canonical temp directory instead of tripping the symlink guard on `/var` -> `/private/var`.
- CI: the Linux job frees runner disk space before the release build, and `cargo test` runs with `--no-fail-fast` so every failing suite is reported in one run.

## [0.1.0-alpha.15] - 2026-10-13

First release built from `main` after merging all outstanding work.

### Added

- Muse Code provider (`muse`) and the whole alpha.9 to alpha.14 line, including ChatGPT managed sign-in.
- The React showcase (shadcn `base-mira`, TanStack Router, zustand) and the helper changes it needed: SPA client routes and `woff2` served from the asset route, metadata cache changes.
- Discovery honors client home overrides (`GEMINI_CLI_HOME`, `GROK_HOME`, `LM_STUDIO_HOME`, plus the existing ones) and Windows path separators, and finds GitHub Copilot and Antigravity CLI profiles. [Tokscale](docs/upstream/tokscale.md) is a pinned knowledge source, not a runtime dependency.

### Changed

- An incompatible OpenCode protocol is rejected before legacy startup. A dormant OpenCode v2 adapter is staged; automatic OpenCode 2 launch stays disabled.

## [0.1.0-alpha.14] - 2026-10-13

### Added

- Muse Code provider: passive discovery (PATH, `%LOCALAPPDATA%\Programs\muse`, `LOCAL_AI_KIT_MUSE_BINARY`), metadata from a read-only `muse serve` (`account/read`, `model/list`, `usage/read`), and sessions over Muse Session Protocol v1 with host-reviewed approvals, optional host MCP servers, images, cancellation, and persistent resumption. See [Muse Code](docs/adapters/muse.md).

## [0.1.0-alpha.13] - 2026-09-30

### Changed

- Sessions can attach larger bounded host MCP tool catalogs: up to 256 tools across at most two servers. Loopback, name, token, read-only, and execution-grant validation are unchanged.

## [0.1.0-alpha.12] - 2026-09-30

### Fixed

- Managed Codex host MCP tools stay callable when the separate code-mode tool host is disabled. Only approved host MCP namespaces are exposed as direct tools, with prefixed namespaces to avoid built-in collisions; normalized collisions are rejected before a thread opens.

## [0.1.0-alpha.11] - 2026-09-30

### Added

- `--executable-only-provider PROVIDER`: retain a provider's private executable candidate for dependent integrations without inspecting its profile roots or publishing its providers, installations, profiles, connections, or routes.

## [0.1.0-alpha.10] - 2026-09-30

### Changed

- `turns.start` accepts UTF-8 text prompts up to 512 KiB; serialized params have a separate ceiling of 1 MiB minus 8 KiB. Other methods keep their 64 KiB bound.
- Managed ChatGPT connection labels and Codex client titles use generic host-derived wording.

## [0.1.0-alpha.9] - 2026-09-29

### Added

- Opt-in managed ChatGPT sign-in: a `codex_managed` connection with `auth.chatgpt.status`, `start`, `cancel`, and `disconnect` RPCs. The helper owns Sign in with ChatGPT OAuth and protected credential storage, separate from the installed Codex client's login. See [ChatGPT managed authentication](docs/integration/chatgpt-managed-auth.md).

## [0.1.0-alpha.8] - 2026-09-28

### Added

- Image input for native Claude Code and Codex sessions. Claude Code receives base64 `image` blocks; Codex receives app-server `image` input items. Codex image turns are fixture-tested only.

## [0.1.0-alpha.7] - 2026-09-28

### Added

- Host-provided MCP servers on sessions (`hostMcpServers`; loopback-only, bearer token never in argv or logs) for Claude Code, Codex, and ACP peers advertising `mcpCapabilities.http`.
- Persistent, restart-surviving session resume (`persistent: true`, sealed `lai1_` tokens) for Claude Code, Codex, OpenCode, and ACP. An unavailable conversation returns `RESUME_UNAVAILABLE` instead of silently starting a fresh one.

## [0.1.0-alpha.6] - 2026-09-28

### Changed

- A stale sign-in now stands until the client reports it signed out; signed-out, expired, or never-checked connections still block.

## [0.1.0-alpha.5] - 2026-09-28

### Added

- `--billing-declarations ABSOLUTE_JSON`: host-supplied billing for discovered connections whose provider leaves it unknown, recorded as user-supplied evidence. See [native profiles](docs/integration/native-profiles.md).

## [0.1.0-alpha.4] - 2026-09-28

### Changed

- `sessions.create` re-reads a connection's live auth when its metadata is older than 60 seconds; signed-out clients fail with `AUTH_REQUIRED`.
- Grok sessions run with no native tools; any tool call in a text-only native-client turn fails the turn. Gemini CLI and Antigravity text-only sessions are refused because their read tools cannot be disabled.
- OpenCode runs as a kit-managed `opencode serve` on a random loopback port with a password.
- Model revisions are stable (semantic content only) and the Codex account identity check uses type and email only.

## [0.1.0-alpha.3] - 2026-09-28

### Fixed

- Codex 0.157+ app-server contract: thread sandbox modes are sent in kebab case and turns send the bare sandbox type. Read-only Codex threads disable shell, unified exec, image generation, tool suggestion, goals, and sleep tools.

## [0.1.0-alpha.2] - 2026-09-27

### Added

- Bounded image input: `inputs.put` (JPEG/PNG, at most 4 MiB, 2048 px, 4 per turn, 8 per session) and optional `turns.start` `inputs`. Image turns are rejected with `CAPABILITY_UNSUPPORTED` unless the model reports `input.image` and the adapter can carry images; images are never silently dropped.

## [0.1.0-alpha.1] - 2026-09-27

First developer alpha: native discovery and metadata, route-aware selection, granted sessions with streaming and cancellation, stdio / HTTP / broker / Native Messaging transports, TypeScript and Python SDKs, React hooks and selectors, and installable bundles.

### Fixed

- ACP authentication failures no longer forward provider text, which could contain login URLs or device codes.

[Unreleased]: https://github.com/thomaslittle/local-ai-kit/compare/v0.1.0-alpha.17...HEAD
[0.1.0-alpha.17]: https://github.com/thomaslittle/local-ai-kit/compare/v0.1.0-alpha.16...v0.1.0-alpha.17
[0.1.0-alpha.16]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.16
[0.1.0-alpha.15]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.15
[0.1.0-alpha.14]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.14
[0.1.0-alpha.13]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.13
[0.1.0-alpha.12]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.12
[0.1.0-alpha.11]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.11
[0.1.0-alpha.10]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.10
[0.1.0-alpha.9]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.9
[0.1.0-alpha.8]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.8
[0.1.0-alpha.7]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.7
[0.1.0-alpha.6]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.6
[0.1.0-alpha.5]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.5
[0.1.0-alpha.4]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.4
[0.1.0-alpha.3]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.3
[0.1.0-alpha.2]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/thomaslittle/local-ai-kit/releases/tag/v0.1.0-alpha.1
