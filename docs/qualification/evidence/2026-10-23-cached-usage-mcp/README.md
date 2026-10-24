# Cached usage MCP synthetic endpoint evidence

Base: `9a40e503c82e3efc15991382322ccf30bc00f806`. Source-only TypeScript SDK slice;
no shared helper/service, credential, persistent grant or actual dot connection.
[Setup and privacy/interest contract](../../../integration/cached-usage-mcp.md).

**Implemented; fixture-tested and platform-executed on Windows, Node 22.20.0.**
One bounded stdio MCP journey, four sequential synthetic cases:

```sh
node --test tests/integration/cached-usage-mcp.test.mjs
```

Final execution: 1 test passed, four owned stdio endpoints, 0.43 seconds total.
The cases cover cached windows, retired cache, empty connection scope and an
actual synthetic subscription `LOCAL_ACCESS_DENIED`. Initialize/list/invalid
calls do not read the cache; each valid tool call reads once. Client request
counters remain unchanged across MCP tool reads. Synthetic host setup for the
denial case uses only mock cache RPC, not a native helper/provider.

Checks include exact scope/redaction, explicit versus missing account mappings,
used/remaining strings, expired reset without recovery inference, stale/unknown/
unsupported windows, no context/quota conflation, denied refresh arguments and
provider RPC, terminal cache clearing and protocol-only stdout. The initial
missing endpoint failed; the live-store disposal issue also failed before its
fix. The captured underlying disposer now clears the snapshot, and terminal
denial retires the existing live cache. No separate unit campaign was run.

`tsc -p packages/sdk-typescript/tsconfig.json --noEmit` passes. A focused SDK emit
to task-owned D: produced 26 files / under 100 KiB; the compiled optional MCP export
imports and constructs without reading the cache. The returned
`quotaPools` validate against the existing canonical `usage.read` result schema;
no generated contract or SDK domain types were hand-edited. The bounded endpoint
check is wired into the SDK/integration verification profiles.

[Receipt and source hashes](receipt.json) · [synthetic structured output](structured-result.json).
The sample is synthetic data, not an account observation or usable endpoint.

## Refresh-fixture CI follow-up — October 24

The [Ubuntu run at `f341eae`](https://github.com/thomaslittle/local-ai-kit/actions/runs/40050350814/job/120741416147)
failed the existing inventory reconciliation journey: `generation-0` remained
visible where `generation-1` was expected. Windows and macOS passed that head.
The CI log does not identify the worker's exact interleaving. A controlled,
bounded blocking-source gate reproduced the same assertion after the test
advanced paused Tokio time through the scan deadline and observed its published
`Timeout`. Yielding runnable tasks does not synchronize blocking-worker delivery.

The fixture now waits for the initial publication, checks that inventory is not
due before 300 seconds, then holds virtual time at admission until the public
snapshot contains the new generation. It advances the remaining time afterward,
preserving the original 335-second budget, scope/no-model assertions and rescan
bound. The source gate and waits remain bounded; no production timeout or
scheduler code changed. No test was disabled and no new unit suite was added.

The single filtered journey passed on Linux/WSL2 (0.09 seconds of test execution);
the existing four-case MCP stdio journey passed on Windows (0.56 seconds total).
Formatting, whitespace and project hygiene passed. Two preliminary diagnostic
runs stopped at the startup wait before reaching the controlled race; their
failed receipts are retained separately. [Focused receipt and hashes](refresh-fixture.json).
These results do not replace the required exact-head hosted CI matrix.

Unqualified: real account mappings/remaining allowances, actual dot MCP access,
live broker/credential binding, published-package consumption, hosted exact-head
CI, and other-platform interoperability. The façade creates no feed: an actual
standalone dot host still needs an explicitly approved cache source/scope/setup.
Tool reads themselves do not subscribe, poll, renew interest, read credentials,
probe, refresh, execute or call model inference. They do not measure this coding
agent's provider allowance.
