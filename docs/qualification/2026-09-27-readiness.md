# Readiness review — September 27, 2026

**Qualified implementation source:** `0c8213842be5b72ea7614b51028a24912b1bdabd` on `feature/initial-implementation`. Both distributions were built from a clean checkout of this commit. A later documentation-only commit records these results; its SHA is not the binary source SHA.

**Readiness:** the supported developer-alpha implementation passed full and release verification on Windows and Ubuntu/WSL2 and is ready to begin a development integration. macOS remains unqualified because GitHub prevented its job from starting. This is not a claim that every installed provider/version has passed live generation, nor that the full original cross-platform release scope is complete.

## Review and fixes

- Fixed OpenCode demo-server authentication: the owned server receives its actual password configuration, readiness requires authenticated success and anonymous 401, and provider credentials stay native.
- Fixed accountless Ollama/LM Studio selection and execution validation through the complete embedded API path. Local reachability is not labeled account authentication.
- Fixed completed-turn native suspension, resumed conversation continuation, and grant-revocation races during session/resume-token publication. Unpublished native sessions are explicitly closed.
- Fixed concurrent successful SDK metadata refreshes, preserved distinct executable installations, and grouped command shims within each installation directory.
- Finished real-data-only demo behavior, account freshness/error presentation, token non-disclosure, text encoding, and metadata-only execution guidance. The launcher uses a temporary helper copy so Windows rebuilds can run while the demo is open.
- Linux qualification exposed reused file IDs in incremental-log rotation. Added birth-time evidence, an opened-handle Unix identity, and a bounded leading fingerprint. Tests cover the original failure and a deterministic missing-identity case. No adapter currently interprets provider quota from logs.
- Fixed test-fixture startup timing assumptions and concurrent Windows pipe-name collisions without removing assertions or weakening explicit timeout tests.

A separate agent reviewed lifecycle/discovery changes and the final rotation correction. Its concrete findings were resolved. This is engineering review, not an independent security certification.

## Executed checks

| Check | Windows x64 | Ubuntu/WSL2 x64 |
|---|---:|---:|
| Full `verify.py --profile all` | Passed | Passed |
| Full `verify.py --profile release` | Passed | Passed |
| Rust workspace tests | 349 passed | 351 passed |
| TypeScript SDK | 63 passed | 63 passed |
| Python SDK | 26 passed | 26 passed |
| Selector core | 28 passed | 28 passed |
| React | 13 passed | 13 passed |
| Contract generation | 11 passed | 11 passed |
| Packaging tools | 17 passed | 17 passed |
| Showroom fixture service | 22 passed | 22 passed |
| Browser interactions | 14 passed | 14 passed |
| Accessibility checks | 32 passed | 32 passed |
| Live-launcher/auth/error/catalog unit checks | 10 passed | 10 passed |
| C# native HTTP consumer | Passed | Passed |

No Rust tests failed or were ignored in the final workspace runs. Both platforms passed formatting, strict all-target/all-feature Clippy, generated-file drift checks, project hygiene, adapter conformance, actual native helper/broker/Native Messaging checks, and standalone/installed consumers. Release verification repeats the full deterministic profile before the additional release checks. Native provider execution used controlled peers or loopback servers, not live model prompts.

Final runs used Rust 1.97.1, Node 22.20.0, isolated Python build environments (Windows 3.13, Linux 3.12), and isolated .NET SDK 8.0.425. Linux ran on kernel 6.6.87.2 under WSL2. Rust tests were serialized (`RUST_TEST_THREADS=1`) and builds bounded to two jobs; Windows and Linux qualification ran separately after shared-machine contention caused earlier fixture startup timeouts. No assertions were removed to obtain a pass.

Machine-readable evidence: [summary](evidence/2026-09-27/summary.json), [Windows full](evidence/2026-09-27/windows-all.json), [Windows release](evidence/2026-09-27/windows-release.json), [Linux full](evidence/2026-09-27/linux-all.json), and [Linux release](evidence/2026-09-27/linux-release.json). Earlier failed/interrupted runs remain in local `.verification/review-2026-09-27-*` logs; they are not successful qualification evidence.

## Clean packages and checksums

Four npm tarballs and the Python wheel installed into fresh consumers on both platforms. The consumers exercised the bundled native helper; React server rendering also passed. Bundle source, clean-state flags, all manifest file hashes, and archive sidecar checksums were verified again when collecting this report.

