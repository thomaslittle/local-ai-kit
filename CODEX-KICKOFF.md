# Codex kickoff â€” Local AI Kit

Copy the following prompt into Codex with `https://github.com/thomaslittle/local-ai-kit` selected as the project repository.

---

Implement Local AI Kit in this existing repository:
https://github.com/thomaslittle/local-ai-kit

The full design, research, 18-task plan, and build instructions are committed. This is a build task, not a request for another plan. No attachment or previous conversation is needed.

Read in this order:
1. Agent instructions: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/AGENTS.md
2. Current checkpoint: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/docs/handoffs/RESUME.md
3. Full build instructions: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/CODEX-BUILD-PROMPT.md
4. Architecture: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/docs/superpowers/specs/2026-09-24-local-ai-kit-design.md
5. Implementation plan: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md
6. Existing source audit: https://github.com/thomaslittle/local-ai-kit/blob/feature/initial-implementation/SOURCE-AUDIT.md

Inspect current git/remote state. Start `feature/initial-implementation` from the latest `main` only if no implementation branch exists; otherwise resume the newer work. Do not force-push, overwrite other work, recreate this repository, or merge to `main` automatically.

Build the standalone language/framework-agnostic toolkit: Rust core, local discovery, evidence-backed account/plan/model/capability/usage data, native-client communication, standard transports, TypeScript/Python/Rust interfaces, reusable selection controls, and a polished public-SDK-only demo. Keep it independent of Maps/image enhancement.

Use the original plan in dependency order, resuming the incomplete native session/execution work identified in RESUME.md. Discovery, the SDK-to-native bridge, and Codex metadata already have verified implementation; do not recreate them. Complete the Codex reference execution integration before broad provider expansion. Include first-class Antigravity/Grok routes, OpenCode, other specified clients, local runtimes, and explicit API routes, with support proven per operation/platform.

The default model selector MUST use the vertical provider rail with models within the chosen provider, based on:
https://github.com/thomaslittle/prmptr/blob/main/components/model-picker.tsx
Also implement the seven alternative selectors and provider-native effort/options described in the spec. All share one framework-neutral selection engine.

Work autonomously in bounded, test-backed commits. Keep `docs/handoffs/RESUME.md` current and push the implementation branch when available. Use the existing audit; revisit exact source files when implementing their adapters rather than restarting the entire research process.

Do not fake detected accounts, quotas, supported models, or passing tests. Keep fixtures visibly separate. Do not consume real inference quota or call extra paid AI/coding services without explicit authorization. No token export, silent paid fallback, account-limit evasion, publication, or unrelated settings changes.

Run the plan's verification and package smoke checks. Report exact branch/commit, runnable demo commands, artifacts, tests, measured performance, support evidence, and remaining work. Missing live accounts must be reported as untested without blocking fixture-based implementation.

Begin with workspace inspection and the current RESUME.md. Continue the unfinished native execution work; do not rebuild already implemented discovery, metadata, SDK, or selector components.
