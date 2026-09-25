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

Native Google/Grok client-to-execution glue was blocked by the tool safety boundary and was not bypassed. Those executors remain unregistered. ACP authentication and launch-recipe fixtures are not proof of a working subscription route. Do not restore ignored incomplete sketches or expose an unrestricted native-policy mode as a supposed read-only sandbox. See `docs/adapters/native-google-grok-status.md`.

Automatic filesystem-watch refresh, the full incremental activity/usage-reader tranche, exhaustive provider credit metadata, and durable cross-process session resumption remain outside the implemented alpha. The original 18-task plan remains broader than this checkpoint. Unknown account fields are intentional, not fabricated successful discovery.

Windows qualification and native Ubuntu execution are separate records. GitHub Actions has been failing before any job steps run; a workflow matrix is not macOS evidence. Tauri currently has an integration guide and compiled embedded-Rust pattern, not a separately qualified Tauri application.

No registry publication, OS signing, main-branch merge, paid inference, additional coding agents, token export, account cycling, or unrelated client configuration changes are authorized by these implementation tests. Preserve those boundaries while completing any remaining work.
