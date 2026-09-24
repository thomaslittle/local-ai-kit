# Local AI Kit Implementation Plan

> **For agentic workers:** Use `superpowers:executing-plans` for inline execution or `superpowers:subagent-driven-development` when independent workers are actually available. Implement task-by-task, with bounded commits and evidence-backed review. All boxes below are initially unchecked; this handoff is a plan, not completed software.

**Goal:** Build a standalone, cross-platform toolkit for local AI discovery, evidence-backed account/model/usage inspection, supported agent communication, language-neutral APIs/SDKs, and reusable provider/model selectors.

**Architecture:** A modular Rust core and service layer power embedded, sidecar, shared-broker, and local HTTP modes. Adapters translate official native protocols or explicitly enabled metadata sources into common contracts; thin SDKs and a public-API-only demo consume those contracts.

**Tech Stack:** Rust/Tokio/Serde, typed schema/OpenAPI generation, TypeScript, React/Vite, optional shadcn-style UI, Python, Vitest/Playwright/pytest, native platform CI. Pin actual compatible dependency/toolchain versions during Task 1.

**Spec:** `docs/superpowers/specs/2026-09-24-local-ai-kit-design.md`.

## Global Constraints

- New standalone project; no Maps, image-enhancement, full IDE, or mandatory hosted service.
- No changes to reference repositories. Preserve licenses/notices for reused code.
- One Rust implementation of provider logic, language-neutral public contracts, optional UI/execution dependencies.
- Importing a package has no filesystem, credential, process, network, download, or inference side effect.
- Passive discovery spawns no agents, contacts no providers, and never infers authentication from file presence.
- Every account/route/model/option/usage claim has a source, scope, and truthful freshness/unknown state.
- No raw credential exports, silent paid fallback, automatic account rotation, or invented provider capabilities.
- Provider-native options and IDs remain authoritative; no permanent model-name or effort-level tables as access truth.
- Public demo uses only public SDK/API surfaces. Synthetic data is isolated and visibly labeled.
- Credentials remain with the native client where practical; undocumented endpoint support is reviewed and opt-in.
- Windows, macOS, and Linux coverage is reported per operation, architecture, and actual test evidence.
- Commit bounded work; do not force-push, publish packages, or sign releases without the required authorization.

## Review Focus

1. Same account discovered through several clients: preserve routes but do not duplicate a verified quota pool. Pin in Tasks 2, 4, and 10.
2. Account switch while stale data and a model selection are cached: invalidate the old identity and reject stale execution. Pin in Tasks 4, 5, and 14.
3. Browser-origin or second-app access to a shared broker: enforce authorization at the service boundary, including event streams and artifacts. Pin in Tasks 3 and 13.
4. A client update changes options, streaming frames, or permissions: fail closed, preserve unknown data safely, and do not execute with guessed flags. Pin in Tasks 6, 8, and 9.
5. Slow/disconnected consumers or user-owned agent processes: do not lose output silently, duplicate a turn, or terminate another app's process. Pin in Tasks 5, 13, and 18.

## Execution and checkpoint rules

Read the spec and source audit once, then maintain `docs/handoffs/RESUME.md` with current commit, completed task IDs, next exact test/command, changed contract versions, and blocked live checks. Revisit only the relevant source when implementing an adapter. Do not repeatedly restart a whole-repository audit.

Each task has a red/green test cycle, a targeted verification command, a bounded commit, and a short evidence entry in `docs/qualification/`. Tests use synthetic homes/accounts by default. Run real account operations only with explicit authorization; otherwise report them as not live tested while continuing deterministic implementation.

Where no implementation exists yet, introduce only enough workspace/build plumbing to execute the new failing test. A setup error unrelated to the intended assertion is not useful red-test evidence. Keep changes for each adapter separate so a reviewer can accept one and reject another.

## Task 1 — Establish source provenance, workspace, and canonical contracts

