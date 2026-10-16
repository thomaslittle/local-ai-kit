# T3 Code provider review

Pinned commit: `42da166d131017335a1158d0b379dd68b8985796`

Compared `main` `cfafb4bb54d244e4612f5681aad3832ea64cb0fb` and nightly `v0.1.1-nightly.20261016.3203` (`e3b35d1b2440b104f0bc2e357377c348121234dd`) against nightly pin `e6bbfdb9ffe8334bed1c9b992f828f04d30db7ea`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (42da166d1310..cfafb4bb54d2)

```diff
 .github/workflows/release.yml                      | 133 +++++++++---------
 apps/server/src/cloud/CliState.test.ts             |   1 -
 apps/server/src/cloud/CliState.ts                  |  11 --
 apps/server/src/cloud/CliTokenManager.test.ts      |   7 -
 apps/server/src/cloud/CliTokenManager.ts           |  26 ----
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  44 +-----
 apps/server/src/cloud/CloudLink.test.ts            |   5 -
 apps/server/src/cloud/CloudLink.ts                 | 109 +++------------
 .../src/cloud/ManagedEndpointRuntime.test.ts       |  32 -----
 apps/server/src/cloud/ManagedEndpointRuntime.ts    |  68 +++------
 apps/server/src/cloud/bootService.test.ts          |  88 ++++++++++--
 apps/server/src/cloud/bootService.ts               | 155 +++++++++++----------
 apps/server/src/cloud/config.ts                    |  15 +-
 apps/server/src/cloud/http.test.ts                 |   3 -
 apps/server/src/cloud/http.ts                      |   3 -
 apps/server/src/cloud/linkChecks.ts                |   9 --
 apps/server/src/cloud/managedTunnelStartup.test.ts |   1 -
 apps/server/src/cloud/managedTunnelStartup.ts      |   7 +-
 apps/server/src/cloud/pinnedRuntime.test.ts        |   3 -
 apps/server/src/cloud/pinnedRuntime.ts             |  28 +---
 apps/server/src/cloud/publicConfig.ts              |  12 --
 apps/server/src/cloud/relayResponse.ts             |   8 +-
 apps/server/src/cloud/selfUpdate.test.ts           |   4 -
 apps/server/src/cloud/selfUpdate.ts                |   5 -
 apps/server/src/cloud/servicePreflight.ts          |   1 -
 apps/server/src/cloud/serviceProtocol.ts           |   5 -
 apps/server/src/cloud/traceRelayRequest.test.ts    |   2 -
 apps/server/src/cloud/traceRelayRequest.ts         |  15 --
 apps/server/src/cloud/updateHandoff.ts             |  13 +-
 .../server/src/provider/AntigravityInstallation.ts |  38 ++++-
 apps/server/src/provider/CodexChatGptAuth.ts       |   6 -
 apps/server/src/provider/CodexChatGptHandoff.ts    |   1 -
 .../server/src/provider/CodexChatGptSessionLock.ts |   1 -
 apps/server/src/provider/CodexInstallation.ts      |  48 +++++--
 .../src/provider/Drivers/AntigravityDriver.test.ts |   4 -
 .../src/provider/Drivers/AntigravityDriver.ts      |  11 +-
 .../src/provider/Drivers/AntigravitySkills.test.ts |   1 -
 .../src/provider/Drivers/AntigravitySkills.ts      |  12 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |  14 --
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  24 +---
 .../src/provider/Drivers/ClaudeExecutable.ts       |  28 +---
 apps/server/src/provider/Drivers/ClaudeHome.ts     |  17 +--
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |   1 -
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |  35 +----
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  11 --
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  77 ++--------
 .../src/provider/Drivers/CodexDriver.test.ts       |   6 +-
 apps/server/src/provider/Drivers/CodexDriver.ts    |  46 +-----
 .../src/provider/Drivers/CodexManagedProvider.ts   |   1 -
 .../Drivers/antigravityUsageReader.test.ts         |  10 --
 .../src/provider/Drivers/antigravityUsageReader.ts |  22 +--
 .../src/provider/Drivers/claudeUsage.test.ts       |   1 -
 apps/server/src/provider/Drivers/claudeUsage.ts    |  20 +--
 .../server/src/provider/Drivers/codexUsage.test.ts |   7 -
 apps/server/src/provider/Drivers/codexUsage.ts     |  43 +-----
 apps/server/src/provider/builtInDrivers.ts         |  38 +----
 apps/server/src/provider/builtInProviderCatalog.ts |   6 -
 apps/server/src/provider/providerInstallation.ts   |   1 -
 docs/operations/release.md                         |  94 ++++++++-----
 .../src/server/AcpRegistryRuntimeCoordinator.ts    |   3 -
 .../src/server/AcpRegistrySupport.test.ts          | 129 ++++++++++++++++-
 .../src/server/AcpRegistrySupport.ts               |  78 ++++++-----
 .../provider-acp-registry/src/server/adapter.ts    |  19 +--
 packages/provider-acp-registry/src/server/auth.ts  |   5 +-
 .../src/server/authenticationState.test.ts         |   1 -
 .../src/server/authenticationState.ts              |   1 -
 .../provider-acp-registry/src/server/copilotAcp.ts |  15 +-
 .../provider-acp-registry/src/server/devinAcp.ts   |   1 -
 .../provider-acp-registry/src/server/driver.ts     |   6 -
 packages/provider-acp-registry/src/server/probe.ts |   7 -
 packages/provider-cursor/src/settings.ts           |   7 -
 .../provider-grok/src/server/acpCliProbe.test.ts   |  10 --
 packages/provider-grok/src/server/acpSupport.ts    |  22 +--
 packages/provider-grok/src/server/adapter.ts       |  22 +--
 packages/provider-grok/src/server/driver.test.ts   |   1 -
 packages/provider-grok/src/server/driver.ts        |   3 +-
 .../provider-grok/src/server/probeEnvironment.ts   |  12 ++
 packages/provider-grok/src/server/skills.test.ts   |  37 ++++-
 packages/provider-grok/src/server/skills.ts        |  34 +----
 packages/provider-grok/src/server/status.ts        |  36 ++---
 packages/provider-grok/src/server/usage.test.ts    |   1 -
 packages/provider-grok/src/server/usage.ts         |  27 +---
 packages/provider-grok/src/server/usageLimits.ts   |  15 +-
 .../src/server/xaiAcpExtension.test.ts             |  17 ---
 .../provider-grok/src/server/xaiAcpExtension.ts    | 144 +++----------------
 .../provider-grok/src/server/xaiBackgroundTasks.ts |   1 -
 packages/provider-muse/src/server/adapter.test.ts  |  10 --
 packages/provider-muse/src/server/adapter.ts       |  71 +---------
 .../provider-muse/src/server/itemPresentation.ts   |   4 -
 .../provider-muse/src/server/maintenance.test.ts   |   1 -
 packages/provider-muse/src/server/maintenance.ts   |   5 -
 .../provider-muse/src/server/modelCatalog.test.ts  |   1 -
 packages/provider-muse/src/server/modelCatalog.ts  |   4 -
 packages/provider-muse/src/server/protocol.ts      |   9 +-
 packages/provider-muse/src/server/sdk.test.ts      |   5 -
 packages/provider-muse/src/server/sdk.ts           |  33 ++---
 packages/provider-muse/src/server/status.ts        |   1 -
 .../provider-muse/src/server/textGeneration.ts     |   3 -
 packages/provider-opencode/src/server/driver.ts    |  44 +-----
 packages/provider-pi/src/server/usage.ts           |   3 +-
 packages/provider-pi/src/settings.ts               |   7 +-
 101 files changed, 773 insertions(+), 1519 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index fb4927a8..972b85c8 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -689,11 +689,8 @@ jobs:
       resource_key: win32-arm64
       cli_archive: true
 
-  # npm gets the same bytes as the GitHub Release: the launcher plus one
-  # package per CLI archive. Preview publishes too, under the `preview`
-  # dist-tag, which nothing resolves unless asked for by name.
-  publish_cli:
-    name: Publish CLI to npm
+  release:
+    name: Publish GitHub Release
     needs:
       [
         preflight,
@@ -702,74 +699,13 @@ jobs:
         test,
         test_server,
         desktop_mac_arm64,
-        desktop_linux_x64,
-        desktop_linux_arm64,
-        desktop_win_x64,
-        desktop_win_arm64,
-      ]
-    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.relay_public_config.result == 'success' && needs.quality.result == 'success' && needs.test.result == 'success' && needs.test_server.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' }}
-    runs-on: ubuntu-24.04 # blacksmith-8vcpu-ubuntu-2404
-    timeout-minutes: 15
-    permissions:
-      contents: read
-      id-token: write
-    steps:
-      - name: Checkout
-        uses: actions/checkout@v6
-        with:
-          ref: ${{ needs.preflight.outputs.ref }}
-          sparse-checkout: |
-            /*
-            !/.repos/
-          sparse-checkout-cone-mode: false
-
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: |
-            args:
-              - --filter=t3...
-              - --filter=@t3tools/scripts...
-
-      - name: Download all CLI archives
-        uses: actions/download-artifact@v8
-        with:
-          pattern: cli-*
-          merge-multiple: true
-          path: release-cli
-
-      - name: Build npm packages from CLI archives
-        run: node scripts/build-npm-platform-packages.ts --archives-dir release-cli --version "${{ needs.preflight.outputs.version }}" --output-dir npm-packages
-
-      # A dry run of every package first: an auth or scope error here (the
-      # @t3code org missing, a package without a trusted publisher) fails
-      # before anything is live, instead of after some platforms already are.
-      - name: Check npm publish access (dry run)
-        run: |
-          if ! node apps/server/scripts/cli.ts publish --packages-dir npm-packages --tag "${{ needs.preflight.outputs.cli_dist_tag }}" --provenance --dry-run --verbose; then
-            echo "::error::npm publish --dry-run failed. Make sure the @t3code npm org exists and that t3 and every @t3code/t3-<platform> package has a trusted publisher registered for .github/workflows/release.yml (see docs/operations/release.md)." >&2
-            exit 1
-          fi
-
-      - name: Publish CLI packages
-        run: node apps/server/scripts/cli.ts publish --packages-dir npm-packages --tag "${{ needs.preflight.outputs.cli_dist_tag }}" --provenance --verbose
-
-  release:
-    name: Publish GitHub Release
-    needs:
-      [
-        preflight,
-        desktop_mac_arm64,
         desktop_mac_x64,
         desktop_linux_x64,
         desktop_linux_arm64,
         desktop_win_x64,
         desktop_win_arm64,
-        publish_cli,
       ]
-    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_mac_x64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' && needs.publish_cli.result == 'success' }}
+    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.relay_public_config.result == 'success' && needs.quality.result == 'success' && needs.test.result == 'success' && needs.test_server.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_mac_x64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' }}
     runs-on: blacksmith-8vcpu-ubuntu-2404
     timeout-minutes: 30
     permissions:
@@ -807,6 +743,18 @@ jobs:
           merge-multiple: true
           path: release-assets
 
+      - name: Download internal Android APK for Nightly
+        if: needs.preflight.outputs.release_channel == 'nightly'
+        continue-on-error: true
+        env:
+          GOOGLE_PLAY_SERVICE_ACCOUNT_JSON: ${{ secrets.GOOGLE_PLAY_SERVICE_ACCOUNT_JSON }}
+          PACKAGE_NAME: com.t3tools.t3code
+          APK_OUTPUT_DIR: release-assets
+        run: |
+          if [ -n "$GOOGLE_PLAY_SERVICE_ACCOUNT_JSON" ]; then
+            node .github/scripts/mirror-play-apk.cjs
+          fi
+
       # Installers verify archives against this file, so it is written from the
       # signed bytes that get uploaded, never from an earlier stage.
       - name: Write CLI archive checksums
@@ -888,6 +836,10 @@ jobs:
               echo 'release-assets/*.yml'
             fi
             echo 'release-assets/*.tar.gz'
+            # Only list APKs when Play has a processed internal build available.
… diff truncated …
```

