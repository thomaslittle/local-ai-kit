# Resume — Local AI Kit

## October 13 Muse Code adapter

Branch `feature/muse-provider`, based on alpha.13 (`470dee4`), adds provider `muse` ("Muse Code"): passive discovery (PATH, `%LOCALAPPDATA%\Programs\muse`, `LOCAL_AI_KIT_MUSE_BINARY`), metadata through a read-only `muse serve` (experimental `account/read` for auth/billing, `model/list`, `usage/read`; no prompt), and sessions over MSP v1 with host-reviewed approvals, host MCP servers (optional), images, cancellation, background workflow/subagent reporting and persistent resumption. It ports T3 Code's Muse provider behaviour; see [Muse Code](../adapters/muse.md) for the permission-mode mapping, event and extension mapping, gaps and evidence. Tests replay T3's recorded transcripts and Muse 1.4.7 echo recordings; live checks used only metadata and the free echo provider. No Meta model turn was run. Released as `v0.1.0-alpha.14`; `verify.py --profile all` passed on Windows (526 Rust tests). The Claude process test's 3 s completion wait flaked under the parallel run and is now 10 s.

## September 29 ChatGPT managed-auth follow-up

Alpha.11 source `5612a13efc7ddcce17f5e7ff3eb80d718b4293cf` adds generic executable-only provider discovery, preserving a private runtime candidate while skipping that provider's profile roots and public routes. Alpha.12 adds direct-only Codex host MCP namespaces for code-mode models. It forces MCP prefixes, preserves already-prefixed names and refuses normalized collisions; shell/exec, code-mode host and unrelated integrations remain disabled. Windows Codex 0.159.2 managed ChatGPT and Claude Code each completed a real, read-only host-tool round trip in a separately owned native test app. Exact published source and asset hashes belong to each immutable release manifest; do not overwrite earlier release files.

The owner-authorized auth work is on `codex/chatgpt-managed-auth`, based on alpha.8. Alpha.9 source is `bdefd56107096298cca0ceec799a68f58fb82f34`; its published assets are immutable. The alpha.10 follow-up fixes the generic turn contract: prompts accept up to 512 KiB of UTF-8 text, with serialized turn params limited separately to 1 MiB minus 8 KiB. Other RPC methods keep their 64 KiB bound, and provider frame guards remain enforced. Tests prove a 130,000-byte prompt reaches a fixture agent unchanged, exercise both bounds and escaping, and cover a 118-tool host allowlist. Two host-specific branding strings now use generic profile wording and the caller's application name.

A separately owned headless alpha.9 managed session completed a real-account read-only Sol turn with exactly `AUTH_OK`; its helper was terminated. A production-built synthetic Current Scene prompt exceeded alpha.9's 64 KiB cap and was rejected before execution. Alpha.10 publication and qualification are recorded by the final release manifest and notes; do not infer them from earlier fixtures. No other agent's dev app was used or stopped for these checks.

The earlier ledger below covers `feature/initial-implementation`. `main` remains unchanged by this auth follow-up. Inspect local and remote HEAD before writing; do not force-push or recreate completed work.

The live playground grants text execution for every provider with a registered execution adapter, including explicit API routes, in an isolated temporary workspace. Native ACP profile adapters join when `LAI_NATIVE_PROFILES` is set. Each turn requires the browser consent checkbox and Run click; the helper leaves paid billing options and file writes disabled. The Playground selects provider and route/model directly from the live catalog. No inference was sent during this follow-up.

The local HTTP demo retrieves its active one-use pairing code from `GET /runtime/pairing-code`; after expiry/redemption, its **Show current pairing code** action explicitly falls back to `POST /runtime/pairing-code` to issue a replacement. TypeScript hosts can use `getPairingCode({ baseUrl })` and `issuePairingCode({ baseUrl })`. Both endpoints are loopback/origin guarded and `no-store`; issuing a replacement leaves already paired clients connected. The paired Codex browser demo was manually verified on September 27, including refresh/reissue/re-pair; metadata checks completed without inference. Focused verification: HTTP transport suite 8/8, TypeScript HTTP SDK 22/22, live demo browser E2E 7/7, generated contract check, SDK/demo build, format check, and transport/CLI Clippy.