**Create:** `Cargo.toml`, `rust-toolchain.toml`, `pnpm-workspace.yaml`, root `package.json`, `pyproject.toml`, `crates/lai-contracts/{Cargo.toml,src/lib.rs,src/evidence.rs,src/identity.rs,src/options.rs,src/usage.rs,src/events.rs,src/errors.rs}`, `protocol/`, `scripts/verify.py`, `docs/architecture/decisions.md`, `docs/adapters/source-provenance.md`, `THIRD_PARTY_NOTICES.md`, `docs/handoffs/RESUME.md`.

**Tests:** `crates/lai-contracts/tests/{observations,identity,option_descriptors,usage_units,wire_compatibility}.rs`.

**Consumes:** supplied design/source audit and read-only upstream code.
**Produces:** versioned `Snapshot`, `Connection`, `AccessRoute`, `Model`, `Observation<T>`, `Capability`, `OptionDescriptor`, `QuotaPool`, `UsageWindow`, `Session`, `Event`, and `DomainError` contracts.

- [ ] Inspect the assigned workspace and git state. Create the independent project there without overwriting existing work. Record exact reference commits, file licenses, and copied/ported symbols. Revalidate current native client protocols, not old model names from this conversation.
- [ ] Write round-trip/schema tests: known stale usage versus unknown quota; large counts and decimal money retain precision; a future option ID survives round-trip; native model IDs do not collapse across routes; malformed records are rejected.
- [ ] Implement canonical types and generators. `Observation` is a tagged union: known values require provenance/time; unavailable values require a reason; absent quota denominator remains absent.
- [ ] Add verification profiles (`core`, `adapters`, `sdk`, `ui`, `all`, `release`) that fail if required checks are missing. Do not implement the runner as a success-printing stub.
- [ ] Run `cargo test -p lai-contracts` and `python scripts/verify.py --profile core`. Commit `feat(contracts): define evidence-backed access and protocol types`.

**Required contract fixture:**

```json
{
  "case": "unknown-denominator",
  "input": {"unit": "requests", "used": "17", "limit": null},
  "expected": {"used": "17", "remaining": null, "usedFraction": null}
}
```

## Task 2 — Platform primitives and isolated test environment

**Create:** `crates/lai-platform/src/{lib,paths,readers,credential_store,process,environment}.rs`, `crates/lai-core/src/{lib,identity,registry}.rs`, `fixtures/virtual-homes/`, `fixtures/fake-clients/`.

**Tests:** `crates/lai-platform/tests/{bounded_reads,profile_paths,credential_failures,process_ownership}.rs`, `crates/lai-core/tests/identity_linking.rs`.

**Consumes:** Task 1 types.
**Produces:** injected `PlatformPaths`, `CredentialStore`, `BoundedReader`, `Clock`, `ProcessLauncher`, and `HttpClient` interfaces; stable opaque connection IDs; a fixture adapter implementing the same adapter contract as production adapters.

- [ ] Write tests for Windows paths with spaces/Unicode, executable shims, XDG homes, overridden profile roots, locked keyrings, absent Linux Secret Service, symlinked roots, and a 1 GiB synthetic log with byte-read counters.
- [ ] Implement reads with per-operation byte limits and explicit permission failures. The passive path must not call the credential-store secret-read method. Open supported state databases read-only; do not race or rewrite native credentials.
- [ ] Implement account linking using authoritative provider/account/workspace identifiers when present. Same email is insufficient. Two profiles retain two connections even when they link to one verified account.
- [ ] Implement process ownership tracking and safe argument passing. Never put prompts, bearer tokens, or arbitrary strings into a shell command line. Handle platform shims through tested adapters rather than blanket `shell=true`.
- [ ] Run `cargo test -p lai-platform -p lai-core`. Commit `feat(platform): add bounded discovery primitives and identity isolation`.

## Task 3 — Service boundary, consent, and initial stdio/HTTP transports

**Create:** `crates/lai-service/src/{lib,methods,authorization,projections}.rs`, `crates/lai-transports/src/{lib,stdio,http,sse,pairing}.rs`, `crates/lai-cli/src/{main,scan,serve,doctor}.rs`.

**Tests:** `crates/lai-service/tests/{grants,data_projection}.rs`, `crates/lai-transports/tests/{stdio_frames,http_auth,origins,event_scope}.rs`.

