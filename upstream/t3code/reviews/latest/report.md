# T3 Code provider review

Pinned commit: `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`

Compared `main` `066c8fc0cd04c97988ff8892e440189545ac6994` and nightly `v0.1.2-nightly.20261021.3473` (`1688f965682d331da16a8c39f049592361f69c8d`) against nightly pin `e3b35d1b2440b104f0bc2e357377c348121234dd`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (cfafb4bb54d2..066c8fc0cd04)

```diff
 .github/workflows/release.yml                      |  133 ++-
 apps/server/src/cloud/CliTokenManager.test.ts      |  328 +++----
 apps/server/src/cloud/CliTokenManager.ts           |  172 +++-
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  512 +++-------
 apps/server/src/cloud/CloudLink.test.ts            |   31 +-
 apps/server/src/cloud/CloudLink.ts                 |   34 +-
 apps/server/src/cloud/ConnectReachability.test.ts  |   89 ++
 apps/server/src/cloud/ConnectReachability.ts       |  312 ++++++
 .../src/cloud/ManagedEndpointRuntime.test.ts       | 1031 ++++----------------
 apps/server/src/cloud/ManagedEndpointRuntime.ts    |  213 +++-
 apps/server/src/cloud/bootService.test.ts          |  358 ++++---
 apps/server/src/cloud/bootService.ts               |  231 +++--
 apps/server/src/cloud/config.ts                    |   10 +-
 apps/server/src/cloud/http.test.ts                 |  292 ++++--
 apps/server/src/cloud/http.ts                      |    5 +
 apps/server/src/cloud/linkChecks.test.ts           |   12 +-
 apps/server/src/cloud/linkChecks.ts                |    3 +-
 apps/server/src/cloud/managedTunnelStartup.test.ts |  105 +-
 apps/server/src/cloud/managedTunnelStartup.ts      |  191 +++-
 apps/server/src/cloud/pinnedRuntime.test.ts        |  288 ++++--
 apps/server/src/cloud/pinnedRuntime.ts             |  297 +++++-
 apps/server/src/cloud/publicConfig.test.ts         |  116 +--
 apps/server/src/cloud/publicConfig.ts              |    5 +
 apps/server/src/cloud/relayResponse.test.ts        |   88 +-
 apps/server/src/cloud/relayResponse.ts             |   48 +-
 apps/server/src/cloud/selfUpdate.test.ts           |  129 +--
 apps/server/src/cloud/selfUpdate.ts                |   96 +-
 .../server/src/cloud/serviceLauncherClient.test.ts |   20 +-
 apps/server/src/cloud/serviceLauncherClient.ts     |   13 +-
 apps/server/src/cloud/servicePreflight.test.ts     |   15 -
 apps/server/src/cloud/serviceProtocol.ts           |    1 +
 apps/server/src/cloud/traceRelayRequest.test.ts    |   39 -
 .../server/src/provider/AntigravityInstallation.ts |   17 +-
 apps/server/src/provider/CodexChatGptAuth.ts       |   91 +-
 apps/server/src/provider/CodexInstallation.ts      |   44 +-
 apps/server/src/provider/CodexProvider.ts          |  217 ++--
 .../src/provider/Drivers/AntigravityDriver.test.ts |  140 ---
 .../src/provider/Drivers/AntigravityDriver.ts      |  801 +++++++--------
 .../src/provider/Drivers/AntigravitySkills.test.ts |   65 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |   10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  516 +++++-----
 .../src/provider/Drivers/ClaudeExecutable.test.ts  |   34 -
 .../server/src/provider/Drivers/ClaudeHome.test.ts |   29 -
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |   49 +-
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |   43 +-
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  501 ++++++----
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  274 +++++-
 .../src/provider/Drivers/CodexDriver.test.ts       |   81 +-
 apps/server/src/provider/Drivers/CodexDriver.ts    |  453 ++++-----
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |   94 +-
 .../server/src/provider/Drivers/CodexHomeLayout.ts |    3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |   51 +-
 .../Drivers/antigravityUsageReader.test.ts         |  158 +--
 .../src/provider/Drivers/antigravityUsageReader.ts |   78 +-
 .../src/provider/Drivers/claudeUsage.test.ts       |   40 +-
 apps/server/src/provider/Drivers/claudeUsage.ts    |    5 +-
 .../server/src/provider/Drivers/codexUsage.test.ts |  304 +++++-
 apps/server/src/provider/Drivers/codexUsage.ts     |  209 +++-
 docs/internals/providers.md                        |    3 +-
 docs/operations/release.md                         |    6 +-
 .../server/AcpRegistryRuntimeCoordinator.test.ts   |   78 +-
 .../src/server/AcpRegistrySupport.test.ts          |  264 -----
 .../src/server/AcpRegistrySupport.ts               |   39 +-
 .../provider-acp-registry/src/server/adapter.ts    |   46 +-
 .../provider-acp-registry/src/server/auth.test.ts  |   17 -
 packages/provider-acp-registry/src/server/auth.ts  |  179 ++--
 .../provider-acp-registry/src/server/claudeAcp.ts  |  117 +++
 .../src/server/driver.test.ts                      |   53 -
 .../provider-acp-registry/src/server/driver.ts     |  807 +++++++--------
 .../provider-acp-registry/src/server/droidAcp.ts   |   84 ++
 packages/provider-acp-registry/src/server/probe.ts |   84 +-
 .../provider-grok/src/server/acpCliProbe.test.ts   |   93 --
 .../provider-grok/src/server/acpSupport.test.ts    |  139 +--
 packages/provider-grok/src/server/acpSupport.ts    |   55 +-
 packages/provider-grok/src/server/adapter.ts       |   62 +-
 packages/provider-grok/src/server/driver.test.ts   |  153 ++-
 packages/provider-grok/src/server/driver.ts        |  322 +++---
 packages/provider-grok/src/server/status.ts        |  179 ++--
 .../provider-grok/src/server/textGeneration.ts     |   37 +-
 packages/provider-grok/src/server/usage.test.ts    |   39 -
 packages/provider-grok/src/server/usageLimits.ts   |   43 +-
 .../src/server/xaiAcpExtension.test.ts             |  710 +++-----------
 .../provider-grok/src/server/xaiAcpExtension.ts    |  149 +--
 .../src/server/xaiBackgroundTasks.test.ts          |   12 -
 packages/provider-muse/src/server/adapter.test.ts  |  475 ++++++++-
 packages/provider-muse/src/server/adapter.ts       |  209 ++--
 packages/provider-muse/src/server/driver.ts        |  249 ++---
 .../src/server/itemPresentation.test.ts            |   32 -
 .../provider-muse/src/server/maintenance.test.ts   |  112 ---
 packages/provider-muse/src/server/maintenance.ts   |   81 +-
 .../provider-muse/src/server/modelCatalog.test.ts  |   80 +-
 packages/provider-muse/src/server/modelCatalog.ts  |    7 +-
 packages/provider-muse/src/server/protocol.ts      |   32 +-
 packages/provider-muse/src/server/sdk.test.ts      |  201 ----
 packages/provider-muse/src/server/status.test.ts   |   50 +-
 .../src/server/textGeneration.test.ts              |   89 +-
 .../provider-muse/src/server/usageLimits.test.ts   |   60 ++
 packages/provider-muse/src/server/usageLimits.ts   |   48 +
 packages/provider-opencode/src/server/driver.ts    |  615 ++++++------
 99 files changed, 7901 insertions(+), 7733 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..6a80220c2 100644
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
@@ -263,7 +256,7 @@ jobs:
         run: vp run typecheck
 
   # Keep tests on their own runners, using the same package split and server
-  # shards as CI, so the release check job does not spend its budget on tests.
+  # shards as CI (test, test_web, test_server in ci.yml). Keep the two in sync.
   test:
     name: Release tests
     needs: [preflight]
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
@@ -298,18 +287,66 @@ jobs:
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
       - name: Test
-        run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
+        run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' --filter '!@t3tools/web' test
+
+  # Web browser tests time out when they share a runner with the other package
+  # suites: Chromium gets too little CPU and diff workers miss their 10s start budget.
+  test_web:
+    name: Release web tests
+    needs: [preflight]
+    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' }}
+    runs-on: blacksmith-8vcpu-ubuntu-2404
+    timeout-minutes: 10
+    steps:
+      - name: Checkout
+        uses: actions/checkout@v6
+        with:
+          ref: ${{ needs.preflight.outputs.ref }}
+          sparse-checkout: |
+            /*
+            !/.repos/
+          sparse-checkout-cone-mode: false
+
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
+
+      - name: Resolve Playwright version
+        id: playwright_version
+        run: echo "version=$(node -p "require('./apps/web/node_modules/playwright/package.json').version")" >> "$GITHUB_OUTPUT"
+
+      - name: Cache Chromium
+        id: chromium_cache
+        uses: actions/cache@v6
+        with:
+          path: ~/.cache/ms-playwright
+          key: playwright-chromium-shell-${{ runner.os }}-${{ runner.arch }}-${{ steps.playwright_version.outputs.version }}
+
+      - name: Install Chromium for web browser tests
+        if: steps.chromium_cache.outputs.cache-hit != 'true'
+        run: node apps/web/node_modules/playwright/cli.js install --only-shell chromium
+
+      - name: Install missing Chromium system dependencies
+        run: |
+          browser=$(find "$HOME/.cache/ms-playwright" -name chrome-headless-shell -type f -print -quit)
+          test -n "$browser"
+          if ldd "$browser" | grep '=> not found'; then
+            node apps/web/node_modules/playwright/cli.js install-deps chromium
+          fi
+
+      - name: Test
+        run: vp run --filter @t3tools/web test
 
+  # Server test files run one at a time, so more small runners beat fewer big ones.
   test_server:
     name: Release server tests ${{ matrix.shard }}
     needs: [preflight]
     if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' }}
-    runs-on: blacksmith-8vcpu-ubuntu-2404
+    runs-on: blacksmith-4vcpu-ubuntu-2404
     timeout-minutes: 10
     strategy:
       fail-fast: false
       matrix:
-        shard: [1, 2, 3]
+        shard: [1, 2, 3, 4, 5, 6, 7, 8]
     steps:
       - name: Checkout
         uses: actions/checkout@v6
@@ -320,12 +357,8 @@ jobs:
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
@@ -521,7 +554,11 @@ jobs:
       # Runs t3#build (which depends on @t3tools/web#build) and
       # @t3tools/desktop#build, so apps/ser
… diff truncated …
```

