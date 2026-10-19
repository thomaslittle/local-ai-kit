# T3 Code provider review

Pinned commit: `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`

Compared `main` `2532e64099a2a14e18427783a79e7ede44f626ce` and nightly `v0.1.1-nightly.20261019.3360` (`1451e7cb23034d29774338608368a15d8845df36`) against nightly pin `e3b35d1b2440b104f0bc2e357377c348121234dd`.

Watched areas: local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. This report does not change Local AI Kit behavior.

## main (cfafb4bb54d2..2532e64099a2)

```diff
 .github/workflows/release.yml                      |   8 ++
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  57 +++++++-
 apps/server/src/cloud/relayResponse.test.ts        |  28 ++++
 apps/server/src/cloud/relayResponse.ts             |  11 ++
 apps/server/src/cloud/selfUpdate.test.ts           |  49 ++++++-
 apps/server/src/cloud/selfUpdate.ts                |  26 ++++
 .../server/src/cloud/serviceLauncherClient.test.ts |  20 ++-
 apps/server/src/cloud/serviceLauncherClient.ts     |  13 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |  10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  66 ++++++++--
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  60 +++++++++
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  17 ++-
 .../src/provider/Drivers/CodexDriver.test.ts       |  25 ++++
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |  39 ++++++
 .../server/src/provider/Drivers/CodexHomeLayout.ts |   3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |  49 +++++--
 docs/internals/providers.md                        |   3 +-
 packages/provider-grok/src/server/adapter.ts       |  18 ++-
 packages/provider-grok/src/server/driver.test.ts   | 122 +++++++++++++++++
 packages/provider-grok/src/server/driver.ts        |  56 +++++---
 packages/provider-muse/src/server/adapter.test.ts  | 144 ++++++++++++++++++++-
 packages/provider-muse/src/server/adapter.ts       |  39 +++++-
 .../provider-muse/src/server/modelCatalog.test.ts  |  28 ++++
 packages/provider-muse/src/server/modelCatalog.ts  |   7 +-
 packages/provider-muse/src/server/protocol.ts      |  32 ++++-
 packages/provider-muse/src/server/status.test.ts   |  31 ++++-
 .../provider-muse/src/server/usageLimits.test.ts   |  60 +++++++++
 packages/provider-muse/src/server/usageLimits.ts   |  48 +++++++
 packages/provider-opencode/src/server/driver.ts    |  16 ++-
 29 files changed, 1004 insertions(+), 81 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..1173e9934 100644
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
 
@@ -521,7 +525,11 @@ jobs:
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
diff --git a/apps/server/src/cloud/CloudLink.lifecycle.test.ts b/apps/server/src/cloud/CloudLink.lifecycle.test.ts
index 7b227aff5..faa11d9c1 100644
--- a/apps/server/src/cloud/CloudLink.lifecycle.test.ts
+++ b/apps/server/src/cloud/CloudLink.lifecycle.test.ts
@@ -1213,10 +1213,20 @@ describe("releaseManagedTunnelOnShutdown", () => {
   });
 
   it.effect.each([
-    { status: 401, rejection: "unauthorized" },
-    { status: 403, rejection: "forbidden" },
-    { status: 409, rejection: "rejected" },
-  ])("preserves a permanent $status relay recovery failure", ({ status, rejection }) => {
+    { status: 401, body: {}, rejection: "unauthorized" },
+    {
+      status: 401,
+      body: {
+        _tag: "RelayAuthInvalidError",
+        code: "auth_invalid",
+        reason: "invalid_bearer",
+        traceId: "trace-revoked",
+      },
+      rejection: "unauthorized",
+    },
+    { status: 403, body: {}, rejection: "forbidden" },
+    { status: 409, body: {}, rejection: "rejected" },
+  ])("preserves a permanent $status relay recovery failure", ({ status, body, rejection }) => {
     const { store } = makeMemorySecretStore([
       [CLOUD_ENDPOINT_RUNTIME_CONFIG, "old-config"],
       [RELAY_URL_SECRET, "https://relay.example.test"],
@@ -1239,7 +1249,44 @@ describe("releaseManagedTunnelOnShutdown", () => {
         store,
         applyConfigCalls,
         requests,
-        respond: () => Response.json({}, { status }),
+        respond: () => Response.json(body, { status }),
+      }),
+    );
+  });
+
+  // A proof signed with a skewed boot clock is rejected as expired. The next
+  // attempt mints a new proof, so startup reconcile must keep retrying it.
+  it.effect("retries a relay link proof rejected as expired", () => {
+    const { store } = makeMemorySecretStore([
+      [CLOUD_ENDPOINT_RUNTIME_CONFIG, "old-config"],
+      [RELAY_URL_SECRET, "https://relay.example.test"],
+      [CLOUD_LINKED_USER_ID, "user-123"],
+      [RELAY_ENVIRONMENT_CREDENTIAL_SECRET, "environment-credential"],
+    ]);
+    const applyConfigCalls: Array<unknown> = [];
+    const requests: Array<HttpClientRequest.HttpClientRequest> = [];
+
+    return Effect.gen(function* () {
+      const link = yield* CloudLink.CloudLink;
+      const error = yield* Effect.flip(link.recoverManagedTunnel("http://127.0.0.1:3773"));
+
+      expect(error).toMatchObject({ _tag: "RelayRequestError", rejection: "unavailable" });
+      expect(error.message).toContain("Trace ID: trace-expired");
+      expect(CloudLink.shouldRetryCloudLink(error)).toBe(true);
+    }).pipe(
+      provideReleaseHarness({
+        store,
+        applyConfigCalls,
+        requests,
+        respond: () =>
+          Response.json(
+            {
+              _tag: "RelayEnvironmentLinkProofExpiredError",
+              code: "environment_link_proof_expired",
+              traceId: "trace-expired",
+            },
+            { status: 401 },
+          ),
       }),
     );
   });
diff --git a/apps/server/src/cloud/relayResponse.test.ts b/apps/server/src/cloud/relayResponse.test.ts
index 8221899ca..59f341aa3 100644
--- a/apps/server/src/cloud/relayResponse.test.ts
+++ b/apps/server/src/cloud/relayResponse.test.ts
@@ -76,6 +76,34 @@ it.effect("makes revoked authorization actionable and non-retryable", () =>
   }),
 );
 
+it.effect("retries an expired link proof but not a revoked bearer, both 401", () =>
+  Effect.gen(function* () {
+    const expired = yield* filterRelayResponse(
+      response(401, {
+        _tag: "RelayEnvironmentLinkProofExpiredError",
+        code: "environment_link_proof_expired",
+        traceId: "trace-expired",
+      }),
+    ).pipe(Effect.mapError(relayRequestError), Effect.flip);
+    const revoked = yield* filterRelayResponse(
+      response(401, {
+        _tag: "RelayAuthInvalidError",
+        code: "auth_invalid",
+        reason: "invalid_bearer",
+        traceId: "trace-auth",
+      }),
+    ).pipe(Effect.mapError(relayRequestError), Effect.flip);
+
+    expect(expired).toMatchObject({ _tag: "RelayRequestError", rejection: "unavailable" });
+    expect(expired.message).toContain("Relay environment link proof expired");
+    expect(expired.message).toContain("date and time");
+    expect(expired.message).toContain("Trace ID: trace-expired");
+    expect(shouldRetryRelayRequest(expired)).toBe(true);
+    expect(revoked).toMatchObject({ rejection: "unauthorized" });
+    expect(shouldRetryRelayRequest(revoked)).toBe(false);
+  }),
+);
+
 it.effect("reports an unrecognized access denial without printing its response body", () =>
   Effect.gen(function* () {
     const error = yield* filterRelayResponse(
diff --git a/apps/server/src/cloud/relayResponse.ts b/apps/server/src/cloud/relayResponse.ts
index bc9008293..f86cc5719 100644
--- a/apps/server/src/cloud/relayResponse.ts
+++ b/apps/server/src/cloud/relayResponse.ts
@@ -63,6 +63,17 @@ export const filterRelayResponse = Effect.fn("cloud.filter_relay_response")(func
     ? `T3 Connect: ${decoded.value.message}. ${recoveryHint(decoded.value)} Trace ID: ${decoded.value.traceId}.`
     : `T3 Connect relay returned HTTP ${response.status} without a recognized error response. Check relay access and any proxy or firewall restrictions, then restart T3 Code.${requestId}`;
 
+  // The only 401 that clears itself: a proof signed with a skewed clock (a host
+  // that booted before NTP) expires on arrival, and the next attempt signs a new
+  // one. It is recognized only from a schema-decoded relay body. Every other
+  // 401, including a revoked bearer or an undecodable body, stays permanent.
+  if (
+    response.status === 401 &&
+    Option.isSome(decoded) &&
+    decoded.value._tag === "RelayEnvironmentLinkProofExpiredError"
+  ) {
+    return yield* new RelayRequestError({ rejection: "unavailable", description });
+  }
   if (response.status === 401) {
     return yield* new RelayRequestError({ rejection: "unauthorized", description });
   }
diff --git a/apps/server/src/cloud/selfUpdate.test.ts b/apps/server/src/cloud/selfUpdate.test.ts
index af202046b..3a852c7bc 100644
--- a/apps/server/src/cloud/selfUpdate.test.ts
+++ b/apps/server/src/cloud/selfUpdate.test.ts
@@ -8,6 +8,7 @@ import * as Effect from "effect/Effect";
 import * as FileSystem from "effect/FileSystem";
 import * as Fiber from "effect/Fiber";
 import * as Path from "effect/Path";
+import * as Stream from "effect/Stream";
 import { HttpClient, HttpClientResponse } from "effect/http";
 import * as ChildProcessSpawner from "effect/process/ChildProcessSpawner";
 
@@ -22,12 +23,13 @@ interface HarnessOptions {
   readonly mode?: "web" | "desktop";
   readonly managed?: boolean;
   readonly preflight?: "ready" | "blocked";
+  readonly beforeDownload?: Effect.Effect<void>;
   readonly requestUpdate?: ServiceLauncherClient.ServiceLauncherClient["Service"]["requestUpdate"];
   readonly desktopAppUpdate?: DesktopAppUpdate.DesktopAppUpdate["Service"];
 }
 
 const archiveBytes = new TextEncoder().encode("not really a tarball");
-const releaseHttpClient = (order: string[]) =>
+const releaseHttpClient = (order: string[], beforeDownload?: Effect.Effect<void>) =>
   HttpClient.make((request) =>
     Effect.gen(function* () {
       if (request.url.endsWith("/SHA256SUMS")) {
@@ -40,6 +42,7 @@ const releaseHttpClient = (order: string[]) =>
           new Response(`${hex}  t3-1.1.0-linux-x64.tar.gz\n`),
         );
       }
+      if (beforeDownload) yield* beforeDownload;
       order.push("download");
       return HttpClientResponse.fromWeb(request, new Response(archiveBytes));
     }),
@@ -116,7 +119,7 @@ const makeHarness = Effect.fn("test.make_self_update_harness")(function* (
         run: () => Effect.die("unexpected desktop app update run"),
       },
     ),
-    Effect.provideService(HttpClient.HttpClient, releaseHttpClient(order)),
+    Effect.provideService(HttpClient.HttpClient, releaseHttpClient(order, options.beforeDownload)),
     Effect.provideService(HostProcess.Platform, "linux"),
     Effect.provideService(HostProcess.Architecture, "x64"),
     Effect.provide(ServerConfig.layer({ ...config, mode: options.mode ?? "web" })),
@@ -418,4 +421,46 @@ it.layer(NodeServices.layer)("server self update", (it) => {
       expect((yield* Fiber.join(first)).updateId).toBe("launcher-id");
     }),
   );
+
+  it.effect("keeps preparing after the progress stream closes", () =>
+    Effect.gen(function* () {
+      const releaseDownload = yield* Deferred.make<void>();
+      const sawDownloading = yield* Deferred.make<void>();
+      const accepte
… diff truncated …
```