| Platform | Local archive | SHA-256 |
|---|---|---|
| Windows | `dist/release/local-ai-kit-0.1.0-alpha.1-x86_64-pc-windows-msvc-0c8213842be5.zip` | `fdb6ed3b08a77482a0111f19c6465ab8b7661b67a383cfebf645c345ccf14fb6` |
| Linux | `dist/linux-release/local-ai-kit-0.1.0-alpha.1-x86_64-unknown-linux-gnu-0c8213842be5.zip` | `7dddcd19a981f4944b00ba6264ca88df5a5f03043f45531bc20e4c174d1ba16d` |

Package evidence: [Windows](evidence/2026-09-27/windows-package.json), [Linux](evidence/2026-09-27/linux-package.json). `dist/release/latest.json` identifies the Windows bundle. Archives are local developer artifacts; nothing was published, signed, notarized, merged to main, or attached to a public release.

## Fresh performance measurements

Measurements use each newly built release helper, nine passive adapters, ten synthetic profiles, zero provider calls, and zero inference requests. They exclude the browser and external provider processes and are not latency guarantees.

| Metric | Windows | Linux/WSL2 |
|---|---:|---:|
| Cold process + scan p95 (20 samples) | 101.27 ms | 6.30 ms |
| Cached stdio snapshot p95 (200 samples) | 0.71 ms | 0.86 ms |
| Peak idle RSS | 11.20 MiB | 7.00 MiB |
| Sampled one-core idle CPU | 0.00% | 0.00% |
| Idle interval | 60.83 s | 61.82 s |
| Browser SDK gzip | 5490 bytes | 5490 bytes |

All recorded performance targets passed. Zero sampled CPU delta is not a claim of no possible CPU use. Full distributions and binary hashes: [Windows](evidence/2026-09-27/windows-performance.json), [Linux](evidence/2026-09-27/linux-performance.json).

## Live browser and provider evidence

The final Windows helper was launched and paired in the browser at `http://127.0.0.1:4499/`. It returned 137 route-specific model entries: two distinct Codex installations with 7 each, Claude Code with 11, Grok with 1, and OpenCode with 111. A repeated metadata check retained the authenticated profiles and catalogs. The earlier 130-entry run preceded the distinct-installation fix. These are not 137 unique models or entitlement guarantees.

Manual checks covered account refresh, provider/model selection, native options, compact/compare/mobile layouts, Usage, and metadata-only Playground controls. No horizontal overflow or mojibake was observed at 1440px/390px viewports; the browser console reported no errors. The automated suites cover all eight selectors, cancellation, approvals, failures, and native resume continuation without exposing a token or silently creating a replacement session.

Ollama and LM Studio were detected but offline. Gemini CLI and Antigravity lacked a compatible registered native runtime. Cursor returned only stale cached plan/identity evidence and no executable models. OpenCode's connected-provider report is not an independent check of every upstream credential. No live inference was sent and no subscription credentials were exported.

Implemented text-execution adapters: Codex, Claude Code, explicitly registered Grok/Gemini CLI/Antigravity ACP, configured OpenCode, Ollama, LM Studio, OpenAI API, Anthropic API, Gemini API, xAI API, and compatible HTTP APIs. Cursor execution remains unsupported. Native resumption is available only through the shared ACP path when the actual peer advertises `loadSession`; it is not inherited by other providers. See the operation-level [support matrix](../adapters/SUPPORT-MATRIX.md).

## Remaining boundaries

- **macOS:** neither compiled, fixture-tested, platform-executed, nor live-account-qualified in this review. The matrix includes macOS, Windows, and Ubuntu, but [run 36302338910](https://github.com/thomaslittle/local-ai-kit/actions/runs/36302338910) has no executed job steps. Its macOS check annotation states: “The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings”. Billing settings were not changed.
- **Provider facts:** absent plan/quota values remain unknown; local billing declarations and installed auth files do not prove subscription access. Live Gemini CLI/Antigravity and generation against real accounts remain unqualified. Their deterministic protocol paths are tested.
- **crntly:** use the [prompt-app integration guide](../integration/prompt-app.md) to connect the host, selectors, full Selection, streaming, cancellation, and app-owned scene validation. The common prompt input is text. A crntly/Tauri production host, scene schema, renderer, and supported live provider versions need app-specific integration qualification; they were not modified here.

Windows/Linux developer-alpha integration can proceed. The macOS execution gate remains externally blocked, so this report does not claim a fully qualified cross-platform release or completion of every original-plan item.
