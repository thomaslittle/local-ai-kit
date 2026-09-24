# Python SDK — implementation preview

An async-first native-helper client with a synchronous facade. Python 3.11+, no runtime third-party dependencies. Imports do not inspect the filesystem, read accounts, start a process, or make network requests.

```python
from local_ai_kit import AsyncClient, AsyncStdioTransport

transport = await AsyncStdioTransport.open(
    executable="/absolute/path/to/local-ai-kit",
    args=["serve", "--stdio"],
)
async with await AsyncClient.connect(transport) as ai:
    snapshot = await ai.snapshot.get()
```

Synchronous callers can use `with Client.from_stdio(executable=..., args=...) as ai: ai.snapshot.get()`. Inside an existing event loop use `AsyncClient`, not the synchronous facade. The sync facade pumps its private event loop during calls; continuous high-volume event consumers should use the async client.

This client speaks Local AI Kit's public JSON-RPC protocol, not Codex/ACP directly. The native Local AI Kit service and provider adapters are not yet available at this checkpoint. The example demonstrates the expected integration shape; the tests use explicit synthetic native peers.

Requests have bounded frames, correlation, deadlines, native error redaction, no automatic retries, and explicit owned-helper cleanup. `asyncio` cancellation cancels the local RPC wait, not necessarily a turn already accepted by the service. Use `turns.cancel` with its turn ID for native turn cancellation. Subscription queues are bounded and report `EVENT_GAP` rather than silently dropping output.

Only safe environment names are inherited by default. The caller explicitly supplies any additional environment entries. This is not a sandbox; the native helper must enforce grants, workspace/tool restrictions, billing, approvals, and cleanup of agent descendants. The SDK terminates only its own helper.

Generated full domain models and an independent Python HTTP transport are not yet shipped. Responses remain Python dictionaries preserving the server's evidence and native values. No discovery/provider logic is duplicated in Python.

Tests: `python -m unittest discover -s packages/sdk-python/tests -v` from the repository root. All account values in fixtures are synthetic. This package is marked private and has not been published or had its public name cleared.
