# ADR-002: Dependency-first distribution

- **Status:** accepted; phased implementation
- **Date:** 2026-10-27
- **Related:** [ADR-001](ADR-001-shared-runtime.md), [browser extension](../integration/browser-extension.md), [pairing](../integration/pairing.md), [shared broker](../integration/shared-broker.md)

## Context

Local AI Kit today ships as a runtime, a CLI/broker, a desktop app and a
developer showcase. An application that wants local AI must find the helper,
install or bundle it, start it, and keep it current. That is a deployment
problem disguised as an integration problem: every consumer re-invents
discovery, startup, token handling and upgrades, and gets it slightly wrong.

The standalone app, the broker and the control center are correct for people who
want them. They must keep working exactly as they do when run on their own. The
change is at the *edge*: installing a dependency should be enough for an app to
use local AI, with no separate install and no service required.

Two constraints shape the design:

1. Provider work (discovery, probes, native execution) must stay out of the
   app's render loop, and a crashing provider client must not take the app down.
2. Safety rules do not change because distribution did: imports never scan or
   read credentials, discovery is staged, grants are explicit, secrets are never
   exported, and transports stay owner-scoped or loopback-only.

## Decision

### D1. The dependency is the product boundary

Every supported ecosystem ships a package that contains (or resolves) the helper
binary for its platform:

| Ecosystem | Package | Shape |
| --- | --- | --- |
| Rust | `local-ai-kit` (facade over `lai-kit`) | crate, links the runtime |
| npm | `@local-ai-kit/sdk` + `@local-ai-kit/cli-<platform>` | optional platform packages |
| Python | `local-ai-kit` wheel | per-platform wheels |

The standalone app, service and control center stay as they are. A dependency
never tries to install them, and never requires them to exist.

### D2. Resolution order: shared broker, then private helper

`LocalAi::start()` (Rust), `createLocalAI()` (npm) and `Client.auto()` (Python)
resolve in this order:

1. **Shared broker.** If a live broker is reachable through the owner-only
   descriptor/socket, connect and use it. This is the mode where several apps
   share one scanner, one cache and one grant table.
2. **Private helper.** Otherwise start a helper owned by this process. In Rust
   that is a bundled or developer-supplied binary via `LAI_HELPER_PATH`;
   in npm/Python it is the platform binary from the package. The helper speaks
   the same stdio JSON-RPC surface, is tied to the parent's lifetime, and never
   publishes a reusable token file.
3. **In-process runtime (Rust only).** If no helper binary exists at all, run
   `lai-kit` on a dedicated background thread inside the app process. This keeps
   the promise that a missing standalone install is never an error, at the cost
   of sharing the app's process for broker scheduling and cache work.

The rule is: **prefer process isolation; degrade to in-process only when there is
no binary to isolate.** The mode is observable (`Mode::{Shared, Child,
InProcess}`) so an app can show it, log it, or refuse to run.

Why not in-process first? Provider clients are already child processes, but the
broker also owns SQLite, timers, backoff and notification delivery. A panic, a
blocking adapter or an OOM in that surface should not be able to kill the host
app. The stdio child costs one short-lived process and gives crash containment,
clean shutdown and a stable protocol boundary. In-process exists for the case
where the package cannot carry a binary (for example a `no-default-features`
build), not as the default.

A shared broker that exists but rejects the token is **not** silently replaced by
a private helper: that is a configuration error the app must surface. Only
"nothing is listening" falls through.

### D3. npm mirrors esbuild and Biome

`@local-ai-kit/sdk` declares `optionalDependencies` on:

- `@local-ai-kit/cli-linux-x64`
- `@local-ai-kit/cli-linux-arm64`
- `@local-ai-kit/cli-linux-x64-musl` (Alpine)
- `@local-ai-kit/cli-linux-arm64-musl`
- `@local-ai-kit/cli-darwin-arm64`
- `@local-ai-kit/cli-darwin-x64`
- `@local-ai-kit/cli-win32-x64`
- `@local-ai-kit/cli-win32-arm64`