**Consumes:** contracts, registry, fixture adapter.
**Produces:** `runtime.handshake`, `snapshot.get`, `adapters.list`, discovery/grant scaffolding backed by real service behavior; versioned authenticated transport; a minimal CLI.

- [ ] Write a conformance test where client A can read one connection and client B cannot see it through snapshots, model lists, usage, diagnostics, SSE, or artifacts. Revocation must take effect on an existing stream.
- [ ] Add framing tests for partial input, concurrent request IDs, oversized frames, invalid JSON, unknown protocol major versions, and logs accidentally written to stdout.
- [ ] Implement per-client grants, filtered projections, loopback-only HTTP, Host/Origin validation, authenticated requests/streams, and user-approved single-use pairing. No wildcard CORS or authorization-by-origin alone.
- [ ] Generate OpenAPI and the JSON-RPC method manifest from service contracts. Implement `/v1/snapshot`, connection/model/usage paths, and `/v1/events` using the same service methods.
- [ ] Run `cargo test -p lai-service -p lai-transports` and a real `local-ai-kit scan --json` against a synthetic home. Commit `feat(service): expose authenticated local discovery interfaces`.

**Wire expectation:**

```json
{"jsonrpc":"2.0","id":1,"method":"snapshot.get","params":{"fields":["connections","revision"]}}
```

The response is a grant-filtered snapshot. It must not trigger enrichment or contain a credential, even if an upstream response has one.

## Task 4 — Staged discovery, evidence cache, and quota normalization

**Create:** `crates/lai-discovery/src/{lib,pipeline,scheduler,watchers,cache,log_tail}.rs`, `crates/lai-core/src/{snapshot,quota,capabilities}.rs`.

**Tests:** `crates/lai-discovery/tests/{passive_safety,partial_results,singleflight,rotation,account_change,backoff}.rs`, `crates/lai-core/tests/quota_pools.rs`.

**Consumes:** platform interfaces, adapters, service grants.
**Produces:** scan jobs/deltas; current snapshots; per-field freshness; bounded refresh scheduling; account-scoped caches; verified shared quota pools.

- [ ] Write tests that passive discovery causes zero spawns/network/secret reads; a hung provider does not block another result; 100 simultaneous refresh requests cause one upstream call; log rotation/truncation does not duplicate counts.
- [ ] Write account-switch and stale-cache tests: old quota remains associated with the old account, new authentication errors are not replaced by a stale “ok,” and a new identity cannot inherit previous grants/catalogs.
- [ ] Implement progressive stages, abortable deadlines, watcher debounce, TTLs, negative caching, jittered backoff respecting Retry-After, and incremental file offsets. Keep scheduling detached from UI renders.
- [ ] Implement quota units/reset parsing, unknown-denominator handling, pool linkage, and separate toolkit-versus-provider usage. Do not add raw percentages together.
- [ ] Run `cargo test -p lai-discovery -p lai-core` and the no-I/O passive-scan conformance check. Commit `feat(discovery): stream scoped metadata with efficient refresh and quota tracking`.

## Task 5 — Generic session engine, approvals, and safe streaming

**Create:** `crates/lai-runtime/src/{lib,session,turn,approval,events,artifact,limits,permissions}.rs`.

**Tests:** `crates/lai-runtime/tests/{lifecycle,cancel,approval_denial,backpressure,disconnect,route_pinning,artifact_scope}.rs`.

**Consumes:** route/option descriptors, grants, process ownership, fixture execution adapter.
**Produces:** all session/turn/approval methods, normalized event stream, scoped artifacts, and cancellation.

- [ ] Use a deterministic fake agent to test full streaming, denied file/tool permissions, option mismatch, duplicate turn ID, cancellation, timeout, and an ambiguous disconnect after a tool action.
- [ ] Implement route/account/workspace pinning and preflight validation; a changed catalog revision returns `SELECTION_STALE` before spawning or spending. Make requested versus enforceable sandbox constraints visible.
- [ ] Implement bounded event queues, sequence IDs, replay/gap handling, and explicit failure on unmanageable backpressure. Output/approval/final events are not silently dropped; only replaceable progress is coalesced.
- [ ] Implement deny-by-default approvals and owned-process cleanup. Parent/child lifetimes are tested on Windows and Unix. No automatic prompt retry or provider/account fallback after a partial turn.
- [ ] Run `cargo test -p lai-runtime -p lai-service -p lai-transports`. Commit `feat(runtime): add permissioned agent sessions and loss-aware streaming`.

