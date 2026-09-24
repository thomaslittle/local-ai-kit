# Python SDK qualification checkpoint

Environment: assistant Linux container, Python 3.13.5. No personal accounts, credentials, or inference used. Uses actual local Python subprocesses running deterministic synthetic protocol peers.

Red: 17 behavior checks errored on the initial scaffold; the import side-effect guard already passed. Green after implementation: `python -m unittest discover -s packages/sdk-python/tests -v` — 18 passed, exit 0.

Verified: async and sync client protocol handshake, concurrent request correlation, no shell interpretation of data, absolute-path validation, timeouts without retries, task cancellation, malformed/oversized output, bounded outgoing frames, child exit, safe native errors, subscription acknowledgement ordering, explicit slow-consumer gaps, method availability, idempotent close, safe environment inheritance, and inert imports.

The Python package was also built as a wheel without network/build isolation, installed to an isolated temporary target, imported outside the repository, and round-tripped a real synthetic subprocess. Publication remains disabled. These are transport tests, not a claim of native provider or Windows/macOS qualification. Python HTTP transport and full generated domain models remain pending.
