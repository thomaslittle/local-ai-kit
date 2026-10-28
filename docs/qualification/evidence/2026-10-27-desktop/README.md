# Desktop app visual pass — 2026-10-27

Native GPUI desktop client for Local AI Kit, built on GPUI Kit 0.8.1 with the
`gpui-fast` feature (GPUI Fast 0.1.12, one dependency graph). Screens captured
from the real application window under X11 on Xvfb with software rendering;
the same binary was also launched on GNOME Wayland. All screenshots except
`Playground_Live_Turn.png` include the final in-app title bar; the live-turn
capture predates that chrome fix.

## Evidence

| File | What it shows |
| --- | --- |
| `Overview.png` | Access summary, constrained allowance windows, next reset, 50+ locally collected observations, province list, open issues |
| `Providers___accounts.png` | Connection list with plan/auth evidence and a detail pane with routes and redacted identifiers |
| `Models.png` | Search, favorites, native IDs, capabilities, provider-native options, route and billing |
| `Usage.png` | Interactive remaining-allowance line, 24h/7d/30d tabs, per-window series, reset schedule, tabular alternative |
| `Connected_apps.png` | Live broker clients, permissions as reported, admin credential in use |
| `Sessions.png` | A completed Playground session with usage |
| `Playground.png` | Explicit model/route/billing summary and empty transcript |
| `Playground_Live_Turn.png` | A real read-only turn streamed through the window (`ready`, 4 output tokens) |
| `Diagnostics.png` | Health, plain-language issues, evidence, remediation, redacted export |
| `Settings.png` | Theme, credential and socket paths (no secrets), refresh cadence, about |
| `Light_Overview.png` | The paper-light theme |
| `Light_Palette.png` | Command palette over the light theme |
| `Small_Overview.png` | 1100×700 laptop-size layout |

## Commands

```sh
cargo build -p lai-desktop
Xvfb :99 -screen 0 1600x1000x24
DISPLAY=:99 WAYLAND_DISPLAY= ./target/debug/local-ai-kit-desktop \
  --token-file ~/.local/state/local-ai-kit/fedora-shared/control-admin-token.json
DISPLAY=:99 import -window root shot.png
```

## Live turn disclosure

One Playground turn was executed through the UI: Claude Code Default profile,
workspace `default` → `/home/tomlit`, `permissionMode: read_only`,
`allowPaid: false`, prompt “reply with exactly ready”. The window streamed
`ready` and reported 4 output tokens. An earlier attempt from the previous
session also consumed one short turn. No other inference, probe, account change
or revocation was performed during this visual pass.

## Status

- **Implemented and compiled:** the whole app and all pages above.
- **Executed:** broker reads, palette filtering, navigation, theme switching,
  search filtering, scrolling, app listing, one live Playground turn.
- **Visually verified:** every screenshot above (Linux/X11, software renderer).
- **Not verified:** Windows, macOS, a hardware-GPU frame-rate measurement,
  screenshots on Wayland, and live cancellation mid-stream.

## OpenCode 2 metadata (same day, after the visual pass)

OpenCode 2.2.0 is authenticated in the CLI but the adapter only accepted 1.x.
The probe now reads the running OpenCode 2 background service through its
owner-only `service.json` and loopback API: provider/model inventory,
`/api/integration/opencode` and `/api/experimental/usage/limits`. A staged
broker proved `auth authenticated (observed)`, 75 models, and rolling/weekly/
monthly allowance with reset times. The desktop shows OpenCode as authenticated,
catalogues its models, and deliberately does not offer them in the Playground
because OpenCode 2 text execution is not wired.

## UI polish pass

Uppercase section eyebrows, dot-joined metadata and decorative status dots were
removed in favour of sentence-case labels, stacked provider/profile names and
explicit states. Usage charts now plot the share used, show a delta, and fall
back to a current figure when the series is flat; the per-window comparison is
a compact bar list rather than a crowded chart. Auth states distinguish
verified from cache-only sessions. Cursor's read-only local cache is enabled;
it reports plan and cached sign-in without claiming verification or inventing
models/usage. Screenshots in this directory were refreshed after the pass.

## Workspace and page-by-page QA pass

The shell now has a drag-resizable sidebar, a collapsed icon rail, persisted
width/collapse state, `Ctrl+B`, a title-bar toggle and a Reset layout action in
Settings and the command palette. Palette and toast entrances use short,
reduced-motion-aware fades. The overview chart and the per-window comparison
were merged into one card so the list can no longer paint outside its row.

QA covered every page at 1440x920, 1280x820 and 1100x700, plus both themes and
the collapsed rail. Exercised: sidebar drag/collapse/reopen/restart persistence,
reset layout, palette navigation and layout actions, theme switch, model search,
usage chart/table toggle, provider selection, scrolling, and scrolling long
detail content. Remaining gaps are listed in the crate README.

## Provider marks, guide and animated workspace