## Task 6 — Codex reference adapter, end to end

**Create:** `crates/lai-adapters/src/codex/{mod,detect,metadata,app_server,events,options,usage}.rs`, `fixtures/codex/`, `docs/adapters/codex.md`.

**Tests:** `crates/lai-adapters/tests/{codex_discovery,codex_protocol,codex_options,codex_usage,codex_execution}.rs`.

**Consumes:** native app-server documentation/current schemas, T3 references, core/runtime interfaces.
**Produces:** reference adapter with separate install/auth/plan/model/option/quota/session capability evidence.

- [ ] Inspect the installed/target app-server schema and supported versions. Record methods such as initialization, account/catalog reads, rate limits, sessions/turns, and approvals only where actually supported.
- [ ] Write synthetic protocol fixtures for subscription auth versus API-key auth, multiple quota buckets, missing plan fields, unknown future effort, pagination, native model default, and a changed account event.
- [ ] Implement through the official installed client. Do not extract a subscription token into a direct OpenAI API adapter. Retain native model IDs/option values and do not port T3's product-specific default-model preferences.
- [ ] Verify discovery -> descriptor-based selection -> streamed test turn -> cancellation/approval through the public service. Live probing is separate and authorized; fixture tests must work without a subscription.
- [ ] Run `python scripts/verify.py --profile adapters --adapter codex`. Commit `feat(codex): connect native accounts models usage and sessions`.

**Milestone A:** the package already performs a genuine useful integration. Do not build every UI variation before reaching this point.

## Task 7 — TypeScript client and first live demo slice

**Create:** `packages/sdk-typescript/src/{index,client,errors,events}.ts`, `packages/sdk-typescript/src/transports/{stdio,http}.ts`, `apps/demo/src/{main,App}.tsx`, initial connection/selection/session routes, `examples/node/`.

**Tests:** `packages/sdk-typescript/tests/{contract,streaming,cancellation,browser-import,side-effects}.test.ts`, `apps/demo/e2e/reference-route.spec.ts`.

**Consumes:** generated contracts, service protocol, Codex/fixture adapters.
**Produces:** managed-stdio Node client, browser-safe HTTP entry point, first public-SDK-only demo.

- [ ] Write tests for side-effect-free imports, no Node modules in browser bundles, protocol-version rejection, abort propagation, event iteration, and correct close behavior.
- [ ] Generate types; implement ergonomic wrappers with exact native error details sanitized. Binary selection verifies platform/version/checksum; downloading/installing is an explicit command, not a silent import side effect.
- [ ] Build a small demo that displays connections, models/native options, and a permission-gated streaming session. Use only the public SDK; do not add a private HTTP route that reads raw credentials.
- [ ] Implement an explicit live/fixture switch. A fixture banner stays visible on every page and generated sample output. “No providers” in live mode remains a real empty state.
- [ ] Run `pnpm --filter @local-ai-kit/sdk test`, its browser bundle check, and the reference demo Playwright test. Commit `feat(sdk): ship typed client and public-api reference demo`.

## Task 8 — Shared ACP engine

**Create:** `crates/lai-adapters/src/acp/{mod,handshake,session,config,events,permissions}.rs`, `fixtures/acp/`, `docs/adapters/acp.md`.

**Tests:** `crates/lai-adapters/tests/{acp_negotiation,acp_configuration,acp_permissions,acp_unknown_fields}.rs`.

**Consumes:** the current ACP specification and generic runtime contracts.
**Produces:** one reusable ACP client layer; adapters supply only client launch/auth/metadata differences.

