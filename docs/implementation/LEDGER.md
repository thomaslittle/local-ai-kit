# Execution ledger - Local AI Kit

## October 24 Linux cached-usage revocation follow-up

Implemented a second revocation check after asynchronous request-body decoding.
The existing synthetic consuming flow reproduced a revoked upload returning 200;
after the fix it returns 403 without another cache read. Fixture-tested and
platform-executed on Linux with Node 24.22.1 and 22.23.4 using
`node --experimental-strip-types examples/mcp/remote/inert-check.mjs`: one
synthetic cache read, zero provider requests/listeners/live credentials or grants,
and unchanged fixture database bytes. No live cache, account mapping, deployment
or plugin activation was performed; these remain unqualified and approval-gated.
SDK strict no-emit type-check, project hygiene/local links and whitespace checks
also pass. `npm ci --ignore-scripts --no-audit --no-fund` installed the locked
workspace dependencies without changing the lockfile. CI for this continuation
is unverified.

Shared user-local Node 24.22.1/npm 11.19.1 and Bun 1.4.4 setup is available;
Node 22.23.4/npm 10.9.9 is retained for consumers requiring npm 10. Official
release archive SHA256 checks passed. No other application source was edited.

## October 23 cache-only usage MCP candidate

Implemented scoped `kit_usage_cached` façade and bounded stdio framing over
existing SDK cache state. Windows synthetic endpoint evidence, strict SDK
type-check and canonical quota-pool validation pass. Disposal and terminal
permission denial clear the live store; tool reads cause no provider/client
requests or interest renewal. [Receipt](../qualification/evidence/2026-10-23-cached-usage-mcp/README.md).
Actual dot registration, account-to-provider mapping and current allowance remain
unqualified and require approved access/setup, not an inferred quota estimate.

## October 22 native SDK bootstrap CI follow-up (release held)

Clean source `aeecd15` passes Windows artifact provenance/licenses/SBOM,
native/npm/Python/React consumers and all enforced 60-second performance targets.
Latest-head Ubuntu and React Doctor CI passed; macOS executed and failed an
SDK fixture that assumed the first cached snapshot preceded host-authorized
background inventory. A delayed local fixture reproduces that failure.

The test now accepts either valid startup ordering while requiring unknown
auth, no models/accounts and no sentinel disclosure. Existing explicit refresh,
credential equality, concurrent reads, revocation and default denial remain.
After removing the diagnostic delay, Node 5/5 and Python 4/4 bridge checks pass.
R8 accepts the sole source diff with no findings. Production/R6/R7 source hashes
are unchanged. New source provenance and CI readiness remain gates; release is
held. See [the receipts and limits](../qualification/evidence/2026-10-22-shared-runtime/ci-bootstrap-alpha18/README.md).

The sibling Python first-read assertion is now aligned too. A combined rerun
also exposed a page-revision comparison across a bootstrap publication; the
empty fixture now brackets the page with snapshots and retains generation
equality at either boundary. Both failures are retained. R9 accepts both test
diffs with no findings; the final bridge passes Node 5/5 plus Python 4/4, and ten
consecutive combined runs pass 90 executions with zero failures/retries. No
production code or timing retry changed. See [the sibling receipts](../qualification/evidence/2026-10-22-shared-runtime/ci-bootstrap-siblings-alpha18/README.md).

## October 22 alpha18 security follow-up (qualification in progress)

After refresh source `cf34b3d` was pushed, default-branch dependency alerts
identified JWT claim validation and two CSS build parsers. The signed-token
fixture failed before the claim change; requiring `exp`, `iss`, `aud` and `sub`
passes. The candidate pins jsonwebtoken 10.3.0 with the ordinary AWS-LC backend,
source-map-js 1.2.2 and selector parser 7.1.6. Fresh npm installation and audit
pass with zero vulnerabilities. Independent R6 approves the scoped changes;
its single guarded Windows auth execution passes 21 tests. No OAuth grant,
scope or provider account was enabled.

