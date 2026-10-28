# crntly-desktop

- **Where:** `apps/studio/compose-core/src/kit.rs` on `feature/gpui-native-rust-studio`.
- **Dependency:** `local-ai-kit = "0.1"` (facade over `lai-kit`).

## Steps

1. Replace the helper discovery/launch, socket path and token-file handling in
   `kit.rs` with `LocalAi::start()`.
2. Keep the app's existing state shape: `LocalAi::snapshot()` returns the same
   `Snapshot` contract the broker already serves, so mapping code should not
   change.
3. Surface `ai.mode()` (`Shared`, `Child`, `InProcess`) in the studio's
   connection indicator instead of a binary "connected" flag.
4. Delete the app's helper-install instruction and its bundled-helper startup
   path; keep `LAI_HELPER_PATH` support only for local development.
5. Keep the GPUI app's existing grants: `--allow-metadata` / execution grants are
   host configuration, not something the dependency grants itself.

## Checks for that repo

- Cold start with no shared broker: the studio goes to `Child` and still lists
  providers.
- Start the OriginLM dashboard first: the studio goes to `Shared` and both show
  the same revision.
- Close the studio without stopping the dashboard.