- [ ] Test version mismatch; omitted optional ACP capabilities; configOptions replacing legacy modes; grouped option values; unknown categories; native options changing after model selection; and provider-originated configuration updates.
- [ ] Implement spec-compliant unsupported defaults for omitted ACP capabilities. Keep our evidence `unknown` distinct from a protocol-defined `unsupported` value.
- [ ] Implement bounded native events, file/terminal request mediation, per-session config changes, cancellation, and capability-gated resumption. Do not advertise callbacks the broker cannot enforce.
- [ ] Preserve raw native IDs and nonsecret extension data without interpreting names as proof of model capability.
- [ ] Run `python scripts/verify.py --profile adapters --adapter acp`. Commit `feat(acp): normalize agent sessions and dynamic configuration`.

## Task 9 — Antigravity, Grok, and Gemini CLI adapters

**Create:** separate `crates/lai-adapters/src/{antigravity,grok,gemini_cli}/` directories with `mod.rs`, `detect.rs`, `metadata.rs`, `execute.rs`, `parse.rs`; corresponding fixtures and adapter docs.

**Tests:** one discovery/metadata/execution suite per adapter, including version-specific configuration and auth evidence.

**Consumes:** shared ACP/native CLI mechanisms, T3 provider code, official vendor/client docs, CodeNotch metadata references.
**Produces:** distinct first-class Google and Grok routes; no conflation of Gemini CLI with Antigravity or of Grok client with model vendor.

- [ ] For each adapter, verify launch flags, authentication ownership, version, account metadata, and permitted integration path. Record evidence gaps rather than add guessed endpoints.
- [ ] Write tests for install-but-not-authenticated, expired auth, unknown plan, configured API route, custom model/vendor route, missing quota, and dynamic native effort/control IDs.
- [ ] Implement Antigravity using current client-supported protocol/session metadata; take advantage of T3's separation between health and authenticated session data. Gemini CLI remains a distinct adapter with eligibility observed for its own connection.
- [ ] Implement Grok through the documented current ACP/headless mechanism. Qualify which account/usage data can be read; a local billing record is not proof that every model or operation is available.
- [ ] Run adapter-specific profiles and one generic ACP conformance suite against each supported route. Commit each provider separately, then record its per-field/platform support status.

## Task 10 — OpenCode, Ollama, and LM Studio

**Create:** `crates/lai-adapters/src/{opencode,ollama,lmstudio}/`, corresponding fixtures/docs, HTTP integration tests.

**Tests:** discovery, model catalog, endpoint auth, input/output support, route identity, stream cancellation, shared account/pool fixtures.

**Consumes:** official OpenCode server/OpenAPI and local-runtime APIs; generic network/runtime layers.
**Produces:** gateway-aware OpenCode routes and local-runtime routes.

- [ ] Write OpenCode fixtures with two upstream providers exposing the same model name and separate keys/billing. Test installed OpenCode versus a configured plan/provider versus a ready execution route.
- [ ] Implement OpenCode's HTTP/session/events integration rather than TUI scraping. Attach only to an explicitly authorized existing server or a broker-owned process. Secure any launched server.
- [ ] Implement Ollama/LM Studio endpoint discovery only at configured/known approved local addresses. Do not scan networks. Enumerate available/loaded models and separate local compute from optional cloud billing routes.
- [ ] Test that loading a model or running inference never happens during passive enumeration. No mock quota percentages for local runtimes.
- [ ] Run three adapter conformance profiles and commit each adapter separately. Update the demo using the existing SDK with no provider-specific UI branch.

## Task 11 — Claude Code, Cursor, and audited metadata readers

**Create:** `crates/lai-adapters/src/{claude_code,cursor}/`, `crates/lai-adapters/src/metadata_sources/`, fixture/docs suites, `docs/security/provider-auth-modes.md`.

**Tests:** native auth-status parsing, isolated profiles, read-only databases, usage refresh backoff, current auth failure with stale quota, native JSON streaming, and rejected credential export.

**Consumes:** official native CLI/client documentation, current provider integration rules, CodeNotch/T3 references.
**Produces:** supported native-client execution where available and separate, clearly classified metadata sources.

