# Remotif bridge attempt history

All runs below used a test-owned virtual home, a unique loopback IPC/endpoint,
and synthetic connections. No live provider, TV, LAN listener, inference, or
desktop notification was used.

## Retained

- `remotifd-shared-bridge-opt-in-skipped.txt`: the first harness invocation ran
  without `REMOTIF_AI_KIT_TEST_BINARY`; the test skipped and is not presented as
  a pass.
- `remotifd-shared-bridge-pass.txt`: real broker run for app-scope filtering,
  revocation, peer survival, broker-kill failure, and restart recovery.
- `remotif-kit-tests-pass.txt`: existing kit tests.

## Not retained as files

These failures occurred before the unique-evidence rule was applied. Their
terminal output remains in the agent transcript, but no original artifact is
claimed:

1. The first compile of `shared_bridge.rs` failed with
   `error[E0277]: Models doesn't implement Debug` because the test formatted a
   `Result<Models, BridgeError>` with `{:?}`. The failure messages were replaced
   with plain assertions.
2. After the cache-invalidation change, the test failed at
   `a revoked app must not receive its former cached catalog`. This exposed
   that a failed refresh still left the old cache available to a no-refresh
   call; the bridge now clears the cache on refresh failure.
3. The next run failed at
   `restart must mint a fresh app credential` because the test read the old
   token file before the restarted broker republished it. The test now removes
   the old token files before restarting and waits for fresh credentials.
4. The final retained run passed after those fixes.

The pass and skip logs above are the durable bridge evidence.
