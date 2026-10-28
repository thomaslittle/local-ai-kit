# xhw_haiwire

- **Where:** `crates/haiwire-ai`.
- **Dependency:** `local-ai-kit = "0.1"`.

## Steps

1. Replace the hand-rolled broker transport in `haiwire-ai` with `LocalAi`.
2. Keep the crate's public API stable where possible: wrap `LocalAi` rather than
   re-exporting it, so the rest of the workspace does not need a breaking change.
3. Report `ai.mode()` alongside the existing connection state.
4. Delete the helper install step from the README and from any setup script.

## Checks for that repo

- A workspace binary using `haiwire-ai` starts with no dashboard running.
- The same binary attaches to the dashboard when it is running.
- No token file or helper path is required from the user.