- [ ] Write tests with work/personal profiles, locked credentials, stale usage, undocumented endpoint disabled, and a native CLI backed by an API key rather than a subscription.
- [ ] Prefer the unmodified installed Claude client and documented structured modes. Do not substitute a third-party API request using its subscription token. Only expose modes the installed version supports.
- [ ] For Cursor, qualify the supported client execution path separately from reading editor/account metadata. If only metadata is implementable with verified sources, execution returns a typed unavailable reason rather than a fabricated success.
- [ ] Extract/port relevant read-only parsers. Classify private endpoints as opt-in community integrations after review; no spoofed user-agent/authentication shortcut as a hidden dependency.
- [ ] Run targeted adapters plus secret-redaction and stale-identity tests. Commit the independent adapters/readers separately and publish evidence-level support entries.

## Task 12 — Explicit direct API access

**Create:** `crates/lai-adapters/src/api/{openai,anthropic,gemini,xai,compatible}/`, `crates/lai-service/src/explicit_keys.rs`, fixtures/docs.

**Tests:** key storage isolation, custom endpoint validation, native stream parsing, endpoint mismatch, billing preflight, model catalog availability, and unsupported operation errors.

**Consumes:** explicit user keys/endpoint grants, current official API documentation, common contracts.
**Produces:** separately labeled metered/API routes; this is not a subscription credential conversion path.

- [ ] Verify each provider's current official models/authentication/streaming surfaces before implementation. Write local mock-server fixtures; no network in normal tests.
- [ ] Implement native API text/session-tool primitives only where supported; client-side conversation state is labeled separately from resumable native agent sessions.
- [ ] Enforce configured endpoint allowlists, bounded redirects, and credential-to-host binding. Custom compatible endpoints require explicit user configuration; compatibility claims name the supported subset.
- [ ] Test that loss of a subscription route does not start an API request without separate approval. An inferred or stale price cannot become a guaranteed cost.
- [ ] Run adapter and billing-consent tests; commit each provider plus the shared explicit-key mechanism in bounded changes.

## Task 13 — Shared broker, Python SDK, and any-language interfaces

**Create:** `crates/lai-transports/src/{local_ipc,broker_lifecycle}.rs`, `packages/sdk-python/`, `examples/{python,rust,dotnet-http,vanilla-web}/`, `docs/integration/{stdio,http,python,rust,dotnet}.md`.

**Tests:** two-client broker ownership, shared refresh singleflight, cross-client data denial, HTTP/SSE auth, Python sync/async contract tests, stdio/HTTP snapshot equality.

**Consumes:** service and execution runtime; generated schema/OpenAPI.
**Produces:** optional shared per-user runtime, Python SDK, tested language-neutral examples.

- [ ] Write a two-process test: A and B share allowed account metadata without duplicated upstream refresh; closing A leaves B's session alive; revoking B closes only B's scopes. An unauthorized third client sees no accounts.
- [ ] Implement protected runtime discovery and Unix socket/Windows named-pipe permissions. Prevent duplicate brokers for the same endpoint through tested locking/handshake logic; keep private sidecars as a supported alternative.
- [ ] Generate Python models and implement async streaming/cancellation plus a thin sync facade. Keep provider logic in Rust; both clients receive the same normalized records.
- [ ] Build examples that consume the actual packaged artifacts. The C# example uses generated/documented HTTP operations without claiming a full native C# SDK exists.
- [ ] Run `python -m pytest packages/sdk-python/tests`, stdio/HTTP equivalence tests, and example smoke checks. Commit `feat(interop): support shared local runtime and cross-language clients`.

## Task 14 — Selection engine and reusable controls

**Create:** `packages/selector-core/src/{index,filter,group,rank,selection,favorites,options}.ts`, `packages/react/`, `packages/ui-react/src/`, corresponding tests/story fixtures.

**Tests:** route identity, unknown capability handling, invalidated efforts, shared quota display, indexed large catalogs, no I/O during render, controlled/uncontrolled selection, keyboard/focus behavior.

**Consumes:** public SDK snapshot/catalog/options types only.
**Produces:** framework-neutral selection state; React bindings; base provider-rail selector and dynamic option controls.

