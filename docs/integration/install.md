# Install

Two ways in, one runtime. Install it for the dashboard and every app built on
it works; add the dependency and the dashboard is already there.

## Get the app

The local control center for every AI client, model, account and subscription on
this machine: usage and quotas with reset times, model routes and billing,
history and trends, and the issues that need attention. Fully local, no
telemetry, no exported secrets.

macOS and Linux:

```sh
curl -fsSL https://<site>/install.sh | sh
```

Windows:

```powershell
irm https://<site>/install.ps1 | iex
```

The installer drops one binary in a user-level location and registers a user
service (launchd agent, systemd user unit or Task Scheduler logon task). No admin
rights. Uninstall with the matching command:

```sh
curl -fsSL https://<site>/uninstall.sh | sh
```

```powershell
irm https://<site>/uninstall.ps1 | iex
```

User data (state, history, grants) is kept unless you pass `--purge`.

Already building from source? Run the desktop app directly:

```sh
cargo run -p lai-desktop -- --token-file ~/.local/state/local-ai-kit/fedora-shared/control-admin-token.json
```

## Add to your project

The SDK connects to a runtime the user already granted this app, starts its own
private helper when none is running, and never requires the standalone app.

### npm, pnpm, bun, yarn

```sh
npm install @local-ai-kit/sdk
pnpm add @local-ai-kit/sdk
bun add @local-ai-kit/sdk
yarn add @local-ai-kit/sdk
```

```ts
import {createLocalAI} from '@local-ai-kit/sdk';

const ai = await createLocalAI(); // no transport argument
const snapshot = await ai.snapshot.get();
console.log(ai.mode, snapshot.revision);
```

The helper ships as per-platform optional dependencies (`@local-ai-kit/cli-*`),
so installation is a normal dependency update. In Electron, list the helper in
`asarUnpack`; in a browser, use the extension or pairing bridge instead.

### Cargo

```sh
cargo add local-ai-kit
```

```rust
let mut ai = local_ai_kit::LocalAi::start()?;
println!("{:?}: {} models", ai.mode(), ai.snapshot()?.models.len());
```

Tauri and GPUI use this crate. A Tauri app can also ship the helper as an
`externalBin` sidecar if it prefers one binary over the crate.

### Python

```sh
pip install local-ai-kit
```

```python
from local_ai_kit import Client

ai = Client.auto()
print(ai.mode, ai.snapshot.get({"fields": ["revision"]}))
```

## Overrides for kit development

| Variable | Purpose |
| --- | --- |
| `LAI_HELPER_PATH` | Use a local source build instead of the packaged binary |
| `LAI_BROKER_ADDRESS` | Shared broker address |
| `LAI_BROKER_TOKEN_FILE` | This application's grant descriptor |

## What happens at runtime

1. A granted shared broker is used when it is running and speaks a compatible
   protocol.
2. Otherwise the SDK starts a private helper owned by the app; it exits with the
   app.
3. Version mismatches fall back to the private helper and are reported, never a
   hard failure.

See [dependency-first clients](dependency-first.md) for the mode contract and
the safety rules.
