# Native HTTP and browser qualification

Executed on the authorized Windows x64 machine, Rust 1.97.1, Node 22.20.0, Python 3.13.7, and installed Playwright Chromium.

- Six native HTTP tests passed: explicit single-use pairing, no startup scan or unauthenticated token bootstrap, origin/Host rejection, query credential rejection, bounded bodies, no request-granted privilege escalation.
- Two native transport streaming tests passed: stdio subscription/acknowledgement/unsubscribe and SSE through the real Rust session service with a synthetic driver.
- Three CLI host argument tests passed, including mutually exclusive transports and explicit workspace requirements for execution.
- Two real browser tests passed against the actual native server and isolated home: paired live discovery and rejected incorrect pairing.
- The opt-in real Codex metadata browser check passed: account/model metadata was retrieved via the native service and displayed in the browser model selector. Zero browser errors; zero provider generation prompts submitted. No real account payload or bearer token is stored in this repository.
- SDKs now recognize a server event-gap notification without terminating unrelated requests. Node and Python regression tests cover this behavior.

This qualification does not certify non-Codex adapters, other operating systems, or live inference. New source uses Axum 0.8.9, its documented SSE facilities, and OS randomness through getrandom. Server interfaces and tests are implementation evidence, not a claim that the entire original plan is completed.

## Owned process cleanup

A new regression initially demonstrated an owned descendant continuing to write after its parent client closed. Native client spawning now uses process-wrap 10.0.1 with a Windows Job Object or Unix process group and kill-on-drop. The Windows test passed with an independently spawned control process still running. Both synthetic processes have short self-expiring lifetimes. The test control uses hidden/no-console startup on Windows to avoid unrelated console startup delay. Unix behavior is implemented through the same wrapper but not certified solely from a Windows run.

Sources: https://docs.rs/process-wrap/10.0.1/process_wrap/ and its Tokio ChildWrapper documentation. The application uses safe wrapper interfaces; no unsafe process-management code was added to this workspace.
