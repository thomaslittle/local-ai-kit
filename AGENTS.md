# Agent instructions — Local AI Kit

Guidance for AI coding agents (and humans who like checklists) working in this repository. Read [CONTRIBUTING.md](CONTRIBUTING.md) first; this file adds the constraints that matter most when you are working autonomously.

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

## Dependency hygiene (owner direction, 2026-10-25)

- At work start, check the important stack's official releases/registries and
  dated release notes when its recorded review is missing or more than 7 days
  old; also check before release and before relevant dependency changes. Review
  urgent upstream advisories promptly, regardless of the review date.
- Record review date, official source links, resolved version/Git SHA, local fork
  patches, compatibility/pin rationale, and the upgrade decision in the linked
  audit or existing dependency guidance. Distinguish latest stable, prerelease,
  Git-only snapshots and supported compatibility pins; prefer compatible,
  supported releases over blindly tracking HEAD.
- Reuse a current review while pins/sources/advisories are unchanged; do not repeat
  unchanged audits or add scheduled model wake-ups for this guidance. Coordinate
  upgrades with active owners and preserve dirty work.
- Validate adopted upgrades with focused builds and existing visual E2E/in-app
  smoke of affected flows (or the smallest consuming flow for non-UI work). Add
  no new unit tests for this maintenance; repeat checks only after a relevant
  change or diagnosed failure. Record evidence and untested platform limits.
- Latest bounded review: [2026-10-25 dependency audit](docs/dependency-audit-2026-10-25.md).
