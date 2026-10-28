# OriginLM landing page

Standalone Vite + React + TypeScript site for OriginLM: the local control center for AI usage and the dependency that puts the user's own AI in an app.

`apps/site` is deliberately **not** part of the root npm workspace. It has its own `package.json` and `package-lock.json`, so changes here never touch the root lockfile owned by other work.

## Run it

```bash
cd apps/site
npm install
npm run dev -- --host 127.0.0.1 --port 5174
```

Open <http://127.0.0.1:5174/>. The dev server binds to loopback only; `strictPort` is on and it never uses 5173 (that is the desktop demo).

Production build and preview:

```bash
npm run build
npm run preview -- --host 127.0.0.1 --port 5174
```

`npm run build` emits static files to `dist/`. There is no server-side runtime and no external request at runtime.

## Structure

- `src/styles/tokens.css` — OriginLM palette and type ramp, copied from `apps/demo/src/styles.css`.
- `src/styles/base.css`, `src/styles/sections.css` — layout, components, sections (plain CSS, no utility framework).
- `src/sections/*` — one file per page section.
- `src/scene/*` — the lazy 3D hero (React Three Fiber, drei, postprocessing).
- `src/data/*` — layers, provider support matrix, integration snippets, video paths.
- `src/assets/brand`, `src/assets/fonts` — brand artwork and Red Hat Display, copied from `apps/demo` with their notices.
- `public/hero-poster.svg` — procedural poster, also used for reduced motion and before the 3D chunk loads.
- `public/media/` — rendered film, WebM, poster and the 8 s loop used as the hero fallback asset.
- `review/` — Playwright capture script and the latest 1440 px / 390 px screenshots.

## Hero 3D

`HeroScene` is lazy-loaded after first paint. The scene is built procedurally (no downloaded models): rounded glass panels, a dashboard panel, provider nodes, and light packets travelling through the stack. Scroll drives the spring-based explode; an idle loop also separates and reassembles the layers. Hover (mouse) or tap (touch) selects a layer.

Fallbacks:

- `prefers-reduced-motion: reduce` → static poster, no three.js mounted.
- no WebGL → static poster.
- before the lazy chunk loads → poster with a quiet loading pulse.

Bundle (production, gzip, measured with `vite build` + `gzip -c`):

| Chunk | Gzip |
| --- | ---: |
| three-core | 181.7 KiB |
| fiber | 53.6 KiB |
| postprocessing | 17.5 KiB |
| HeroScene (incl. r3f-post and geometry) | 10.9 KiB |
| drei | 0.4 KiB |
| **3D total** | **264.1 KiB / 270.5 kB** |

The page shell (`index` + `react-vendor`) is separate and loads before the 3D chunk.

For first paint and battery, the page shows the static poster instead of mounting three.js when
the viewport is under 900 px, the visitor prefers reduced motion, or the WebGL renderer is a
software rasterizer (headless CI, some VMs). A real desktop GPU gets the full scene. Add `?3d`
to the URL to force the scene for review captures.

## Accessibility and Lighthouse

Production preview, Lighthouse 12 (Chrome headless):

| Run | Performance | Accessibility | Best practices | SEO |
| --- | ---: | ---: | ---: | ---: |
| Desktop | 100 | 100 | 100 | 100 |
| Mobile | 96 | 100 | 100 | 100 |

The mobile run uses the static poster, so Total Blocking Time stays at 0 ms. Reports are kept
in `review/lighthouse-desktop.json` and `review/lighthouse-mobile.json`.

## Product film

The promo generated with fframes lives in `media/video` (see its README). The site embeds the
1080p H.264 film with a WebM alternative and a poster frame:

- `public/media/originlm-promo.mp4` — 52.6 s, 5.9 MB, silent.
- `public/media/originlm-promo.webm` — 52.6 s, 2.7 MB, silent.
- `public/media/originlm-promo-poster.jpg` — frame at 27 s.
- `public/media/originlm-loop.mp4` — 8 s seamless stack loop (hero fallback asset).

Set `hasVideo` in `src/data/video.ts` to swap the player back to the "in post-production"
placeholder if the media is removed.

## Review

```bash
# with the dev server running on 5174
node review/capture.mjs
node review/sections.mjs
```

These use `playwright-core` with the system Chrome (or Playwright's cached headless shell) and write screenshots plus a short hero recording under `review/`. They do not move the real cursor, take window focus, or touch 5173.

## Content sources

- Positioning: `/home/tomlit/lai-positioning.md` and `docs/design/ADR-002-dependency-first-distribution.md`.
- Provider claims: `docs/adapters/SUPPORT-MATRIX.md`, condensed in `src/data/providers.ts`.
- Install package names: the in-repo package names (`@local-ai-kit/sdk`, `local-ai-kit`, `local-ai-kit-preview`). They are pre-release; the site says so.
- Install domain: `originlm.dev` is a placeholder until deployment is approved. See `NOTES.md`.
