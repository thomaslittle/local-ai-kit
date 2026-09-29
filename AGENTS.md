# Agent instructions — Local AI Kit

## Read before implementing

1. [README.md](README.md) and [docs/handoffs/RESUME.md](docs/handoffs/RESUME.md).
2. [CODEX-BUILD-PROMPT.md](CODEX-BUILD-PROMPT.md).
3. [SOURCE-AUDIT.md](SOURCE-AUDIT.md).
4. [Architecture specification](docs/superpowers/specs/2026-09-24-local-ai-kit-design.md).
5. [Implementation plan](docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md).

This is the designated standalone repository, `thomaslittle/local-ai-kit`. The owner requested implementation via Codex after this planning bootstrap. Do not create a replacement repository, redo the architecture, or modify the reference applications.

## Git and checkpoints

Inspect `git status`, current HEAD, branch, and remote refs before writing. Start `feature/initial-implementation` from the latest clean `main`; resume existing work on that branch rather than replacing it. Make bounded commits, keep `docs/handoffs/RESUME.md` current, and use the configured `origin` without force-push. Do not merge into `main` or change repository visibility without separate authorization. A pre-existing file named under a task's "Create" list must be extended safely, not blindly replaced.

Implement tasks in dependency order and use their targeted test-first verification. Product manifests, locked toolchains, native/SDK/UI code, distribution scripts, and CI already exist. Resume from the ledger and verified source checkpoint; do not restart Task 1 or overwrite implemented modules. Do not run nonexistent scripts and call their absence a product failure. Never replace required checks with success-printing stubs.

## React demo (apps/demo)

- Stack: React 19 with the React Compiler, Vite 8, Tailwind CSS v4, shadcn/ui (style `base-mira`, Base UI, Phosphor icons), TanStack Router, zustand. Add or refresh UI primitives only with the shadcn CLI (`npx shadcn@latest add <name> --cwd apps/demo`, with `pnpm-workspace.yaml` temporarily moved aside so the CLI uses npm); do not hand-write copies of shadcn components.
- State lives in the framework-free store (`src/state/store.ts`). Components subscribe to slices; do not add `useEffect` for anything but subscriptions to external systems.
- Pages are real routes. Every route in `src/lib/pages.ts` must also be listed in the fixture server (`apps/demo/server.mjs`) and the native helper (`crates/lai-transports/src/http/routes.rs`).
- `npm run doctor` (React Doctor) must report no warnings or errors. It is a required pull-request check (`.github/workflows/react-doctor.yml`). Fix root causes; a suppression needs a written reason next to it.
- Run the demo with `npm run dev` (Vite + synthetic fixture, `http://127.0.0.1:5173/`).

## Product invariants

- Rust is the canonical implementation of detection/provider behavior. SDKs are thin; React/Tauri/Electron are optional consumers, never core requirements.
- Imports do no scanning, credential reading, spawning, network calls, downloads, or inference. Discovery is staged and permissioned.
- Preserve provider/installation/profile/account/connection/route/model/billing/pool identity and field-level evidence. Unknown and unsupported are distinct. Do not fabricate access, quotas, prices, capability metadata, or effort settings.
- Preserve provider-native model IDs and option descriptors. No hard-coded universal Low/Medium/High model controls.
- Never export subscription tokens as API keys, read browser cookies, enable paid fallback silently, rotate accounts to evade limits, or race native credential refresh.
- A discovery signal is not permission to execute. Scope grants, workspace access, tools, artifacts, and process ownership. Cancel only owned work.
- The demo uses only public SDK APIs and includes all eight selector variants, with the vertical provider rail as the default. Keep synthetic data visibly separate from live results.
- No Maps/image-enhancement product logic belongs in this toolkit.

## Evidence and authorization

Use virtual homes, synthetic credentials, fake clients, and mock servers by default. Installing or configuring a new dependency is not permission to consume an AI account. Live inference/paid checks require explicit authorization; missing account/platform access does not prevent fixture-based implementation. Never launch extra hosted coding services or charge external AI APIs as part of this work without authorization.

Record source commits, licenses, adapter operations, platforms, tests, and limits. Preserve upstream notices when reusing code. Do not publish registries, buy/reset credits, sign releases, or alter unrelated local client settings. Distinguish planned, implemented, fixture-tested, platform-executed, and live-qualified status in every report.

If the Superpowers skills are installed, use the implementation and verification skills referenced in the plan. If they are unavailable, follow the plan's explicit steps rather than block on installing a workflow plugin.
