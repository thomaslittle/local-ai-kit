# Local AI Kit

**Discover, inspect, select, and use the AI access already configured on a user's computer, through one integration.**

[![CI](https://github.com/thomaslittle/local-ai-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/thomaslittle/local-ai-kit/actions/workflows/ci.yml)
[![React Doctor](https://github.com/thomaslittle/local-ai-kit/actions/workflows/react-doctor.yml/badge.svg)](https://github.com/thomaslittle/local-ai-kit/actions/workflows/react-doctor.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Status: alpha](https://img.shields.io/badge/status-alpha-orange.svg)

Local AI Kit is a Rust toolkit with an embedded library, a standalone native helper, authenticated local transports, TypeScript and Python SDKs, and optional React selectors. Your application does not have to reproduce client-specific paths, metadata parsers, or streaming protocols for each AI client a user might have installed.

> **Alpha.** The native service, supported execution adapters, SDKs, selectors, and installable bundles work, but the API can still change between `0.1.0-alpha.N` releases. Nothing is published to npm, PyPI, or crates.io yet; use [GitHub release bundles](https://github.com/thomaslittle/local-ai-kit/releases) or build from source. Provider coverage is operation-specific: read the [support matrix](docs/adapters/SUPPORT-MATRIX.md) before relying on a provider.

## API Reference

**[Read the API Reference](docs/integration/api-reference.md)** for a safe end-to-end quickstart, transport/pairing lifecycle, permissions, inventory and event semantics, and recovery. Browse **[all methods with request/response examples](docs/reference/methods.md)**, the **[wire schemas](docs/reference/schemas.md)**, and **[HTTP routes](docs/reference/http.md)**. These pages are generated from the canonical contracts with validated synthetic examples.

## What it does

- **Passive discovery.** Finds installed AI clients and profiles (Codex, Claude Code, Grok, Gemini CLI, Antigravity, OpenCode, Muse Code, Pi, Cursor, GitHub Copilot, Ollama, LM Studio, and API routes you configure) from metadata only. No scanning at import time, no secrets read, no network calls.
- **Evidence-backed inventory.** Provider, installation, profile, account, connection, route, model, billing, and quota-pool identities stay distinct, with field-level evidence. *Unknown* and *unsupported* are different states, and unknowns are never filled in with guesses.
- **Selection.** Route-aware, framework-neutral selection logic that keeps each provider's native model IDs and option descriptors. There are no hard-coded universal "Low/Medium/High" controls.
- **Granted sessions.** Streaming text sessions, approvals, cancellation, image input, host MCP tools, resumable sessions, and scoped result artifacts, only after a host grants them explicitly.
- **Explicit native-tool sessions.** Host MCP defaults to read-only, host-tools-only behavior. An operator may separately grant native CLI shell, file, web, and build tools with `native_policy`; see the [host MCP contract and provider details](docs/integration/host-mcp.md).
- **Thin SDKs.** Rust is the canonical implementation. The [TypeScript](packages/sdk-typescript/) and [Python](packages/sdk-python/) SDKs are typed protocol clients generated from the Rust contracts. The React [hooks](packages/react/) and [eight selector layouts](packages/ui-react/) are optional.

### Safety model

Finding an installation does **not** establish an active subscription, and a discovery signal is never permission to execute.

- The installed client owns its credentials. Subscription tokens are never exported as API keys, browser cookies are never read, and nothing silently falls back to paid usage or rotates accounts to evade limits.
- Metadata, execution, workspace access, tools, and paid options are separate, revocable, per-application grants.
- Local transports are loopback/owner-scoped and authenticated; cancellation only touches processes the toolkit started.

See the [threat model](docs/security/threat-model.md) and [SECURITY.md](SECURITY.md).

## Quick start

You need the pinned Rust toolchain (installed automatically by `rustup` from [`rust-toolchain.toml`](rust-toolchain.toml)), Node.js 22.16+, and Python 3.11+.

```sh
git clone https://github.com/thomaslittle/local-ai-kit.git
cd local-ai-kit
npm ci --ignore-scripts
npm run build
cargo build --locked -p lai-cli
```

The helper is `target/debug/local-ai-kit` (`.exe` on Windows). Run a metadata-only scan of this machine:

```sh
target/debug/local-ai-kit scan --json
```

To see exactly what discovery finds without touching your real home directory, point it at an empty directory: `scan --json --home /absolute/empty/dir`.

### One shared runtime and the control center

```sh
target/debug/local-ai-kit serve --ipc local-ai-kit-user --independent \
  --token-file "$HOME/.local-ai-kit/broker-token.json" \
  --admin-token-file "$HOME/.local-ai-kit/broker-admin-token.json" \
  --allow-passive-discovery --idle-timeout 900
target/debug/local-ai-kit-control \
  --token-file "$HOME/.local-ai-kit/broker-admin-token.json"
```

Ordinary applications receive the normal token; the admin token is only for
the control center because it can list and revoke connected apps. Omitting
`--admin-token-file` leaves that management surface unavailable. For multiple
apps, replace `--token-file` with `--grants-file` (see
[the shared-broker guide](docs/integration/shared-broker.md)); each app then
gets a distinct token and identity, and revocation is app-scoped.

The broker owns its lifetime; closing a consumer does not stop it. The control
center is a native Rust terminal UI over the public authenticated API, with
Overview, accounts, models, usage, issues, applications, diagnostics, safe
refresh, issue snooze/acknowledge, app revocation, and redacted export. It never
opens SQLite or calls a provider. See the
[shared broker guide](docs/integration/shared-broker.md) and the
[decision record](docs/design/ADR-001-shared-runtime.md).

### Run the showcase

```sh
npm run dev
```

Open <http://127.0.0.1:5173/>, choose **Show current pairing code**, then **Pair**. The demo shows your real detected installations after an explicit browser pairing and never sends a prompt unless you grant a session and click **Run live turn**. With no paired runtime it falls back to clearly labeled example data. See the [live demo guide](docs/integration/live-demo.md) and [`apps/demo`](apps/demo/README.md).

## Use it from your app

```ts
import { createLocalAI } from '@local-ai-kit/sdk';
import { openBundledTransport } from '@local-ai-kit/sdk/node';

const transport = await openBundledTransport({
  bundleDirectory: '/absolute/path/to/extracted/bundle',
  args: ['serve', '--stdio', '--allow-passive-discovery'],
});
const ai = await createLocalAI({ transport });
try {
  await ai.discovery.refresh({ level: 'passive' });
  console.log(await ai.connections.list());
} finally {
  await ai.close();
}
```

| Integration | Where to start |
|---|---|
| Install a built bundle (helper, npm tarballs, Python wheel) | [Distribution](docs/integration/distribution.md) |
| Native helper CLI, stdio / HTTP / broker transports | [Native helper](docs/integration/native-helper.md), [pairing](docs/integration/pairing.md), [shared broker](docs/integration/shared-broker.md) |
| Prompt-driven desktop app (provider/model selection, grants, streaming) | [Prompt-app guide](docs/integration/prompt-app.md) |
| Register Gemini CLI / Antigravity / Grok profiles; declare billing | [Native profiles](docs/integration/native-profiles.md) |
| Rust embedding | [`lai-kit`](crates/lai-kit/) and its [examples](crates/lai-kit/examples/) |
| Python | [`examples/python`](examples/python/), [Python SDK](packages/sdk-python/) |
| React | [`examples/react`](examples/react/), [React hooks](packages/react/), [UI selectors](packages/ui-react/) |
| C# over HTTP, Tauri, browser extension | [`examples/dotnet-http`](examples/dotnet-http/), [`examples/tauri`](examples/tauri/), [`examples/browser-extension`](examples/browser-extension/) |
| API guide, method examples, wire schemas, HTTP routes | [API Reference](docs/integration/api-reference.md), [methods](docs/reference/methods.md), [schemas](docs/reference/schemas.md), [HTTP](docs/reference/http.md) |
| Contract generation, OpenAPI, JSON Schema | [Generated API](docs/integration/generated-api.md), [`protocol/`](protocol/) |
| Add a provider adapter | [Adapter authoring guide](docs/adapters/AUTHORING.md) |

## Repository layout

| Path | Contents |
|---|---|
| `crates/lai-contracts`, `lai-core` | Canonical serializable contracts; selection and permission-scoped projections. No I/O. |
| `crates/lai-discovery`, `lai-platform` | Explicit passive inventory; injected OS boundaries. |
| `crates/lai-adapters`, `lai-chatgpt-auth` | Provider integrations (native clients, ACP, HTTP/API routes); host-owned ChatGPT sign-in. |
| `crates/lai-runtime`, `lai-service`, `lai-transports` | Owner-scoped sessions; the shared native service; stdio, HTTP/SSE, broker, and Native Messaging transports. |
| `crates/lai-store` | Broker-owned SQLite snapshot, metadata, bounded journal, issues, and notification delivery state. |
| `crates/lai-control` | Optional native Rust terminal control center over the public broker API. |
| `crates/lai-update` | Ed25519 update verification, staging, active-session lease, activation, and rollback. |
| `crates/lai-kit`, `lai-cli` | The embeddable Rust facade; the `local-ai-kit` helper executable. |
| `packages/` | `sdk-typescript`, `sdk-python`, `selector-core`, `react`, `ui-react`. |
| `apps/demo` | The React showcase (uses only the public SDK). |
| `protocol/` | Generated JSON Schema, OpenAPI, and method list. |
| `docs/`, `examples/`, `scripts/` | Documentation, host-integration examples, build/verify/release tooling. |

## Platform status

| Platform | Status |
|---|---|
| Windows x64 | Verified; unsigned prerelease helper bundles are attached to [releases](https://github.com/thomaslittle/local-ai-kit/releases). |
| Linux x64 | Verified on Ubuntu (WSL2) and exercised in CI; no release bundle yet. |
| macOS | Built and tested in CI only; not manually qualified. |

Status words in this repository are deliberate: **planned**, **implemented**, **fixture-tested**, **platform-executed**, and **live-qualified** mean different things. Most provider paths are fixture-tested; live qualification is called out per provider in the [support matrix](docs/adapters/SUPPORT-MATRIX.md) and in the [qualification records](docs/qualification/).

## Verify

```sh
python -m venv .venv-build            # then activate it with your shell
python -m pip install -r scripts/requirements-build.txt
npx --no-install playwright install chromium
python scripts/verify.py --profile all
```

`verify.py --profile all` runs the Rust workspace, SDK, generator, selector, browser, and accessibility checks against synthetic homes, fake clients, and mock servers. It never spends an AI allowance. `--profile release` additionally needs a .NET 8 SDK, builds a checksummed bundle, tests clean tarball and wheel consumers, and measures an idle baseline. See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.

## Documentation

- [Documentation index](docs/README.md)
- [Support matrix](docs/adapters/SUPPORT-MATRIX.md) and [adapter docs](docs/adapters/)
- [Architecture specification](docs/design/architecture.md) and [threat model](docs/security/threat-model.md)
- [Changelog](CHANGELOG.md) and [development log](docs/implementation/DEVELOPMENT-LOG.md)
- [Troubleshooting](docs/troubleshooting.md)

## Contributing, security, license

Contributions are welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md) and the [Code of Conduct](CODE_OF_CONDUCT.md). Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).

Local AI Kit is released under the [MIT License](LICENSE). It builds on and credits several open-source projects; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and the [source audit](docs/design/source-audit.md). Provider and product names are trademarks of their owners; their use here does not imply affiliation or endorsement.
