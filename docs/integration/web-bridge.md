# Web bridge

Plain websites need a deliberate way to reach the local runtime. The extension
plus Native Messaging host is the primary path and opens no port. Loopback HTTP
with pairing is the fallback.

## Extension first (no open port)

The host is the existing native messaging binary. Registration is per browser
and per extension identity:

```sh
node scripts/build_extension.mjs
python scripts/native_host.py --binary target/release/local-ai-kit-native-host \
  --destination /absolute/private/host --browser chrome \
  --extension-id YOUR_CHROME_EXTENSION_ID --install
```

Firefox identifies extensions by id, not by a `chrome-extension://` origin:

```sh
python scripts/native_host.py --binary target/release/local-ai-kit-native-host \
  --destination /absolute/private/host --browser firefox \
  --extension-id originlm@your-domain.example --install
```

- Chrome/Chromium/Edge write `allowed_origins`; Firefox writes
  `allowed_extensions`. The Rust host accepts either shape and checks the caller
  against the exact entry (`valid_caller`), never a prefix.
- Each website origin the extension talks to is approved once by the user in the
  extension UI and stored as a revocable grant. A page cannot grant itself.
- Pages never receive credentials; they receive results.
- The host registration is per user and reversible with `--unregister`.

**Gap:** the example extension currently has no page-facing content script. Until
that content bridge ships, `detectExtension()` answers `false` and sites use the
pairing fallback. The host and manifests are ready for it.

## Pairing fallback (loopback)

The helper can serve loopback HTTP with a one-time pairing code:

```sh
local-ai-kit serve --http --port 47821 --allow-passive-discovery
```

- Strict origin allowlist; the pairing code is single-use and short lived.
- The page exchanges the code for a short-lived, scoped token.
- Chrome's Private Network Access preflights are answered explicitly; a request
  without the right headers is rejected rather than silently retried.
- `DEFAULT_PAIRING_ORIGIN` in `@local-ai-kit/web` is `http://127.0.0.1:47821`;
  the service install can bind it when the user wants web access.

## Detection

```ts
import {detect, badgeLabel} from '@local-ai-kit/web';
const bridge = await detect(); // 'extension' | 'pairing' | 'none'
```

- `extension`: the content bridge answered a postMessage ping.
- `pairing`: a loopback `/runtime/info` probe answered; the page still needs the
  explicit code exchange.
- `none`: show "Connect your local AI" with the install page.

Detection never starts a scan, never grants access and never reads a credential.

## Badge

| State | Copy |
| --- | --- |
| extension or pairing with providers | `Local AI connected · Claude, Ollama` (max three, then `+N`) |
| reachable but no providers yet | `Local AI connected` |
| nothing available | `Not detected` + install link |
| sandboxed page (Flatpak/Snap/App Store) | `Sandboxed` with a one-line explanation |
