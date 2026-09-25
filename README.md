# Local AI Kit

**Discover, inspect, select, and use locally configured AI access through one integration.**

A Rust implementation with an embedded library, standalone native helper, authenticated local transports, TypeScript/Python SDKs, and optional React selectors. Applications do not need to reproduce client-specific paths, metadata parsers, or streaming protocols.

> **Unpublished developer alpha.** The native service, supported execution adapters, SDKs, reusable selectors, and installable bundles work. Provider coverage is operation-specific. Gemini CLI, Antigravity ACP, and Grok can be registered explicitly as native profiles and use the installed client's own ACP/login path; this does not imply universal subscription access, known quotas, or live-generation qualification. Read the [support matrix](docs/adapters/SUPPORT-MATRIX.md) and [qualification records](docs/qualification/).

## What is included

| Piece | Integration surface |
|---|---|
| Native engine | Passive discovery; account/plan/model/usage evidence; route-aware selection; granted sessions, streaming, approvals, cancellation, and scoped result artifacts. |
| Rust facade | [`lai-kit`](crates/lai-kit/) embeds the same implementation without a separate process. |
| TypeScript and Python | Thin clients for the actual native service, generated domain types, bounded streaming, cancellation, and verified bundle resolution. |
| Local transports | Stdio JSON-RPC, paired HTTP/SSE, per-application Unix socket/Windows named-pipe broker, and opt-in browser Native Messaging. |
| Optional UI | Framework-neutral selection plus React hooks and eight reusable selector layouts, with native model options. |
| Developer tools | Live/synthetic showroom, generated OpenAPI/JSON Schema, contribution guidance, package smoke tests, and measured benchmarks. |

Finding an installation does **not** establish an active subscription. Unknown plans, usage, capabilities, and billing remain unknown. The installed client owns its credentials; subscription tokens are never turned into generic API keys. Imports never scan, launch clients, contact providers, or generate output.

## Build from source

Development requires the pinned Rust toolchain, Node 22.16+, and Python 3.11+.

```sh
npm ci --ignore-scripts
npm run build
cargo build --locked -p lai-cli
```

The executable is `target/debug/local-ai-kit` (`.exe` on Windows). `scan --json` performs explicit metadata-only filesystem discovery. An empty or unverified account result is not replaced with sample data.

## Run the showroom

For a no-account demonstration of every selector:

```sh
npm run demo
```

Open `http://127.0.0.1:4499`. This mode is explicitly **synthetic** and cannot inspect credentials or call a provider.

For the native, paired demo on Windows:

```powershell
npm run build:demo
.\target\debug\local-ai-kit.exe serve --http --assets "$PWD\apps\demo\dist" --allow-passive-discovery --allow-metadata codex
```

On Unix, use `./target/debug/local-ai-kit` and an absolute path to `apps/demo/dist`. Enter the printed one-use pairing code, then explicitly scan or check a connection. The metadata grant above allows Codex inspection, not inference. Actual turns additionally require a provider execution grant, a named workspace, and an explicit UI action. See the [live demo guide](docs/integration/live-demo.md).

## Register an existing native AI client

Native subscription-style integrations are explicit. A host-owned JSON file names the exact installed client/profile; it does not contain credentials:

```json
{
  "profiles": [
    {
      "id": "grok-personal",
      "label": "Grok personal",
      "provider": "grok",
      "executable": "C:\\\\Users\\\\you\\\\.grok\\\\bin\\\\grok.exe",
      "profileHome": "C:\\\\Users\\\\you\\\\.grok",
      "billing": "subscription"
    }
  ]
}
```

Start the helper with `--native-profiles ABSOLUTE_JSON --allow-metadata grok` to inspect the registered native catalog. Text sessions additionally require `--allow-execution grok --workspace ABSOLUTE_DIRECTORY`. The installed client owns authentication; the toolkit sends no copied token/API key. Billing in this example is a host declaration and remains labeled user-supplied evidence.

## Use the packages

The [distribution guide](docs/integration/distribution.md) shows how to build a checksummed native bundle and install its four npm tarballs or Python wheel. Registry publication and signing are separate, unperformed operations. The standalone native executables need neither Node nor Python.

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
} finally { await ai.close(); }
```

Python and Rust examples use the same normalized service. [C# HTTP](examples/dotnet-http/), [React](examples/react/), [Tauri](examples/tauri/), and [browser extension](examples/browser-extension/) examples show optional host integration; check each example's qualification, not just its source presence.

The [React package](packages/react/) supplies hooks. The [UI package](packages/ui-react/) supplies provider rail, compact, command-palette, capability-first, comparison, allowance-aware, preset, and mobile selectors. Both are optional; the SDK and native core do not require React.

## Verify and package

Install Python test/build dependencies in a project virtual environment using `scripts/requirements-build.txt`, and install Playwright Chromium with `npx --no-install playwright install chromium`.

```sh
python scripts/verify.py --profile all
python scripts/verify.py --profile release
```

The release profile additionally requires a .NET 8 SDK for the plain HTTP example. It runs lint, adapter conformance, checksummed packaging, isolated tarball/wheel consumers, and a 60-second idle benchmark. Nothing publishes packages or spends an AI allowance. Results are saved in `.verification/` and bundles in `dist/release/`; manifests identify the exact source commit and any dirty working tree.

Automated browser checks cover interaction, live pairing, and common WCAG A/AA violations. They are not a complete manual accessibility certification. Platform and live-account qualification remain distinct from unit/fixture test results.

## Documentation and limitations

Start with the [support matrix](docs/adapters/SUPPORT-MATRIX.md), [native helper](docs/integration/native-helper.md), [generated API](docs/integration/generated-api.md), [result artifacts](docs/integration/artifacts.md), and [adapter authoring guide](docs/adapters/AUTHORING.md).

Native Gemini CLI, Antigravity ACP, and Grok routes require explicit profile registration; passive detection alone never enables them. Grok metadata has been qualified without inference on one Windows client, while Gemini CLI/Antigravity live qualification remains unavailable on the current PC. Cursor exposes only separately authorized, stale non-secret cached metadata. Direct API routes never substitute for subscription access without explicit billing permission. ACP native routes now support capability-gated suspend/resume through owner-scoped opaque tokens while the local runtime remains alive; raw provider session IDs are never exposed. Tokens intentionally expire on revoke/runtime restart rather than pretending the toolkit owns durable provider history. Exhaustive provider quota metadata and macOS qualification are not claimed yet. Opt-in local metadata watching is implemented through `discovery.watch`; it invalidates evidence without auto-probing providers.

The original [architecture](docs/superpowers/specs/2026-09-24-local-ai-kit-design.md) and [18-task plan](docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md) remain the full intended scope, not a checklist retrospectively declared complete. [RESUME.md](docs/handoffs/RESUME.md) and the [implementation ledger](docs/implementation/LEDGER.md) distinguish implemented work from unresolved scope.

## License

Original code uses [MIT](LICENSE). Preserve [third-party notices](THIRD_PARTY_NOTICES.md). The repository is private and the working package name is not an availability-cleared public brand. No registry release, notarization, signed binary, or hosted credential service is implied.

This toolkit is independent of Maps and image enhancement. `HANDOFF-MANIFEST.json` records the historical planning bootstrap only; current distribution checksums live in each build's `manifest.json`.
