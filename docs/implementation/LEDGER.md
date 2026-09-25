# Execution ledger â€” Local AI Kit

Base: `1a49571b2ea5759b383966f1eae80162f4f7f4b1`. Branch: `feature/initial-implementation`. No external coding agent or paid inference used.

## Implemented/tested slices

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

OpenCode: four catalog tests red->green; two native HTTP/runtime tests red->green; explicit gateway configuration red->green. Source schema pinned to 0f549842. Execution is text-only with echoed deny-all native permissions, session-scoped streams, and owned abort/delete. Ruling: billing declarations remain user_supplied; private gateway keys are discarded and never converted into account evidence.

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
