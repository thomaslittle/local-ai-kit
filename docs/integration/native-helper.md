# Native helper and SDK bridge

The native binary now runs the Rust service. It does not substitute the showcase's synthetic service.

## Build and verify

```sh
npm install --ignore-scripts
cargo build --locked -p lai-cli
npm run build
python scripts/test_native_bridge.py
```

The verification command launches the actual helper from Node and Python with synthetic temporary home directories. It performs native filesystem discovery, concurrent requests, permission denial, and revocation. No provider client or credential is needed.

## Direct CLI

```sh
local-ai-kit scan --json
local-ai-kit serve --stdio --allow-passive-discovery
```

Serving alone grants no scanning permission. SDK import, handshake, and snapshot reads do not scan. `--allow-passive-discovery` authorizes metadata-only requests from the process owner. `--home ABSOLUTE_DIRECTORY` isolates all roots, PATH, and home overrides; useful for deterministic examples/tests. It never falls back to a real home.

The stream is newline-delimited JSON-RPC 2.0 over UTF-8. Negotiate `runtime.handshake` before other operations. A request ID is required for work; notifications cannot initiate scans. Frames and concurrent requests are bounded. stdout contains only protocol messages. Caller identity comes from the native host, not request parameters. There is no HTTP listener or browser pairing in this increment.

The handshake advertises only implemented methods. Agent execution and event subscriptions remain unavailable; clients must not imply they work because the SDK has a corresponding method wrapper.

## Node

```js
import { createLocalAI } from '@local-ai-kit/sdk';
import { openStdioTransport } from '@local-ai-kit/sdk/node';
const transport = await openStdioTransport({
  executable: '/absolute/path/to/local-ai-kit',
  args: ['serve', '--stdio', '--allow-passive-discovery'],
});
const ai = await createLocalAI({ transport });
try {
  await ai.discovery.refresh({ level: 'passive' });
  const snapshot = await ai.snapshot.get();
  console.log(snapshot.connections);
} finally { await ai.close(); }
```

Use the full `.exe` path on Windows. No shell wrapper is needed to launch this native helper.

## Python

```python
from local_ai_kit import Client
with Client.from_stdio(
    executable='/absolute/path/to/local-ai-kit',
    args=['serve', '--stdio', '--allow-passive-discovery'],
) as ai:
    ai.discovery.refresh({'level': 'passive'})
    print(ai.snapshot.get()['connections'])
```

## Current evidence

Windows x64: Rust 1.97.1, Node 22.20.0, Python 3.13.7. 71 Rust tests plus six Node/Python-to-native integration tests pass. A single passive scan of the authorized development PC found 21 client/profile connections across nine providers in approximately 34 ms including process startup. This is one observation, not a percentile benchmark or a count of active subscriptions. Accounts, plans, usage, and models remain unprobed by passive scanning.
