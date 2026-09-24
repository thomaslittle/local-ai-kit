# Local AI Kit

**One integration to discover, inspect, select, and use locally configured AI access.**

A standalone Rust-centered toolkit with a native helper, JSON-RPC protocol, TypeScript/Python SDKs, and framework-neutral model selection.

> **Current status: working native discovery + Codex metadata alpha.** The SDK-to-native bridge is implemented and tested. Codex account/plan/model/effort/usage metadata has been exercised through an installed Windows client. Native generation and the other execution adapters are not finished. The selector showcase remains explicitly synthetic. This is not a release-ready implementation of the entire 18-task plan.

## Run the native implementation

Requires a Rust toolchain, Node.js, and Python for development. Registry packages and cross-platform native distributions have not been published.

```sh
npm ci --ignore-scripts
npm run build
cargo build --locked -p lai-cli
python scripts/test_native_bridge.py
```

The binary is `target/debug/local-ai-kit` (`local-ai-kit.exe` on Windows).

```sh
local-ai-kit scan --json
local-ai-kit serve --stdio --allow-passive-discovery
```

Passive scanning checks supported local installation/profile metadata. It does not read tokens, verify subscriptions, start provider clients, contact providers, or run inference. A detected client is not an authenticated account.

## Inspect an existing Codex account

The [public Python SDK example](examples/python/inspect_codex.py) works through the actual native service. Explicitly authorize the native client metadata probe:

```powershell
python examples/python/inspect_codex.py --binary ./target/debug/local-ai-kit.exe --allow-metadata
```

Without `--allow-metadata`, the example only performs passive discovery. `--connection ID` selects a specific discovered profile. The native helper separately enforces `--allow-codex-metadata`; RPC callers cannot grant this to themselves.

Read the [native helper guide](docs/integration/native-helper.md) and [Codex adapter details](docs/adapters/codex.md). The installed client owns its login; the package never turns subscription tokens into generic API keys. No generation request is submitted by this example. Sensitive billing/permission settings remain gated.

## What is implemented

| Area | Current evidence |
| --- | --- |
| Rust contracts/platform/core/service | Typed evidence, bounded reads, route identity, scoped projections, refresh deduplication, selection validation, and revocation; Windows tests. |
| Passive inventory | Codex, Claude Code, Gemini CLI, Antigravity, Grok, OpenCode, Cursor, Ollama, LM Studio installation/profile patterns. Authentication remains unchecked until separately probed. |
| Native helper | Real `scan --json` and bounded concurrent stdio JSON-RPC with required handshake and explicit discovery authorization. |
| Codex metadata | Account/plan, dynamic models, native effort/service-tier choices, input modalities, percent quota windows, cached probes and backoff; installed Windows client qualified without inference. |
| SDKs | TypeScript HTTP/SSE/stdio and Python async/sync stdio. Both have tests against the real compiled native helper, not only fake peers. |
| Selection engine | Route-aware search/filter/grouping, option validation, reconciliation, and shared-quota presentation. |
| Showcase | Eight selector layouts, including the vertical provider rail, plus synthetic connection/usage/session demonstrations. |

## Selector showcase

```sh
npm run build
node apps/demo/server.mjs --fixture
```

Open `http://127.0.0.1:4499`. The `--fixture` label is intentional. This preview does not silently substitute sample accounts for failed real discovery. The native stdio bridge does not yet provide the native HTTP/SSE host needed to connect this browser showcase to live data.

## Verification

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
npm run test:unit
python -m unittest discover -s packages/sdk-python/tests -v
python scripts/test_native_bridge.py
```

See [qualification records](docs/qualification/) for exact tested scope. Windows execution, synthetic fixtures, and installed-account metadata are distinct evidence. macOS/Linux live qualification, native generation, approvals/resumption, shared multi-app broker, remaining adapters, native HTTP/SSE, React packages, generated SDK domain types, release packaging, and the full performance/security gates remain unfinished.

## Continue implementation

Work on `feature/initial-implementation`, not `main`. Read [AGENTS.md](AGENTS.md) and the [current checkpoint](docs/handoffs/RESUME.md), then the [build instructions](CODEX-BUILD-PROMPT.md). Resume existing code; do not restart the original audit or recreate the implemented SDKs.

| Document | Purpose |
| --- | --- |
| [Architecture specification](docs/superpowers/specs/2026-09-24-local-ai-kit-design.md) | Product boundaries, identities, evidence, quotas, SDKs, security, demo, selectors. |
| [18-task implementation plan](docs/superpowers/plans/2026-09-24-local-ai-kit-implementation.md) | Dependency-ordered tasks and release gates, not a claim that all tasks are complete. |
| [Source audit](SOURCE-AUDIT.md) | T3 Code, CodeNotch, Prmptr, and protocol references. |
| [Implementation ledger](docs/implementation/LEDGER.md) | Incremental work and technical decisions. |
| [Resume checkpoint](docs/handoffs/RESUME.md) | Exact current state and next actions. |

## Licensing and scope

The working name `local-ai-kit` is not an availability-cleared public package brand. The repository remains private. Original material uses the [MIT license](LICENSE); reused code must retain its notices. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), [SECURITY.md](SECURITY.md), and [CONTRIBUTING.md](CONTRIBUTING.md).

The toolkit is independent of Maps and image enhancement. `HANDOFF-MANIFEST.json` is a historical planning-bootstrap manifest, not the hash manifest for the evolving implementation.
