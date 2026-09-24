# Native browser demo and HTTP API

Build with `cargo build -p lai-cli`, `npm run build`, and `node scripts/build_demo.mjs`.

On Windows:

```powershell
.\target\debug\local-ai-kit.exe serve --http --port 4499 --assets "$PWD\apps\demo\dist" --allow-passive-discovery --allow-codex-metadata
```

On macOS/Linux use `./target/debug/local-ai-kit` and an absolute `--assets` path. Open the printed local URL, enter the single-use pairing code from the terminal, then explicitly choose **Scan local clients**. Select a connection and **Check account and models** to authorize its supported metadata probe under the host's existing grant. Unsupported providers report an explicit error; there is no synthetic fallback.

The pair code expires after five minutes and allows ten failed attempts. Bearer sessions expire after an hour. The embedding Rust host may call `HttpServer::issue_pairing_code()` to authorize another app; otherwise restart the CLI to issue another code. Tokens stay in the browser client closure, not URLs, local storage, provider credentials, or logs.

## Explicit execution

Add `--allow-codex-execution --workspace ABSOLUTE_DIRECTORY` to authorize native sessions in the named `default` workspace. This does not start inference. A user must choose a model and explicitly start a turn in Playground. The demo requests read-only execution. Optional CLI flags `--allow-writes`, `--review-native-requests`, and `--allow-paid-options` only raise the host's maximum permission; a request must also select/authorize the corresponding behavior. No metered fallback is automatic.

The HTTP API binds 127.0.0.1 only. Host and Origin are checked, pairing is single-use, API methods require a bearer and protocol handshake, and caller permissions are enforced by the same Rust service used by stdio. Native apps without a browser Origin can use the explicit bearer flow. A page served from another origin is not implicitly trusted.

`POST /v1/pair` takes `{code}`. `POST /v1/rpc` takes the standard JSON-RPC envelope. Read-only REST projections include `/v1/snapshot`, `/v1/connections`, `/v1/models`, and `/v1/usage`. `/v1/events` exposes authenticated SSE with `sessionId` and `after` filters; lost history produces an explicit gap. Stdio supports the equivalent `events.subscribe` / `events.unsubscribe` lifecycle, with bounded output and typed event errors.

## Verification

`node --test scripts/test_live_demo.mjs` exercises real native HTTP and Chromium against an isolated home. `node scripts/check_live_metadata_ui.mjs --allow-metadata` is an explicitly opt-in real Codex metadata browser test; it submits no generation request. Native transport and stream tests are in `crates/lai-transports/tests`.

The existing `node apps/demo/server.mjs --fixture` remains a separate, visibly synthetic demonstration. It is not the native service.
