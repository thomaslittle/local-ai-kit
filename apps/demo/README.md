# Developer showcase — implementation preview

A working React application consuming the public TypeScript SDK and selector core. It is explicitly synthetic by default and is **not** evidence of working native account detection.

From the repository root, with Node 22.16+ installed:

```sh
npm install --ignore-scripts
npm run build
node apps/demo/server.mjs --fixture
```

Open the loopback URL printed by the server (default port 4499). `--fixture` is required. Imports do not start a listener. No model account, provider credential, keyring read, paid API call, or native agent is used.

## Stack

React 19 + Vite + Tailwind CSS v4 with local shadcn-style primitives (`apps/demo/src/components/ui/`). The production bundle emits separate `index.html` / `assets/*.js` / `assets/*.css` files — never inlined — so the fixture preview server and the native helper can serve them under their `script-src 'self'; style-src 'self'` content security policy. `node scripts/build_demo.mjs` produces `apps/demo/dist`, which is also what `npm run demo` serves through the native helper.

## Working pages

Overview, connections/account evidence, models, per-pool usage, selector lab, session playground, and safe diagnostics/integration snippets.

All eight selectors share the same selection state: vertical provider rail, compact combobox, keyboard command palette, capability-first filter, comparison table, allowance-aware groups, task presets, and mobile drill-down. Exact route/account identity is retained. Options are rendered from descriptors, including missing and future effort choices; sensitive options require a visible confirmation. Favorites distinguish separate routes to identical model names.

The Playground is a chat interface adapted from the [shadcn chatbot-template](https://github.com/shadcn-ui/chatbot-template) patterns (see the reuse ledger in `THIRD_PARTY_NOTICES.md`): an empty state with suggestions, a message thread with a pending indicator, and a prompt composer with an inline provider-grouped model control. It exercises public SDK grants, selection validation, session creation, text events, approval denial/acceptance, and cancellation through the same permission gates as before — every turn still needs an explicit route, a consent checkbox, and a Run click. No inference occurs. Synthetic quota pools preserve shared identity and missing totals. A failed explicit live connection clears all synthetic data; live execution is disabled in this preview UI.

Large lists are bounded to 50 matching rows with search rather than full scrolling virtualization. Favorites are in-memory only. Persistent recents, production pairing, live provider integration, and the remaining architecture tasks are still pending.

## Tests

`node --test apps/demo/tests/*.test.mjs` runs 22 service and real local HTTP/SSE tests.

`python scripts/test_demo.py` runs 16 Playwright interactions against the actual UI and public SDK using an in-memory test transport. It bundles `apps/demo/tests/fixture-entry.tsx` offline with the pinned esbuild compiler, loads the real Tailwind stylesheet from `apps/demo/dist` (run `node scripts/build_demo.mjs` first), and renders with `set_content` because browser navigation is administratively blocked in the development container; it does not change browser policies. Set `CHROMIUM_EXECUTABLE` to an approved installed Chromium binary when Playwright's default browser is unavailable. Real HTTP/SSE is tested separately. This is not a claim of browser-to-HTTP end-to-end qualification.

`node --test scripts/test_accessibility.mjs` runs the automated WCAG A/AA checks and connection-state assertions against the fixture server. It mounts the public demo through the scripted `#demo-fixture` hash flow (fixture bootstrap over HTTP plus `staleAuth`, `extraProfiles`, `mode`, `probeResults`, and `bulkProbeHook` display flags); hash fragments never reach the server, whose asset routes reject query strings by design.
