# T3 Code provider review

Pinned commit: `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`

Compared `main` `812b7006f8a12c5c8e55ea65a9cfcebfd328872f` and nightly `v0.1.1-nightly.20261017.3238` (`70d60e03a7a35c6fa131825bbfa05ec893e69912`) against nightly pin `e3b35d1b2440b104f0bc2e357377c348121234dd`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (cfafb4bb54d2..812b7006f8a1)

No watched-path changes.

## nightly (v0.1.1-nightly.20261017.3238)

```diff
 apps/server/src/provider/CodexProvider.ts | 8 ++++++--
 1 file changed, 6 insertions(+), 2 deletions(-)

diff --git a/apps/server/src/provider/CodexProvider.ts b/apps/server/src/provider/CodexProvider.ts
index f15ad138e..f4f123dc1 100644
--- a/apps/server/src/provider/CodexProvider.ts
+++ b/apps/server/src/provider/CodexProvider.ts
@@ -36,7 +36,6 @@ import {
 import { resolveSpawnCommand } from "@t3tools/shared/shell";
 import { codexAppServerArgs, resolveCodexLaunchArgs } from "./codexLaunchArgs.ts";
 import {
-  AUTH_PROBE_TIMEOUT_MS,
   buildServerProvider,
   COMPACT_SLASH_COMMAND,
   providerModelsFromSettings,
@@ -71,6 +70,11 @@ const CODEX_APP_SERVER_PROBE_FORCE_KILL_AFTER = "2 seconds" as const;
 const CODEX_CATALOG_REFRESH_TIMEOUT = "3 seconds" as const;
 const CODEX_CATALOG_REFRESH_POLL = "100 millis" as const;
 const CODEX_CLIENT_NAME = "T3 Code";
+// Codex's probe does a full handshake plus account, models, and skills requests, which can
+// legitimately run past the shared 10s AUTH_PROBE_TIMEOUT_MS on a loaded or slow machine
+// (most often reported on Windows). Give Codex its own longer budget instead of raising the
+// timeout for every provider.
+const CODEX_AUTH_PROBE_TIMEOUT_MS = 20_000;
 
 const CODEX_PRESENTATION = {
   displayName: "Codex",
@@ -660,7 +664,7 @@ export const checkCodexProviderStatus = Effect.fn("checkCodexProviderStatus")(fu
     ...(managedAuth ? { skipNativeUsage: true } : {}),
   }).pipe(
     Effect.scoped,
-    Effect.timeoutOption(Duration.millis(AUTH_PROBE_TIMEOUT_MS)),
+    Effect.timeoutOption(Duration.millis(CODEX_AUTH_PROBE_TIMEOUT_MS)),
     Effect.result,
   );
```

Do not move the pin until a maintainer ports a fact and updates the snapshot in the same change.
