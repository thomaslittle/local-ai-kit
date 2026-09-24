# OpenCode gateway

The adapter uses the public local OpenCode HTTP API. It never reads OpenCode's credential files. Explicit metadata permission permits `/provider`; the parser includes only connected upstreams and drops keys, headers, raw options, and endpoint URLs. Native model IDs are retained under distinct upstream route IDs. Advertised variants and context bounds become native option descriptors and evidence-backed fields.

The standard gateway response does not verify a subscription plan or publish account allowance. These remain unknown. Optional `providerBilling` declarations are host-supplied and labeled as such; catalog prices are not treated as proof of subscription access.

```json
{"endpoints":[{"id":"gateway","label":"OpenCode","kind":"opencode","baseUrl":"http://127.0.0.1:4096/","providerBilling":{"openai":"subscription"}}]}
```

Use `--endpoints ABSOLUTE_JSON --allow-metadata opencode`. The detected client can also check its documented default loopback port; no other ports are scanned. A separately configured `keyEnv` supplies a password for the gateway's HTTP Basic authentication with username `opencode`. API keys for upstream providers remain owned by OpenCode.

## Text sessions

`--allow-execution opencode --workspace ABSOLUTE_DIRECTORY` enables explicit sessions through the common SDK. The host preserves the selected upstream provider ID independently from the native model ID. Only text-only sessions are exposed. The adapter creates its own native session with a deny-all tool permission ruleset and verifies that the server echoes the restriction **before sending a prompt**. It refuses write/review modes rather than claiming an OS sandbox. It does not change the gateway's global configuration.

The adapter subscribes to the native event stream before posting `prompt_async`, filters events to the owned session and assistant messages, preserves incremental text, and reconciles the final message without duplicating output. Errors and premature stream closure fail explicitly. Cancellation requests abort only that owned native session. Explicit close deletes that session, not the user's existing conversations. No persisted cross-process resumption is claimed. A forcibly killed helper may leave an inert native session record on the gateway.

Gateway authentication is not independently verified identity for every upstream account. Account changes inside the gateway need native metadata/protocol evidence; the toolkit does not infer account IDs from an email or hidden credential. Prefer a separately managed server/profile for strict isolation.

## Sources and evidence

- https://opencode.ai/docs/server/
- https://opencode.ai/docs/permissions/
- `anomalyco/opencode` SDK v2 schema revision `0f549842ee746e400b1f72516b0b2e292e267e2c` (`SessionCreateData`, `SessionPromptData`, `PermissionRuleset`, `Provider`, and streaming event records).

Four gateway catalog tests and two actual mock-server/runtime session tests passed after observed red tests. These verify parsing, routing, permission rejection, streaming, and owned cleanup; they do not certify a live upstream subscription. No live inference was submitted.
