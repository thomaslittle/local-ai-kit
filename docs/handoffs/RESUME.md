# Resume — Local AI Kit

**Checkpoint date:** 2026-09-24  
**Repository:** <https://github.com/thomaslittle/local-ai-kit>  
**Bootstrap branch:** `main`  
**Implementation branch:** `feature/initial-implementation` (create from current `main` only if absent)  
**Product implementation:** not started; 0 of 18 tasks completed.

## What is present

The complete original architecture specification, 18-task implementation plan, and source audit; a repository-linked full build prompt and kickoff prompt; root agent instructions; README/docs navigation; MIT license; attribution, security, and contribution guidance; and a checksum manifest for this planning bootstrap.

The three original design/plan/audit documents are preserved byte-for-byte from the conversation handoff. The build prompt has been adapted for this repository and links directly to the committed docs. Product code, dependency manifests, CI workflows, generated contracts, installed SDKs, and a working demo have not been created.

## First action

Read [AGENTS.md](../../AGENTS.md) and [CODEX-BUILD-PROMPT.md](../../CODEX-BUILD-PROMPT.md), then the [specification](../superpowers/specs/2026-09-24-local-ai-kit-design.md), [implementation plan](../superpowers/plans/2026-09-24-local-ai-kit-implementation.md), and [source audit](../../SOURCE-AUDIT.md).

Inspect repository state and refresh refs before creating or resuming the implementation branch:

```sh
git status --short --branch
git remote -v
git fetch origin
git rev-parse HEAD
git branch --all
```

A commit cannot embed its own final hash; resolve the exact current HEAD with git. Use this file's latest committed status, not an older chat SHA, when continuing work.

Begin **Task 1: Establish source provenance, workspace, and canonical contracts**. Existing bootstrap files appearing in Task 1's "Create" list must be extended rather than discarded. Pin compatible toolchains, write real contract tests, and add verification profiles; no success-printing stubs.

## Verification status

- Planning bootstrap: document presence, local Markdown links, original source-byte preservation, unchecked task state, and manifest checksums are checked before push.
- Product compilation, unit/contract/SDK/UI tests, and platform/runtime checks: **not run; product not implemented**.
- Provider account discovery and live inference qualification: **not performed**.
- Performance figures in the specification: **targets only**, not measured results.

## Scope and unresolved evidence

Follow the specification; do not restart architecture research or modify T3 Code, either CodeNotch repository, or Prmptr. No product/registry naming clearance or publication is claimed. No AI-provider credentials or paid inference are authorized by this bootstrap.

Native client availability, integration policy, model descriptors, quota fields, and platform behavior must be qualified per adapter during implementation. Missing live accounts should produce honest untested entries, not fake successes or a halt to deterministic implementation.

## Maintain this checkpoint

After meaningful commits, record completed task IDs, branch and resolved commit context, exact tests and results, current public contract revisions, next action, and any genuinely blocked live evidence. Keep implementation progress separate from documentation/bootstrap progress.
