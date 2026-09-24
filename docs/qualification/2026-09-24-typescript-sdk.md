# TypeScript SDK checkpoint

Executed on Linux in the assistant container with Node 22.16.0 and TypeScript 5.8.3. Uses synthetic local child processes and injected HTTP streams only; no personal account or inference used.

HTTP/SSE: observed 18 failing red tests on explicit scaffolds, then 18 passing tests on implementation. Stdio: observed 14 red tests; first green attempt passed 13 and exposed a scheduling-sensitive overflow test. The fixture/test was changed to synchronize with an explicit flood acknowledgement (not a timing sleep), then all 14 passed.

`python scripts/verify.py --profile javascript`: exit 0, builds both TypeScript packages, runs 24 selector tests plus 32 SDK tests, all passing. Total 56 behavioral tests. Native Rust tests have not run green; desktop and CI limitations are recorded separately.

Verified: exact JSON-RPC names and correlation, protocol-major negotiation, side-effect-free construction, explicit billed-route handling in selectors, authentication headers, no redirects/cookies, bounded bytes/queues, malformed data, timeouts, AbortSignal, no automatic retries, EOF/event-gap semantics, owned-child close, environment filtering, stderr suppression, and no shell interpretation of prompt data.

This is a usable tested transport/selection implementation slice. It does not establish a working native discovery service, live provider integration, full generated SDK domain types, or complete Task 7/13/14 acceptance.