When the current platform has no packaged binary, the SDK fails with
`PLATFORM_UNSUPPORTED` and the install-page URL, never with a generic runtime
error. A caller-supplied transport or `LAI_HELPER_PATH` still works.

Each package contains exactly one binary and a small manifest. The SDK resolves
`require.resolve('@local-ai-kit/cli-<platform>/package.json')`, verifies the
binary is executable, and spawns it with pipes. Installation is a normal
dependency update; nothing is downloaded at import time, and no postinstall
script runs. Works in Node, Electron's main process and Bun. Renderer processes
get the web bridge instead (Phase 5).

`LAI_HELPER_PATH` overrides resolution for kit development and for CI fixtures.
It is honoured before any packaged binary and is never read from the network.

### D4. Python wheels carry the binary

One wheel per platform (`manylinux`, `macosx`, `win_amd64`) includes the helper
under `local_ai_kit/_bin/`. `Client.auto()` uses the shared broker first, then
the bundled binary, then raises a single actionable error describing both
attempts. A source checkout may point at `LAI_HELPER_PATH`.

### D5. Publishing is a release-engineering task, not a side effect

Version-tag release workflow publishes crates, npm packages (including platform
packages), wheels and signed, checksummed GitHub bundles, with provenance
attestations. **Nothing is published until the maintainer has approved the
release and supplied the registry credentials.** The workflow is prepared and
dry-run first; a dry run uploads nothing and prints exactly what would be
published. Consumers get updates as ordinary dependency bumps; a Renovate or
Dependabot configuration is recommended per ecosystem rather than a custom
updater.

### D6. A one-line install exists for people who want the service

`curl -fsSL https://<site>/install.sh | sh` (macOS, Linux) and
`irm https://<site>/install.ps1 | iex` (Windows, plus a winget manifest) install
one binary into a user-level location and register a **user** service:

- launchd `LaunchAgent` on macOS,
- `systemd --user` unit on Linux,
- Task Scheduler logon task on Windows.

No admin rights, no system-wide paths. The service idles near zero CPU, restarts
on crash, starts on demand, and self-updates in the background with checksum and
signature verification. Each installer ships a matching uninstall command that
stops and removes the unit, its binary and its state descriptor.

### D7. Status stays out of the way

Four surfaces, one source of truth:

1. **SDK status:** `ai.status()` returns mode, providers and connected apps;
   sites render a small badge ("Local AI connected · Claude, Ollama" /
   "Not detected" with an install link).
2. **Tray/menu bar:** on by default, easy to disable; lists connected apps and
   models, Pause and Open Control Center. No popups.
3. **Notifications:** only for decisions the user must make (new app approval, a
   provider signed out).
4. **`local-ai-kit status`:** the same information in the terminal.

### D8. Plain websites use the extension first

The primary bridge for web apps is the existing browser extension plus Native
Messaging host: no port is opened, each origin needs a one-time approval stored
as a revocable grant, and the page receives results only, never credentials.

Loopback HTTP with pairing remains the fallback: strict origin allowlist, a
one-time pairing code, short-lived scoped tokens, and Chrome Private Network
Access handled explicitly. The web SDK detects the extension first, then
pairing, then shows the "Connect your local AI" flow.

### D9. Version compatibility is negotiated, not assumed

The SDK and the broker exchange `runtime.handshake` and compare protocol
versions. If a running shared broker is older or newer than this SDK can talk
to, the SDK closes that connection, falls back to its private bundled copy, and
records `FALLBACK_PROTOCOL_MISMATCH` with both versions in the status surface.
A version mismatch is never a fatal error and never steals the shared broker's
work. A shared broker that rejects the *grant* is still fatal (D2).

### D10. Host packaging gotchas

- **Electron:** the helper must be listed in `asarUnpack`; a binary inside an
  asar cannot execute. The SDK documents the exact glob and fails with a
  readable error when it detects an asar path without an unpacked sibling.
