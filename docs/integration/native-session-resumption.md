# Native session resumption

Completed turns remain suspendable. The demo continues the resumed session when the route, options, workspace, and billing consent are unchanged. A failed or timed-out suspend leaves that session unusable because the provider's conversation state is uncertain. Revocation and publication are synchronized across grant lifetimes; restoring the same grant cannot revive an older in-flight session or token.

Local AI Kit supports resumption only when the selected native adapter explicitly reports that it can load an existing provider session. The current shared ACP path checks the provider's `loadSession` capability; API/local chat routes, Codex, Claude, and other integrations do not inherit resumption merely because they support multi-turn text.

## Public flow

1. Create and use a normal session.
2. When the session is idle or completed and `session.nativeResumption === true`, call `sessions.suspend({ sessionId })`.
3. The service asks the native driver to detach without closing the provider session.
4. The service returns a cryptographically random, opaque `resumeToken`. It never returns the provider's native session ID.
5. Call `sessions.resume({ resumeToken })` from the same authorized application.
6. The service consumes the token, revalidates the connection and account identity, route and billing classification, model revision, canonical workspace path, and current grants, and only then asks the same native route to load the original session.

Tokens are owner-scoped and one-time. Another app cannot consume or invalidate a token, so an unauthorized attempt does not burn it for its owner. Revocation deletes that application's tokens. Tokens live only in the local runtime's memory and intentionally stop working after a helper restart; there is no transcript import or toolkit-owned conversation database disguised as provider resumption.

## Safety properties

- Suspending an active turn is rejected.
- Suspending a closed session is rejected.
- Unsupported native drivers never expose a resume token.
- Resuming a changed account, route, billing classification, model revision, workspace path, or grant fails closed before the provider is reopened.
- A failed/invalid token does not start a new provider conversation as fallback.
- Raw provider session IDs and opaque resume tokens never appear in diagnostics, events, or rendered demo content. The demo keeps the token only in private in-memory controller state until resume/revoke or page teardown.
- Direct API and local-runtime routes remain ordinary toolkit sessions unless their own adapter later implements a genuine native load-session primitive.

The demo exposes **Suspend native session** and **Resume native session** only when those methods are negotiated and the current session declares native resumption support.
