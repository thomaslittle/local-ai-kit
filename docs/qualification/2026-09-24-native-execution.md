# Native execution development evidence

The generic Rust session engine and Codex execution driver are implemented with synthetic native-protocol peers, not a substitute production provider. Tests exercised text streaming before the turn acknowledgement, native approval denial, cancellation, account-change rejection, sandbox-response rejection, idempotency boundaries, owner-scoped results, and service-level permission checks.

Source: https://developers.openai.com/codex/app-server/ (reviewed September 24, 2026), with the already generated schemas from installed Codex 0.156.1. No third-party source was copied for these modules.

## Boundaries

Session creation performs metadata validation but does not submit a generation request. Text requests use the same selected model and native effort values, an ephemeral native thread, standard service tier unless explicitly authorized, and a native read-only or explicitly authorized workspace-write sandbox. Native resumption across process restarts is not advertised for ephemeral sessions. Unknown native requests and account changes fail closed. Read-only sessions never approve native escalation. A native sandbox is provider enforcement, not a guarantee that an arbitrary third-party executable is harmless.

The public service requires a separate host-owned execution grant with named workspace roots. RPC cannot install a grant, choose an executable, override native credentials, or enable writes or paid use beyond that grant.

Tests are fixture/protocol qualification. Live generation and full operating-system sandbox/process-tree qualification are not claimed by this checkpoint.

## Lifecycle review fixes

Regression tests now cover bounded replay pages with a continuation cursor, recycling closed session capacity, metadata becoming stale during a previously authorized turn, session-specific billing consent, and native account re-verification before a new Codex prompt. These tests failed on the earlier implementation and passed after the fixes. Current identity, explicit revocation and workspace constraints remain enforced during streaming; mere metadata age does not cancel existing work. A new turn still requires fresh validation. Native client account changes are checked before submitting the prompt.