## nightly (v0.1.1-nightly.20261019.3360)

```diff
 .github/workflows/release.yml                      |   8 ++
 apps/server/src/cloud/CloudLink.lifecycle.test.ts  |  57 +++++++-
 apps/server/src/cloud/relayResponse.test.ts        |  28 ++++
 apps/server/src/cloud/relayResponse.ts             |  11 ++
 apps/server/src/cloud/selfUpdate.test.ts           |  49 ++++++-
 apps/server/src/cloud/selfUpdate.ts                |  26 ++++
 .../server/src/cloud/serviceLauncherClient.test.ts |  20 ++-
 apps/server/src/cloud/serviceLauncherClient.ts     |  13 +-
 apps/server/src/provider/CodexProvider.ts          |   8 +-
 .../src/provider/Drivers/AntigravityUsage.ts       |  10 +-
 apps/server/src/provider/Drivers/ClaudeDriver.ts   |  66 ++++++++--
 .../src/provider/Drivers/ClaudeSkills.test.ts      |  60 +++++++++
 apps/server/src/provider/Drivers/ClaudeSkills.ts   |  17 ++-
 .../src/provider/Drivers/CodexDriver.test.ts       |  25 ++++
 .../src/provider/Drivers/CodexHomeLayout.test.ts   |  39 ++++++
 .../server/src/provider/Drivers/CodexHomeLayout.ts |   3 +-
 .../src/provider/Drivers/CodexManagedProvider.ts   |  49 +++++--
 docs/internals/providers.md                        |   3 +-
 packages/provider-grok/src/server/adapter.ts       |  18 ++-
 packages/provider-grok/src/server/driver.test.ts   | 122 +++++++++++++++++
 packages/provider-grok/src/server/driver.ts        |  56 +++++---
 packages/provider-muse/src/server/adapter.test.ts  | 144 ++++++++++++++++++++-
 packages/provider-muse/src/server/adapter.ts       |  39 +++++-
 .../provider-muse/src/server/modelCatalog.test.ts  |  28 ++++
 packages/provider-muse/src/server/modelCatalog.ts  |   7 +-
 packages/provider-muse/src/server/protocol.ts      |  32 ++++-
 packages/provider-muse/src/server/status.test.ts   |  31 ++++-
 .../provider-muse/src/server/usageLimits.test.ts   |  60 +++++++++
 packages/provider-muse/src/server/usageLimits.ts   |  48 +++++++
 packages/provider-opencode/src/server/driver.ts    |  16 ++-
 30 files changed, 1010 insertions(+), 83 deletions(-)

diff --git a/.github/workflows/release.yml b/.github/workflows/release.yml
index 972b85c84..1173e9934 100644
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
 
@@ -521,7 +525,11 @@ jobs:
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
diff --git a/apps/server/src/cloud/CloudLink.lifecycle.test.ts b/apps/server/src/cloud/CloudLink.lifecycle.test.ts
index 7b227aff5..faa11d9c1 100644
--- a/apps/server/src/cloud/CloudLink.lifecycle.test.ts
+++ b/apps/server/src/cloud/CloudLink.lifecycle.test.ts
@@ -1213,10 +1213,20 @@ describe("releaseManagedTunnelOnShutdown", () => {
   });
 
   it.effect.each([
-    { status: 401, rejection: "unauthorized" },
-    { status: 403, rejection: "forbidden" },
-    { status: 409, rejection: "rejected" },
-  ])("preserves a permanent $status relay recovery failure", ({ status, rejection }) => {
+    { status: 401, body: {}, rejection: "unauthorized" },
+    {
+      status: 401,
+      body: {
+        _tag: "RelayAuthInvalidError",
+        code: "auth_invalid",
+        reason: "invalid_bearer",
+        traceId: "trace-revoked",
+      },
+      rejection: "unauthorized",
+    },
+    { status: 403, body: {}, rejection: "forbidden" },
+    { status: 409, body: {}, rejection: "rejected" },
+  ])("preserves a permanent $status relay recovery failure", ({ status, body, rejection }) => {
     const { store } = makeMemorySecretStore([
       [CLOUD_ENDPOINT_RUNTIME_CONFIG, "old-config"],
       [RELAY_URL_SECRET, "https://relay.example.test"],
@@ -1239,7 +1249,44 @@ describe("releaseManagedTunnelOnShutdown", () => {
         store,
         applyConfigCalls,
         requests,
-        respond: () => Response.json({}, { status }),
+        respond: () => Response.json(body, { status }),
+      }),
+    );
+  });
+
+  // A proof signed with a skewed boot clock is rejected as expired. The next
+  // attempt mints a new proof, so startup reconcile must keep retrying it.
+  it.effect("retries a relay link proof rejected as expired", () => {
+    const { store } = makeMemorySecretStore([
+      [CLOUD_ENDPOINT_RUNTIME_CONFIG, "old-config"],
+      [RELAY_URL_SECRET, "https://relay.example.test"],
+      [CLOUD_LINKED_USER_ID, "user-123"],
+      [RELAY_ENVIRONMENT_CREDENTIAL_SECRET, "environment-credential"],
+    ]);
+    const applyConfigCalls: Array<unknown> = [];
+    const requests: Array<HttpClientRequest.HttpClientRequest> = [];
+
+    return Effect.gen(function* () {
+      const link = yield* CloudLink.CloudLink;
+      const error = yield* Effect.flip(link.recoverManagedTunnel("http://127.0.0.1:3773"));
+
+      expect(error).toMatchObject({ _tag: "RelayRequestError", rejection: "unavailable" });
+      expect(error.message).toContain("Trace ID: trace-expired");
+      expect(CloudLink.shouldRetryCloudLink(error)).toBe(true);
+    }).pipe(
+      provideReleaseHarness({
+        store,
+        applyConfigCalls,
+        requests,
+        respond: () =>
+          Response.json(
+            {
+              _tag: "RelayEnvironmentLinkProofExpiredError",
+              code: "environment_link_proof_expired",
+              traceId: "trace-expired",
+            },
+            { status: 401 },
+          ),
       }),
     );
   });
diff --git a/apps/server/src/cloud/relayResponse.test.ts b/apps/server/src/cloud/relayResponse.test.ts
index 8221899ca..59f341aa3 100644
--- a/apps/server/src/cloud/relayResponse.test.ts
+++ b/apps/server/src/cloud/relayResponse.test.ts
@@ -76,6 +76,34 @@ it.effect("makes revoked authorization actionable and non-retryable", () =>
   }),
 );
 
+it.effect("retries an expired link proof but not a revoked bearer, both 401", () =>
+  Effect.gen(function* () {
+    const expired = yield* filterRelayResponse(
+      response(401, {
+        _tag: "RelayEnvironmentLinkProofExpiredError",
+        code: "environment_link_proof_expired",
+        traceId: "trace-expired",
+      }),
+    ).pipe(Effect.mapError(relayRequestError), Effect.flip);
+    const revoked = yield* filterRelayResponse(
+      response(401, {
+        _tag: "RelayAuthInvalidError",
+        code: "auth_invalid",
+        reason: "invalid_bearer",
+        traceId: "trace-auth",
+      }),
+    ).pipe(Effect.mapError(relayRequestError), Effect.flip);
+
+    expect(expired).toMatchObject({ _tag: "RelayRequestError", rejection: "unavailable" });
+    expect(expired.message).toContain("Relay environment link proof expired");
+    expect(expired.message).toContain("date and time");
+    expect(expired.message).toContain("Trace ID: trace-expired");
+    expect(shouldRetryRelayRequest(expired)).toBe(true);
+    expect(revoked).toMatchObject({ rejection: "unauthorized" });
+    expect(shouldRetryRelayRequest(revoked)).toBe(false);
+  }),
+);
+
 it.effect("reports an unrecognized access denial without printing its response body", () =>
   Effect.gen(function* () {
     const error = yield* filterRelayResponse(
diff --git a/apps/server/src/cloud/relayResponse.ts b/apps/server/src/cloud/relayResponse.ts
index bc9008293..f86cc5719 100644
--- a/apps/server/src/cloud/relayResponse.ts
+++ b/apps/server/src/cloud/relayResponse.ts
@@ -63,6 +63,17 @@ export const filterRelayResponse = Effect.fn("cloud.filter_relay_response")(func
     ? `T3 Connect: ${decoded.value.message}. ${recoveryHint(decoded.value)} Trace ID: ${decoded.value.traceId}.`
     : `T3 Connect relay returned HTTP ${response.status} without a recognized error response. Check relay access and any proxy or firewall restrictions, then restart T3 Code.${requestId}`;
 
+  // The only 401 that clears itself: a proof signed with a skewed clock (a host
+  // that booted before NTP) expires on arrival, and the next attempt signs a new
+  // one. It is recognized only from a schema-decoded relay body. Every other
+  // 401, including a revoked bearer or an undecodable body, stays permanent.
+  if (
+    response.status === 401 &&
+    Option.isSome(decoded) &&
+    decoded.value._tag === "RelayEnvironmentLinkProofExpiredError"
+  ) {
+    return yield* new RelayRequestError({ rejection: "unavailable", description });
+  }
   if (response.status === 401) {
     return yield* new RelayRequestError({ rejection: "unauthorized", description });
   }
diff --git a/apps/server/src/cloud/selfUpdate.test.ts b/apps/server/src/cloud/selfUpdate.test.ts
index af202046b..3a852c7bc 100644
--- a/apps/server/src/cloud/selfUpdate.test.ts
+++ b/apps/server/src/cloud/selfUpdate.test.ts
@@ -8,6 +8,7 @@ import * as Effect from "effect/Effect";
 import * as FileSystem from "effect/FileSystem";
 import * as Fiber from "effect/Fiber";
 import * as Path from "effect/Path";
+import * as Stream from "effect/Stream";
 import { HttpClient, HttpClientResponse } from "effect/http";
 import * as ChildProcessSpawner from "effect/process/ChildProcessSpawner";
 
@@ -22,12 +23,13 @@ interface HarnessOptions {
   readonly mode?: "web" | "desktop";
   readonly managed?: boolean;
   readonly preflight?: "ready" | "blocked";
+  readonly beforeDownload?: Effect.Effect<void>;
   readonly requestUpdate?: ServiceLauncherClient.ServiceLauncherClient["Service"]["requestUpdate"];
   readonly desktopAppUpdate?: DesktopAppUpdate.DesktopAppUpdate["Service"];
 }
 
 const archiveBytes = new TextEncoder().encode("not really a tarball");
-const releaseHttpClient = (order: string[]) =>
+const releaseHttpClient = (order: string[], beforeDownload?: Effect.Effect<void>) =>
   HttpClient.make((request) =>
     Effect.gen(function* () {
       if (request.url.endsWith("/SHA256SUMS")) {
@@ -40,6 +42,7 @@ const releaseHttpClient = (order: string[]) =>
           new Response(`${hex}  t3-1.1.0-linux-x64.tar.gz\n`),
         );
       }
+      if (beforeDownload) yield* beforeDownload;
       order.push("download");
       return HttpClientResponse.fromWeb(request, new Response(archiveBytes));
     }),
@@ -116,7 +119,7 @@ const makeHarness = Effect.fn("test.make_self_update_harness")(function* (
         run: () => Effect.die("unexpected desktop app update run"),
       },
     ),
-    Effect.provideService(HttpClient.HttpClient, releaseHttpClient(order)),
+    Effect.provideService(HttpClient.HttpClient, releaseHttpClient(order, options.beforeDownload)),
     Effect.provideService(HostProcess.Platform, "linux"),
     Effect.provideService(HostProcess.Architecture, "x64"),
     Effect.provide(ServerConfig.layer({ ...config, mode: options.mode ?? "web" })),
@@ -418,4 +421,46 @@ it.layer(NodeServices.layer)("server self update", (it) => {
       expect((yield* Fiber.join(first)).updateId).toBe("launcher-id");
     }),
   );
+
+  it.effect("keeps preparing after the progress stream closes", () =>
+    Effect.gen(function* () {
+      const releaseDownload = yield* Deferred.make<void>();
+      const sawDow
… diff truncated …
```

Do not move the pin until a maintainer ports a fact and updates the snapshot in the same change.
