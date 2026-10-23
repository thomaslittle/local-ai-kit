# Native helper and SDK bridge

The standalone helper, embedded Rust facade, shared broker, and live showroom use the same native service. The separate synthetic demo never substitutes for a failed native connection.

## Build and verify

```sh
npm ci --ignore-scripts
npm run build
cargo build --locked -p lai-cli
python scripts/test_native_bridge.py
```

Bridge tests run Node/Python against the actual executable with isolated temporary homes. They check discovery, concurrent requests, permission denial, broker ownership, and revocation without needing a provider account.

## Direct CLI

```sh
local-ai-kit scan --json
local-ai-kit serve --stdio --allow-passive-discovery
```

Imports, handshake, and cached snapshot reads do not scan. `--allow-passive-discovery` permits metadata-only requests. `--home ABSOLUTE_DIRECTORY` isolates roots/PATH/home overrides for deterministic tests and never falls back to the real home.

Stdio uses bounded UTF-8 newline JSON-RPC. The host supplies caller identity; request parameters cannot grant permissions or choose an arbitrary executable. Handshake precedes other operations. Subscribe to native events only when advertised. Large model catalogs can be read with [bounded pages](catalog-pages.md).

Direct CLI stdio output runs on a dedicated bounded worker rather than Tokio's
provider blocking pool. The existing five-second output deadline still closes an
unresponsive connection. A timeout reports only `stage`, `response_bytes` and
`elapsed_ms`; it never includes request IDs, prompts, account data or response
contents. Do not increase a consumer's request timeout to compensate for an
output-worker failure. A timed-out OS write may finish before the owned worker
exits; cancellation drops the output channel and does not enqueue another frame.

## TypeScript

```ts
import { createLocalAI } from '@local-ai-kit/sdk';
import { openStdioTransport } from '@local-ai-kit/sdk/node';
const transport = await openStdioTransport({
  executable: '/absolute/path/to/local-ai-kit',
  args: ['serve', '--stdio', '--allow-passive-discovery'],
});
const ai = await createLocalAI({ transport });
try {
  await ai.discovery.refresh({ level: 'passive' });
  console.log(await ai.connections.list());
} finally { await ai.close(); }
```

Use the full `.exe` path on Windows. The package's optional `openBundledTransport` verifies a local distribution manifest before launching; no shell or implicit download is required.

## Python

```python
from local_ai_kit import Client
with Client.from_stdio(
    executable='/absolute/path/to/local-ai-kit',
    args=['serve', '--stdio', '--allow-passive-discovery'],
) as ai:
    ai.discovery.refresh({'level': 'passive'})
    print(ai.connections.list())
```

## Metadata, execution, and browser access

Provider metadata requires a separate host flag, for example `--allow-metadata codex`. Actual turns additionally require `--allow-execution codex --workspace ABSOLUTE_WORKSPACE` and explicit session/turn requests. A metadata probe is not an inference request. Inspect the [support matrix](../adapters/SUPPORT-MATRIX.md) for each provider's actual operations.

HTTP/SSE and browser pairing are implemented. Use `serve --http --assets ABSOLUTE_DEMO_DIRECTORY --allow-passive-discovery`; enter the one-use code printed by the host. See the [live demo guide](live-demo.md), [distribution guide](distribution.md), and [artifact interface](artifacts.md). Closing a client releases its own scopes, not another application's broker session. Static assets are limited to `html`, `css`, `js`, `mjs`, `svg`, and `woff2` files under the assets root (2 MiB cap); the exact paths `/overview`, `/connections`, `/models`, `/usage`, `/selector-lab`, `/playground`, and `/diagnostics` serve `index.html` for client-side routing, and every other path without a matching file is a 404.

Current results live in the [qualification records](../qualification/). Earlier test counts were incremental checkpoints, not current totals. Passive inventory never reports an active subscription merely from a local installation. Native Google/Grok execution, automatic service-level watch refresh, and persistent cross-process resumption remain explicitly unavailable where documented.