## nightly (v0.1.1-nightly.20261016.3203)

```diff
 .github/workflows/release.yml                      | 133 +++++++++---------
 apps/server/src/cloud/CliState.test.ts             |   1 -
 apps/server/src/cloud/CliState.ts                  |  11 --
 apps/server/src/cloud/CliTokenManager.test.ts      |   7 -
 apps/server/src/cloud/CliTokenManager.ts           |  26 ----
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  44 +-----
 apps/server/src/cloud/CloudLink.test.ts            |   5 -
 apps/server/src/cloud/CloudLink.ts                 | 109 +++------------
 .../src/cloud/ManagedEndpointRuntime.test.ts       |  32 -----
 apps/server/src/cloud/ManagedEndpointRuntime.ts    |  68 +++------
 apps/server/src/cloud/bootService.test.ts          |  88 ++++++++++--
 apps/server/src/cloud/bootService.ts               | 155 +++++++++++----------
 apps/server/src/cloud/config.ts                    |  15 +-
 apps/server/src/cloud/http.test.ts                 |   3 -
 apps/server/src/cloud/http.ts                      |   3 -
 apps/server/src/cloud/linkChecks.ts                |   9 --
 apps/server/src/cloud/managedTunnelStartup.test.ts |   1 -
 apps/server/src/cloud/managedTunnelStartup.ts      |   7 +-
 apps/server/src/cloud/pinnedRuntime.test.ts        |   3 -
 apps/server/src/cloud/pinnedRuntime.ts             |  28 +---
 apps/server/src/cloud/publicConfig.ts              |  12 --
 apps/server/src/cloud/relayResponse.ts             |   8 +-
 apps/server/src/cloud/selfUpdate.test.ts           |   4 -
 apps/server/src/cloud/selfUpdate.ts                |   5 -
 apps/server/src/cloud/servicePreflight.ts          |   1 -
 apps/server/src/cloud/serviceProtocol.ts           |   5 -
 apps/server/src/cloud/traceRelayRequest.test.ts    |   2 -
 apps/server/src/cloud/traceRelayRequest.ts         |  15 --
 apps/server/src/cloud/updateHandoff.ts             |  13 +-
 .../server/src/provider/AntigravityInstallation.ts |  38 ++++-
 apps/server/src/provider/CodexChatGptAuth.ts       |   6 -
 apps/server/src/provider/CodexChatGptHandoff.ts    |   1 -
 .../server/src/provider/CodexChatGptSessionLock.ts |   1 -
 apps/server/src/provider/CodexInstallation.ts      |  48 +++++--
 .../src/provider/Drivers/AntigravityDriver.test.ts |   4 -
 .../src/provider/Drivers/AntigravityDriver.ts      |  11 +-
 .../src/provider/Drivers/AntigravitySkills.test.ts |   1 -
 .../src/provider/Drivers/AntigravitySkills.ts      |  12 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |  14 --
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  24 +---
 .../src/provider/Drivers/ClaudeExecutable.ts       |  28 +---
 apps/server/src/provider/Drivers/ClaudeHome.ts     |  17 +--
 .../provider/Drivers/ClaudeSkillDispatch.test.ts   |   1 -
 .../src/provider/Drivers/ClaudeSkillDispatch.ts    |  35 +----
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  11 --
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  77 ++--------
 .../src/provider/Drivers/CodexDriver.test.ts       |   6 +-
 apps/server/src/provider/Drivers/CodexDriver.ts    |  46 +-----
 .../src/provider/Drivers/CodexManagedProvider.ts   |   1 -
 .../Drivers/antigravityUsageReader.test.ts         |  10 --
 .../src/provider/Drivers/antigravityUsageReader.ts |  22 +--
 .../src/provider/Drivers/claudeUsage.test.ts       |   1 -
 apps/server/src/provider/Drivers/claudeUsage.ts    |  20 +--
 .../server/src/provider/Drivers/codexUsage.test.ts |   7 -
 apps/server/src/provider/Drivers/codexUsage.ts     |  43 +-----
 apps/server/src/provider/builtInDrivers.ts         |  38 +----
 apps/server/src/provider/builtInProviderCatalog.ts |   6 -
 apps/server/src/provider/providerInstallation.ts   |   1 -
 docs/operations/release.md                         |  94 ++++++++-----
 .../src/server/AcpRegistryRuntimeCoordinator.ts    |   3 -
 .../src/server/AcpRegistrySupport.test.ts          | 129 ++++++++++++++++-
 .../src/server/AcpRegistrySupport.ts               |  78 ++++++-----
 .../provider-acp-registry/src/server/adapter.ts    |  19 +--
 packages/provider-acp-registry/src/server/auth.ts  |   5 +-
 .../src/server/authenticationState.test.ts         |   1 -
 .../src/server/authenticationState.ts              |   1 -
 .../provider-acp-registry/src/server/copilotAcp.ts |  15 +-
 .../provider-acp-registry/src/server/devinAcp.ts   |   1 -
 .../provider-acp-registry/src/server/driver.ts     |   6 -
 packages/provider-acp-registry/src/server/probe.ts |   7 -
 packages/provider-cursor/src/settings.ts           |   7 -
 .../provider-grok/src/server/acpCliProbe.test.ts   |  10 --
 packages/provider-grok/src/server/acpSupport.ts    |  22 +--
 packages/provider-grok/src/server/adapter.ts       |  22 +--
 packages/provider-grok/src/server/driver.test.ts   |   1 -
 packages/provider-grok/src/server/driver.ts        |   3 +-
 .../provider-grok/src/server/probeEnvironment.ts   |  12 ++
 packages/provider-grok/src/server/skills.test.ts   |  37 ++++-
 packages/provider-grok/src/server/skills.ts        |  34 +----
 packages/provider-grok/src/server/status.ts        |  36 ++---
 packages/provider-grok/src/server/usage.test.ts    |   1 -
 packages/provider-grok/src/server/usage.ts         |  27 +---
 packages/provider-grok/src/server/usageLimits.ts   |  15 +-
 .../src/server/xaiAcpExtension.test.ts             |  17 ---
 .../provider-grok/src/server/xaiAcpExtension.ts    | 144 +++----------------
 .../provider-grok/src/server/xaiBackgroundTasks.ts |   1 -
 packages/provider-muse/src/server/adapter.test.ts  |  10 --
 packages/provider-muse/src/server/adapter.ts       |  71 +---------
 .../provider-muse/src/server/itemPresentation.ts   |   4 -
 .../provider-muse/src/server/maintenance.test.ts   |   1 -
 packages/provider-muse/src/server/maintenance.ts   |   5 -
 .../provider-muse/src/server/modelCatalog.test.ts  |   1 -
 packages/provider-muse/src/server/modelCatalog.ts  |   4 -
 packages/provider-muse/src/server/protocol.ts      |   9 +-
 packages/provider-muse/src/server/sdk.test.ts      |   5 -
 packages/provider-muse/src/server/sdk.ts           |  33 ++---
 packages/provider-muse/src/server/status.ts        |   1 -
 .../provider-muse/src/server/textGeneration.ts     |   3 -
 packages/provider-opencode/src/server/driver.ts    |  44 +-----
 packages/provider-pi/src/server/usage.ts           |   3 +-
 packages/provider-pi/src/settings.ts               |   7 +-
 101 files changed, 773 insertions(+), 1519 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index fb4927a86..972b85c84 100644
--- a/.github/workflows/release.yml
+++ b/.github/workflows/release.yml
@@ -689,11 +689,8 @@ jobs:
       resource_key: win32-arm64
       cli_archive: true
 
-  # npm gets the same bytes as the GitHub Release: the launcher plus one
-  # package per CLI archive. Preview publishes too, under the `preview`
-  # dist-tag, which nothing resolves unless asked for by name.
-  publish_cli:
-    name: Publish CLI to npm
+  release:
+    name: Publish GitHub Release
     needs:
       [
         preflight,
@@ -702,74 +699,13 @@ jobs:
         test,
         test_server,
         desktop_mac_arm64,
-        desktop_linux_x64,
-        desktop_linux_arm64,
-        desktop_win_x64,
-        desktop_win_arm64,
-      ]
-    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.relay_public_config.result == 'success' && needs.quality.result == 'success' && needs.test.result == 'success' && needs.test_server.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' }}
-    runs-on: ubuntu-24.04 # blacksmith-8vcpu-ubuntu-2404
-    timeout-minutes: 15
-    permissions:
-      contents: read
-      id-token: write
-    steps:
-      - name: Checkout
-        uses: actions/checkout@v6
-        with:
-          ref: ${{ needs.preflight.outputs.ref }}
-          sparse-checkout: |
-            /*
-            !/.repos/
-          sparse-checkout-cone-mode: false
-
-      - name: Setup Vite+
-        uses: voidzero-dev/setup-vp@v1
-        with:
-          node-version-file: package.json
-          cache: true
-          run-install: |
-            args:
-              - --filter=t3...
-              - --filter=@t3tools/scripts...
-
-      - name: Download all CLI archives
-        uses: actions/download-artifact@v8
-        with:
-          pattern: cli-*
-          merge-multiple: true
-          path: release-cli
-
-      - name: Build npm packages from CLI archives
-        run: node scripts/build-npm-platform-packages.ts --archives-dir release-cli --version "${{ needs.preflight.outputs.version }}" --output-dir npm-packages
-
-      # A dry run of every package first: an auth or scope error here (the
-      # @t3code org missing, a package without a trusted publisher) fails
-      # before anything is live, instead of after some platforms already are.
-      - name: Check npm publish access (dry run)
-        run: |
-          if ! node apps/server/scripts/cli.ts publish --packages-dir npm-packages --tag "${{ needs.preflight.outputs.cli_dist_tag }}" --provenance --dry-run --verbose; then
-            echo "::error::npm publish --dry-run failed. Make sure the @t3code npm org exists and that t3 and every @t3code/t3-<platform> package has a trusted publisher registered for .github/workflows/release.yml (see docs/operations/release.md)." >&2
-            exit 1
-          fi
-
-      - name: Publish CLI packages
-        run: node apps/server/scripts/cli.ts publish --packages-dir npm-packages --tag "${{ needs.preflight.outputs.cli_dist_tag }}" --provenance --verbose
-
-  release:
-    name: Publish GitHub Release
-    needs:
-      [
-        preflight,
-        desktop_mac_arm64,
         desktop_mac_x64,
         desktop_linux_x64,
         desktop_linux_arm64,
         desktop_win_x64,
         desktop_win_arm64,
-        publish_cli,
       ]
-    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_mac_x64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' && needs.publish_cli.result == 'success' }}
+    if: ${{ !failure() && !cancelled() && needs.preflight.result == 'success' && needs.relay_public_config.result == 'success' && needs.quality.result == 'success' && needs.test.result == 'success' && needs.test_server.result == 'success' && needs.desktop_mac_arm64.result == 'success' && needs.desktop_mac_x64.result == 'success' && needs.desktop_linux_x64.result == 'success' && needs.desktop_linux_arm64.result == 'success' && needs.desktop_win_x64.result == 'success' && needs.desktop_win_arm64.result == 'success' }}
     runs-on: blacksmith-8vcpu-ubuntu-2404
     timeout-minutes: 30
     permissions:
@@ -807,6 +743,18 @@ jobs:
           merge-multiple: true
           path: release-assets
 
+      - name: Download internal Android APK for Nightly
+        if: needs.preflight.outputs.release_channel == 'nightly'
+        continue-on-error: true
+        env:
+          GOOGLE_PLAY_SERVICE_ACCOUNT_JSON: ${{ secrets.GOOGLE_PLAY_SERVICE_ACCOUNT_JSON }}
+          PACKAGE_NAME: com.t3tools.t3code
+          APK_OUTPUT_DIR: release-assets
+        run: |
+          if [ -n "$GOOGLE_PLAY_SERVICE_ACCOUNT_JSON" ]; then
+            node .github/scripts/mirror-play-apk.cjs
+          fi
+
       # Installers verify archives against this file, so it is written from the
       # signed bytes that get uploaded, never from an earlier stage.
       - name: Write CLI archive checksums
@@ -888,6 +836,10 @@ jobs:
               echo 'release-assets/*.yml'
             fi
             echo 'release-assets/*.tar.gz'
+            # Only list APKs when Play has a processed internal build availabl
… diff truncated …
```

Do not move the pin until a maintainer ports a fact and updates the snapshot in the same change.
