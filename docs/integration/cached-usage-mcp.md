# Read-only cached usage over MCP

The optional TypeScript SDK façade exposes one MCP tool: **`kit_usage_cached`**.
It takes `{}`. It reads an already-maintained `SnapshotStore`/`LiveSnapshotStore`
cache; it has no client, transport, credential, grant, discovery, refresh, probe,
execution or polling interface. Import and construction are inert. A tool call
uses `getSnapshot()` once and does not renew a broker interest lease.

This source module is not in the previously published alpha.18 bundle; an approved
build/package update is needed before a real host consumes it. This is implemented
source and a synthetic endpoint demonstration, **not a live
dot connection**. No persistent grant, credential, endpoint, tunnel or shared
service change is created by this slice.

## API and access scope

After building the SDK, an existing Node host can compose:

```ts
import {createCachedUsageMcp} from '@local-ai-kit/sdk/mcp';
import {serveCachedUsageMcp} from '@local-ai-kit/sdk/mcp/node';

// alreadyMaintainedStore belongs to the existing, approved broker-client host.
// Do not create a second store/subscription or start a broker here.
const handle = createCachedUsageMcp({
  store: alreadyMaintainedStore,
  scope: {
    connectionIds: ['approved-connection-id'],
    accountIds: ['approved-kit-account-id'],
  },
  mappings: [{
    consumerAccountId: 'operator-verified-consumer-account',
    kitAccountId: 'approved-kit-account-id',
  }],
});
await serveCachedUsageMcp(handle); // the host's dedicated MCP stdin/stdout
```

Scope lists are mandatory, copied at construction and limited to 32 IDs each.
An empty connection list discloses no cache rows; an empty account list excludes
known-account pools. Accountless pools remain unmapped and require an explicitly
allowed connection. This projection only narrows the
underlying approved broker client's scope; it grants nothing. Tool arguments
cannot select another account, alter scope, refresh, probe or connect. The host
must pass an already-projected SDK cache, not unscoped storage or a token-bearing
configuration object. An error or cleared/closed cache returns unavailable data with
no retained account rows. Known pools are kept distinct; similar identities and
shared-pool relationships are never inferred.

An optional explicit mapping binds a consumer account identifier to a Kit account
ID already in the allowlist. It is an operator assertion, not an email/model-name
match. A mapping absent from the current cache is unknown. Other visible accounts
remain unmapped. A native Codex quota window does not prove that the T3 Sol session
is spending the same account's allowance. An actual dot setup must verify that
mapping separately.

## Result and privacy

`structuredContent` and the JSON text content contain the same report:

- `state`: `cached`, `unavailable` or `invalid`; `mode` retains `synthetic` versus
  `live`, plus the cache revision/observation time when valid.
- `quotaPools`: canonical [QuotaPool/UsageWindow/Observation/UsageReading](../../protocol/contracts.schema.json)
  shapes. Account, connection, route and pool identities stay separate. Values
  retain wire decimal strings: used, limit and remaining, with null when absent.
  Window reset/duration observations are retained independently.
- `connections`, explicit `mappings` and `unmappedAccountIds`: scoped, opaque
  identities only. No account display labels, email, authentication data or model
  conversation/context is exported.
- `contextCounters: "not_reported"`: these are provider quota observations, not a
  model context-capacity counter, billable-token total or estimate of all usage.

Known observations retain evidence kind, adapter/version, observation time,
verification and freshness. Free-form evidence references, private labels,
diagnostics and raw reason text are excluded. Window labels become `Usage window`.
Unknown/unavailable reasons are redacted to stable codes; an explicit
`CAPABILITY_UNSUPPORTED` stays unavailable with that reason. Missing pools/windows
are **unknown**, not zero, full allowance, signed-out, healthy or unsupported.

The façade may only degrade freshness: an expired observation, a future/old
observation time or a loading cache is stale. Its conservative age ceiling is
60 seconds by default (host-configurable, 1 second–24 hours). It never promotes a
stale observation or resets usage when a reset deadline passes. Remaining values
are preserved from the canonical reading, not recomputed from a clock or model
context limit. Malformed usage quantities or oversized cache projections fail closed with `invalid` and no
usage rows; the report has a 128 KiB bound.

