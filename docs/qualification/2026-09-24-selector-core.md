# Selector core verification

Environment: assistant Linux container, Node 22.16.0, TypeScript 5.8.3. No account/network access or inference used.

Red: 24 assertion paths failed against explicit unimplemented function scaffolds. Green after implementation: `node --experimental-strip-types --test packages/selector-core/tests/*.test.mjs` — 24 passed, 0 failed. `tsc -p packages/selector-core/tsconfig.json` — exit 0.

A single 5,000-model index/search test completed in approximately 10 ms in this environment. This is a smoke measurement, not a p95 product benchmark or native-runtime performance qualification.

Coverage: route collisions and multi-account identity; included/metered/unknown billing; expired/stale auth; unknown/conditional capabilities; indexed search; provider grouping; missing/duplicate identifiers; native future options; numeric constraints; selection reconciliation; immutable inputs; shared pool display.

Native Rust contracts still await a green compilation run. The Windows device went offline after its red test run. GitHub Actions run 35984612761 failed before runner assignment (no steps executed); cause not established. Do not characterize that result as a product test failure or success.

This completes a tested headless selection slice, not all of Task 14 (React/UI integration remains) or the release plan.
