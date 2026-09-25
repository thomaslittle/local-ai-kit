# Verified developer-alpha build

**Verified source:** `991ec2bd827cf6e3b6bc884c453ba1ca44bb2b91` on `feature/initial-implementation`.

**Status:** the supported developer-alpha implementation has passing Windows release-profile verification and Linux native/package qualification. **The full original 18-task scope is not complete.** A successful verification command does not change the unresolved scope below. Registry publishing, signing, notarization, and merging to main were not performed.

The source worktree was clean for both distributable builds. Any later report-only commit documents these artifacts; it does not change their source SHA. Machine-readable [verification summary](evidence/verification-summary.json), [Windows package results](evidence/windows-package.json), and [Linux package results](evidence/linux-package.json) are committed alongside this report.

## Executed verification

| Check | Result |
|---|---|
| Windows Rust workspace | 301 passed; two explicitly ignored service-watcher integration tests. |
| Linux Rust workspace, release profile | 301 passed; the same two explicitly ignored tests. |
| TypeScript SDK | 60 passed on Windows and Linux. |
| Python SDK | 25 passed on Windows and Linux. |
| Framework-neutral selectors | 27 passed on Windows. |
| React hooks/components | 13 passed on Windows. |
| Contract/API generation | 11 passed; generated files checked for drift. |
| Packaging-tool tests | 17 passed, including cold-cache peer installation checks. |
| Synthetic showroom service | 22 passed. |
| Chromium interactions | 13 showroom tests and two real native HTTP pairing tests passed. |
| Automated accessibility | 22 checks passed across showroom pages and reusable React components. |
| C# consumer | .NET 8 example compiled and completed real native pairing, passive scan, snapshot read, and owner cleanup. |
| Installed JS/Python consumers | Four npm tarballs and the Python wheel installed into fresh consumer environments on both platforms and talked to the bundled native helper. |

The Windows `python scripts/verify.py --profile release` command exited zero. It also ran formatting, strict Clippy, adapter conformance, native bridge/broker tests, a synthetic adapter example, distribution checksums, package smoke tests, and a 60-second performance sample. Two pending tests are deliberately visible as ignored; they are not counted as successful implementation.

## Platform and evidence limits

Windows: Windows 11 build 26100, x86-64, Rust 1.97.1, Node 22.20.0, Python 3.13, and isolated .NET SDK 8.0.425. Linux: Ubuntu under WSL2, x86-64, kernel 6.6.87.2, glibc 2.39, Rust 1.90.0, Node 22.20.0, and Python 3.12.3. The Linux compiler differs from the pinned Windows toolchain and is explicitly recorded rather than presented as identical qualification.

Linux qualification includes the native release-profile suite, SDK tests, clean package consumers, and benchmarks. The full browser/C# profile was executed on Windows, not repeated on Linux. Automated axe checks cover common WCAG A/AA failures and do not constitute a manual screen-reader audit or complete accessibility certification.

No new provider generation was performed. Native-provider execution tests used controlled peers/local protocol servers. Earlier installed-client metadata checks remain separate evidence; they do not certify paid generation for every account. GitHub Actions jobs have been failing before executing steps; the available job summaries do not establish the cause. No macOS execution or unsigned-binary distribution policy certification is claimed.

## Clean local artifacts

Artifacts are stored under the Windows project worktree, not attached to a GitHub Release or published registry:

- `dist/release/local-ai-kit-0.1.0-alpha.1-x86_64-pc-windows-msvc-991ec2bd827c.zip` — 9,091,298 bytes; SHA-256 `9210cae549c895c6a8d145f4f46ec9b0e0f758de003c31105480cbb6fd03e6ec`.
- `dist/linux-release/local-ai-kit-0.1.0-alpha.1-x86_64-unknown-linux-gnu-991ec2bd827c.zip` — 10,962,354 bytes; SHA-256 `6bebb8c1e6cb8c07c27e6b05d7dd37477a4f753dcd611410424a9fec52d8f5f2`.

Each bundle includes native executables, four npm tarballs, the Python wheel, native-served demo, generic extension example, API schemas, notices, and dependency inventory. `manifest.json` reports source, platform, dirty state, binary checksums, and signing=false. Checksums detect changes; they are not publisher signatures.

Use [distribution instructions](../integration/distribution.md). `dist/release/latest.json` selects the current Windows bundle. Builds and checks did not register browser hosts or enable provider overages.

## Measured performance

The same clean-source release binaries were measured with nine registered passive adapters and ten synthetic profiles. The fixture contains no provider network traffic or inference. This is narrower than the original 20-adapter target and excludes browser/provider-process memory.

| Metric | Windows x64 | Ubuntu/WSL2 x64 |
|---|---:|---:|
| Cold process + scan p95, 20 samples | 25.652 ms | 3.820 ms |
| Cached stdio snapshot p95, 200 samples | 0.580 ms | 0.646 ms |
| Peak idle resident memory | 7.727 MiB | 6.625 MiB |
| Idle CPU delta, one core | 0.0% | 0.0% |
| Idle sampling interval | 60.55 s | 62.37 s |
| Browser SDK gzip size | 4,866 bytes | 4,866 bytes |

Zero sampled CPU delta is not a claim of no possible CPU use. These measurements are not latency guarantees or inference benchmarks. See full [Windows](evidence/windows-performance.json) and [Linux](evidence/linux-performance.json) distributions and binary hashes.

## Unresolved original scope

1. Native Google/Grok subscription execution glue remains blocked/unregistered. Experimental ACP recipes are not a completed subscription connector. No bypass was attempted.
2. The tested OS metadata watcher primitive is not connected to service invalidation; two corresponding integration tests remain explicitly ignored. Automatic refresh is not advertised.
3. Public `models.page` works through native transports and generated SDKs. The tested TypeScript assembly helper is not root-exported or connected to the current showroom/store after its integration write was blocked. Very large live showroom snapshots therefore retain their response-size limit.
4. Durable cross-process session resumption, the complete incremental activity/usage-reader tranche, exhaustive credit normalization, and macOS qualification remain unfinished. Tauri is an integration guide, not a separately executed Tauri application.

The supported implementation can be installed and exercised now. These gaps must remain visible until separately implemented and verified; do not mark the entire plan complete, turn ignored tests into successful evidence, or confuse explicit API routes with subscription access.

Review in this session was author self-review, not an independent security audit. The review found and fixed the empty event-journal cursor issue and the offline React peer-cache assumption; focused regressions failed before the fixes and passed afterward.
