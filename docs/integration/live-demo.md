# Native browser demo and HTTP API

Run `npm run demo` to build the public SDK-backed assets and start the native paired HTTP helper on port 4499. The default command is live only and never uses the synthetic fixture service.

On Windows:

```powershell
npm run demo
```

On macOS/Linux use the same `npm run demo` command. Open the printed local URL and enter the single-use pairing code from the terminal. After pairing, the browser performs passive discovery and sequential metadata-only checks for providers the host allows: Codex, Claude Code, Grok, Gemini CLI, Antigravity, OpenCode, Ollama, LM Studio, and Cursor's cached metadata. The page shows progress and an outcome for every detected profile. Checks may launch a native metadata client or contact a configured local endpoint, but they do not request model generation. Gemini CLI and Antigravity still need explicit native profile registration with `--native-profiles`; Cursor's cached identity and plan remain stale and cannot verify authentication. No execution or inference grant is enabled. A profile appears as authenticated only after a fresh provider result confirms it. Unsupported providers report an explicit error; there is no synthetic fallback.

When the OpenCode CLI is available, the demo starts a temporary loopback-only OpenCode server protected by a random Basic Auth password. The password is passed only through child-process environments; a temporary endpoint file contains the environment variable name, never its value. The server is stopped and its temporary config is removed when the helper exits. OpenCode's `/provider` response supplies models for its connected providers. A nonempty connected-provider list marks the OpenCode profile authenticated as reported by OpenCode; it does not independently validate each upstream credential or prove plan, billing, or quota. No inference request is sent.

The pair code expires after five minutes and allows ten failed attempts. Bearer sessions expire after an hour. The embedding Rust host may call `HttpServer::issue_pairing_code()` to authorize another app; otherwise restart the CLI to issue another code. Tokens stay in the browser client closure, not URLs, local storage, provider credentials, or logs.

## Explicit execution

Add `--allow-codex-execution --workspace ABSOLUTE_DIRECTORY` to authorize native sessions in the named `default` workspace. This does not start inference. A user must choose a model and explicitly start a turn in Playground. The demo requests read-only execution. Optional CLI flags `--allow-writes`, `--review-native-requests`, and `--allow-paid-options` only raise the host's maximum permission; a request must also select/authorize the corresponding behavior. No metered fallback is automatic.

The HTTP API binds 127.0.0.1 only. Host and Origin are checked, pairing is single-use, API methods require a bearer and protocol handshake, and caller permissions are enforced by the same Rust service used by stdio. Native apps without a browser Origin can use the explicit bearer flow. A page served from another origin is not implicitly trusted.

`POST /v1/pair` takes `{code}`. `POST /v1/rpc` takes the standard JSON-RPC envelope. Read-only REST projections include `/v1/snapshot`, `/v1/connections`, `/v1/models`, and `/v1/usage`. `/v1/events` exposes authenticated SSE with `sessionId` and `after` filters; lost history produces an explicit gap. Stdio supports the equivalent `events.subscribe` / `events.unsubscribe` lifecycle, with bounded output and typed event errors.

## Verification

`node --test scripts/test_live_demo.mjs` exercises real native HTTP and Chromium against an isolated home. `node scripts/check_live_metadata_ui.mjs --allow-metadata` is an explicitly opt-in real Codex metadata browser test; it submits no generation request. Native transport and stream tests are in `crates/lai-transports/tests`.

The in-memory fixture service remains available to automated tests only. The public browser bootstrap has no fixture fallback.