- **Tauri:** both routes are documented — link the crate (preferred: in-process
  or child through the facade) or ship the helper as an `externalBin` sidecar.
  The sidecar route must use the target-triple suffix Tauri expects.
- **Flatpak, Snap and the Mac App Store:** these sandboxes cannot start host CLIs
  or read provider metadata outside the sandbox. The client detects the sandbox
  (env markers, bundle paths, missing host socket) and reports an explicit
  `sandboxed` status, never "no providers found". It does not attempt to escape
  the sandbox.

### D11. Signing and provenance

| Platform | Artifact signing | Secrets needed (never purchased here) |
| --- | --- | --- |
| macOS | Developer ID Application signing + notarization for the helper in npm/wheel/bundle packages and for the installer | `APPLE_CERTIFICATE_P12`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`, `APPLE_APP_SPECIFIC_PASSWORD`, `APPLE_TEAM_ID` |
| Windows | Authenticode signing of the helper and installer | `WINDOWS_CERTIFICATE_PFX`, `WINDOWS_CERTIFICATE_PASSWORD`, timestamp server URL |
| Linux | SHA-256 checksums plus minisign or cosign signatures | `MINISIGN_SECRET_KEY` (or `COSIGN_PRIVATE_KEY`) and its password |
| npm | `--provenance` with GitHub OIDC | `NPM_TOKEN` only for the first publish/2FA fallback |
| PyPI | Trusted Publishing (OIDC) preferred, token fallback | `PYPI_API_TOKEN` only if OIDC is unavailable |
| crates.io | `cargo publish` with a scoped token | `CARGO_REGISTRY_TOKEN` |

Gatekeeper and SmartScreen block unsigned binaries, so an unsigned release is a
dry-run artifact, not a published one. This ADR lists the certificates and
secrets; it does not buy or generate them.

### D12. Several private copies at once

Without a shared broker, each application may run its own private helper. That
is intentional isolation, not a bug: each copy carries its own grant file keyed
by app identity, and no copy can see another's accounts or grants. Duplicate
discovery work is bounded by the fact that only the first launch of a given app
probes; the rest read its state. The install page explains that installing the
background service converges these copies under one control center, with one
grant table and one set of provider probes.

### D13. Clean uninstall

The uninstall command removes, in order: the user service (launchd agent,
systemd user unit or Task Scheduler task), tray autostart, Native Messaging
manifests for every browser it installed, and the helper binaries. User data
(state database, history, grants) is kept by default and removed only with an
explicit `--purge` (Windows: `-Purge`). Uninstall is idempotent, never requires
admin rights, and never touches another user's files.

### D14. Privacy statement

The install page and the README carry the same statement as SECURITY.md: no
telemetry, no remote relay, nothing leaves the machine; provider credentials are
read only by the native client that already owns them and are never exported;
grants are per app and revocable; the SDK reports the same facts in-app. The
statement is generated from one source so they cannot drift.

### D15. Size budget

The helper ships inside every consumer's installer, so its size is a product
constraint:

- release builds use `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"`,
  `panic = "abort"` where the platform allows it;
- the release workflow records the stripped binary size per target and fails the
  dry run if a platform exceeds the agreed budget (initial target: 40 MB per
  desktop target, 60 MB for musl static);
- the measured sizes are reported in the release notes and on the install page.

### D16. Tests for resolution

Phase 1 needs end-to-end coverage of each resolution path, not only unit tests:

1. shared broker present and accepting the grant;
2. shared broker absent, bundled helper starts;
3. shared broker present but protocol-incompatible, fallback recorded;
4. `LAI_HELPER_PATH` overrides the packaged binary and a bad override fails;
5. sandboxed environment reports `sandboxed`, not "no providers".

These run headless against fixture helpers. They must never take over the
developer's mouse, keyboard, window focus or an interactive session; no test may
raise a real window.

### D17. Consumers migrate in their own repositories

crntly-desktop, remotif and xhw_haiwire are migrated by their own maintainers in
their own repos, using the published crate and auto mode. This repository
prepares a short migration note per app; it does not edit sibling repositories.

## Phases