Provider glyphs now come from the same monochrome SVGs used by the React
showcase (`apps/demo/src/assets/providers`; LobeHub MIT, Cursor from Simple
Icons CC0). They appear in providers, quota cards, models and the Playground.
A new **How to use** page documents the broker JSON-RPC surface with copyable
examples, a TypeScript SDK snippet, and an in-app component/API gallery whose
links open the system browser.

The sidebar width now animates over 170ms (reduced-motion aware), collapses to
the icon rail and persists both width and collapse state. An embedded WebView
was evaluated and is not buildable on this host: WebKitGTK/GTK development
packages are missing and the GPUI webview supports X11/XWayland only, while
this session is Wayland-native.

Evidence: `How_to_use.png`, `Guide-gallery2.png`, `Sidebar_collapsed.png`,
refreshed `Overview.png` and `Providers___accounts.png`.

## OriginLM rebrand (same day, final)

The desktop product is now **OriginLM**, using the supplied asset pack. The
sidebar header, the collapsed rail and the Settings mark use the app icon: the
graphite symbol on the amber `#EBA31A` tile, with no wordmark and no tagline.
Both themes use graphite `#231F20`, amber `#EBA31A` and off-white `#FAF9F5`.
Red Hat Display (variable, OFL 1.1) is bundled and registered before the first
window; the mono face stays Adwaita Mono. The supplied horizontal lockups stay
in `crates/lai-desktop/assets/brand/` for exports.

Provider marks were enlarged into tinted, ringed tiles: 52px in Overview quota
and next-reset cards, 48px in the Providers list, 72px in the provider detail
header and 44px in the model detail header. Quota cards lead with the provider
name and show the provider-native window label underneath.

**Cursor auth correction.** The broker used to derive `needs_auth` from any auth
state that was not `authenticated`, so a cache-only Cursor session produced a
"Sign-in required" issue even though the local account cache is signed in. The
broker now reports `metadata_cached` when an account cache exists but the auth
state is unknown, and unknown or unchecked sessions surface as an informational
"Sign-in not checked" instead of telling the user to sign in. Cursor keeps its
"Signed in, cache only" chip and banner; nothing claims verified access.

**Fixture/platform evidence:** screenshots below are the real app on Linux/X11
(Xvfb, software renderer), reading the live local broker. The demo screenshots
are the built React showcase served by the loopback fixture
(`node apps/demo/server.mjs --fixture`). Windows/macOS are not verified. No
provider account action was performed for this pass.

| File | What it shows |
| --- | --- |
| `OriginLM_Overview.png` | Dark theme, logo-only shell, large provider tiles |
| `OriginLM_Providers.png` | Provider list and detail with large marks |
| `OriginLM_Models.png` | Model list and 44px provider tile in the detail header |
| `OriginLM_Cursor_Providers.png` | Cursor verified through `cursor-agent`: Premium, authenticated, 270 models |
| `OriginLM_Cursor_Detail.png` | Cursor detail with CLI evidence and inferred subscription billing |
| `OriginLM_Usage.png` | Usage page including the Cursor "Included usage" window |
| `OriginLM_How_to_use.png` | Broker API guide, off-white theme |
| `OriginLM_Guide_gallery.png` | Native component gallery, amber progress |
| `OriginLM_Light_Overview.png` | Off-white theme |
| `OriginLM_Light_Providers.png` | Off-white theme, provider list |
| `OriginLM_Settings.png` | About block with the OriginLM app icon |
| `OriginLM_Narrow_Overview.png` | 1100x700 layout |
| `OriginLM_Sidebar_collapsed.png` | Collapsed icon rail with the app icon |
| `OriginLM_Demo_Overview.png` | React showcase, dark theme, amber tile in the sidebar |
| `OriginLM_Demo_Selector_lab.png` | Selector lab with the OriginLM palette |
| `OriginLM_Demo_Connections.png` | React connections page, dark theme |
| `OriginLM_Demo_Light.png` | React showcase, off-white theme |

Brand sources and licences are recorded in `crates/lai-desktop/README.md`,
`apps/demo/src/assets/brand/README.md`, `apps/demo/src/assets/fonts/README.md`
and `crates/lai-desktop/assets/brand/PROVENANCE.md`.

## Cursor: CLI metadata and opt-in allowances

Cursor was moved off the cache-only path. The adapter now probes the installed
`cursor-agent` CLI (`status`, `about`, `models`, `--version`) under
`--allow-metadata cursor`, so the app shows a verified session, the plan tier and
the native model catalog instead of "signed in, cache only" and zero models.

Allowance percentages are not exposed by the CLI. They come from Cursor's own
dashboard service and require the separate `--allow-usage cursor` grant, which
also requires the metadata grant. The adapter reads the session token Cursor
already stored in `state.vscdb` into a zeroizing buffer, uses it only for
`GetCurrentPeriodUsage` and `GetPlanInfo`, and never logs, serializes, exports or
writes it. Evidence kind is `community_endpoint`; the values are Cursor-reported,
not inferred. Without the grant the cached identity/plan is shown as before.

Cursor is still not offered in the Playground or as an execution route.
