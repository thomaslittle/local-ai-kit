# remotif

- **Where:** `editor/crates/remotif-kit`.
- **Dependency:** `local-ai-kit = "0.1"`.

## Steps

1. Replace the custom client loop in `remotif-kit` with `LocalAi::start()` and
   `call`/`snapshot`.
2. If remotif embeds the runtime currently, drop the embed and use the facade;
   it keeps in-process as a fallback when no helper binary exists.
3. Expose `ai.mode()` in the editor's connection status.
4. Remove any instruction that asks users to install the helper first.

## Checks for that repo

- First launch after install resolves `Child` and exposes the provider list.
- With the dashboard running, resolves `Shared` without a second scanner.
- Quitting the editor leaves the dashboard running.