| Phase | Deliverable | Exit criteria |
| --- | --- | --- |
| 1 | Rust facade, npm platform packages, Python wheels, `LAI_HELPER_PATH`, `Mode` | A fixture helper proves shared → child → in-process resolution; packaged binary resolution is tested without downloading anything |
| 2 | Release workflow, provenance, dry run, dependency-bump guidance | Dry run lists every artifact and uploads nothing; credentials documented |
| 3 | Install page in the docs site and native app | Tabs for npm, pnpm, bun, yarn, cargo, pip and quick starts for Tauri, Electron, GPUI, Node, Python |
| 4 | Install/uninstall scripts, user services, tray, notifications, `status` | Install, enable, crash-restart, self-update verification and uninstall are exercised on each platform or explicitly marked unqualified |
| 5 | Extension bridge, pairing fallback, web SDK detection | Origin approval is revocable; no open port on the extension path; PNA behaviour documented |
| 6 | Migration notes for crntly, remotif, haiware | Each note names the file to replace and the auto-mode call |
| 7 | Landing page and product video | Lighthouse 90+ performance/accessibility, reduced-motion honored, hero fallback, video renders committed or linked |

Phase 7 is the public home. It is a new `apps/site` (Vite + React +
TypeScript, static output, separate from `apps/demo`) with a premium dark
surface, crisp type, generous spacing and subtle motion, and a Lighthouse target
of 90+ for performance and accessibility. It contains: hero with copy-ready
install tabs (npm, pnpm, bun, yarn, cargo, pip, curl/PowerShell); scroll-driven
"discover → grant → session" steps; integration tabs (Tauri, Electron, GPUI,
Node, Python, web) with real minimal code; a provider list generated from
`docs/adapters/SUPPORT-MATRIX.md` with honest per-operation limits; security and
privacy with a threat-model link; a status-badge and tray mock; the product
video; docs/GitHub/changelog links and a footer.

The hero is a procedural React Three Fiber scene (with drei and postprocessing,
light bloom and depth of field): a single stacked "device" that comes apart into
labeled glass layers — app, SDK, transport, runtime, adapters, provider nodes —
with light packets traveling down and back up. Hovering a layer explains it.
Reduced motion and missing WebGL get a static poster. The 3D bundle stays under
about 300 KB gzipped plus models, and the hero text paints before it loads.

The product video lives in `media/video` as an fframes project (Rust + SVG, GPU
render): 45–60 s at 1080p and 4K telling the same exploded-layer story, plus a
6–10 s seamless loop for the hero fallback and 16:9/1:1/9:16 social cuts. Export
MP4 (H.264), WebM and a poster; commit the source and put oversized renders in
releases or LFS. Music or voiceover must be licensed or omitted. Nothing is
posted publicly, no domain or paid hosting is bought, and previews use free
tiers only with the maintainer's approval.

Phases ship as separate branches and pull requests, each merged only after CI is
green. Phase 2 never publishes on its own; publication is a maintainer action.

## Consequences

- Applications stop shipping their own client, token plumbing and startup
  policy; they declare a dependency and read a mode.
- The broker gains a second supported lifetime (private stdio child) that
  behaves like the existing parent-owned mode, with no reusable descriptor.
- Two binaries exist on disk in some installs (the app's copy and the service's).
  They are independent by design: the shared broker is only used when it is
  already running, never installed on demand.
- In-process mode is the weakest isolation and is documented as such; it is the
  fallback that makes "no separate install" true, not the recommended mode.
- Publishing becomes a gated, auditable release step with provenance, not a
  developer convenience.

## Open questions

- Whether the shared broker address should be a well-known socket name
  (`local-ai-kit-user`) or the existing descriptor path; the facade will accept
  both, preferring the descriptor when it exists.
- Whether npm platform packages ship a single binary or a signed archive; a bare
  binary keeps esbuild-style resolution simple, a signed archive would need a
  verification step at runtime.
- Which service manager owns self-update: the helper or the tray process. The
  current lean is the helper, with the tray only reporting.
