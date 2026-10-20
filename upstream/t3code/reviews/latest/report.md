# T3 Code provider review

Pinned commit: `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`

Compared `main` `d36b7da2485bc889b320a285deea181f637d760a` and nightly `v0.1.1-nightly.20261020.3411` (`d36b7da2485bc889b320a285deea181f637d760a`) against nightly pin `e3b35d1b2440b104f0bc2e357377c348121234dd`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (cfafb4bb54d2..d36b7da2485b)

```diff
 .github/workflows/release.yml                      |  45 +-
 apps/server/src/cloud/CliTokenManager.test.ts      | 204 +++++-
 apps/server/src/cloud/CliTokenManager.ts           |  92 ++-
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  | 166 ++++-
 apps/server/src/cloud/CloudLink.test.ts            |   4 +
 apps/server/src/cloud/CloudLink.ts                 |  34 +-
 apps/server/src/cloud/ConnectReachability.test.ts  | 153 ++++
 apps/server/src/cloud/ConnectReachability.ts       | 312 ++++++++
 .../src/cloud/ManagedEndpointRuntime.test.ts       | 210 ++++++
 apps/server/src/cloud/ManagedEndpointRuntime.ts    | 213 ++++--
 apps/server/src/cloud/bootService.test.ts          | 195 ++++-
 apps/server/src/cloud/bootService.ts               | 198 +++--
 apps/server/src/cloud/config.ts                    |  10 +-
 apps/server/src/cloud/http.test.ts                 | 265 ++++++-
 apps/server/src/cloud/http.ts                      |   5 +
 apps/server/src/cloud/linkChecks.ts                |   3 +-
 apps/server/src/cloud/managedTunnelStartup.test.ts | 206 ++++++
 apps/server/src/cloud/managedTunnelStartup.ts      | 189 ++++-
 apps/server/src/cloud/pinnedRuntime.test.ts        | 161 +++++
 apps/server/src/cloud/pinnedRuntime.ts             |  30 +-
 apps/server/src/cloud/publicConfig.test.ts         |  26 +
 apps/server/src/cloud/publicConfig.ts              |   5 +
 apps/server/src/cloud/relayResponse.test.ts        |  50 ++
 apps/server/src/cloud/relayResponse.ts             |  48 +-
 apps/server/src/cloud/selfUpdate.test.ts           |  49 +-
 apps/server/src/cloud/selfUpdate.ts                |  81 ++-
 .../server/src/cloud/serviceLauncherClient.test.ts |  20 +-
 apps/server/src/cloud/serviceLauncherClient.ts     |  13 +-
 apps/server/src/cloud/serviceProtocol.ts           |   1 +
 .../server/src/provider/AntigravityInstallation.ts |  17 +-
 apps/server/src/provider/CodexChatGptAuth.ts       |  91 ++-
 apps/server/src/provider/CodexInstallation.ts      |  42 ++
 apps/server/src/provider/CodexProvider.ts          | 217 ++++--
 .../src/provider/Drivers/AntigravityDriver.ts      | 801 ++++++++++----------
 .../src/provider/Drivers/AntigravityUsage.ts       |  10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   | 516 +++++++------
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |  38 +-
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |  43 +-
 .../src/provider/Drivers/ClaudeSkills.test.ts      | 333 +++++++++
 apps/server/src/provider/Drivers/ClaudeSkills.ts   | 272 ++++++-
 .../src/provider/Drivers/CodexDriver.test.ts       |  40 +
 apps/server/src/provider/Drivers/CodexDriver.ts    | 453 ++++++------
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |  39 +
 .../server/src/provider/Drivers/CodexHomeLayout.ts |   3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |  51 +-
 .../src/provider/Drivers/antigravityUsageReader.ts |  78 +-
 .../src/provider/Drivers/claudeUsage.test.ts       |  31 +-
 apps/server/src/provider/Drivers/claudeUsage.ts    |   5 +-
 .../server/src/provider/Drivers/codexUsage.test.ts | 255 ++++++-
 apps/server/src/provider/Drivers/codexUsage.ts     | 210 +++++-
 docs/internals/providers.md                        |   3 +-
 .../src/server/AcpRegistrySupport.test.ts          |  49 ++
 .../src/server/AcpRegistrySupport.ts               |  37 +-
 .../provider-acp-registry/src/server/adapter.ts    |  46 +-
 packages/provider-acp-registry/src/server/auth.ts  | 179 +++--
 .../provider-acp-registry/src/server/claudeAcp.ts  | 117 +++
 .../provider-acp-registry/src/server/driver.ts     | 805 +++++++++++----------
 .../provider-acp-registry/src/server/droidAcp.ts   |  84 +++
 packages/provider-acp-registry/src/server/probe.ts |  84 ++-
 packages/provider-grok/src/server/acpSupport.ts    |  51 +-
 packages/provider-grok/src/server/adapter.ts       |  60 +-
 packages/provider-grok/src/server/driver.test.ts   | 126 ++++
 packages/provider-grok/src/server/driver.ts        | 322 +++++----
 packages/provider-grok/src/server/status.ts        | 177 ++---
 .../provider-grok/src/server/textGeneration.ts     |  37 +-
 packages/provider-grok/src/server/usageLimits.ts   |  43 +-
 .../provider-grok/src/server/xaiAcpExtension.ts    | 101 ++-
 packages/provider-muse/src/server/adapter.test.ts  | 494 ++++++++++++-
 packages/provider-muse/src/server/adapter.ts       | 209 ++++--
 packages/provider-muse/src/server/driver.ts        | 249 ++++---
 .../provider-muse/src/server/maintenance.test.ts   |  97 ++-
 packages/provider-muse/src/server/maintenance.ts   |  81 +--
 .../provider-muse/src/server/modelCatalog.test.ts  |  28 +
 packages/provider-muse/src/server/modelCatalog.ts  |   7 +-
 packages/provider-muse/src/server/protocol.ts      |  32 +-
 packages/provider-muse/src/server/status.test.ts   |  31 +-
 .../provider-muse/src/server/usageLimits.test.ts   |  60 ++
 packages/provider-muse/src/server/usageLimits.ts   |  48 ++
 packages/provider-opencode/src/server/driver.ts    | 615 ++++++++--------
 79 files changed, 7975 insertions(+), 2830 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..21e9e7ba9 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -50,6 +50,7 @@ jobs:
   # have already run. Scheduled runs also decide here whether a nightly is due.
   resolve_commit:
     name: Resolve release commit
+    if: github.repository == 'pingdotgg/t3code'
     runs-on: blacksmith-8vcpu-ubuntu-2404
     timeout-minutes: 5
     outputs:
@@ -128,12 +129,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
         env:
           pnpm_config_cache_dir: ${{ runner.temp }}/pnpm-metadata
 
@@ -246,12 +243,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       - name: Ensure Electron runtime is installed
         run: vp run --filter @t3tools/desktop ensure:electron
@@ -280,12 +273,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       - name: Ensure Electron runtime is installed
         run: vp run --filter @t3tools/desktop ensure:electron
@@ -297,6 +286,10 @@ jobs:
           sudo sed -i 's|http://|https://|g' /etc/apt/blacksmith-ubuntu-mirrors.txt /etc/apt/sources.list.d/ubuntu.sources
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
+      # The web package's browser project runs React tests in headless Chromium.
+      - name: Install Chromium for web browser tests
+        run: node apps/web/node_modules/playwright/cli.js install --with-deps --only-shell chromium
+
       - name: Test
         run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
 
@@ -320,12 +313,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       # No Electron setup here: `t3` (apps/server) has no Electron dependency
       # and none of its tests touch the runtime. Only the non-server `test`
@@ -521,7 +510,11 @@ jobs:
       # Runs t3#build (which depends on @t3tools/web#build) and
       # @t3tools/desktop#build, so apps/server/dist holds the server bundle
       # plus the web client and apps/desktop/dist-electron the Electron main.
+      # The release channel is already stable, nightly, or preview. Builds in
+      # other repositories report "dev" telemetry.
       - name: Build JS bundle
+        env:
+          T3CODE_TELEMETRY_CHANNEL: ${{ github.repository == 'pingdotgg/t3code' && needs.preflight.outputs.release_channel || 'dev' }}
         run: vp run build:desktop
 
       # Two paths under apps/ so the artifact root is apps/; consumers download
@@ -558,7 +551,7 @@ jobs:
       clerk_cli_oauth_client_id: ${{ needs.relay_public_config.outputs.clerk_cli_oauth_client_id }}
       relay_url: ${{ needs.relay_public_config.outputs.relay_url }}
       label: macOS arm64
-      runner: blacksmith-12vcpu-macos-26
+      runner: macos-26-xlarge
       platform: mac
       target: dmg
       arch: arm64
@@ -581,7 +574,7 @@ jobs:
       clerk_cli_oauth_client_id: ${{ needs.relay_public_config.outputs.clerk_cli_oauth_client_id }}
       relay_url: ${{ needs.relay_public_config.outputs.relay_url }}
       label: macOS x64
-      runner: blacksmith-12vcpu-macos-26
+      runner: macos-26-xlarge
       platform: mac
       target: dmg
       arch: x64
diff --git a/apps/server/src/cloud/CliTokenManager.test.ts b/apps/server/src/cloud/CliTokenManager.test.ts
index a17e33f12..76e8efa3d 100644
--- a/apps/server/src/cloud/CliTokenManager.test.ts
+++ b/apps/server/src/cloud/CliTokenManager.test.ts
@@ -14,6 +14,7 @@ import * as Schema from "effect/Schema";
 import * as Terminal from "effect/Terminal";
 import * as TestClock from "effect/testing/TestClock";
 import * as HttpClient from "effect/http/HttpClient";
+import * as HttpClientError from "effect/http/HttpClientError";
 import * as HttpClientResponse from "effect/http/HttpClientResponse";
 
 import * as ServerSecretStore from "../auth/ServerSecretStore.ts";
@@ -184,6 +185,205 @@ const expiredStoredToken = new TextEncoder().encode(
   JSON.stringify({ accessToken: "expired", refreshToken: "refresh-token-1", expiresAtEpochMs: 0 }),
 );
 
+const provideRefreshManager = (
+  httpClient: HttpClient.HttpClient,
+  secrets = Layer.mock(ServerSecretStore.ServerSecretStore)({
+    get: () => Effect.succeedSome(expiredStoredToken),
+  }),
+) =>
+  Effect.provide(
+    CliTokenManager.layer.pipe(
+      Layer.provide(
+        Layer.mergeAll(
+          Layer.succeed(HttpClient.HttpClient, httpClient),
+          secrets,
+          Layer.mock(ExternalLauncher.ExternalLauncher)({}),
+          NodeServices.layer,
+        ),
+      ),
+      Layer.provideMerge(ConfigProvider.layer(ConfigProvider.fromEnv({ env: TEST_ENV }))),
+    ),
+  );
+
+it.effect(
+  "persists rotated credentials under the lock despite shutdown, while queued callers can cancel",
+  () =>
+    Effect.gen(function* () {
+      const requestStarted = yield* Deferred.make<void>();
+      const releaseRequest = yield* Deferred.make<void>();
+      const persistStarted = yield* Deferred.make<void>();
+      const releasePersist = yield* Deferred.make<void>();
+      let stored = expiredStoredToken;
+      let reads = 0;
+      const httpClient = HttpClient.make((request) =>
+        Effect.gen(function* () {
+          yield* Deferred.succeed(requestStarted, undefined);
+          yield* Deferred.await(releaseRequest);
+          return HttpClientResponse.fromWeb(
+            request,
+            Response.json({
+              access_token: "rotated-access",
+              refresh_token: "rotated-refresh",
+              expires_in: 3600,
+              token_type: "bearer",
+            }),
+          );
+        }),
+      );
+      const secrets = Layer.mock(ServerSecretStore.ServerSecretStore)({
+        get: () =>
+          Effect.sync(() => {
+            reads++;
+         
… diff truncated …
```