The first isolated-target full run retained a Retry-After fixture failure.
A deliberately delayed source reproduces that failure as a virtual timeout
before `RateLimited` delivery. The fixture now holds fake time until the public
issue proves publication. Its 25 library and 20 integration tests pass, and R7
accepts the ordering without a Critical or Important finding. Production cache
and supervisor sources are unchanged from R5. The full Windows rerun passes
all 24 commands (638 Rust tests plus SDK, generation, integration, browser,
accessibility and React Doctor checks). The Retry-After mutation fails at the
intended assertion; restored source then passes ten combined refresh runs,
450 executions without failures or retries. Formatting, strict Clippy, 638
all-feature Rust tests, all eight adapter conformance groups and the real C#
native consumer pass. Qualification peaks at 8.03 GiB active output with no
pressure stop. Exact-source artifact and remote-readiness checks remain gates;
release remains held.

Resource recovery relocated exactly 664 owned generated PDBs, 54,956,384,256
bytes (51.182 GiB), to retained task-owned D: storage; zero files were deleted.
The unique D: lane has a 54 GiB retained-symbol budget and 12 GiB active-output
budget, with an 11 GiB observed stop threshold, C:/D: free-space reserves, two
jobs, no symbols/incremental output and no Rust compiler wrapper per process.
The initial run inherited global kache output outside its measured target;
the guard now disables that wrapper without changing global configuration or
removing shared cache entries. All other targets and live processes are preserved.
See [the evidence and limits](../qualification/evidence/2026-10-22-shared-runtime/release-security-alpha18/README.md).

## October 22 Sol refresh lanes (local qualification; release held)

Preserved the interrupted `cache.rs` / `metadata.rs` edits; the failed `lib.rs`
tool call had not applied. Implemented independently admitted inventory,
metadata and authoritative usage/reset lanes, active/idle interest, bounded fair
queues, shared coalescing/backoff and owned cancellation. Inventory opt-in is
separate from provider grants. Timers retain stale readings and quota warnings;
they never infer a reset or recovery.

Five independent review rounds drove failure-first fixes for journal scope,
atomic success/error generations, fairness beyond 64 sources, failed-SQLite
scan staging, and cancellation under the commit mutex. Round 4 accepted the
runtime but retained a nondeterministic restart-test failure. The test now waits
for cache flight retirement/publication before advancing fake time. Round 5
approved the ordering and ran ten default combined suites: 25 library plus 20
integration tests each, 450 executions, zero failures or retries. No production
code changed after round 4. Earlier failures and passing retries remain visible.

The alpha18 full `verify.py --profile all` passes all 24 commands on Windows:
638 Rust tests, 28 selector tests, 68 TypeScript SDK tests, 27 Python SDK tests,
13 React tests, native bridge/generation/packaging checks, 16 Python browser
checks, 16 Node browser checks, 34 accessibility checks and React Doctor with no
issues. All-target/all-feature workspace tests also pass (638 passed, zero failed) in 62. Formatting
and strict all-target/all-feature Clippy pass. Optimized binaries compile. The
C# real native HTTP consumer passes using the checksum-verified workspace-local
.NET 8 SDK with an empty NuGet source configuration. Logs and source hashes are under
`docs/qualification/evidence/2026-10-22-shared-runtime/refresh-lanes/sol-20261022/`.
The initial missing-Playwright dependency error, offline npm cache miss, and
one Windows error32 temporary-directory rename failure are retained. Pinned
local dependencies were installed; the unchanged six-test resumption retry and
full rerun pass. The interfering handle was not identified; no antivirus cause
is asserted and no retry/suppression was added to the product or fixture.

Three real guard mutations previously failed assertions, were restored
byte-for-byte, and passed. The final default offline scan reports 15 inspected matches: an existing
synthetic OAuth RSA fixture, two generated type declarations and 12 independently
recomputed source checksums. The earlier scan 46 reported only the first three. Dispositions retain hashes and reasons; the scanner exit remains
1 and no ignore rule was added. No signing credential was generated or used.

Current Codex/Claude/Muse adapters provide only profile-bound full operations
and connection-scoped pools with unknown cross-profile sharing. Coalesce all
verified routes/apps in that connection; never merge similar identities.
Verified cross-connection pool operations, actual OS suspend/resume and live
provider accounts remain unqualified. Sleep/wall-clock fixtures pass. No live
credential, paid inference, GUI input, audio or OS notification was used here.

