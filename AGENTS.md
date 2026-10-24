# Agent instructions — Local AI Kit

Guidance for AI coding agents (and humans who like checklists) working in this repository. Read [CONTRIBUTING.md](CONTRIBUTING.md) first; this file adds the constraints that matter most when you are working autonomously.

## Owner implementation direction (2026-10-24)

Applies to every coding agent working in this repository, including OpenAI-backed
agents. For implementation scope and verification effort, this direction
supersedes older blanket test-first, full-suite or multi-configuration requirements
here and in linked guides; safety and permission boundaries remain unchanged.

- Deliver small, working, user-visible product increments. Code volume, speculative
  scaffolding and test infrastructure are not substitutes for a working product.
- Verify the changed user flow with focused visual end-to-end/in-app checks using
  the existing surface and harness. For a non-UI slice, exercise its smallest
  relevant consuming flow. Stop when the bounded result is demonstrated.
- No exhaustive unit-test campaigns, new CI, or repetitive qualification or
  certification loops. Rerun only after a relevant change or a diagnosed failure
  that warrants the retry, not merely to produce another green report.
- Keep essential build, authorization/security and data-integrity checks narrow
  and proportional to the change. Never bypass safeguards or claim untested
  behavior works; distinguish fixture evidence from live behavior.
- Reuse existing infrastructure. Avoid duplicate harnesses, extra services and
  unnecessary worker fan-out. Do not expand scope to unrelated cleanup or checks.

## Read before changing code

1. [README.md](README.md) for what the project is and how to build it.
2. [docs/adapters/SUPPORT-MATRIX.md](docs/adapters/SUPPORT-MATRIX.md) for what each provider actually supports today.
3. [docs/design/architecture.md](docs/design/architecture.md) and [docs/security/threat-model.md](docs/security/threat-model.md) for intended boundaries.
4. [docs/implementation/DEVELOPMENT-LOG.md](docs/implementation/DEVELOPMENT-LOG.md) and [docs/implementation/LEDGER.md](docs/implementation/LEDGER.md) for what was verified and what is deliberately unclaimed.
5. [docs/design/implementation-plan.md](docs/design/implementation-plan.md) is a historical plan; its file layout is illustrative and its checkboxes are not a status tracker.

## Working agreements

- Inspect `git status`, the current branch, and recent history before writing. Do not overwrite or revert changes you did not make. Do not force-push. Do not merge into `main` or change repository settings or visibility unless the maintainers ask you to.
- Make bounded commits with Conventional Commit prefixes (`feat(scope):`, `fix(scope):`, `docs:`, `chore:`).
- Use test-first verification for the area you change; run the targeted checks listed in [CONTRIBUTING.md](CONTRIBUTING.md). Never replace a required check with a stub that prints success, and do not report a nonexistent script as a product failure.
- Extend a file safely rather than replacing it wholesale. Keep generated files (`protocol/*`, `packages/*/src/generated.*`) in sync by regenerating them (`python scripts/generate_contracts.py`), never by hand.

## React demo (`apps/demo`)

- Stack: React 19 with the React Compiler, Vite, Tailwind CSS v4, shadcn/ui (style `base-mira`, Base UI, Phosphor icons), TanStack Router, zustand. Add or refresh UI primitives only with the shadcn CLI (`npx shadcn@latest add <name> --cwd apps/demo`, with `pnpm-workspace.yaml` temporarily moved aside so the CLI uses npm); do not hand-write copies of shadcn components.
- State lives in the framework-free store (`src/state/store.ts`). Components subscribe to slices; do not add `useEffect` for anything but subscriptions to external systems.
- Pages are real routes. Every route in `src/lib/pages.ts` must also be listed in the fixture server (`apps/demo/server.mjs`) and the native helper (`crates/lai-transports/src/http/routes.rs`).
- `npm run doctor` (React Doctor) must report no warnings or errors; it is a required pull-request check (`.github/workflows/react-doctor.yml`). Fix root causes; a suppression needs a written reason next to it.
- Run the demo with `npm run dev` (native helper plus Vite hot reload at `http://127.0.0.1:5173/`); there is no separate non-HMR demo command. Real detected data comes first; example data is only a labeled fallback when no runtime is paired.

## Product invariants

- Rust is the canonical implementation of detection and provider behavior. SDKs are thin; React, Tauri, and Electron are optional consumers, never core requirements.
- Imports do no scanning, credential reading, spawning, network calls, downloads, or inference. Discovery is staged and permissioned.
- Preserve provider/installation/profile/account/connection/route/model/billing/pool identity and field-level evidence. Unknown and unsupported are distinct. Do not fabricate access, quotas, prices, capability metadata, or effort settings.
- Preserve provider-native model IDs and option descriptors. No hard-coded universal Low/Medium/High model controls.
- Never export subscription tokens as API keys, read browser cookies, enable paid fallback silently, rotate accounts to evade limits, or race native credential refresh.
- A discovery signal is not permission to execute. Scope grants, workspace access, tools, artifacts, and process ownership. Cancel only owned work.
- The demo uses only public SDK APIs and includes all eight selector variants, with the vertical provider rail as the default. Keep synthetic data visibly separate from live results.

## Evidence and authorization

Use virtual homes, synthetic credentials, fake clients, and mock servers by default. Installing or configuring a dependency is not permission to consume an AI account. Live inference or paid checks require explicit authorization from the maintainers; missing account or platform access does not prevent fixture-based work. Do not launch extra hosted coding services or charge external AI APIs as part of a change.

Record source commits, licenses, adapter operations, platforms, tests, and limits, and preserve upstream notices when reusing code. Do not publish to registries, buy or reset credits, sign releases, or alter unrelated local client settings. In every report distinguish **planned**, **implemented**, **fixture-tested**, **platform-executed**, and **live-qualified** status.
