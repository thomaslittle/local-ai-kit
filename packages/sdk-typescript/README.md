# @local-ai-kit/sdk

```ts
import {createLocalAI} from '@local-ai-kit/sdk';
const ai = await createLocalAI(); // shared broker, or a private helper child
console.log(ai.mode, (await ai.snapshot.get()).revision);
```

# Local AI Kit TypeScript SDK (`@local-ai-kit/sdk`)

> Alpha. Not yet published to npm; install the tarball from a release bundle or build from source (see [distribution](../../docs/integration/distribution.md)).

Browser-safe HTTP/JSON-RPC and SSE client, plus a separate Node stdio entry point. No package import scans, reads credentials, starts processes, or makes requests. No provider-specific discovery logic lives here.

```ts
import { createHttpTransport, createLocalAI } from '@local-ai-kit/sdk';
const ai = await createLocalAI({
  transport: createHttpTransport({ baseUrl: 'http://127.0.0.1:4499', token: pairingToken }),
});
try {
  const snapshot = await ai.snapshot.get();
  // Request grants explicitly before enrichment or execution.
} finally {
  await ai.close();
}
```

For a native helper, import `openStdioTransport` from `@local-ai-kit/sdk/node`, pass its absolute executable path and explicit argument array, then give that transport to `createLocalAI`. The helper must implement this project's public protocol; this SDK does not launch Codex/Grok/etc. directly.

Domain and RPC types are generated from the Rust contracts (`src/generated.ts`); regenerate with `python scripts/generate_contracts.py` and verify with `npm run check:generated`.

## Implemented behavior

- Protocol-major handshake and advertised-method checks.
- HTTP requests use bearer headers, omit cookies, and reject redirects; literal loopback hosts only.
- Request deadlines, AbortSignal propagation, explicit close, no automatic retries/fallback.
- Frame/body bounds, response ID validation, redacted error messages.
- SSE preserves UTF-8/chunk/line boundaries and reports explicit event gaps.
- Stdio launches only the explicitly supplied absolute binary with `shell:false`; approved environment names only, no inherited loader injection.
- Concurrent requests are correlated. Events arriving in the subscribe-response chunk are retained. Bounded event queues fail with `EVENT_GAP` rather than silently losing output.

The native service remains responsible for grants, workspace enforcement, secret redaction in successful payloads, native approvals, quota/billing authorization, and cleaning up its own agent descendants. Closing the SDK terminates its owned helper, not arbitrary user processes. A timed-out/cancelled RPC is not proof an already accepted native turn stopped; call `turns.cancel` for a known turn ID.

## Development

Node >=22.16, TypeScript 7. Run `tsc -p tsconfig.json`, then `node --experimental-strip-types --test tests/*.test.mjs`. Tests use local synthetic peers and injected responses, never paid accounts. Build produces the browser entry point; the Node entry point remains native ESM with separate declarations.

The transports are tested against synthetic peers and, in the package smoke test, the real native helper; none of that sends provider inference. No runtime installer, automatic binary download, or registry publication is enabled.
