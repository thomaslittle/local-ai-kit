# Synthetic showcase service qualification

This is a test/demo harness, not native account discovery or a replacement for the planned Rust broker. All model, account, plan, and quota records explicitly carry synthetic evidence. It has no credential reader, provider API, or native agent launcher.

Environment: assistant Linux container, Node 22.16.0. `node --test apps/demo/tests/*.test.mjs` — 22 passed: 15 service/selection/session tests and 7 actual local HTTP/SSE tests.

Test-first checks covered synthetic identity, advertised methods, no scan on snapshot read, native model duplicates, shared quotas, unknown denominators, grants, stale selections, unsupported options, metered consent, stream/cancel, approval denial, revocation, diagnostics, closed-session mutation, bounded history, and exact event cursors. The final two lifecycle/bounds regressions were observed failing before the fix, then passed.

Real HTTP tests cover loopback binding, bearer auth, denied Host/Origin, bounded bodies, safe static paths, error redaction, and an actual streaming simulated turn through the shipped HTTP SDK. The Host test uses node:http to control the Host header; browser fetch does not permit that override.

The fixture bootstrap token belongs only to this demonstration harness. It is never a provider credential and is not a production pairing protocol. The native per-client permission service, live metadata, live inference, and platform certification remain unimplemented.