- [ ] Port the Prmptr provider rail/T3 instance-aware interaction, keeping account/route keys. Do not reuse Prmptr's app-specific enums/localStorage names or name-based legacy/default assumptions.
- [ ] Write pure tests where two accounts share a model ID, a changed model removes the saved effort, a native future effort value appears, and a provider disappears while focused. A valid selection always retains its route and descriptor revision.
- [ ] Implement capability/billing/availability filters and explanatory resolution. Unknown facts never satisfy strict required-capability filters. Ranking uses declared comparable metrics/user preferences; do not manufacture model-quality scores.
- [ ] Implement descriptors as independent controls with native choice IDs, display defaults, dependent updates, disabled reasons, and permission/billing-sensitive confirmation.
- [ ] Run selector unit/benchmark tests, component keyboard tests, and accessibility checks. Commit `feat(selectors): add route-aware provider rail and native model controls`.

## Task 15 — Complete demo and eight-selector showroom

**Create:** `apps/demo/src/pages/{Overview,Connections,Models,Usage,SelectorLab,Playground,Diagnostics}.tsx`, `apps/demo/src/selectors/`, `apps/demo/src/fixtures/`, `apps/demo/e2e/`, `docs/integration/ui.md`.

**Tests:** a public-method-to-demo coverage manifest, eight-selector selection equivalence, live/fixture isolation, responsive navigation, model-option updates, approval/cancel, redacted diagnostics/export.

**Consumes:** only public SDK and selector/UI packages.
**Produces:** the complete local demonstration product.

- [ ] Create a coverage manifest mapping each public operation to a demo control or documented guarded action. Test the manifest; no advertised feature may exist only in fixture screenshots.
- [ ] Implement all eight variants: provider rail; grouped combobox; command palette; capability-first; comparison table; allowance-aware route picker; task/preset picker; mobile drill-down. They share one selection model.
- [ ] Implement overview, connection/profile/account evidence, catalogs, per-pool usage, playground, and diagnostics. Show safe TS/Python/curl snippets generated from the selected configuration with no secrets.
- [ ] Add explicit synthetic scenarios from the spec. Live mode cannot fall back to fixture accounts on failure. Use honest empty, stale, unverified, denied, and unavailable states.
- [ ] Run `pnpm test:unit`, `pnpm test:ui`, and the UI accessibility/large-catalog suite. Commit UI slices separately, then `feat(demo): exercise discovery selection usage and sessions through public APIs`.

## Task 16 — Native Messaging and host integration examples

**Create:** `crates/lai-transports/src/native_messaging.rs`, `examples/browser-extension/`, `examples/tauri/`, `docs/integration/{browser-extension,tauri,wsl}.md`, opt-in WSL registration helpers.

**Tests:** framing/message limits, unregistered extension rejection, large artifact chunking, local grants, host registration/unregistration, stopped WSL distro not started by scan.

**Consumes:** service protocol and explicit per-client grants.
**Produces:** generic transport/integration examples unrelated to the original Maps idea.

- [ ] Implement Native Messaging framing with the browser's documented size constraints. Stream large artifacts through bounded authorized chunks; never expose a general arbitrary-file-read endpoint.
- [ ] Provide explicit host registration instructions/install action. Importing the JS package cannot silently install native hosts or obtain OS permissions.
- [ ] Add a minimal extension that lists authorized connections/models and runs a permitted text task. Add a minimal Tauri example using the Rust service without making Tauri a dependency of the core.
- [ ] Add opt-in WSL environment identity and path translation tests. Record Windows-host versus WSL-native connections separately.
- [ ] Run native-message conformance, example packaging smoke checks, and the relevant Windows test job. Commit `feat(integrations): demonstrate desktop and extension hosts`.

## Task 17 — Adapter contribution kit and compatibility documentation

**Create:** `docs/adapters/{AUTHORING,SUPPORT-MATRIX,VERSIONING}.md`, `fixtures/adapter-template/`, `scripts/adapter_conformance.py`, `docs/qualification/`, `docs/security/threat-model.md`, `docs/troubleshooting.md`.

**Tests:** a new synthetic adapter registers without changes to SDK/UI sources; unknown protocol fields/options survive inspection; version-gated unsupported operations fail safely.

**Consumes:** completed adapters and public contracts.
**Produces:** a repeatable path for adding providers without reworking the product.

