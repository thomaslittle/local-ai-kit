# Local AI Kit API reference

Start here to integrate the native runtime. This guide explains authorization and lifecycle; the generated pages give the exact wire fields and examples.

| Reference | Contents |
|---|---|
| [All JSON-RPC methods](../reference/methods.md) | Parameters, results, authority, synthetic request/response pairs and representative failures |
| [Wire schemas](../reference/schemas.md) | Named domain, request and result schemas, with required keys, unions and constraints |
| [Local HTTP routes](../reference/http.md) | Pairing, RPC, REST reads and session SSE from OpenAPI |
| [Machine-readable contracts](../../protocol/) | JSON Schema, method manifest and OpenAPI |
| [Generated SDK types](generated-api.md) | How Rust contracts become TypeScript/Python types and reference pages |

This describes the alpha.18 API. Protocol version `1.0` and runtime version `0.1.0-alpha.18` are different version axes. A compiled method or schema does not promise provider support, a granted scope, or a live-qualified operation. Consult the actual handshake, the [support matrix](../adapters/SUPPORT-MATRIX.md), and the [qualification records](../qualification/).

## Safe end-to-end quickstart

Use an approved local helper from a [verified release bundle](distribution.md) or your own build. No package registry publication is required. This example needs only Python 3.11+ and that executable; it does not install software or download a helper.

From a repository checkout, run:

```sh
python examples/python/api_quickstart.py /absolute/path/to/local-ai-kit
```

On Windows, pass the full absolute `.exe` path, quoted if it contains spaces. The [complete example](../../examples/python/api_quickstart.py) creates a temporary, empty discovery home, launches its own stdio helper, negotiates `1.0`, explicitly refreshes passive inventory, reads a snapshot and a model page, then closes only that helper. It prints counts, not identity or credential material. It grants no provider metadata, execution, workspace, paid access or OS notifications. Importing the example does nothing.

The essential wire exchange is:

```json
{"jsonrpc":"2.0","id":1,"method":"runtime.handshake","params":{"protocolVersion":"1.0","clientName":"api-reference-quickstart"}}
```

After accepting the handshake response, send and await each request in order:

```json
{"jsonrpc":"2.0","id":2,"method":"discovery.refresh","params":{"level":"passive"}}
{"jsonrpc":"2.0","id":3,"method":"snapshot.get","params":{}}
{"jsonrpc":"2.0","id":4,"method":"models.page","params":{"limit":16}}
```

The temporary home should produce no configured connections or models. A host that grants passive discovery also authorizes background inventory, so an initial snapshot can arrive before or after that publication. Awaiting explicit refresh before reading avoids assuming a particular startup ordering. This example is not evidence of a real subscription or permission to run a model.

For a normal application, create a transport explicitly, connect the [TypeScript or Python SDK](native-helper.md), and close it in `finally`/a context manager. Replace the empty home with real discovery only after the user agrees. Keep live results separate from the fictional examples in the method reference.

## Transports and ownership

The same native service is accessible through these host-selected surfaces:

| Surface | Setup and authentication | Lifetime / event delivery |
|---|---|---|
| Embedded Rust | Host creates `lai-kit`/service policy directly | Host owns service and grants; no daemon is required |
| Child helper stdio | Approved executable, `serve --stdio`; host controls arguments and inherited pipes | Parent owns this helper; newline JSON-RPC and optional native event subscriptions |
| Shared local broker | Explicit IPC address and a credential delivered on stdin through `connect --ipc` | Parent-owned control channel or separately configured independent lifetime; consumers own their connections |
| Loopback HTTP | Host explicitly enables HTTP, then local user pairs | Header-authenticated RPC/REST; session SSE; independent paired clients |
| Browser Native Messaging | Registered host and allowed browser extension identity | Framed native transport; see the [extension guide](browser-extension.md) |

An SDK import does not discover installations, read credentials, spawn a helper, connect, download, or infer. An explicit transport factory can launch a process. `openSharedBrokerTransport` is an explicit opt-in startup path; the `connect --ipc` bridge itself only connects. If an application is configured for a broker, a broker failure is not permission to start a separate scanner.

### Stdio framing and negotiation

