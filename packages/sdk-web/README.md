# @local-ai-kit/web

Browser bridge detection for OriginLM. Importing performs no request.

```ts
import {badgeLabel, detect} from '@local-ai-kit/web';

const bridge = await detect(); // 'extension' | 'pairing' | 'none'
document.body.dataset.originlm = badgeLabel(bridge, providers);
```

The extension path needs its page content bridge (see
[docs/integration/web-bridge.md](../../docs/integration/web-bridge.md)); the
pairing fallback probes a loopback origin and leaves the code exchange to the
explicit connect flow.