## nightly (v0.1.2-nightly.20261021.3473)

```diff
 .github/workflows/release.yml                      |  133 ++-
 apps/server/src/cloud/CliTokenManager.test.ts      |  328 +++----
 apps/server/src/cloud/CliTokenManager.ts           |  172 +++-
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  512 +++-------
 apps/server/src/cloud/CloudLink.test.ts            |   31 +-
 apps/server/src/cloud/CloudLink.ts                 |   34 +-
 apps/server/src/cloud/ConnectReachability.test.ts  |   89 ++
 apps/server/src/cloud/ConnectReachability.ts       |  312 ++++++
 .../src/cloud/ManagedEndpointRuntime.test.ts       | 1031 ++++----------------
 apps/server/src/cloud/ManagedEndpointRuntime.ts    |  213 +++-
 apps/server/src/cloud/bootService.test.ts          |  358 ++++---
 apps/server/src/cloud/bootService.ts               |  231 +++--
 apps/server/src/cloud/config.ts                    |   10 +-
 apps/server/src/cloud/http.test.ts                 |  292 ++++--
 apps/server/src/cloud/http.ts                      |    5 +
 apps/server/src/cloud/linkChecks.test.ts           |   12 +-
 apps/server/src/cloud/linkChecks.ts                |    3 +-
 apps/server/src/cloud/managedTunnelStartup.test.ts |  105 +-
 apps/server/src/cloud/managedTunnelStartup.ts      |  191 +++-
 apps/server/src/cloud/pinnedRuntime.test.ts        |  288 ++++--
 apps/server/src/cloud/pinnedRuntime.ts             |  297 +++++-
 apps/server/src/cloud/publicConfig.test.ts         |  116 +--
 apps/server/src/cloud/publicConfig.ts              |    5 +
 apps/server/src/cloud/relayResponse.test.ts        |   88 +-
 apps/server/src/cloud/relayResponse.ts             |   48 +-
 apps/server/src/cloud/selfUpdate.test.ts           |  129 +--
 apps/server/src/cloud/selfUpdate.ts                |   96 +-
 .../server/src/cloud/serviceLauncherClient.test.ts |   20 +-
 apps/server/src/cloud/serviceLauncherClient.ts     |   13 +-
 apps/server/src/cloud/servicePreflight.test.ts     |   15 -
 apps/server/src/cloud/serviceProtocol.ts           |    1 +
 apps/server/src/cloud/traceRelayRequest.test.ts    |   39 -
 .../server/src/provider/AntigravityInstallation.ts |   17 +-
 apps/server/src/provider/CodexChatGptAuth.ts       |   91 +-
 apps/server/src/provider/CodexInstallation.ts      |   44 +-
 apps/server/src/provider/CodexProvider.ts          |  223 +++--
 .../src/provider/Drivers/AntigravityDriver.test.ts |  140 ---
 .../src/provider/Drivers/AntigravityDriver.ts      |  801 +++++++--------
 .../src/provider/Drivers/AntigravitySkills.test.ts |   65 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |   10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  516 +++++-----
 .../src/provider/Drivers/ClaudeExecutable.test.ts  |   34 -
 .../server/src/provider/Drivers/ClaudeHome.test.ts |   29 -
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |   49 +-
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |   43 +-
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  501 ++++++----
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  274 +++++-
 .../src/provider/Drivers/CodexDriver.test.ts       |   81 +-
 apps/server/src/provider/Drivers/CodexDriver.ts    |  453 ++++-----
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |   94 +-
 .../server/src/provider/Drivers/CodexHomeLayout.ts |    3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |   51 +-
 .../Drivers/antigravityUsageReader.test.ts         |  158 +--
 .../src/provider/Drivers/antigravityUsageReader.ts |   78 +-
 .../src/provider/Drivers/claudeUsage.test.ts       |   40 +-
 apps/server/src/provider/Drivers/claudeUsage.ts    |    5 +-
 .../server/src/provider/Drivers/codexUsage.test.ts |  304 +++++-
 apps/server/src/provider/Drivers/codexUsage.ts     |  209 +++-
 docs/internals/providers.md                        |    3 +-
 docs/operations/release.md                         |    6 +-
 .../server/AcpRegistryRuntimeCoordinator.test.ts   |   78 +-
 .../src/server/AcpRegistrySupport.test.ts          |  264 -----
 .../src/server/AcpRegistrySupport.ts               |   39 +-
 .../provider-acp-registry/src/server/adapter.ts    |   46 +-
 .../provider-acp-registry/src/server/auth.test.ts  |   17 -
 packages/provider-acp-registry/src/server/auth.ts  |  179 ++--
 .../provider-acp-registry/src/server/claudeAcp.ts  |  117 +++
 .../src/server/driver.test.ts                      |   53 -
 .../provider-acp-registry/src/server/driver.ts     |  807 +++++++--------
 .../provider-acp-registry/src/server/droidAcp.ts   |   84 ++
 packages/provider-acp-registry/src/server/probe.ts |   84 +-
 .../provider-grok/src/server/acpCliProbe.test.ts   |   93 --
 .../provider-grok/src/server/acpSupport.test.ts    |  139 +--
 packages/provider-grok/src/server/acpSupport.ts    |   55 +-
 packages/provider-grok/src/server/adapter.ts       |   62 +-
 packages/provider-grok/src/server/driver.test.ts   |  153 ++-
 packages/provider-grok/src/server/driver.ts        |  322 +++---
 packages/provider-grok/src/server/status.ts        |  179 ++--
 .../provider-grok/src/server/textGeneration.ts     |   37 +-
 packages/provider-grok/src/server/usage.test.ts    |   39 -
 packages/provider-grok/src/server/usageLimits.ts   |   43 +-
 .../src/server/xaiAcpExtension.test.ts             |  710 +++-----------
 .../provider-grok/src/server/xaiAcpExtension.ts    |  149 +--
 .../src/server/xaiBackgroundTasks.test.ts          |   12 -
 packages/provider-muse/src/server/adapter.test.ts  |  475 ++++++++-
 packages/provider-muse/src/server/adapter.ts       |  209 ++--
 packages/provider-muse/src/server/driver.ts        |  249 ++---
 .../src/server/itemPresentation.test.ts            |   32 -
 .../provider-muse/src/server/maintenance.test.ts   |  112 ---
 packages/provider-muse/src/server/maintenance.ts   |   81 +-
 .../provider-muse/src/server/modelCatalog.test.ts  |   80 +-
 packages/provider-muse/src/server/modelCatalog.ts  |    7 +-
 packages/provider-muse/src/server/protocol.ts      |   32 +-
 packages/provider-muse/src/server/sdk.test.ts      |  201 ----
 packages/provider-muse/src/server/status.test.ts   |   50 +-
 .../src/server/textGeneration.test.ts              |   89 +-
 .../provider-muse/src/server/usageLimits.test.ts   |   60 ++
 packages/provider-muse/src/server/usageLimits.ts   |   48 +
 packages/provider-opencode/src/server/driver.ts    |  615 ++++++------
 99 files changed, 7906 insertions(+), 7734 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..6a80220c2 100644
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
@@ -263,7 +256,7 @@ jobs:
         run: vp run typecheck
 
   # Keep tests on their own runners, using the same package split and server
-  # shards as CI, so the release check job does not spend its budget on tests.
+  # shards as CI (test, test_web, test_server in ci.yml). Keep the two in sync.
   test:
     name: Release tests
     needs: [preflight]
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
@@ -298,18 +287,66 @@ jobs:
           sudo apt-get update && sudo apt-get install -y libsecret-1-dev pkg-config build-essential
 
       - name: Test
-        run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' test
+        run: vp run --parallel --concurrency-limit 4 --filter '!t3' --filter '!@t3tools/monorepo' --filter '!@t3tools/web' test
+
+  # Web browser tests time out when they share a runner with the other package
+  # suites: Chromium gets too little CPU and diff workers miss their 10s start budget.
+  test_web:
+    name: Release web tests
+    needs: [preflight]
+    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' }}
+    runs-on: blacksmith-8vcpu-ubuntu-2404
+    timeout-minutes: 10
+    steps:
+      - name: Checkout
+        uses: actions/checkout@v6
+        with:
+          ref: ${{ needs.preflight.outputs.ref }}
+          sparse-checkout: |
+            /*
+            !/.repos/
+          sparse-checkout-cone-mode: false
+
+      - name: Setup dependencies
+        uses: ./.github/actions/setup-ci-deps
+
+      - name: Resolve Playwright version
+        id: playwright_version
+        run: echo "version=$(node -p "require('./apps/web/node_modules/playwright/package.json').version")" >> "$GITHUB_OUTPUT"
+
+      - name: Cache Chromium
+        id: chromium_cache
+        uses: actions/cache@v6
+        with:
+          path: ~/.cache/ms-playwright
+          key: playwright-chromium-shell-${{ runner.os }}-${{ runner.arch }}-${{ steps.playwright_version.outputs.version }}
+
+      - name: Install Chromium for web browser tests
+        if: steps.chromium_cache.outputs.cache-hit != 'true'
+        run: node apps/web/node_modules/playwright/cli.js install --only-shell chromium
+
+      - name: Install missing Chromium system dependencies
+        run: |
+          browser=$(find "$HOME/.cache/ms-playwright" -name chrome-headless-shell -type f -print -quit)
+          test -n "$browser"
+          if ldd "$browser" | grep '=> not found'; then
+            node apps/web/node_modules/playwright/cli.js install-deps chromium
+          fi
+
+      - name: Test
+        run: vp run --filter @t3tools/web test
 
+  # Server test files run one at a time, so more small runners beat fewer big ones.
   test_server:
     name: Release server tests ${{ matrix.shard }}
     needs: [preflight]
     if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' }}
-    runs-on: blacksmith-8vcpu-ubuntu-2404
+    runs-on: blacksmith-4vcpu-ubuntu-2404
     timeout-minutes: 10
     strategy:
       fail-fast: false
       matrix:
-        shard: [1, 2, 3]
+        shard: [1, 2, 3, 4, 5, 6, 7, 8]
     steps:
       - name: Checkout
         uses: actions/checkout@v6
@@ -320,12 +357,8 @@ jobs:
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
@@ -521,7 +554,11 @@ jobs:
       # Runs t3#build (which depends on @t3tools/web#build) and
       # @t3tools/desktop#build, so apps/se
… diff truncated …
```

Do not move the pin until a maintainer ports a fact and updates the snapshot in the same change.