**Image input (unreleased, after `v0.1.0-alpha.1`):** `inputs.put` stages bounded JPEG/PNG images (4 MiB, 2048 px, 4 per turn, 8 per session) per owner and session, and `turns.start` accepts optional `inputs`. The handshake advertises `turn.input.image`; protocol stays `1.0` because the change is additive and methods are negotiated. Model `input.image` support and adapter carriage are both required, otherwise `CAPABILITY_UNSUPPORTED`. HTTP providers and ACP carry images with fixture-tested request shapes. Codex, Claude Code, and OpenCode report unsupported. Only Ollama/LM Studio catalogs currently report `input.image`, so image turns on direct API routes are rejected. Native stdio and HTTP `/v1/rpc` request frames rose to 8 MiB; Native Messaging stays 1 MiB. No live provider received an image. See [prompt-app image input](../integration/prompt-app.md#image-input).

**Host MCP servers (unreleased, after `v0.1.0-alpha.6`):** `sessions.create` takes optional `hostMcpServers` (at most 2 loopback `http://127.0.0.1|[::1]:<port>/...` servers, bearer token, allowed tool names). The handshake advertises `session.host_mcp`. Claude Code attaches them through a private per-session `--mcp-config` file and drops `--safe-mode` (which ignores such servers); Codex through per-thread `mcp_servers.<name>` config with every other configured server disabled; ACP through `session/new` when the peer advertises `mcpCapabilities.http`. Other routes return `CAPABILITY_UNSUPPORTED`. Host tool calls are allowed and reported as neutral `tool` events; any other tool still fails the turn. Fixture-tested for all three; Claude Code and Codex wiring verified live without inference. One live Claude Haiku turn ran before the safe-mode fix and correctly called no tool; a live tool-calling turn is still outstanding. See [host MCP servers](../integration/host-mcp.md).

**Persistent resumption (unreleased, after `v0.1.0-alpha.6`):** `sessions.create` takes `persistent: true`; with a kit state directory, `sessions.suspend` returns a sealed `lai1_` token (ChaCha20-Poly1305, per-install `resume.key`) that survives helper restarts, and `sessions.resume` revalidates account, route, model id, workspace and application identity, returning `RESUME_UNAVAILABLE` instead of starting a fresh conversation. The handshake and each execution route advertise `session.resume.persistent`. Codex uses non-ephemeral threads and `thread/resume`; Claude Code `--session-id`/`--resume`; OpenCode keeps and reloads its session; ACP keeps `loadSession`. Fixture-tested for all; Claude Code Haiku live-verified across a helper restart. See [native session resumption](../integration/native-session-resumption.md).

## Current implementation

The previously qualified clean implementation source is `0c8213842be5b72ea7614b51028a24912b1bdabd`. Windows and Ubuntu/WSL2 passed both `verify.py --profile all` and `--profile release`, including clean package consumers, C# native HTTP, strict lint, adapter conformance, and new distribution benchmarks. Rust totals: 349 Windows / 351 Linux, zero failed or ignored. Each platform passed TypeScript SDK 63, Python SDK 26, selectors 28, React 13, browser 14, and accessibility 32. The pairing follow-up source is `2abedb94e883bfacaed809ba3883e6c0b966af8a`; it passed Windows targeted transport, SDK, and browser checks documented above. Linux/macOS were not rerun specifically for this additive pairing change. See the [current readiness report](../qualification/2026-09-27-readiness.md) and its machine-readable evidence; its platform results remain tied to their recorded source SHA.

The review preserved the existing worktree and fixed OpenCode loopback authentication, accountless local-runtime execution, completed-turn suspension, revoke/regrant publication races, resumed demo continuation, concurrent SDK probes, and distinct-installation discovery. Linux qualification then exposed reused file IDs in log rotation; birth-time evidence and a bounded leading fingerprint fixed it. Full/release verification above includes that correction. Earlier `43b7737` artifacts are superseded.

The final paired browser returned 137 route-specific model entries: two Codex installations with 7 each, Claude Code 11, Grok 1, and OpenCode 111. Checks are metadata-only; no inference was sent. Ollama/LM Studio were detected but offline. The [application integration guide](../integration/prompt-app.md) covers a crntly-style host, provider/model selection, grants, streaming, and scene-output validation.

The live launcher runs a temporary helper copy so Windows builds can proceed while the demo stays open. `npm run dev` (which starts the same launcher) grants metadata only. Its owned OpenCode server requires temporary Basic Auth; startup verifies anonymous HTTP 401 and an authenticated response.

External macOS blocker: GitHub run `36302338910` created jobs with no executed steps. Check annotations explicitly report failed recent account payments or an insufficient spending limit. No billing settings were changed. macOS remains unqualified; current Windows/Linux results are independent local platform execution. The September 25 WSL EIO interruption is historical and does not describe the successful September 27 run.

The native service is connected end to end to the TypeScript/Python SDKs, embedded Rust facade, native-served paired demo, shared broker, and Native Messaging bridge. Supported Codex/Claude/OpenCode/local/API text routes use the real session engine. Generated domain types/OpenAPI, scoped artifacts, owner cleanup, bundle resolution, four npm packages, Python wheel, and the eight reusable React selectors are implemented.

The former palette failure is fixed in `af96d05`: exact account phrases rank before incidental substrings. The current suites include 14 showroom browser tests and 32 automated accessibility checks across the showroom and reusable React components. The C# HTTP example has now been compiled and exercised against an actual isolated native host, not just inspected as source.

See `docs/adapters/SUPPORT-MATRIX.md`, `docs/implementation/LEDGER.md`, `docs/qualification/`, and `README.md` for operation-level support and runnable commands.

## Verification and packaging

- `python scripts/verify.py --profile all` runs the native, SDK, generator, component, protocol-bridge, package-tool, browser, and accessibility checks.
- `python scripts/verify.py --profile release` additionally requires a .NET 8 SDK, runs C# integration and strict lint/conformance, builds a checksummed bundle, tests clean tarball/wheel consumers, and measures a 60-second idle baseline.
- Build-only Python dependencies are in `scripts/requirements-build.txt`; use a project virtual environment. Browser tests use Playwright Chromium. No normal test requires a paid account.
- On the development PC, the isolated Python environment is `.verification/build-tools/Scripts/python.exe`; the optional .NET SDK is `.verification/dotnet/dotnet.exe`.
- The current bundle pointer is `dist/release/latest.json`. Its manifest records source SHA, platform, profile, dirty state, and file hashes. `.verification/verify-last.json` and the per-check logs record actual results.
- Do not count an older benchmark or a dirty development bundle as qualification of a later clean commit.

## Unresolved original scope — do not relabel it complete

Native Gemini CLI, Antigravity ACP, and Grok execution are now available only through explicitly registered native profiles and connection-scoped grants. Passive detection alone never enables execution. The shared ACP driver is fixture-qualified; Grok metadata was additionally checked against the installed Windows client without inference. Gemini CLI/Antigravity live qualification remains untested on this PC because compatible runtimes are not installed. See `docs/adapters/native-google-grok-status.md`.

The opt-in local metadata watcher is wired through `discovery.watch`, the public paged-catalog helper is wired into the SDK/store/showroom, explicit Gemini/Antigravity/Grok native profiles are connected to shared ACP metadata/execution, and capability-gated ACP native suspend/resume is implemented with one-time owner-scoped tokens. The provider-agnostic incremental log tailer handles append, partial records, truncation, rotation, line windows, cancellation, and huge sparse files with bounded reads; no adapter currently interprets usage from logs. Metadata probes use Retry-After-aware exponential/jitter backoff. Evidence gaps for these implemented slices include provider-specific credit/usage fields without a verified source and macOS execution. The broader 18-task plan still has other explicitly incomplete scope in the implementation ledger. Unknown account fields are intentional, not fabricated successful discovery.

Windows qualification and native Ubuntu execution are separate records. GitHub Actions has been failing before any job steps run; a workflow matrix is not macOS evidence. Tauri currently has an integration guide and compiled embedded-Rust pattern, not a separately qualified Tauri application.

**2026-09-27 — owner authorization:** the repository owner authorized merging this branch into `main` and publishing a GitHub prerelease (`v0.1.0-alpha.1`) with SHA-256-checksummed, unsigned bundle assets so Crntly can install a pinned helper. That release was built from a clean commit on this branch; see its release notes for exactly what was re-verified (no .NET SDK was available, so the C# integration step of `--profile release` did not run). Registry publication and signing remain unperformed.

No registry publication, OS signing, paid inference, additional coding agents, token export, account cycling, or unrelated client configuration changes are authorized by these implementation tests. Preserve those boundaries while completing any remaining work.

## Historical clean-build qualification — source 991ec2b

Executable/source checkpoint: `991ec2bd827cf6e3b6bc884c453ba1ca44bb2b91`. Both Windows and Linux distributions were built from this clean source and passed isolated npm-tarball/Python-wheel consumers against their native executables. Windows full release profile exited zero; Linux release-profile native tests and SDK/package/benchmark checks exited zero. Each platform ran 301 native tests with two explicitly ignored watcher-integration tests. A later documentation-only commit records this evidence without changing the binary source.

Read `docs/qualification/RELEASE-CANDIDATE.md` and `docs/qualification/evidence/verification-summary.json` for exact filenames, hashes, tests, toolchains, and unresolved scope. Windows bundle pointer: `dist/release/latest.json`; Linux archives copied to `dist/linux-release/`. The local artifacts are not GitHub Release assets and are not published registry packages.

Final review fixed the default event-stream cursor for empty histories and removed offline package-smoke dependence on warm registry metadata. Do not recreate these fixes. The full original plan remains incomplete at the explicit boundaries above; verify future changes before changing any support claim.

## Historical native subscription checkpoint — September 25

Explicit native profiles now wire Gemini CLI, Antigravity ACP, and Grok into the shared metadata/execution service. Registration is connection-scoped and contains no provider token/API key. Shared ACP fixtures cover negotiation, model/options, permissions, streaming, cancellation, and cleanup. A metadata-only Windows check against Grok CLI 1.0.41 reused the existing native login and returned Grok 4.7 + reasoning_effort; no model turn was sent. Gemini CLI and the dedicated Antigravity ACP runtime were not available for live qualification on this PC.

Capability-gated ACP native resumption is implemented: `sessions.suspend` returns an opaque runtime-scoped token and `sessions.resume` revalidates the same account/route/workspace before using provider `loadSession`. Incremental metadata tailing and refresh backoff are also implemented and tested. Final remaining high-value scope is release/macOS qualification and any provider-specific usage fields that can be obtained from documented/verified interfaces without inventing data. No registry publication/main merge has been performed.

## Historical continuation status — September 25, 2026

The worktree resumed from local/remote `d1705fd92a2d19b38bf8bf1134ee2c543c7ce82b` on `feature/initial-implementation`; the earlier metadata qualification source was `471c072a8ffde52436427d5a33b44e55ed31e3c1`. Session resumption is capability-gated and tested for account, route, billing, model revision, workspace path, owner, one-time consumption, revocation, runtime restart, active/closed sessions, ACP capability/native-ID mismatch, and no fallback after failed load. The provider-agnostic log tail has bounded identity-aware rotation, truncation, partial records, and cancellation tests; no adapter currently consumes it. See the release-candidate report for current platform evidence.


## Historical checkpoint — September 25, 2026

Implementation source `702d98d7cb77ba788f09ce5e890149fef8908ac2` passed Windows `verify.py --profile all`, strict format/Clippy, and `verify.py --profile release`. The clean Windows distribution, package smoke, and performance evidence are recorded in `docs/qualification/RELEASE-CANDIDATE.md`. It was pushed to `origin/feature/initial-implementation`; `main` was not changed.

Linux WSL2 executed the full Rust test workspace and the deterministic SDK/integration suites up to Playwright demo tests. Chromium cache cleanup then failed with EIO and the WSL distro stopped launching; no current-source Linux package was built. GitHub Actions run `36140653176` failed before any steps appeared on Windows, Ubuntu, or macOS, and logs are unavailable. macOS remains unexecuted/unqualified. The exact current evidence and limits are in the release-candidate report.

## October 5 OpenCode protocol compatibility checkpoint

Audit against T3 Code `ca952677321abf9f3762ae4bb0129454e5a63955`
(`opencodeVersionProbe.ts`, `Drivers/OpenCodeDriver.ts`, MIT) found that
OpenCode 2 uses a separate launch/auth/HTTP/event contract. This kit checkpoint
still implements the OpenCode 1.x protocol. Before starting an owned server,
`opencode --version` is read with a four-second bounded metadata capture.
Unknown or unsupported generations fail with `PROTOCOL_INCOMPATIBLE` before
legacy `serve --pure` flags or HTTP routes are used. No installed client is
upgraded, downgraded, reconfigured, or used for inference by this check.

A synthetic executable reproduced the old incorrect legacy launch and now
verifies that a reported v2 executable never starts that server. Existing
fake gateway catalog and loopback reply tests remain green. Full OpenCode 2
communication is a separate adapter follow-up; this guard does not claim
that support. No upstream implementation was copied for this checkpoint.
