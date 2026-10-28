# Cached usage producer and T3 source integration

Base: `3c439561efb058658c8bc96666ca67d6bfde9074` (green PR32).
T3 source inspected read-only at `5bb54a0f77c6655fdb2ab551612ca1ca1fcd3667`;
its checkout/branch were unchanged. [Source composition](../../../../examples/mcp/t3code/README.md)
and [scope/approval contract](../../../integration/cached-usage-mcp.md#cache-producer-and-t3-host-binding).

**Implemented, fixture-tested and platform-executed on Windows, Node 22.20.0.**

- One producer-to-caller lifecycle journey uses the real SDK SnapshotStore with
  synthetic data. Construction is inert; explicit attachment observes one local
  subscription. Duplicate attach, copied scope, explicit account mapping, hidden
  identities, caller isolation, zero source reads on denied/tool calls, source
  updates, stale/reset preservation, clock rewind, detach/late-callback retirement,
  source disposal and binding revocation pass. Its first execution failed because
  the adapter module did not exist.
- Final focused execution ran that journey and the existing four-case stdio
  journey: **2 tests passed, zero failed**, 0.48 seconds total. The owner's one
  synthetic `store.load()` drives an update; producer/tool reads never initiate
  client/provider work. No new unit suite or native build was run.
- The native Effect toolkit module type-checks against T3's installed Effect
  **4.0.5**. An isolated in-memory Effect toolkit, real SDK producer and per-call
  synthetic context exercise five calls: allowed, different caller, excess
  arguments, retired source, revoked binding. All assertions pass; broker/provider
  request count **0**, network listeners **0**. The registered callable receives
  each request's context; it does not retain the first approved caller.
- SDK strict no-emit type check, focused SDK emit, project hygiene/link checks and
  staged whitespace pass. Copied/emitted source demonstration output is **95,738
  bytes**, below its 2 MiB cap. Dependencies are reused read-only; no installation,
  global configuration, credential read, persistent data or shared service change.

[Receipt, exact source and log hashes](receipt.json). The receipt records retained
private-log hashes without publishing local paths or any live account data.

**Not deployed or live-qualified:** T3's complete HTTP/OAuth/layer composition,
actual dot tool registration, a selected approved cache host, cross-process cache
transport, account/billing mappings, provider allowances and live cached reads.
T3 currently has no Kit SnapshotStore service. These factories do not create one
or convert T3 transcript/context counters into Kit quota windows. No previous
release contains this source module. Exact-head hosted CI remains a separate gate.

Remaining approval must identify the cache owner/feed policy, exact Kit account
and connection IDs, operator-confirmed consumer account mapping, destination T3
environment and external MCP client session. Building/installing the candidate,
composing/deploying or restarting that host, any new feed or persistent access,
and one real cached cloud read each require their stated scope. No permission is
inferred from existing orchestration credentials. No model inference is needed.
