# Dependency-first clients

This unpublished source preview supports local packed artifacts and Cargo path dependencies.
See [quickstarts](quickstarts.md) for concrete local installation. An application can use local AI by installing a dependency. It does not need the
standalone app, the broker or the control center, and it never installs them.

Every client resolves in the same order:

1. **Shared broker** the user already granted this application.
2. **Private helper**: a child process owned by this application.
3. **In-process** runtime — Rust only, and only when no helper binary exists.

Automatic clients expose `fallbackReason` (TS), `fallback_reason` (Python), or
`fallback_reason()` (Rust), set to `FALLBACK_PROTOCOL_MISMATCH` when a shared
version mismatch selected a private/embedded runtime. Otherwise it is null/None.
The active mode is always observable, and a missing standalone install is never
an error. A shared broker that is listening but rejects the grant is an error,
not a reason to fall back.

## Rust

```toml
[dependencies]
local-ai-kit = { path = "/absolute/path/to/local-ai-kit/crates/local-ai-kit" }
```

```rust
let mut ai = local_ai_kit::LocalAi::start()?;
let snapshot = ai.snapshot()?;
match ai.mode() {
    local_ai_kit::Mode::Shared => { /* one scanner shared by several apps */ }
    local_ai_kit::Mode::Child => { /* isolated helper owned by this app */ }
    local_ai_kit::Mode::InProcess => { /* fallback, no binary packaged */ }
}
println!("{} models", snapshot.models.len());
```

`StartOptions` narrows the policy (passive discovery is the default; probes are
opt-in) and can point at a specific helper, address or state directory. The
private helper is preferred over in-process because it keeps broker work out of
the application process; in-process is the fallback that makes "no separate
install" true, and it is reported as `Mode::InProcess`.

Tauri and GPUI use this crate. `LocalAi` also exposes `call`, `scan` and
`status`.

## npm

```ts
import {createLocalAI} from '@local-ai-kit/sdk';

const ai = await createLocalAI(); // no transport argument
try {
  const snapshot = await ai.snapshot.get();
  console.log(ai.mode, snapshot.revision);
} finally { await ai.close(); }
```

The helper ships as per-platform optional dependencies, the way esbuild and Biome
do:

```
@local-ai-kit/cli-linux-x64  @local-ai-kit/cli-linux-arm64
@local-ai-kit/cli-darwin-arm64  @local-ai-kit/cli-darwin-x64
@local-ai-kit/cli-win32-x64
```

The SDK resolves `@local-ai-kit/cli-<platform>/package.json` and the binary it
declares, so installation is a normal dependency update: no postinstall script,
no download at import time. The Node transport is intended for Node and Electron's main process; Bun execution must be qualified separately.
Renderer processes use the web bridge instead.

`node scripts/npm-platform-packages.mjs --binary <path> --target <triple> --out <dir>`
builds those packages for a release. The workspace `package.json` stays free of
platform dependencies so `npm ci` is identical on every machine; the preparation
workflow stages private local packs with matching optional dependencies.

An explicit transport still works exactly as before:
`createLocalAI({transport: await openSharedBrokerTransport({...})})`.

## Python

```python
from local_ai_kit import Client

with Client.auto() as ai:
    print(ai.mode, ai.snapshot.get({"fields": ["revision"]}))
```

(`AsyncClient.auto()` is the async form.) Wheels bundle the helper under
`local_ai_kit/_bin/`, and resolution follows the same order.

## Overrides

| Variable | Purpose |
| --- | --- |
| `LAI_HELPER_PATH` | Use a local source build instead of the packaged binary. Must be an absolute path to a real executable; a `.cmd`/`.bat`/`.ps1` wrapper is rejected. |
| `LAI_BROKER_ADDRESS` | Shared broker address (`$XDG_RUNTIME_DIR/local-ai-kit/user.sock` on Linux, `local-ai-kit-user` on Windows by default) |
| `LAI_BROKER_TOKEN_FILE` | This application's grant descriptor (`~/.local-ai-kit/broker-token.json` by default) |

A shared descriptor must be owner-only (`0600`), may not be a symlink, and its
`address` must match the configured one. Nothing is scanned or read at import
time; all work starts on the first call.

## Private helper lifetime

The helper is started as `serve --ipc <unique socket>` with the parent-owned
lifetime: it prints one single-use descriptor and stops when the application
closes its control channel or exits. It never publishes a reusable token file,
so the broker can detect parent-channel closure; OS crash/shutdown behavior still needs platform qualification. The socket lives in the OS
temporary directory with an owner-restricted, per-process name.

## Safety rules that did not change

- Imports perform no scan, probe, credential read, spawn or network call.
- Each application still has its own grant; a shared broker is only used when a
  valid descriptor exists, and revocation is per app.
- Provider credentials and tokens are never exported, logged or written.
- Transports are owner-only sockets/pipes, never an open port.

## Publishing

Nothing is published by this repository without the maintainer's approval. The
release workflow prepares unsigned local artifacts only, with no publication
authority. See [release gates](../release/PUBLISHING.md).
