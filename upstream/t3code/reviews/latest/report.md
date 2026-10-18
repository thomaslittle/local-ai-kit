# T3 Code provider review

Pinned commit: `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`

Compared `main` `9c161462c94f7d9d6d9169da5c841457cbadf213` and nightly `v0.1.1-nightly.20261018.3302` (`d0918f1659f4c19172796c8d1910ca6e4bc18b51`) against nightly pin `e3b35d1b2440b104f0bc2e357377c348121234dd`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (cfafb4bb54d2..9c161462c94f)

```diff
 .github/workflows/release.yml | 4 ++++
 1 file changed, 4 insertions(+)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..715edafee 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -297,6 +297,10 @@ jobs:
           sudo sed -i 's|http://|https://|g' /etc/apt/blacksmith-ubuntu-mirrors.txt /etc/apt/sources.list.d/ubuntu.sources
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
+      # The web package's browser project runs React tests in headless Chromium.
+      - name: Install Chromium for web browser tests
+        run: node apps/web/node_modules/playwright/cli.js install --with-deps --only-shell chromium
+
       - name: Test
         run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
```

## nightly (v0.1.1-nightly.20261018.3302)

```diff
 .github/workflows/release.yml             | 4 ++++
 apps/server/src/provider/CodexProvider.ts | 8 ++++++--
 2 files changed, 10 insertions(+), 2 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..715edafee 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -297,6 +297,10 @@ jobs:
           sudo sed -i 's|http://|https://|g' /etc/apt/blacksmith-ubuntu-mirrors.txt /etc/apt/sources.list.d/ubuntu.sources
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
+      # The web package's browser project runs React tests in headless Chromium.
+      - name: Install Chromium for web browser tests
+        run: node apps/web/node_modules/playwright/cli.js install --with-deps --only-shell chromium
+
       - name: Test
         run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
 
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
