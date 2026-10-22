# Release qualification

The current alpha18 shared-runtime candidate is on `feat/shared-runtime` after
refresh source `cf34b3d`. Its claim/dependency security follow-up, independent
reviews, bounded build ownership and retained failed runs are recorded in the
[October 22 ledger](2026-10-22-shared-runtime.md) and
[security evidence](evidence/2026-10-22-shared-runtime/release-security-alpha18/README.md).
Release is held pending the final source gates and artifact qualification.
Earlier platform results below apply only to their named source revisions.

September 27 implementation source: `0c8213842be5b72ea7614b51028a24912b1bdabd`. Windows and Ubuntu/WSL2 full/release verification, clean consumers, bundle checksums, and fresh benchmarks passed. macOS was externally blocked before job execution. Read the [September 27 readiness report](2026-09-27-readiness.md) for those counts, artifacts, fixes, and limits.

## Historical verified developer-alpha build

September 27 review is recorded in [the current handoff](../implementation/DEVELOPMENT-LOG.md) and [implementation ledger](../implementation/LEDGER.md). The measurements below remain evidence for their named historical commits. GitHub run `36141317640` has no executed steps: its check annotations reported an account-level runner restriction. This is an external CI blocker, not a macOS test result.

**Verified source:** `991ec2bd827cf6e3b6bc884c453ba1ca44bb2b91` on `feature/initial-implementation`.

**Status at that source revision:** the supported developer-alpha implementation had passing Windows release-profile verification and Linux native/package qualification. **The full original 18-task scope was not complete.** These results apply only to source `991ec2b`; later changes are not covered by the historical counts or artifacts below. Registry publishing, signing, notarization, and merging to main were not performed.

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

## Work completed after this artifact checkpoint

These items landed after source `991ec2b`; they are not qualified by the artifact hashes, test counts, and performance measurements above:

- `discovery.watch` is wired through the service and invalidates only affected metadata; it does not probe or infer.
- `models.page` is available through the SDK/store/showroom for large catalogs.
- Gemini CLI, Antigravity ACP, and Grok have explicit host-registered native profiles and connection-scoped execution. ACP fixture coverage is implemented. Only Grok has the recorded Windows live metadata-only check; no inference was sent.
- ACP native suspend/resume is gated by `loadSession`. It uses one-time owner-scoped runtime-memory tokens and revalidates account, route, billing, model revision, workspace, and grants. It does not promise durable history across helper restart.
- `lai_discovery::log_tail` provides bounded, cancellable newline-record reads, file identity/anchor rotation checks, truncation reset, partial-record buffering, and symlink-safe native reads. No provider adapter currently uses it; it does not interpret usage.
- Metadata probe failures expose sanitized retryability/Retry-After information and use bounded negative-cache backoff. Generation requests are never retried automatically.

## Historical qualification status — September 25, 2026

The changes after `991ec2b` have a fresh Windows release run on clean source `702d98d7cb77ba788f09ce5e890149fef8908ac2`; see the current-source evidence below. The Windows distribution and package consumers were rebuilt from that exact SHA. Linux verification ran on WSL2 through the full Rust workspace and most deterministic SDK/integration checks, but did not finish the browser demo or produce a current-source Linux distribution. macOS remains unexecuted and unqualified.

GitHub Actions run `36140653176` created Windows, Ubuntu, and macOS jobs but failed all three before any steps appeared. `gh run view --log` returned `log not found`, so the workflow service exposes no job output or external cause. Do not assign a billing or account explanation without evidence.

The remaining truthful gaps include provider-specific plan/usage values without documented sources, current-source Linux package qualification, macOS platform execution, live Gemini CLI/Antigravity qualification, and other original-plan scope marked incomplete in the implementation ledger. Tauri remains an integration guide rather than a separately qualified application. API billing remains separate from native subscription access.

No registry release, OS signing/notarization, main merge, live generation, or paid fallback was performed.

Review in this session was author self-review, not an independent security audit. The review found and fixed the empty event-journal cursor issue and the offline React peer-cache assumption; focused regressions failed before the fixes and passed afterward.


## Historical Windows qualification — September 25, 2026

Source 702d98d7cb77ba788f09ce5e890149fef8908ac2, clean working tree, Windows 11 x64:

- verify.py --profile release: passed. This profile runs the full deterministic all suites first.
- cargo test --workspace --all-targets --locked: 332 passed, 0 failed, 0 ignored across 78 test binaries.
- cargo fmt --all --check and strict all-target/all-feature Clippy: passed.
- Release profile also passed the C# HTTP consumer against the local helper, adapter conformance, bundle validation, isolated npm tarball consumers, installed Python wheel with the actual native helper, React server rendering, and the 60-second performance measurement.
- Current Windows archive: dist/release/local-ai-kit-0.1.0-alpha.1-x86_64-pc-windows-msvc-702d98d7cb77.zip; SHA-256 9b889d5a2fcf05c75f232419edd60871085541e4a1e7175e6dc5e95babaa42af.
- Packaged native binary SHA-256: 0a183d80a4b2b5e1d65f466ad86547600127105bc4c200d0232cb9ec82f75b44. Package smoke reports manifest, standalone native binary, npm tarballs, Python wheel, and React SSR all passing; inference requests: zero.
- Fresh benchmark on that binary: 20 cold process-and-scan samples, p50 16.01 ms / p95 34.28 ms; 200 warm stdio snapshots, p50 0.35 ms / p95 0.48 ms; 60.40-second idle run, peak RSS 8.65 MiB, 0.0% one-core CPU. All recorded performance targets passed.

## Historical Linux and macOS status — September 25, 2026

The WSL2 run at the preceding clean source checkpoint 11d1926 passed the full Rust workspace and TypeScript/Python SDK, generated-contract, React, helper/broker, and deterministic integration checks up to the browser demo. The large-log test has since been made synthetic and passed on Windows, so the prior WSL run does not verify that final test-fixture change. Playwright was initially missing from the fresh Linux environment. After installing the pinned dependency, Chromium downloaded, but Playwright returned EIO while removing its cache lock. WSL then failed to launch Ubuntu with Wsl/Service/CreateInstance/E_FAIL and later Wsl/Service/E_UNEXPECTED, including after terminating and shutting down the distro. No current-source Linux package, consumer smoke, or benchmark was produced. The old 991ec2b Linux artifact remains historical and does not qualify current source.

GitHub Actions run 36140653176 created Windows, Ubuntu, and macOS jobs but failed all three before any steps appeared. gh run view --log returned log not found, so the workflow service exposes no job output or external cause. Do not assign a billing or account explanation without evidence.

Windows has full current-source platform execution and release/package qualification. Linux has partial platform execution at the previous source revision but no current-source full or release qualification. macOS has no platform execution evidence. The workflow matrix lists windows-latest, ubuntu-latest, and macos-latest, but that is configuration only.

The remaining truthful gaps include provider-specific plan/usage values without documented sources, current-source Linux package qualification, macOS platform execution, live Gemini CLI/Antigravity qualification, and other original-plan scope marked incomplete in the implementation ledger. Tauri remains an integration guide rather than a separately qualified application. API billing remains separate from native subscription access.

No registry release, OS signing/notarization, main merge, live generation, or paid fallback was performed.