## Interest leases and inference

The existing broker APIs `usage.read`, `snapshot.get` and inventory-event reads
renew a client interest lease. These methods do not perform provider inference,
but active interest can speed up already-host-authorized metadata/usage lanes.
`createLiveSnapshotStore(...).connect()` also owns an explicit initial cached read
and an inventory subscription; it must not be started again for each tool call.

This façade **does not call any of those methods**, connect a store, subscribe,
poll, refresh or probe. It reads the existing in-process cache. Consequently tool
reads do not change broker cadence or read credentials. An existing host's prior
subscription continues with its prior policy; this is not a promise that another
host's authorized metadata scheduler stops reading native account state.

The live store's disposal now calls its captured underlying disposer, and terminal
`LOCAL_ACCESS_DENIED`/`CLOSED` inventory failures retire that store/cache. A changed
authorization requires a new host-owned store. Transport loss alone is not proof
of revocation: retained data stays cache-only and becomes conservatively stale;
the façade does not claim that the broker is currently connected.

If dot has no existing cache host, creating that input feed is a separate setup
decision: either reuse an approved host cache, or review a single new subscriber
and its interest behavior explicitly. Do not silently create a subscriber, launch
a fallback scanner, or describe ordinary RPC reads as cadence-neutral.

## MCP transport and demonstration

The optional stdio adapter uses newline-delimited UTF-8 JSON-RPC with 64 KiB input
frames and 512 KiB output frames. It supports `initialize`,
`notifications/initialized`, `ping`, `tools/list` and `tools/call` for the one tool.
It negotiates MCP `2025-06-18`; incompatible clients should disconnect. It offers
no sampling, elicitation, prompts, resources, HTTP listener or provider methods.
The tool's read-only/closed-world annotations are hints; actual safety comes from
the cache-only implementation and host scope. See the MCP
[lifecycle](https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle),
[stdio framing](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports),
and [tool contract](https://modelcontextprotocol.io/specification/2025-06-18/server/tools).

The source-checkout demonstration requires Node 22.16+:

```sh
node --experimental-strip-types examples/mcp/cached-usage-demo.mjs
# Send MCP initialize, initialized, tools/list, then tools/call:
# {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"kit_usage_cached","arguments":{}}}
```

The demo is explicitly synthetic. It imports no broker transport and has no token
or file/environment discovery. Client actions throw if invoked. The bounded
end-to-end journey starts owned stdio children, verifies negotiation/scope/
redaction/stale reset/unknown/unsupported/unavailable/terminal-denial behavior, rejects refresh
arguments and provider RPC, and checks zero client requests from tool reads:

```sh
node --test tests/integration/cached-usage-mcp.test.mjs
```

## Exact setup required before dot is connected

1. Select an existing approved cache host and the exact visible Kit account and
   connection IDs. Verify which consumer account actually uses that allowance;
   otherwise leave it unmapped and report unknown.
2. Approve a dedicated local stdio MCP connection to that host's adapter and its
   explicit read-only allowlists. Anyone holding its pipes can read that scoped
   report; do not share those pipes or put credentials in MCP arguments/output.
3. If a new broker app/feed is necessary, separately approve its identity and
   connection scope. It needs cached inventory/usage visibility only: no passive
   scanning permission, metadata-probe permission, execution/workspace/paid/write
   grant, admin access or identity reveal. Credential provisioning and persistence
   remain host/operator work, not this tool.
4. Reuse one store/subscription. Review startup interest semantics before enabling
   a new feed, and clear the cache on denial/revocation/disconnection according to
   the owning host's lifecycle. Do not kill or restart a peer/shared broker.

No actual dot registration, credential provisioning, persistent grant or network
exposure was performed. Live account mapping, current provider allowance, dot
connection and hosted interoperability remain **unqualified** until that approved
setup and its evidence exist. [Synthetic evidence](../qualification/evidence/2026-10-23-cached-usage-mcp/README.md).
