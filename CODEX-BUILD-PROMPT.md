# Codex build prompt — Local AI Kit

Repository: <https://github.com/thomaslittle/local-ai-kit>. This repository already contains the complete handoff. No archive or previous chat is required. Read and execute the instructions below in this repository.

---

Build **Local AI Kit** as a standalone, production-minded, open-source developer toolkit. `local-ai-kit` is a working identifier, not a cleared public package/product name. This is an implementation task: use the supplied design and execution plan, implement incrementally, and verify what you build. Do not stop at scaffolding, a design document, mock dashboards, or provider logos.

## Read first

1. [Agent instructions](https://github.com/thomaslittle/local-ai-kit/blob/main/AGENTS.md).
2. [Project README](https://github.com/thomaslittle/local-ai-kit/blob/main/README.md).
3. [Current resume checkpoint](https://github.com/thomaslittle/local-ai-kit/blob/main/docs/handoffs/RESUME.md).
4. [Source audit](https://github.com/thomaslittle/local-ai-kit/blob/main/SOURCE-AUDIT.md).
5. [Architecture specification](https://github.com/thomaslittle/local-ai-kit/blob/main/docs/superpowers/specs/2026-09-24-local-ai-kit-design.md).
6. [18-task implementation plan](https://github.com/thomaslittle/local-ai-kit/blob/main/docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md).

Use this existing standalone repository, `thomaslittle/local-ai-kit`. Inspect `main` and any existing `feature/initial-implementation` work; create that implementation branch from current `main` only if it does not already exist. Do not recreate the repository or restart work already recorded in `RESUME.md`. Inspect its git status and current remote state before writing. Do not overwrite existing work, force-push, or modify any of the reference repositories. Do not publish npm/PyPI/crates.io packages, create a public repository, or distribute signed releases without separate authorization. Keep the existing specification, plan, source audit, attribution, and checkpoints. Evolve the README truthfully as features become real. Extend pre-seeded documents safely even when a task lists them under "Create". Push bounded commits to the implementation branch when remote writes are available; do not merge to `main` automatically.

## Product goal

Make it extremely easy for applications in any language or framework to discover the AI integrations and accounts already configured on a user's computer, inspect their actual capabilities and available access, and communicate with supported native agents/clients or explicitly configured API routes through a stable interface.

Expose installed clients and versions; profiles/workspaces/accounts; verified authentication and plan evidence; actual route-scoped model catalogs; native model/session options including effort where available; capability and tool information; usage/reset/balance windows and shared quota pools; health/freshness; and permissioned execution with streaming, approvals, cancellation, and native resumption where supported.

Keep this product completely independent of Maps and image enhancement. Do not build a browser consumer application or specialized media feature into its core. The browser-extension example in the plan is only a generic host/transport demonstration.

## Architecture to implement

Use a modular Rust core with optional discovery and execution layers. One service implementation powers embedded Rust use, a managed native sidecar, an optional shared per-user broker, JSON-RPC over stdio, and authenticated local HTTP/SSE. Generate domain schemas/OpenAPI/SDK types rather than maintaining duplicate provider logic in every language.

Ship a TypeScript SDK with separate browser/Node entry points, a Python SDK, a Rust integration surface, complete protocol docs, and tested language-neutral examples. Native binaries must be usable without requiring end users to compile Rust. Package imports must not scan, read credentials, spawn agents, contact providers, download software, or run inference.

The shared broker deduplicates metadata and quota work across authorized apps. Embedded/private-sidecar modes remain useful and must not pretend to deduplicate across unrelated processes.

Separate the provider, client installation, profile, authenticated connection, account, access route, model author/model ID, billing mode, and quota pool. One model reached through two accounts or gateways must not lose its execution or billing identity. Account deduplication needs authoritative evidence, not a shared email or similar label.

## Existing code to inspect and reuse appropriately

- https://github.com/pingdotgg/t3code — official-client integration patterns, Codex app-server, ACP drivers, option descriptors, and provider-instance-aware selection. Relevant verified paths are in `SOURCE-AUDIT.md`.
- https://github.com/thomaslittle/codenotch-crossplatform — existing Rust discovery/read-only metadata parsers, usage snapshots, and credential-location/platform knowledge.
- https://github.com/vinzdg/codenotch — additional provider discovery and platform implementations. Recheck which metadata mechanisms are official contracts versus community/private endpoints.
- https://github.com/thomaslittle/prmptr — the exact UI interaction reference is `components/model-picker.tsx`: a vertical provider rail with model selection inside the chosen provider group.

Recheck current commit IDs and licenses before porting code. Preserve notices. Use focused parts and tests, not entire applications. Do not pull T3's full application state/runtime graph, Tauri, Electron, Next.js, or React into the Rust core. Do not inherit application-specific default-model overrides, provider enums, localStorage keys, or model-name heuristics as universal package policy.

## Metadata and runtime correctness

Use field-level evidence and freshness. Unknown, unsupported, unavailable, stale, inferred, and verified must remain distinguishable. An installed CLI or present credential file is not proof of an active plan or usable model. Old usage cannot override a new authentication failure. Account changes invalidate account-scoped caches and selections.

Do not invent exact quotas, context sizes, prices, effort values, or a global remaining-token number. Preserve usage units and pool identities; two integrations using the same verified account bucket do not create two allowances. Counts without denominators have no percentage gauge. Report toolkit-session usage separately from provider-wide account usage.

Discover models and options dynamically using the actual route. Preserve native IDs, supported values, current/default state, scope, dependencies, and descriptor revisions. If a model has no effort selector, show none. If a future native effort appears, handle it without a hard-coded model-name patch. Validate the current selection again before execution. Label tool-mediated capabilities separately from base-model capabilities.

Start with the complete Codex vertical slice, then build the shared ACP layer and first-class Antigravity/Grok routes, OpenCode, other documented native-client routes, local runtimes, and explicit API access according to the plan. Gemini CLI and Antigravity are different adapters. A coding client can route to models from another vendor. A support matrix must describe operations individually, not one all-purpose “supported” flag.

Use supported native protocols/clients and their existing logins where permitted. Never make the product depend on credential theft, browser-cookie scraping, client impersonation, or exporting subscription tokens as generic API keys. Undocumented metadata endpoints require a clearly classified, reviewed, opt-in adapter path. Do not silently substitute paid API calls when a subscription route fails.

## Security and optimization

Implement explicit per-app/per-connection grants, scoped workspace/tools/network permissions, deny-by-default approvals, revocation, redacted diagnostics, bounded artifacts, and owned-process cleanup. No tokens in SDK snapshots, browser state, query strings, logs, or generated snippets. Treat a spawned process as a crash boundary, not a security sandbox. Reject execution constraints the native client cannot enforce rather than claim they work.

The HTTP bridge must be authenticated, loopback-only by default, protected against unauthorized origins/Host values, and paired only through explicit user action. CORS alone is not authorization. A public webpage cannot silently scan a visitor's machine; the live demo is local or explicitly paired.

Keep startup/cache reads fast through progressive scans, bounded I/O, incremental log reads, file watchers, per-field TTLs, singleflight refreshes, Retry-After/backoff, stable IDs, lazy process startup, and indexed catalogs. No recursive full-disk scans, repeated whole-log reads, aggressive polling of every provider, or scan-on-render behavior. Measure the budgets in the spec; do not claim performance from intuition.

## Demo and reusable UI are required deliverables

Build a polished local demo using only public SDK/API methods, not private app backdoors. Include connections/account/plan evidence, route/model/capability explorer, native option controls, per-pool usage, a permission-gated session playground with streaming/approvals/cancel, diagnostics, and safe API/integration examples.

Implement the **T3/Prmptr-style vertical provider rail** as the default selector, with distinct accounts/routes and models within the selected provider. Add all seven alternatives in the spec: compact grouped combobox, command palette, capability-first selector, comparison table, allowance-aware route picker, task/preset picker, and mobile drill-down.

All eight use the same framework-neutral selection state and dynamic descriptors. Provide headless logic, React hooks, optional styled components, theming, density/render slots, favorites/recents, keyboard/screen-reader support, and large-list virtualization. Do not make users adopt React to consume discovery or execution.

Use deterministic, visibly labeled synthetic scenarios for demonstrations and tests. Live mode must never substitute fake accounts or usage if real detection fails. Every advertised public feature must have a working demo path or an explicitly explained permission/support restriction.

## Execution discipline and acceptance

Follow the 18 tasks in order of dependency. Use test-first cycles, small modules, bounded commits, and targeted checks. Work independently within the approved design; do not repeatedly ask for approval after every component. Ask only for a genuinely non-resolvable credential, permission, or destructive decision. If an account/platform is unavailable for a live test, continue fixture/contract implementation and record the missing evidence honestly.

Do not spend real inference quota merely to discover capabilities. Any live generation test needs explicit authorization; do not purchase credits, redeem resets, enable paid overages, or change unrelated client settings. If authorized live checks are unavailable, do not claim they passed.

Keep `docs/handoffs/RESUME.md` updated after meaningful commits with exact state, next action, tests, and unresolved evidence. Do not restart finished audits or rewrite completed modules when context is resumed. Use parallel workers only for independent tasks after shared contracts are stable; keep one integration owner and avoid overlapping edits.

Run the contract, Rust, TypeScript, Python, UI/accessibility, security, performance, generated-file, and package smoke checks listed in the plan. Test actual installable artifacts, not only workspace aliases. Distinguish compiled, fixture-tested, platform-executed, and live-account-qualified support.

Your completion report must include: exact commit/branch; how to run the CLI and local demo; usable package/artifact paths; provider support by operation/platform; test results; actual benchmark measurements; security/integration limitations; and exact remaining work. Do not call the product complete because it renders a mock catalog. If the session must end early, preserve a clean resumable checkpoint and report precisely what is implemented and what remains.

**Begin by inspecting the workspace and source references, then implement Task 1 and continue through the plan.**