## nightly (v0.1.1-nightly.20261020.3411)

```diff
 .github/workflows/release.yml                      |  45 +-
 apps/server/src/cloud/CliTokenManager.test.ts      | 204 +++++-
 apps/server/src/cloud/CliTokenManager.ts           |  92 ++-
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  | 166 ++++-
 apps/server/src/cloud/CloudLink.test.ts            |   4 +
 apps/server/src/cloud/CloudLink.ts                 |  34 +-
 apps/server/src/cloud/ConnectReachability.test.ts  | 153 ++++
 apps/server/src/cloud/ConnectReachability.ts       | 312 ++++++++
 .../src/cloud/ManagedEndpointRuntime.test.ts       | 210 ++++++
 apps/server/src/cloud/ManagedEndpointRuntime.ts    | 213 ++++--
 apps/server/src/cloud/bootService.test.ts          | 195 ++++-
 apps/server/src/cloud/bootService.ts               | 198 +++--
 apps/server/src/cloud/config.ts                    |  10 +-
 apps/server/src/cloud/http.test.ts                 | 265 ++++++-
 apps/server/src/cloud/http.ts                      |   5 +
 apps/server/src/cloud/linkChecks.ts                |   3 +-
 apps/server/src/cloud/managedTunnelStartup.test.ts | 206 ++++++
 apps/server/src/cloud/managedTunnelStartup.ts      | 189 ++++-
 apps/server/src/cloud/pinnedRuntime.test.ts        | 161 +++++
 apps/server/src/cloud/pinnedRuntime.ts             |  30 +-
 apps/server/src/cloud/publicConfig.test.ts         |  26 +
 apps/server/src/cloud/publicConfig.ts              |   5 +
 apps/server/src/cloud/relayResponse.test.ts        |  50 ++
 apps/server/src/cloud/relayResponse.ts             |  48 +-
 apps/server/src/cloud/selfUpdate.test.ts           |  49 +-
 apps/server/src/cloud/selfUpdate.ts                |  81 ++-
 .../server/src/cloud/serviceLauncherClient.test.ts |  20 +-
 apps/server/src/cloud/serviceLauncherClient.ts     |  13 +-
 apps/server/src/cloud/serviceProtocol.ts           |   1 +
 .../server/src/provider/AntigravityInstallation.ts |  17 +-
 apps/server/src/provider/CodexChatGptAuth.ts       |  91 ++-
 apps/server/src/provider/CodexInstallation.ts      |  42 ++
 apps/server/src/provider/CodexProvider.ts          | 223 ++++--
 .../src/provider/Drivers/AntigravityDriver.ts      | 801 ++++++++++----------
 .../src/provider/Drivers/AntigravityUsage.ts       |  10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   | 516 +++++++------
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |  38 +-
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |  43 +-
 .../src/provider/Drivers/ClaudeSkills.test.ts      | 333 +++++++++
 apps/server/src/provider/Drivers/ClaudeSkills.ts   | 272 ++++++-
 .../src/provider/Drivers/CodexDriver.test.ts       |  40 +
 apps/server/src/provider/Drivers/CodexDriver.ts    | 453 ++++++------
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |  39 +
 .../server/src/provider/Drivers/CodexHomeLayout.ts |   3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |  51 +-
 .../src/provider/Drivers/antigravityUsageReader.ts |  78 +-
 .../src/provider/Drivers/claudeUsage.test.ts       |  31 +-
 apps/server/src/provider/Drivers/claudeUsage.ts    |   5 +-
 .../server/src/provider/Drivers/codexUsage.test.ts | 255 ++++++-
 apps/server/src/provider/Drivers/codexUsage.ts     | 210 +++++-
 docs/internals/providers.md                        |   3 +-
 .../src/server/AcpRegistrySupport.test.ts          |  49 ++
 .../src/server/AcpRegistrySupport.ts               |  37 +-
 .../provider-acp-registry/src/server/adapter.ts    |  46 +-
 packages/provider-acp-registry/src/server/auth.ts  | 179 +++--
 .../provider-acp-registry/src/server/claudeAcp.ts  | 117 +++
 .../provider-acp-registry/src/server/driver.ts     | 805 +++++++++++----------
 .../provider-acp-registry/src/server/droidAcp.ts   |  84 +++
 packages/provider-acp-registry/src/server/probe.ts |  84 ++-
 packages/provider-grok/src/server/acpSupport.ts    |  51 +-
 packages/provider-grok/src/server/adapter.ts       |  60 +-
 packages/provider-grok/src/server/driver.test.ts   | 126 ++++
 packages/provider-grok/src/server/driver.ts        | 322 +++++----
 packages/provider-grok/src/server/status.ts        | 177 ++---
 .../provider-grok/src/server/textGeneration.ts     |  37 +-
 packages/provider-grok/src/server/usageLimits.ts   |  43 +-
 .../provider-grok/src/server/xaiAcpExtension.ts    | 101 ++-
 packages/provider-muse/src/server/adapter.test.ts  | 494 ++++++++++++-
 packages/provider-muse/src/server/adapter.ts       | 209 ++++--
 packages/provider-muse/src/server/driver.ts        | 249 ++++---
 .../provider-muse/src/server/maintenance.test.ts   |  97 ++-
 packages/provider-muse/src/server/maintenance.ts   |  81 +--
 .../provider-muse/src/server/modelCatalog.test.ts  |  28 +
 packages/provider-muse/src/server/modelCatalog.ts  |   7 +-
 packages/provider-muse/src/server/protocol.ts      |  32 +-
 packages/provider-muse/src/server/status.test.ts   |  31 +-
 .../provider-muse/src/server/usageLimits.test.ts   |  60 ++
 packages/provider-muse/src/server/usageLimits.ts   |  48 ++
 packages/provider-opencode/src/server/driver.ts    | 615 ++++++++--------
 79 files changed, 7980 insertions(+), 2831 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..21e9e7ba9 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -50,6 +50,7 @@ jobs:
   # have already run. Scheduled runs also decide here whether a nightly is due.
   resolve_commit:
     name: Resolve release commit
+    if: github.repository == 'pingdotgg/t3code'
     runs-on: blacksmith-8vcpu-ubuntu-2404
     timeout-minutes: 5
     outputs:
@@ -128,12 +129,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
         env:
           pnpm_config_cache_dir: ${{ runner.temp }}/pnpm-metadata
 
@@ -246,12 +243,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       - name: Ensure Electron runtime is installed
         run: vp run --filter @t3tools/desktop ensure:electron
@@ -280,12 +273,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       - name: Ensure Electron runtime is installed
         run: vp run --filter @t3tools/desktop ensure:electron
@@ -297,6 +286,10 @@ jobs:
           sudo sed -i 's|http://|https://|g' /etc/apt/blacksmith-ubuntu-mirrors.txt /etc/apt/sources.list.d/ubuntu.sources
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
+      # The web package's browser project runs React tests in headless Chromium.
+      - name: Install Chromium for web browser tests
+        run: node apps/web/node_modules/playwright/cli.js install --with-deps --only-shell chromium
+
       - name: Test
         run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
 
@@ -320,12 +313,8 @@ jobs:
             !/.repos/
           sparse-checkout-cone-mode: false
 
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: true
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
 
       # No Electron setup here: `t3` (apps/server) has no Electron dependency
       # and none of its tests touch the runtime. Only the non-server `test`
@@ -521,7 +510,11 @@ jobs:
       # Runs t3#build (which depends on @t3tools/web#build) and
       # @t3tools/desktop#build, so apps/server/dist holds the server bundle
       # plus the web client and apps/desktop/dist-electron the Electron main.
+      # The release channel is already stable, nightly, or preview. Builds in
+      # other repositories report "dev" telemetry.
       - name: Build JS bundle
+        env:
+          T3CODE_TELEMETRY_CHANNEL: ${{ github.repository == 'pingdotgg/t3code' && needs.preflight.outputs.release_channel || 'dev' }}
         run: vp run build:desktop
 
       # Two paths under apps/ so the artifact root is apps/; consumers download
@@ -558,7 +551,7 @@ jobs:
       clerk_cli_oauth_client_id: ${{ needs.relay_public_config.outputs.clerk_cli_oauth_client_id }}
       relay_url: ${{ needs.relay_public_config.outputs.relay_url }}
       label: macOS arm64
-      runner: blacksmith-12vcpu-macos-26
+      runner: macos-26-xlarge
       platform: mac
       target: dmg
       arch: arm64
@@ -581,7 +574,7 @@ jobs:
       clerk_cli_oauth_client_id: ${{ needs.relay_public_config.outputs.clerk_cli_oauth_client_id }}
       relay_url: ${{ needs.relay_public_config.outputs.relay_url }}
       label: macOS x64
-      runner: blacksmith-12vcpu-macos-26
+      runner: macos-26-xlarge
       platform: mac
       target: dmg
       arch: x64
diff --git a/apps/server/src/cloud/CliTokenManager.test.ts b/apps/server/src/cloud/CliTokenManager.test.ts
index a17e33f12..76e8efa3d 100644
--- a/apps/server/src/cloud/CliTokenManager.test.ts
+++ b/apps/server/src/cloud/CliTokenManager.test.ts
@@ -14,6 +14,7 @@ import * as Schema from "effect/Schema";
 import * as Terminal from "effect/Terminal";
 import * as TestClock from "effect/testing/TestClock";
 import * as HttpClient from "effect/http/HttpClient";
+import * as HttpClientError from "effect/http/HttpClientError";
 import * as HttpClientResponse from "effect/http/HttpClientResponse";
 
 import * as ServerSecretStore from "../auth/ServerSecretStore.ts";
@@ -184,6 +185,205 @@ const expiredStoredToken = new TextEncoder().encode(
   JSON.stringify({ accessToken: "expired", refreshToken: "refresh-token-1", expiresAtEpochMs: 0 }),
 );
 
+const provideRefreshManager = (
+  httpClient: HttpClient.HttpClient,
+  secrets = Layer.mock(ServerSecretStore.ServerSecretStore)({
+    get: () => Effect.succeedSome(expiredStoredToken),
+  }),
+) =>
+  Effect.provide(
+    CliTokenManager.layer.pipe(
+      Layer.provide(
+        Layer.mergeAll(
+          Layer.succeed(HttpClient.HttpClient, httpClient),
+          secrets,
+          Layer.mock(ExternalLauncher.ExternalLauncher)({}),
+          NodeServices.layer,
+        ),
+      ),
+      Layer.provideMerge(ConfigProvider.layer(ConfigProvider.fromEnv({ env: TEST_ENV }))),
+    ),
+  );
+
+it.effect(
+  "persists rotated credentials under the lock despite shutdown, while queued callers can cancel",
+  () =>
+    Effect.gen(function* () {
+      const requestStarted = yield* Deferred.make<void>();
+      const releaseRequest = yield* Deferred.make<void>();
+      const persistStarted = yield* Deferred.make<void>();
+      const releasePersist = yield* Deferred.make<void>();
+      let stored = expiredStoredToken;
+      let reads = 0;
+      const httpClient = HttpClient.make((request) =>
+        Effect.gen(function* () {
+          yield* Deferred.succeed(requestStarted, undefined);
+          yield* Deferred.await(releaseRequest);
+          return HttpClientResponse.fromWeb(
+            request,
+            Response.json({
+              access_token: "rotated-access",
+              refresh_token: "rotated-refresh",
+              expires_in: 3600,
+              token_type: "bearer",
+            }),
+          );
+        }),
+      );
+      const secrets = Layer.mock(ServerSecretStore.ServerSecretStore)({
+        get: () =>
+          Effect.sync(() => {
+            reads++;
+         
… diff truncated …
```

Do not move the pin until a maintainer ports a fact and updates the snapshot in the same change.
