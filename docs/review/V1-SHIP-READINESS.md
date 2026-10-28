# OriginLM v1 ship-readiness report

Status: **source-preview implementation and local qualification pass; latest hosted
CI qualification is pending**. See [draft PR #55](https://github.com/thomaslittle/local-ai-kit/pull/55).
No registry publication, signing, deployment or live inference was performed.
Thomas stopped video work; existing media is retained without further qualification.

## What changed

The [principal review ledger](V1-PRINCIPAL-REVIEW.md) records 42 findings and their
changes. The branch integrates distribution through status/tray and web bridge,
then hardens owned helper lifecycle, protocol errors, grants, bounded reads,
inventory replay, saved prices, installer cleanup and signed updater activation.
The updater now installs the signed bytes and preserves a monotonic version floor.
Node/Python broker and bridge processes share a minimal environment while preserving
standard OS directory overrides. Rust, Node and Python handle long Unix temporary
paths without exceeding socket limits.

The control center prioritizes allowances, resets, models, issues and observed
costs. Exact estimates require token counts and an editable, dated, sourced price
row; missing evidence stays unknown. GPUI exposes the same cost scope, readable
chart scale/order and the in-app mini-view. Shared brand/input tokens and one
licensed WOFF2 asset connect the web, site and native themes.

The site uses real example-mode surfaces, equal-weight See / Build messaging,
prerendered content, static fallbacks and a desktop hero that stays visible while
scrolling separates all five layers. Independent finish review findings are all
resolved. [Current captures](screens/README.md) include 1440px and 390px web/site
views, collapsed/exploded states and native Xvfb evidence.

## Executed locally

The complete `scripts/verify.py --profile all` pass ran 26 commands successfully.
[Machine-readable evidence](V1-VERIFICATION.json) preserves counts and the benchmark's
own source/binary identity.

| Area | Result and scope |
|---|---|
| Rust | 681 tests, workspace/all-targets/locked; clippy all-targets/all-features with `-D warnings`; formatting passes |
| JS SDK/selection | 28 selector-core, 90 TypeScript SDK and 3 web-SDK tests pass |
| Python | 40 SDK tests pass |
| React and app | 14 packaged React tests, 26 app unit checks, 16 browser-flow checks pass |
| Accessibility | 34 automated app/package WCAG-tagged checks pass; route checks reject nested main landmarks |
| Native integration | 5 Node, 3 Python and one broker-bridge integration check pass against actual native helper transports |
| Contracts and hygiene | Generator suites, source-generated output checks, typecheck, project hygiene and React Doctor pass; no Doctor issues |
| Local packages | Fresh offline Node tarballs auto-start/close the packaged helper; installed Python platform wheel auto-starts/closes its bundled helper; fresh packed Rust-source consumer starts/scans/drops its in-process runtime |
| Electron packaging | Linux Electron 45.1.0 ASAR with packed SDK/native dependency auto-starts and closes its physical unpacked helper under private Xvfb; missing unpacked binary has a typed error |
| Service removal | 22 packaging tests pass or skip appropriately: 21 pass locally, one Windows-only fixture awaits Windows CI. Four Unix uninstall cases cover canonical registrations, XDG/macOS paths, retention/purge and home protection |
| Service installation | Throwaway archive/checksum install, real installed helper startup/shutdown, ordinary removal and purge pass. Service-manager calls are stubbed; actual user-service registration is not claimed |
| Native UI | Linux GPUI build/clippy and isolated Xvfb captures pass; examples only |
| Site | Production build/prerender, two scroll/static/no-script behavior tests and Hallmark render sweep pass. Site and all eight app routes have 0 sloplint failures; recorded finish answers and the existing copy-control hit-target waiver remain explicit |
| Lighthouse | Core categories: desktop 100/100/100/100; mobile 95/100/100/100 (performance/accessibility/best practices/SEO) |

The lazy scene plus Three.js chunk totals **143,847 bytes gzip**. The local release
helper is about **23.4 MiB**. On this Linux host with synthetic profiles, cold
process/scan p95 was **2.20 ms**, warm stdio snapshot p95 **0.26 ms**, and the
60-second idle sample peaked at **13.08 MiB RSS** with no measurable CPU use.
These measurements are one host's evidence, not universal hardware promises.

## Hosted qualification

An earlier checkpoint passed Linux full verification and all five artifact-preparation
targets. Its macOS full-verification job never acquired a hosted runner and was
cancelled for ARM capacity constraints. The main matrix now pins Ubuntu 24.04,
Windows 2025 and macOS 15 Intel; ARM Mac artifact preparation remains. The latest
head must finish these checks before CI qualification is complete. The preceding
head exposed two fixture errors: a macOS `/var` symlink expectation and a shadowed
PowerShell stub variable. Both assertions are corrected; their hosted rerun is pending.

## Honest limits and owner actions

- Packages remain unpublished previews. Public release still needs a qualified
  portability baseline, registry credentials and signing configuration. Conservative
  Linux wheel tags do not establish a manylinux baseline.
- macOS/Windows GUI execution, real launchd/systemd/task registration and hardware
  frame rates are not established by Linux Xvfb or fixture tests.
- The Tauri command-build attempt encountered missing local GTK/WebKit development
  packages. Its guide is an existing-app fragment; it is not a qualified standalone
  Tauri scaffold. Core Rust and Node host patterns are executed separately.
- A generic hosted website cannot yet call a page-facing extension bridge. The
  implemented browser paths are registered extension pages and the helper's own
  paired loopback dashboard.
- The mini-view is inside the app; no OS tray is available in this GPUI build.
- Provider operations remain fixture-tested unless the support matrix says otherwise.
  No live account, paid operation or inference was used in this review.
- Thomas owns eventual publication/signing identities, domain and hosting decisions.
  None are represented as configured or complete.