- [ ] Implement an adapter manifest describing operations, credential/network/process scopes, supported client versions, platform checks, and source classification. Registration does not execute downloaded code.
- [ ] Supply a small synthetic adapter with deterministic discovery, one model, one option, one quota record, and streamed text through the real interface. It is clearly a test/example adapter.
- [ ] Require fixtures, negative cases, license/source notes, and a per-operation support report for contributions. “Not live tested” is a supported report state, not a passing badge.
- [ ] Add a redacted diagnostics export and schema-based leak tests. Record future work separately rather than turn unimplemented features into successful empty responses.
- [ ] Run `python scripts/adapter_conformance.py --all` and docs/example checks. Commit `docs(adapters): publish evidence-based contribution and compatibility kit`.

## Task 18 — Performance, security, packaging, and release qualification

**Create:** `benchmarks/`, `.github/workflows/{ci,platform-smoke,release-check}.yml`, `scripts/{benchmark,package_smoke}.py`, `docs/performance/BASELINE.md`, `docs/qualification/RELEASE-CANDIDATE.md`.

**Tests:** all previous suites; measured passive/cached/idle budgets; malicious protocol frames; origin/grant/secret leakage; real process-tree termination; package imports; clean-machine platform smoke checks.

**Consumes:** the full implementation.
**Produces:** release-candidate artifacts and an honest evidence report; publication remains separately authorized.

- [ ] Measure the exact targets in the spec. Include machine/build/OS/dataset and excluded inference/UI memory. Investigate regressions; document any missed target with actual measurements and a bounded remediation issue. Never replace a benchmark with a guessed number.
- [ ] Run adversarial grant, origin, Host, redirect, path/symlink, command-argument, credential-redaction, and provider-response tests. Test that denied approvals and ambiguous disconnects do not trigger retries/fallback.
- [ ] Build native targets and run supported-platform smoke tests. Distinguish cross-compiled from executed. Create checksummed artifacts/SBOM/license notices; validate minimal consumers from package tarballs/wheels, not just workspace aliases.
- [ ] Verify `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, `pnpm check:generated`, `pnpm typecheck`, `pnpm test:unit`, `pnpm test:ui`, `python -m pytest packages/sdk-python/tests`, and `python scripts/verify.py --profile release`.
- [ ] Perform explicitly authorized low-cost live qualification where accounts exist. Record untested accounts/operations as untested. Do not spend reset credits, enable overages, purchase anything, or alter unrelated client configuration.
- [ ] Commit release preparation, update `RESUME.md`, and report exact commit, artifact paths, test outcomes, support matrix, actual performance, and remaining limitations. Verify the remote SHA only if a push to an authorized remote was requested/performed. Do not publish registries automatically.

## Review and integration checkpoints

- **Milestone A (Task 6):** one real reference adapter connects the whole service/runtime chain.
- **Milestone B (Tasks 7–11):** SDK/demo baseline and multiple protocol families, including first-class Google/Grok routes where supported.
- **Milestone C (Tasks 12–15):** explicit APIs, cross-language interfaces, reusable selectors, and full feature showroom.
- **Milestone D (Tasks 16–18):** host examples, contributor kit, platform/security/performance qualification.

A reviewer should be able to reject an adapter's unsupported authentication method without rejecting the package architecture or inventing a replacement. Keep uncertain integrations visible and isolated while completing the verified ones.

## Plan coverage map

| Spec area | Tasks |
|---|---|
| Standalone/modular/language-neutral core | 1–3, 7, 13 |
| Discovery/account/evidence correctness | 1, 2, 4, 6, 9–12 |
| Quota pools and usage semantics | 1, 4, 6, 10, 11, 15 |
| Native client communication/approvals | 5, 6, 8–12 |
| SDKs and public protocol | 1, 3, 7, 13, 16 |
| Provider rail and all selector variants | 14, 15 |
| Demo feature coverage | 7, 15 |
| Consent/credential/process security | 2–5, 11–13, 16, 18 |
| Optimization and cross-platform support | 2, 4, 13, 14, 16, 18 |
| Maintenance/contribution/release honesty | 1, 17, 18 |