Use UTF-8 JSON-RPC 2.0, one JSON object per newline. Reserve stdout for protocol frames and stderr for redacted diagnostics. Use unique request IDs and correlate responses by ID: concurrent replies may arrive out of order. Native Messaging uses its own length framing instead of newline framing. Respect transport size limits and bound your pending requests.

Call [`runtime.handshake`](../reference/methods.md#method-runtime-handshake) before other operations. Supply `clientName` and `protocolVersion`, then inspect `protocolVersion`, `runtimeVersion`, `methods` and `capabilities` in the response. The service negotiates supported protocol-major compatibility; do not compare the protocol string with an alpha release string. Re-handshaking the same native stream is a conflict. Unsupported versions fail with `PROTOCOL_INCOMPATIBLE`.

The advertised method list describes the transport's compiled surface. It does not expand caller permissions: an ordinary client may see `apps.list` advertised and still receive `LOCAL_ACCESS_DENIED`. Provider operations depend on the connection's actual capabilities and the host's grants. Treat unknown extension fields conservatively; preserve provider-native IDs and options.

Native stream subscriptions use [`events.subscribe`](../reference/methods.md#method-events-subscribe), returning `subscriptionId`, then `events.event` / `events.error` notifications. Unsubscribe the ID when finished. JSON-RPC notifications without an ID do not initiate service work. The HTTP RPC router does not provide these stream subscription intercepts; use session SSE there.

### Shared broker lifecycle

See the [complete broker setup and grant manifest](shared-broker.md). Windows uses an owner-only local named pipe; Unix uses a private socket. A second owner cannot replace a live endpoint.

In parent-owned mode, broker stdin/stdout is a trusted parent control channel. Its descriptor contains an expiring, one-use credential; `{"action":"authorize"}` can issue another. Closing the parent control input stops the broker. Do not publish or log its control output.

In independent mode, `--independent` requires an owner token file or a multi-app grants manifest. Each configured app gets a distinct reusable credential and identity; an optional admin credential is separate. The broker survives consumer disconnection, subject to its configured idle timeout. App credentials last until shutdown or administrative revocation; parent-issued one-use credentials expire after five minutes. Token files remain readable by that OS user and do not isolate mutually hostile processes of the same user.

Disconnect only your transport. Do not kill a shared broker on ordinary app exit. `apps.list` / `apps.revoke` require administrative authority; revoking an app-scoped reusable grant removes its reconnect credential and live connections while leaving peer apps usable. `grants.revoke` concerns the current client's service policy; it is not a substitute for administrative app-token revocation.

### HTTP pairing

Use the exact loopback origin printed by the host. The server checks Host, Origin and cross-site browser requests. Never expose it as a remote API or place credentials in query strings.

1. Explicitly retrieve `GET /runtime/pairing-code`; if exhausted or expired, explicitly issue a replacement with `POST /runtime/pairing-code`.
2. Show the local pairing decision to the user and redeem the code once with `POST /v1/pair`, using the [OpenAPI request shape](../reference/http.md).
3. Keep the returned bearer token in memory. Send `Authorization: Bearer <paired-token>` on protected routes, including `/v1/rpc` and SSE.
4. Perform the handshake, then call only advertised, granted methods. Treat an HTTP authentication failure as a new pairing/recovery decision.

The random code is single-use, expires after five minutes and stops being available after ten failed redemptions. Code responses are `no-store`. A paired token has a one-hour lifetime. Rotating the pending code does not disconnect existing paired clients. Pairing authenticates the client; it does not grant metadata, paid usage or execution. See [pairing details](pairing.md) and the [HTTP route reference](../reference/http.md).

## Discovery, scopes and explicit authority

There are separate permission boundaries:

| Operation | Required decision |
|---|---|
| Passive installation/profile inventory | Host passive-discovery permission; no provider launch or inference |
| Account/model/capability/usage metadata | Host permission for the provider or exact connection; may use the native client, logged-in state or provider network |
| Account identity disclosure | Caller projection permits identity reveal; metadata permission alone is not a disclosure rule |
| Model execution | Separate execution grant for provider/connection and a host-mapped workspace; additional paid/write/native-tool policy as needed |
| Host tools, staged inputs and artifacts | Session ownership plus the corresponding workspace/tool/input/artifact scope |
| Broker administration / OS notifications | Separate administrative or host/operator authorization |

`connectionIds` in host policy restrict visible connections and related resources. These are toolkit identities, not interchangeable with provider/account IDs. The broker grants manifest supplies per-app scopes; RPC parameters cannot choose a different client identity or grant themselves broader visibility. A selector choosing an account/route/model does not change the host policy.

[`discovery.refresh`](../reference/methods.md#method-discovery-refresh) implements passive discovery; stronger `level` strings return `CAPABILITY_UNSUPPORTED`. Cached reads and handshake do not initiate a scan. CLI `--allow-passive-discovery` separately enables autonomous inventory before serving. `--allow-metadata PROVIDER` permits supported background metadata/usage observations. Client interest changes cadence, never permission.

[`discovery.watch`](../reference/methods.md#method-discovery-watch) requires passive permission and configured scan roots. Filesystem watch events invalidate cached work; independently authorized refresh scheduling reconciles it. The inventory lane, slow metadata lane and faster usage lane use bounded admission, backoff and coalescing. Sharing is currently justified within one connection; do not assume that similar account names across connections establish a shared quota pool.

[`connections.probe`](../reference/methods.md#method-connections-probe) explicitly requests metadata for one connection. It retains authorization and provider Retry-After checks. A successful empty catalog and a failed probe are different. [`grants.request`](../reference/methods.md#method-grants-request) reports `requires_host_approval`; calling it does not install a grant. Managed ChatGPT sign-in has a [separate host-owned flow](chatgpt-managed-auth.md); it is not implicit discovery or an execution fallback.

## Inventory identity, evidence and freshness

Do not flatten the following records into a single provider label:

| Record | Meaning |
|---|---|
| [Provider](../reference/schemas.md#schema-provider) | Product/provider identity and supported operation descriptors |
| [Installation](../reference/schemas.md#schema-installation) / [Profile](../reference/schemas.md#schema-profile) | Detected native installation and its profile context |
| [Account](../reference/schemas.md#schema-account) | Account identity observations; disclosure can be redacted |
| [Connection](../reference/schemas.md#schema-connection) | A specific configured access path and its observed authentication/plan/billing state |
| [AccessRoute](../reference/schemas.md#schema-accessroute) | Route from a connection to models and capabilities, retaining billing and pool references |
| [Model](../reference/schemas.md#schema-model) | Toolkit ID plus exact provider-native `nativeId`, connection/route association and revision |
| [Capability](../reference/schemas.md#schema-capability) | Observed support/evidence for an operation, not a host authorization |
| [QuotaPool](../reference/schemas.md#schema-quotapool) / [UsageReading](../reference/schemas.md#schema-usagereading) | Verified or unknown quota scope, provider window/counters/reset observations |

An [Observation](../reference/schemas.md#schema-observation) is a tagged union. `known` carries `value`, evidence source, `observedAt`, freshness and verification; some fields also carry `expiresAt`. `unknown` and `unavailable` carry a reason and check time. Specialized `Observation2`, `Observation3`, etc. are generated instantiations for different value types, not escalating confidence levels.

Keep `fresh` versus `stale`, and `reported` versus `observed` versus `inferred`, visible to consumers. A stale known value is not a new observation; an unknown value is not zero, false, “free”, signed out, or unsupported. A local installation alone proves neither authentication nor subscription. `mode: "synthetic"` and synthetic evidence belong only to clearly labeled examples.

Provider reset times are observations. Timer expiry can mark usage stale and schedule a new provider read; it never resets counters or clears a warning as proof of recovery. Unknown sharing cannot be replaced by a guessed cross-account quota pool. Preserve billing/price uncertainty and never choose paid fallback silently.

[OptionDescriptor](../reference/schemas.md#schema-optiondescriptor) carries the provider-native option ID, control union, choices, scope and current/default evidence. Render the supplied select/Boolean/number/text/unknown control. Do not invent a universal effort ladder or translate unrelated native IDs into one setting. See the [selector and prompt-app flow](prompt-app.md).

## Snapshots, pages and change events

[`snapshot.get`](../reference/methods.md#method-snapshot-get) returns the caller's projected cache, including revision, journal cursor, health and issues. `fields` is an optional projection: keys you did not request can be absent. The generated `Snapshot` is the full result schema; a projected object intentionally does not satisfy all its required keys. Reconcile by stable identities rather than replacing the selected account/route/model merely because rows refresh.

Persisted cache is rehydrated as stale and reverified. `runtime.health` reports registered drivers and service health, not proof of account login, a live model or a grant. Diagnostic extension fields can describe storage and refresh state, but are not a substitute for field-level evidence.

### Catalog pages

Use [`models.page`](../reference/methods.md#method-models-page) for large catalogs. Runtime page limits are 1–500 (default 100); a response has a byte bound and can contain fewer rows. Keep route filters unchanged when following `nextCursor`. Cursors bind to the caller, filter and catalog revision; they are opaque, not offsets to edit. A missing/null `nextCursor` ends the page sequence.

On `SELECTION_STALE`, discard that page sequence and begin again. Bound total rows/bytes and expose partial results if your application stops early. Do not concatenate pages from different revisions. A `Selection.revision` is the selected **model's** revision, not the top-level snapshot revision. Retain route ID, model ID and native option values and revalidate before session creation. Details: [bounded catalog pages](catalog-pages.md).

### Session event cursors

[`events.read`](../reference/methods.md#method-events-read) with `sessionId` returns an `EventPage`. Session cursors and inventory cursors belong to different journals and are not interchangeable. On native streams, subscribe from a retained session cursor and handle `events.error`. On HTTP, `GET /v1/events?sessionId=...&after=...` is a **session** SSE stream; `after` is a bounded decimal session sequence. Authenticate in a header, not in the URL. SSE `gap` and typed `EVENT_GAP` require recovery instead of treating missing deltas as a complete transcript.

For reconnect, establish a new transport, authenticate and handshake again, then re-read the scoped state and resume from an appropriate retained cursor where valid. A helper restart can invalidate ephemeral sessions/resume tokens; inspect native-resumption capability and handle a failed resume explicitly. Reconnection alone does not authorize replaying a prompt or tool call.

### Inventory events and contract gap

Alpha.18 implements `events.read` with `{"scope":"inventory","after":"<snapshot-cursor>"}`. It returns an inventory page with `events`, `cursor`, `tailCursor`, `hasMore` and `revision`; entries are [InventoryChange](../reference/schemas.md#schema-inventorychange) hints. Read a scoped snapshot after a hint and retain its cursor. Anonymous `inventory/*` hints stand in for historical resource IDs whose scope can no longer be verified. On `EVENT_GAP` or an epoch change, discard the old cursor and do a full scoped resynchronization. A hint is not a complete account/model object or an execution event.

**Known canonical-contract gap:** `EventsParams` currently omits `scope`, and the generated `events.read` result describes session `EventPage`, not the inventory page. Therefore generic generated request types/OpenAPI alternatives do not describe this runtime extension completely. The SDK's dedicated TypeScript inventory helpers and Python `subscribe_inventory` implement it. Do not pretend the extension validates against the current method schema, or hand-edit generated protocol files to add it. The documentation validator retains this mismatch explicitly and checks the shared `InventoryChange` payload. See [shared-broker inventory usage](shared-broker.md).

## Metadata versus execution

Inventory reads, discovery and metadata probes do not start model turns. Metadata may still launch a native client or use the network under an explicit provider grant. Showing a model row is not permission to call it, and a selector's eligible result does not override execution policy.

The execution flow is: inspect capabilities and native options → resolve an exact route/model selection → obtain host-approved execution/workspace/tool policy → `sessions.create` (or explicit `sessions.resume`) → stage supported inputs → `turns.start` → consume session events → resolve approvals when requested → cancel/close owned work. Follow the [prompt-app guide](prompt-app.md) for SDK code; the method reference contains individual wire examples, not a ready-to-run paid execution script.

`workspaceId` selects an existing host-mapped workspace, not an arbitrary filesystem path. Default read-only host tools and explicitly granted `native_policy` are different modes. MCP credentials and tools have their own per-session boundary; see [host MCP](host-mcp.md). Session options must use the selected model's descriptors and supported update scope. A provider may not support images, native resumption or every event/tool mode; consult its capability/evidence and the support matrix.

`inputs.put` stages supported input bytes for an owned session; it does not trigger inference. Artifact IDs arrive through session events; `artifacts.read` reads those scoped results rather than arbitrary filesystem paths. Keep bounded `maxBytes` and wire string offsets; see [artifacts](artifacts.md). Resume tokens are opaque and sensitive. Native resumption is adapter-specific; it never means importing a transcript into a silent new paid turn.

`turns.cancel` asks the service to cancel its owned turn. Closing a transport or timing out an SDK wait is not proof that a submitted provider operation was rolled back. Do not cancel unrelated native processes or a peer app's broker session. Cancellation, cleanup and event retention have independent lifetimes.

## Errors and recovery

Native JSON-RPC error responses carry the useful typed domain error in `error.data`:

```json
{"jsonrpc":"2.0","id":12,"error":{"code":-32000,"message":"Local AI Kit request failed","data":{"code":"RATE_LIMITED","message":"Synthetic retry example","retryable":true,"retryAfterMs":2000}}}
```

`DomainError.code`, `message`, `retryable` and optional `retryAfterMs` are defined in the [schema](../reference/schemas.md#schema-domainerror). Numeric JSON-RPC codes classify the envelope: parse error `-32700`, invalid envelope `-32600`, invalid parameters `-32602`, unsupported method/capability `-32601`, and other domain failures `-32000`. Transport/SDK-local failures can have additional local codes; they are not automatically canonical `ErrorCode` variants.

| Condition | Recovery |
|---|---|
| `INVALID_REQUEST`, `CAPABILITY_UNSUPPORTED`, `PROTOCOL_INCOMPATIBLE` | Correct request/feature/version; do not blindly retry |
| `LOCAL_ACCESS_DENIED`, `AUTH_REQUIRED` or an HTTP authentication rejection | Stop denied work; use explicit host approval, account sign-in or local pairing as appropriate |
| `SELECTION_STALE`, `EVENT_GAP` | Re-read scoped inventory or restart the affected page/event sequence |
| `RATE_LIMITED`, `TIMEOUT`, provider/transport failures | Inspect the actual `retryable` flag and Retry-After; retry only an operation safe to repeat |
| `STORAGE_UNAVAILABLE` | Retain a degraded/unknown state, surface the storage problem and use explicit recovery; do not invent a successful commit |

Use bounded backoff with coalescing and cancellation for safe reads/metadata. Honor `retryAfterMs`; a clock reaching a reset deadline does not itself prove quota recovery. A retryable failure does not authorize switching account/route, granting paid access, replaying inference/tools, or spawning a fallback scanner. Serialize catalog recovery with active execution in your app so recovery cannot replace a transport under a live turn.

Issues are durable health observations. Acknowledge/snooze changes presentation, not whether the provider problem has recovered. Notification preferences configure the runtime coordinator; delivering OS notifications additionally requires an explicitly configured/authorized host sink. The documentation quickstart enables none of these actions.

## Contract generation and validation

The single authoring source is Rust [`lai-contracts`](../../crates/lai-contracts/). Its generated JSON feeds the method/schema/HTTP pages. Curated descriptions and fictional examples live in [`api-examples.json`](../reference/api-examples.json); they are documentation, not a second API definition. Required method coverage, all example parameters/results and domain examples are validated using JSON Schema. Unrestricted `true` result schemas remain labeled as unrestricted, even when a source-backed example is more specific.

```sh
# Lightweight docs-only checks; no helper, provider or Rust build:
python scripts/generate_api_reference.py --check
python -m unittest discover -s tests/generation
python scripts/check_project.py

# After changing canonical Rust contracts (requires the build toolchain):
python scripts/generate_contracts.py
python scripts/check_generated.py
```

Use the build dependencies from `scripts/requirements-build.txt` for the validator. Regenerate the Markdown with `python scripts/generate_api_reference.py` after editing curated examples or checked-in contract JSON produced by the canonical generator. Canonical generation also regenerates these pages, and CI checks for drift. The lightweight path does not establish that checked-in JSON matches newly edited Rust; the full canonical check does.

Further reading: [native helper](native-helper.md), [shared broker](shared-broker.md), [pairing](pairing.md), [profile registration](native-profiles.md), [distribution](distribution.md), [security model](../security/threat-model.md), and [troubleshooting](../troubleshooting.md).
