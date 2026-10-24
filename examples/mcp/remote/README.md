# Direct dot → Kit cached usage candidate

Source/config templates only. **No live endpoint, principal, access grant or private
plugin registration exists for this route.** It makes a direct MCP tool call, not
a T3 agent turn. Existing migration checkpoint branches remain unchanged.

## Minimal hosting decision

If the owner selects the already-reachable T3 HTTPS MCP host, PR33's native
toolkit can serve this same cached tool **without starting a T3 agent**. That is
the fewest-host-change option, after approved cache/identity binding and host
deployment. Existing T3 access still does not imply Kit account-data permission.
It must not be described as a usage-only connector while advertising a broader
tool catalog; a dedicated private connector should use the restricted route below.

Prefer one already-approved HTTPS host/reverse proxy for both future adapters,
with **separate usage-only and Remotif action routes/principals**. The proposed
Kit path is `/mcp/kit-usage`; it is not an existing endpoint. Keep Remotif's tool
catalog, TV authority and credentials out of this route. This is conceptually
compatible with Remotif's source-only adapter `7ed4b8b`; its files/endpoint were
not modified or assumed available. No second framework, server, listener, tunnel,
poller or provider client is created here.

`createCachedUsageRemoteMcp` is a request handler for an approved host. It uses
stateless Streamable HTTP POST with JSON responses, MCP `2025-06-18`. GET/DELETE
return 405; no SSE subscription/session or background feed is started. It accepts
initialize, notifications, ping, tools/list and the one `kit_usage_cached {}` call.
Authentication and exact principal plus `kit.usage.cached.read` scope checks
precede cache access. Origins/path/protocol and 64 KiB input limits are checked;
responses are `no-store`. Browser origins are denied unless explicitly selected.
The existing host must enforce TLS, verified authority/forwarded headers, request
timeouts/concurrency limits and its own authentication. Do not mount this as an
unauthenticated route or trust a client-supplied principal/model/account identity.

## Reuse PR32/33 source contracts

The preferred source is PR33's already-maintained, already-authorized in-process
producer. Reading its published projection adds no source/store/broker calls:

```ts
import {createCachedUsageRemoteMcp} from '@local-ai-kit/sdk/mcp/remote';

const usageRoute = createCachedUsageRemoteMcp({
  source: existingApprovedUsageProducer,
  publicOrigin: approvedHttpsOrigin,
  path: '/mcp/kit-usage',
  principalId: approvedUsageOnlyPrincipal,
  authenticate: existingHostAuthenticate,
});
// Approved host router passes its verified Request to usageRoute.handle(request).
// existingHostAuthenticate returns verified {subject, scopes}, never raw tokens.
// Explicit retirement: usageRoute.revoke(); no peer/store/service is stopped.
```

The result reuses PR32's canonical quota pools/windows/readings and redaction:
used/remaining/reset, observation/evidence, freshness and unknown/unsupported
states stay distinct. Account/connection scope and explicit consumer mapping are
fixed by producer construction, not tool arguments. Private labels/references are
removed. Window IDs and reported durations remain; missing window/model semantics
must be reported unknown rather than guessed. No context-capacity or transcript
counter is presented as subscription quota. A tool cannot refresh/probe or infer
recovery from elapsed resets.

### Existing local SQLite snapshot: explicit owner-side export only

The bounded approved inspection found the broker's existing cache at
`%LOCALAPPDATA%/local-ai-kit/state.db`. Do not rescan accounts or copy that database
into the repo/plugin/cloud. A cloud-only host cannot read the local file. After
Linux migration, the actual owner-host cache location must be selected anew.

[owner-cache.mjs](owner-cache.mjs) is an **optional privileged owner-host example**,
not an SDK/broker client API. The broker remains the only writer. Ordinary app
grants do not authorize direct database access. The owner must separately approve
this process reading the cached snapshot and exporting only the selected report;
`ownerApprovedCacheRead` defaults false and construction refuses without it.
Prefer the existing producer above when available.

After that approval, the host can compose `createSqliteUsageCache` with PR32's
`createCachedUsageReader({store: cache, scope: approvedScope, mappings: approvedMappings})`
and pass `{read}` as the remote route's source. The reader opens an existing file
read-only, sets `query_only`, reads one bounded snapshot (8 MiB maximum), then
closes. It does no migrations, refreshes, discovery, credential reads or retries.
It may take SQLite read locks; absent/locked/oversized/malformed data yields
unavailable, never a provider fallback. No subscription/interest lease is renewed.
Only the sanitized projection is returned; the full snapshot stays on its owner
host. Node 22.20's built-in SQLite API is experimental; review the selected host
runtime before production activation. No dependency installation is required.

## Exact action-time approvals still required

1. Name the existing host/provider/account and the **real verified HTTPS origin
   and route**. Approve any deployment/restart and local-to-HTTPS relay/exposure
   separately; none is selected or created by this candidate. Do not assume a
   Remotif endpoint or use another app's credential.
2. Select the existing producer, or explicitly approve owner-side access to the
   local cached snapshot (process identity, absolute path and OS read permission).
   A new broker feed would be a different approval with interest/cadence effects;
   this implementation does not create one.
3. Select exact Kit account/connection IDs and any confirmed consumer/billing
   account mapping. The earlier read did not prove which account funds this T3
   session; names/model selection do not establish a mapping. Empty/unmapped
   state remains unknown. No live IDs were put in these templates.
4. Provision one usage-only principal/credential through the selected host's
   secure mechanism and approve its data scope/persistence. Do not generate a
   token in a test, paste it in chat/config, reuse a provider token or extract an
   existing credential. The host validates the bearer and returns subject/scope;
   the Kit handler never stores, parses or returns the raw credential.
5. Approve private plugin creation/registration and enter the credential only in
   the secure connection UI. Then verify one direct authenticated read of the
   selected real cache, with zero provider work and honest observation/mapping.
   Only that successful direct read qualifies the connection as active.

[host-config.json.template](host-config.json.template) is disabled/unbound.
[plugin-template](plugin-template/plugin.json) uses Agent Plugins 1.0 and HTTPS
Streamable HTTP plus secure bearer entry. It is intentionally not upload-ready:
after endpoint verification, copy to a `local-ai-kit-usage` package and replace
the endpoint placeholder in a separate `mcp.json`; never put the credential in it.
The skill answers quota questions through `kit_usage_cached` directly and stops
on a missing/denied connection instead of launching an agent.

## Bounded inert source check

```sh
node --experimental-strip-types examples/mcp/remote/inert-check.mjs
```

It creates only a temporary synthetic SQLite file and in-memory Request objects;
no listener, endpoint, live credentials/grants, real cache read or provider calls.
Checks inert construction, owner-export denial by default, auth-before-cache,
inert initialization/list, scoped stale/unknown data, explicitly unmapped account,
preserved resets, denied refresh/origin, revocation (including requests revoked during body upload) and unchanged
database bytes.
The owned fixture is removed afterward. This is not deployed HTTPS/plugin/live
qualification, and does not replace the candidate's required CI.