Alpha18 version preparation updates Rust/npm packages and the previously
lagging Python wheel (`0.1.0a11` to `0.1.0a18`); no dependency upgrade was added.
Latest published release remains `v0.1.0-alpha.17`. Exact-source CI, clean Windows
release artifact/checksums and consumer release pins remain gates; no merge or
publication has occurred. Kit/Remotif target main; Crntly targets only
`feature/gpui-native-rust-studio`. Keep existing GUI recovery/scope/draft receipts
and consumer changes intact. No parity, bot or rendering work is included.
Kit owns no GUI lock or Remotif target directory.

## Shared runtime - October 22, 2026

Branch `feat/shared-runtime` (PR [#28](https://github.com/thomaslittle/local-ai-kit/pull/28)), from `556e35e`, merged with `origin/main` after dependency maintenance. Added `lai-store` SQLite state, atomic snapshot/metadata/journal revisions, epoch cursors and bounded replay, persistent health/issues, one notification coordinator, explicit independent broker lifetime with owner/admin token descriptors and stale-endpoint recovery, a public-API-only native Rust control center, Ed25519 update verification/staging/activation/rollback, inventory SDK subscriptions, and repaired upstream classification/report-only CI. Remotif and Crntly shared-broker integrations are in their own worktrees; both authenticated to a real broker process on Windows.

Verified on Windows: 598 Rust tests, 68 TypeScript SDK, 13 React, 27 Python; CI run [39743883610](https://github.com/thomaslittle/local-ai-kit/actions/runs/39743883610) passed Ubuntu/macOS/Windows at code commit `6026590`; `cargo fmt` and workspace clippy; real independent broker/control-center/app-revocation E2E; Remotif and Crntly opt-in cross-process broker tests; release debug bundle and unsigned single-executable update manifest. Deliberately unclaimed: full GUI E2E for both consumers, live provider accounts, signed/published/installed updates, Linux/macOS shared-runtime execution, and per-app isolation in independent mode. See [the acceptance ledger](../qualification/2026-10-22-shared-runtime.md).

## Readiness review — September 27, 2026

Preserved and reviewed the dirty real-data demo/provider work on `74138c5`. Added regressions and fixes for accountless Ollama/LM Studio execution through the embedded facade; completed-turn suspend; fail-closed suspend errors/timeouts; native resumed-conversation continuation in the browser; grant-lifetime synchronization of session/token publication and revocation; concurrent successful SDK probes; Windows installation/shim identity; and OpenCode authenticated startup. Tests caught and corrected cold-process timing assumptions, a two-second heartbeat watchdog, and concurrent Windows test pipe-name collisions. Explicit timeout tests still enforce their short deadlines.

Ruling: retain distinct executable directories as separate installation routes, while grouping Windows command shims within one directory. Do not collapse different installed versions merely to simplify the account list. Configuration-only OpenCode retains stable identity when its data directory appears.

The first manual paired browser run returned 130 model entries; after preserving distinct installations, the final clean-source run returned 137 from Codex, Claude Code, Grok, and OpenCode; no model turn was sent. UI regression coverage includes continuing a resumed session without calling sessions.create, token non-disclosure, all eight selectors, and real-data-only failure states. Added `docs/integration/prompt-app.md` for application integration boundaries.

The earlier GitHub-hosted runs failed before any step started because of an account-level runner restriction. WSL starts again; new qualification must be reported independently of September 25 evidence.

Final clean implementation source `0c8213842be5b72ea7614b51028a24912b1bdabd` passed full and release profiles on Windows and Ubuntu/WSL2. Rust: 349 Windows / 351 Linux, zero failed/ignored; each platform: TypeScript SDK 63, Python SDK 26, selectors 28, React 13, browser 14, accessibility 32. Clean packages, actual helper consumers, C#, strict lint/conformance, checksums, and benchmarks passed. Linux found an inode-reuse rotation defect during qualification; bounded leading fingerprints and birth-time identity corrected it before these final runs. Separate review found no blocking issues in that fix. See [current readiness evidence](../qualification/2026-09-27-readiness.md).

## Historical checkpoint — September 25, 2026

The current source checkpoint `702d98d7cb77ba788f09ce5e890149fef8908ac2` passed Windows full/release verification and package/performance qualification; 332 Rust tests passed with zero failures or ignored tests. Linux WSL2 passed the Rust workspace and most deterministic SDK/integration suites at the prior source revision but did not complete UI/release qualification after a local Playwright/WSL runtime I/O failure; the final synthetic large-log fixture has only Windows execution evidence. Historical Windows/Linux artifacts at `991ec2b` do not qualify current source. Native subscription profiles for Gemini CLI, Antigravity, and Grok are implemented with explicit host registration and connection-scoped grants. Grok has metadata-only Windows evidence; Gemini CLI and Antigravity do not have live qualification. `sessions.suspend` / `sessions.resume` use ACP `loadSession` only when advertised, owner-scoped one-time in-memory tokens, and checks against the saved account, route, billing, model revision, workspace path, and current authorization. The incremental metadata tailer is provider-agnostic infrastructure; no adapter currently consumes it or derives quota from logs. Metadata failures use bounded retry backoff and sanitized Retry-After metadata.

The worktree base at continuation was `d1705fd92a2d19b38bf8bf1134ee2c543c7ce82b`. The clean Windows release distribution, isolated consumers, and fresh performance report were regenerated from the current source checkpoint and passed. GitHub Actions run `36128810500` failed before any steps were created on Windows, Ubuntu, and macOS; the run provides no external cause. The workflow includes all three platforms, but macOS execution evidence remains absent. Older entries below are chronological evidence from earlier checkpoints and do not describe the current feature set unless explicitly updated here.

Base: `1a49571b2ea5759b383966f1eae80162f4f7f4b1`. Branch: `feature/initial-implementation`. No external coding agent or paid inference used.

## Historical implementation timeline

The entries below preserve earlier results, limitations, and next steps at their original checkpoints. They are not the current support or qualification status; use the September 27 report above.

## Implemented/tested slices (historical)

- Canonical Rust contracts: 13 Windows tests pass on Rust 1.97.1.
- Native filesystem primitives: 7 Windows tests pass, including requested-byte reads of a sparse 1 GiB file. Unix-only behavior still requires a Unix Rust runner.
- Headless selector: 24 Node tests pass and TypeScript build passes.
- TypeScript HTTP/SSE/stdio SDK: 32 tests pass, actual synthetic subprocesses and local HTTP streams.
- Python async/sync SDK: 18 tests pass, plus isolated wheel installation.
- Synthetic showcase service: 22 tests pass, including real HTTP/SSE.
- Eight-selector browser interface: 13 Chromium tests pass via offline test transport; real browser-to-HTTP navigation still unqualified due managed browser policy.
- JavaScript tarball/Python wheel clean-consumer checks passed in the assistant container.

These are partial deliverables across Tasks 1/2/7/13/14/15. No complete native provider or full release acceptance is claimed.

## Native discovery increment

Task 2/4 passive inventory tests are being introduced before implementation. Scans are explicitly invoked and must use metadata only, no file-content/credential reads, process spawning, or provider network calls. Inventories preserve profile/environment identity, stream partial snapshots, and never infer active subscriptions from installed files. Next: run the native inventory red tests, implement registry/scanner, and connect it to the real native service/CLI.

## Development environment

The Windows device briefly disconnected, and GitHub CI failed before runner assignment. Tested SDK/UI work continued in the Linux container instead of using paid agents. The device has reconnected; native tests now run on Windows. Pre-offline local changes were preserved in labeled git stashes before fast-forwarding the worktree.

## Continuation: native service completion

Ruling: remote HEAD `99aee26` still contained a service stub, so complete that before bridging SDKs; the prior response overstated committed service status. Kept existing contracts and fixtures. All 16 service tests now pass (four new boundary tests observed red first), and the 58-test Rust workspace suite passes on Windows. Original task scopes remain partial. Next: CLI/stdio integration, then Codex.

## Native CLI and SDK bridge

Eight transport and five CLI tests observed red then green. The complete 71-test Rust suite passes. Six additional Node/Python tests exercise the actual executable and native service. Added explicit scan authorization, isolated test homes, handshake gating, and bounded correlated stdio. Next: Codex metadata adapter, keeping paid inference disabled.

## Codex metadata through the actual native helper

Implemented and tested parser, native-process protocol, Windows native/npm executable resolution, separate per-connection/provider metadata grants, cached probe merging, and both SDK probe methods. Installed Codex 0.156.1 returned real account/plan, seven models/options, and usage through Python -> native helper -> Codex, without any prompt.

Rulings: Codex's initial account announcement is accepted before its authoritative account read, while later account changes invalidate metadata. Connection-scoped account IDs are not treated as proof of cross-client account equivalence. Percent windows are not token balances. Metadata-only routes keep generation explicitly unsupported.

Author self-review fixed stable-snapshot revision churn, failure-backoff storms, and billing/permission-sensitive option checks. 107 Rust tests and six real-helper SDK tests pass; clippy passes. The full execution/broker/provider expansion tasks remain incomplete.

## Native execution continuation
Eight runtime, six Codex protocol, and five service execution tests went red and then green. Separate host execution/workspace grants; no live generation. Ephemeral sessions deliberately do not promise cross-process native resumption. Ruling: finish the existing service chain before adapter expansion.

## Native HTTP and browser increment
Six HTTP tests, two native stream tests, three host CLI tests, and two actual Chromium tests passed. Real Codex metadata reached the browser without generation. Native static assets are served by Rust; the separate Node fixture server is never a live fallback. SDK event-gap handling preserves unrelated requests.

## Lifecycle review fixes
Bounded event replay, closed-session capacity recycling, session-level billing consent, stale metadata during an active turn, and native account checks before each prompt now have red/green regressions. Workspace tests and strict Clippy passed. No generation was submitted.

## Resumed provider expansion
Remote/local checkpoint 0ce8f18 already includes execution, paired HTTP/SSE, and Claude native integration. Preserved the pending gateway-route changes and verified the whole baseline. Ruling: finish route identity before adding gateways; metadata alone must never imply an installed execution driver. A new gateway runtime-capability regression failed first. Shared ACP/native adapter expansion follows the original Tasks 8-12.

## ACP core
Task 8 increment: nine catalog and eight owned-child protocol tests observed red then green. Native models, grouped controls, capability defaults, permission denial, event routing, cancellation, and model-switch detection are tested. Ruling: ACP alone is not a sandbox or subscription evidence; register execution only through a provider-specific permission policy. First live Grok check used initialize/session-new only, with tools disabled. Full native workspace: 177 passing tests.

## HTTP provider foundation continuation
Resumed existing worktree at 9a32086; retained in-progress Grok/ACP metadata and command-capture work. Five HTTP tests failed at missing implementation, then passed after endpoint-bound authentication, redirect rejection, byte/deadline limits, and NDJSON/SSE line reading. Full Windows Rust workspace: 189 tests passed. No inference or provider credential export. Original 18-task release still in progress.

HTTP providers: catalog tests 7 red->green; frame tests 2 red->green; runtime/HTTP integration 2 red->green; endpoint registry 4 red->green; CLI endpoint tests 2 red->green plus one negative case. Full Windows workspace 207 passed. Ruling: explicit host textModels assertions are tagged user_supplied when catalogs do not publish text capability; no model-name eligibility guesses. Endpoint credentials remain an explicit host-resolver responsibility.

OpenCode: four catalog tests red->green; two native HTTP/runtime tests red->green; explicit gateway configuration red->green. Source schema pinned to 0f549842. Execution is text-only with echoed deny-all native permissions, session-scoped streams, and owned abort/delete. A nonempty OpenCode `/provider` connected list is reported as profile-level authentication evidence; it does not validate every upstream credential. Billing declarations remain user_supplied; private gateway keys are discarded and never converted into account evidence.

Generated-contract increment: six type-generator tests red->green, four protocol-document tests red->green, Rust request-map tests red->green, and three runtime-introspection regression tests red->green. The Python stub regression caught boolean JSON Schema response types and received a focused fix. TypeScript consumer compilation rejects incomplete sessions, unknown probe arguments, and unrequested projection fields.

React packages: SnapshotStore six red->green, hooks four pass, selectors nine pass including six original red cases. The selector views depend only on public SDK records and pure selector-core. No automatic load/scan on mount, mutable host snapshot, client ownership transfer, or sensitive-option approval by default. TypeScript inference/default-object issue fixed; all four package builds pass.

Task 13/16: shared IPC and Native Messaging implemented against existing public service. Native framing and two-client broker RED->GREEN, 240 Windows Rust tests; Node/Python actual broker integration passed. Ruling: broker bootstrap tokens use trusted parent stdout control frames, never diagnostics or argv; browser host installs only through an explicit script. Publication/main unchanged.

Owned artifacts: runtime and public-service regressions observed red then green. Full native 250 tests passed; strict Clippy and generated contracts rebuilt. Ruling: store bounded results in memory for 15 minutes, never import arbitrary native paths; this avoids file/symlink credential disclosure and leaves result saving explicit.

Embedded Rust facade and lifecycle: one lai-kit import now assembles the shared CLI provider service. Four facade and three host-lifecycle cases pass; TypeScript/Python close sends native owner cleanup before transport close. A shared broker can release closed clients without losing another app. Explicit included-usage denial is now checked before subscription selection, regardless of generic paid-option consent or stale reset metadata.

## Continuation: selector regression resolved

Resumed existing worktree at 978824642e9ea26ad7ecbdf1b9400567173ae0f5; remote matched and all pending work was retained. The palette's `work workspace` query treated the `work` substring in every `workspace` equally. Added an exact-phrase relevance pass in the shared headless selector, retaining broad matches, stable tie order, billing preference, and route identity. New focused regression failed before the change. All 27 selector tests and all 13 real Chromium showroom tests now pass; all four TypeScript package builds pass. Previously blocked native execution glue remains unmodified/unavailable; no inference or credential export is authorized by these tests.

## Distribution and additional qualification

The full Windows `all` profile passed with 291 Rust workspace tests, 27 selector tests, SDK and generated-contract suites, real native-helper/broker bridges, 13 showroom interactions, two native browser-pairing checks, and 22 automated accessibility checks. The plain HTTP C# example separately built and completed a real pairing/scan/close flow using .NET 8.0.425. Added these checks to the release workflow and kept all package registries private/unpublished. Automatic accessibility tests do not certify manual screen-reader behavior.

GitHub Actions run 36082359183 failed with no executed job steps; the available job summary does not explain the underlying runner failure. Do not present it as a passing Linux/macOS CI run or infer a billing cause. Local Ubuntu qualification is separate and must name the exact source snapshot tested.

## Catalog paging and opt-in platform watcher

Added public `models.page` with caller/filter/revision-bound cursors, 1–500-row limits, a serialized byte budget, and stale/permission validation. Five native page regressions failed before implementation and now pass. The typed TypeScript/Python method families and generated protocol artifacts include paging. A bounded TypeScript assembly helper has five passing tests, but its package-root/demo integration write was blocked and is not claimed complete.

Added `lai-platform::MetadataWatcher` using notify 8.2.0 OS events, explicit roots and metadata filenames, no credential-content reads or recursive profile traversal. Three Windows tests failed before implementation and now pass. Service integration was blocked; two service watcher acceptance tests remain explicitly ignored, and no watcher RPC is advertised. Do not count the ignored tests as passing or this primitive as automatic service refresh.

The full Windows `all` profile passed after these additions with the two documented ignored service-watch tests. Next: qualify exact clean-commit distributions and native Ubuntu execution; preserve remaining original-plan gaps rather than marking the full project complete.

## Final review corrections

Author review found that an empty runtime journal returned no `tailCursor` and accepted a cursor ahead of its history. Two regressions failed first, then passed after empty journals were routed through the existing validated journal reader. Native streaming tests also pass. This corrects default subscriptions before the first turn without changing provider or permission behavior.

Clean Linux consumer testing exposed `ENOTCACHED` for React registry metadata: npm ci had installed the peers, but the offline smoke test relied on a separate cached packument present on Windows. Four new peer-fixture tests failed first. The smoke now packs the already locked installed React peers and their actual runtime dependencies as explicit temporary tarballs; dependency resolution remains strict, with no force/legacy-peer flags. Windows package smoke passes with this change. Linux rerun and final exact-commit bundles follow.

## Clean Windows/Linux artifact checkpoint

Source `991ec2bd827cf6e3b6bc884c453ba1ca44bb2b91`, dirty=false, qualified on both platforms. Windows `verify.py --profile release` exited zero (227.18 seconds); native tests: 301 passed and two ignored. Linux release-profile native tests: 301 passed and two ignored; both SDK suites and clean package consumers passed after the peer-cache correction. The resulting Windows/Linux archives and full SHA-256 values are recorded in docs/qualification/evidence/verification-summary.json. No inference, publication, signing, or main merge occurred.

Windows measured cold scan p95 25.652 ms, cached read p95 0.580 ms, idle peak 7.727 MiB. Linux measured 3.820 ms, 0.646 ms, and 6.625 MiB respectively. These are the documented nine-adapter/ten-profile fixture results, not universal guarantees. Binary hashes and raw sample summaries are committed as safe evidence JSON.

Remaining original scope is explicitly listed in RELEASE-CANDIDATE.md. This checkpoint completes packaging/verification of supported alpha paths, not the entire original 18-task scope or the blocked provider/watch/UI glue.

- Native subscription connectors: added explicit host-owned native profile registration for Gemini CLI, Antigravity ACP, and Grok. Passive detection remains inert; execution capability is connection-scoped, not provider-wide. ACP success is official-protocol auth evidence; plan/quota remain unknown unless reported, and host billing declarations remain user-supplied evidence.
- Grok metadata qualification: Grok CLI 1.0.41 reused its existing native login without token export and returned Grok 4.7 plus reasoning_effort. No prompt/inference was submitted.
- Verification: full deterministic Windows profile passed after the native connector wiring, alongside strict workspace Clippy and targeted ACP/CLI/native-profile regressions.

- Refresh/usage infrastructure: added a provider-agnostic bounded incremental line tailer with partial-record buffering, truncation/replacement rotation detection, sparse-file bounded reads, and no automatic quota interpretation. Added Retry-After-aware, exponential+jitter metadata negative caching and exposed only sanitized retry timing through Rust/TypeScript/Python errors. No generation request is retried automatically.


## Fresh source qualification — September 25, 2026

Checkpoint `702d98d7cb77ba788f09ce5e890149fef8908ac2`: Windows `verify.py --profile all`, strict formatting/Clippy, and `verify.py --profile release` passed. The clean Windows package, consumers, artifact hashes, and fresh benchmark are recorded in `docs/qualification/RELEASE-CANDIDATE.md`. No live inference was sent.

Linux WSL2 completed `cargo test --workspace --all-targets`, SDK tests, generated-contract checks, React tests, helper/broker tests, synthetic adapter, and deterministic integration checks before Playwright demo execution encountered a missing dependency. After installing the pinned dependency, Chromium downloaded but its cache cleanup failed with EIO; the distro subsequently failed to start. Linux release/package qualification remains incomplete. GitHub Actions run `36140653176` failed before any step ran on Windows, Ubuntu, or macOS; no logs or cause are exposed. macOS remains unexecuted. Earlier ledger entries and `991ec2b` artifact evidence remain historical.


## Current-source package evidence

At source 702d98d7cb77ba788f09ce5e890149fef8908ac2, the Windows release profile passed. The archive hash, package consumer results, binary hash, benchmark measurements, and limitations are in docs/qualification/RELEASE-CANDIDATE.md. The final large-log fixture models a 1 GiB source without allocating disk and checks that the tail reader returns fewer than 5,000 bytes.

Current-source Linux release/package qualification and macOS execution are still absent. The prior WSL suite stopped at Playwright, the retry encountered WSL filesystem/startup I/O errors, and GitHub Actions run 36140653176 exposed no executed steps or logs.
