# Resume — Local AI Kit

Work branch: `feature/initial-implementation`. `main` remains unchanged. Inspect local and remote HEAD before writing; do not force-push or recreate completed work.

## Current implementation

The native service is connected end to end to the TypeScript/Python SDKs, embedded Rust facade, native-served paired demo, shared broker, and Native Messaging bridge. Supported Codex/Claude/OpenCode/local/API text routes use the real session engine. Generated domain types/OpenAPI, scoped artifacts, owner cleanup, bundle resolution, four npm packages, Python wheel, and the eight reusable React selectors are implemented.

The former palette failure is fixed in `af96d05`: exact account phrases rank before incidental substrings. All 13 showroom browser interaction tests pass. An additional 22 automated accessibility checks cover the showroom and reusable React components. The C# HTTP example has now been compiled and exercised against an actual isolated native host, not just inspected as source.

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

The opt-in local metadata watcher is wired through `discovery.watch`, the public paged-catalog helper is wired into the SDK/store/showroom, explicit Gemini/Antigravity/Grok native profiles are connected to shared ACP metadata/execution, and capability-gated ACP native suspend/resume is implemented with one-time owner-scoped tokens. The provider-agnostic incremental log tailer handles append, partial records, truncation, rotation, line windows, cancellation, and huge sparse files with bounded reads; no adapter currently interprets usage from logs. Metadata probes use Retry-After-aware exponential/jitter backoff. Evidence gaps for these implemented slices include provider-specific credit/usage fields without a verified source, fresh release qualification of newer source, and macOS execution. The broader 18-task plan still has other explicitly incomplete scope in the implementation ledger. Unknown account fields are intentional, not fabricated successful discovery.

Windows qualification and native Ubuntu execution are separate records. GitHub Actions has been failing before any job steps run; a workflow matrix is not macOS evidence. Tauri currently has an integration guide and compiled embedded-Rust pattern, not a separately qualified Tauri application.

No registry publication, OS signing, main-branch merge, paid inference, additional coding agents, token export, account cycling, or unrelated client configuration changes are authorized by these implementation tests. Preserve those boundaries while completing any remaining work.

## Exact clean-build qualification

Executable/source checkpoint: `991ec2bd827cf6e3b6bc884c453ba1ca44bb2b91`. Both Windows and Linux distributions were built from this clean source and passed isolated npm-tarball/Python-wheel consumers against their native executables. Windows full release profile exited zero; Linux release-profile native tests and SDK/package/benchmark checks exited zero. Each platform ran 301 native tests with two explicitly ignored watcher-integration tests. A later documentation-only commit records this evidence without changing the binary source.

Read `docs/qualification/RELEASE-CANDIDATE.md` and `docs/qualification/evidence/verification-summary.json` for exact filenames, hashes, tests, toolchains, and unresolved scope. Windows bundle pointer: `dist/release/latest.json`; Linux archives copied to `dist/linux-release/`. The local artifacts are not GitHub Release assets and are not published registry packages.

Final review fixed the default event-stream cursor for empty histories and removed offline package-smoke dependence on warm registry metadata. Do not recreate these fixes. The full original plan remains incomplete at the explicit boundaries above; verify future changes before changing any support claim.

## September 25 native subscription checkpoint

Explicit native profiles now wire Gemini CLI, Antigravity ACP, and Grok into the shared metadata/execution service. Registration is connection-scoped and contains no provider token/API key. Shared ACP fixtures cover negotiation, model/options, permissions, streaming, cancellation, and cleanup. A metadata-only Windows check against Grok CLI 1.0.41 reused the existing native login and returned Grok 4.7 + reasoning_effort; no model turn was sent. Gemini CLI and the dedicated Antigravity ACP runtime were not available for live qualification on this PC.

Capability-gated ACP native resumption is implemented: `sessions.suspend` returns an opaque runtime-scoped token and `sessions.resume` revalidates the same account/route/workspace before using provider `loadSession`. Incremental metadata tailing and refresh backoff are also implemented and tested. Final remaining high-value scope is release/macOS qualification and any provider-specific usage fields that can be obtained from documented/verified interfaces without inventing data. No registry publication/main merge has been performed.

## Current continuation status — September 25, 2026

The worktree resumed from local/remote `d1705fd92a2d19b38bf8bf1134ee2c543c7ce82b` on `feature/initial-implementation`; the earlier metadata qualification source was `471c072a8ffde52436427d5a33b44e55ed31e3c1`. Session resumption is capability-gated and tested for account, route, billing, model revision, workspace path, owner, one-time consumption, revocation, runtime restart, active/closed sessions, ACP capability/native-ID mismatch, and no fallback after failed load. The provider-agnostic log tail has bounded identity-aware rotation, truncation, partial records, and cancellation tests; no adapter currently consumes it. See the release-candidate report for current platform evidence.


## Latest checkpoint — September 25, 2026

Implementation source `702d98d7cb77ba788f09ce5e890149fef8908ac2` passed Windows `verify.py --profile all`, strict format/Clippy, and `verify.py --profile release`. The clean Windows distribution, package smoke, and performance evidence are recorded in `docs/qualification/RELEASE-CANDIDATE.md`. It was pushed to `origin/feature/initial-implementation`; `main` was not changed.

Linux WSL2 executed the full Rust test workspace and the deterministic SDK/integration suites up to Playwright demo tests. Chromium cache cleanup then failed with EIO and the WSL distro stopped launching; no current-source Linux package was built. GitHub Actions run `36140653176` failed before any steps appeared on Windows, Ubuntu, or macOS, and logs are unavailable. macOS remains unexecuted/unqualified. The exact current evidence and limits are in the release-candidate report.
