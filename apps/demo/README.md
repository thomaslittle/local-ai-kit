# Developer showcase — implementation preview

A working, framework-neutral application consuming the public TypeScript SDK and selector core. It is explicitly synthetic by default and is **not** evidence of working native account detection.

From the repository root, with Node 22.16+ and TypeScript 5.8.3 installed:

```sh
npm install --ignore-scripts
npm run build
node apps/demo/server.mjs --fixture
```

Open the loopback URL printed by the server (default port 4499). `--fixture` is required. Imports do not start a listener. No model account, provider credential, keyring read, paid API call, or native agent is used.

## Working pages

Overview, connections/account evidence, models, per-pool usage, selector lab, simulated session playground, and safe diagnostics/integration snippets.

All eight selectors share the same selection state: vertical provider rail, compact combobox, keyboard command palette, capability-first filter, comparison table, allowance-aware groups, task presets, and mobile drill-down. Exact route/account identity is retained. Options are rendered from descriptors, including missing and future effort choices; sensitive options require a visible confirmation. Favorites distinguish separate routes to identical model names.

The playground exercises public SDK grants, selection validation, session creation, simulated text events, approval denial/acceptance, and cancellation. No inference occurs. Synthetic quota pools preserve shared identity and missing totals. A failed explicit live connection clears all synthetic data; live execution is disabled in this preview UI.

This is a vanilla DOM example, not the final React hook/component package. Large lists are bounded to 50 matching rows with search rather than full scrolling virtualization. Favorites are in-memory only. Full light-theme UI, persistent recents, production pairing, live provider integration, React packaging, and the remaining architecture tasks are still pending.

## Tests

`node --test apps/demo/tests/*.test.mjs` runs 22 service and real local HTTP/SSE tests.

`python scripts/test_demo.py` runs 13 Playwright interactions against the actual UI and public SDK using an in-memory test transport. It uses offline `set_content` rendering because browser navigation is administratively blocked in the development container; it does not change browser policies. Real HTTP/SSE is tested separately. This is not a claim of browser-to-HTTP end-to-end qualification.

Install Python Playwright for UI tests and set `CHROMIUM_EXECUTABLE` to an approved installed Chromium binary when it is not `/usr/bin/chromium`. `scripts/bundle_demo_test.mjs` uses the pinned TypeScript compiler to bundle only local test modules; it is not part of the shipped HTTP entry point.
