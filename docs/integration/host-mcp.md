# Host-provided MCP servers

A host application can run a loopback MCP server exposing its own tools (for example design-editor actions) and attach it to a native-client session. The agent can then call **only** those tools. The client's own shell, file, and web tools stay disabled, and any other tool call fails the turn.

## Contract

`sessions.create` accepts an optional, additive `hostMcpServers` list. Omitting it (or sending `[]`) is unchanged behavior.

```json
{
  "selection": { "routeId": "...", "modelId": "...", "revision": "...", "options": {} },
  "workspaceId": "scratch",
  "hostMcpServers": [
    {
      "name": "design",
      "url": "http://127.0.0.1:48123/mcp",
      "bearerToken": "<32-512 printable ASCII characters>",
      "allowedTools": ["echo_color", "set_fill"]
    }
  ]
}
```

Validation is strict; any violation is `INVALID_REQUEST` before a client is launched:

| Field | Rule |
| --- | --- |
| list | at most 2 servers, at most 256 allowed tools in total, unique names |
| `name` | `^[a-z][a-z0-9_]{0,31}$` |
| `url` | `http://127.0.0.1:<port>/...` or `http://[::1]:<port>/...`; port 1-65535; a path is required; no other host, no `https`, no userinfo, whitespace, `\` or fragment |
| `bearerToken` | 32-512 characters, printable ASCII without spaces |
| `allowedTools` | non-empty, unique, each `^[a-z][a-z0-9_]{0,63}$` |

Further rules:

- The existing execution grant is required, and the session must be read-only (no `permissionMode` or `read_only`). `workspace_write` or `native_policy` with host servers is `INTEGRATION_RESTRICTED`.
- Billing rules are unchanged: metered, prepaid or mixed routes still need `allowPaid` plus the grant's paid consent.
- The token is never logged or echoed. `HostMcpServer`'s `Debug` output redacts it, sessions/events/errors never contain it, and no adapter places it on a command line.
- The handshake advertises `session.host_mcp` when an execution driver is registered. Support is still per route: integrations that cannot attach host servers return `CAPABILITY_UNSUPPORTED`.
- Host tool calls surface as the neutral `tool` event, `name` = `mcp__<server>__<tool>`, `status` only (no arguments or results).
- Session resumption carries the same host servers to the resumed native session.

## Per-provider support

| Integration | Wiring | Status |
| --- | --- | --- |
| Claude Code | Per-session `--mcp-config <file>` in a private temp directory (`0600` on Unix; per-user temp ACL on Windows), `--strict-mcp-config`, `--tools ""`, `--allowedTools=mcp__<server>__<tool>,...`. The file is deleted on session close. | Fixture-tested; live wiring verified without inference (server connected with the bearer token and listed its tools). |
| Codex app-server | Per-thread `thread/start` config `mcp_servers.<name>` = `{url, http_headers.Authorization, enabled_tools, default_tools_approval_mode: "approve", required: true}`. The token travels only over the app-server stdio pipe. Every other MCP server configured in the Codex home is disabled for the thread (`mcp_servers.<other>.enabled = false`); a configured server with the host server's name is refused (`INTEGRATION_RESTRICTED`). Sandbox stays read-only, approval `never`, shell/exec disabled. | Fixture-tested; live `thread/start` against Codex 0.157.1 verified without inference (host server `connected`, `authStatus: bearerToken`, other servers `disabled`). |
| ACP (native profiles, e.g. Grok) | `session/new` / `session/load` `mcpServers: [{type:"http", name, url, headers:[{name:"Authorization", value:"Bearer …"}]}]`, only when the peer advertises `agentCapabilities.mcpCapabilities.http`; otherwise `CAPABILITY_UNSUPPORTED`. A permission request for a host tool is answered `allow_once`. | Fixture-tested only. |
| Muse Code | `session/start` / `session/resume` `config.mcpServers.<name> = {transport: "streamableHttp", url, headers: {Authorization: "Bearer …"}, mode: "optional"}`, after `initialize` grants `sessionMcp` (else `CAPABILITY_UNSUPPORTED`). The token travels only over the stdio pipe. `optional` keeps the turn running without the tools when Muse cannot reach the server, as T3 Code does. | Fixture-tested; live session start with an unreachable server verified on Muse 1.4.7 (echo provider, no inference). |
| OpenCode, HTTP/API endpoints | — | `CAPABILITY_UNSUPPORTED` |

## Enforcement

- **Claude Code:** `system/init` may advertise only the allowed `mcp__<server>__<tool>` names; a streamed `content_block_start` or `tool_use` block for anything else (`Bash`, `Read`, another server, a server tool) fails the turn with `INTEGRATION_RESTRICTED`. Startup waits for every host server to report `connected` through the `mcp_status` control request (no prompt); a failed or never-connecting server is `PROVIDER_UNAVAILABLE`.
- **Codex:** in read-only, unreviewed sessions an `mcpToolCall` item for an allowed `(server, tool)` is reported; any other `mcpToolCall`, `commandExecution`, `fileChange`, `dynamicToolCall`, `webSearch`, `imageGeneration` or `collabAgentToolCall` interrupts the turn with `INTEGRATION_RESTRICTED`.
- **Muse Code:** a `toolCall` named `mcp__<server>__<tool>` for an allowed pair is a host tool; any other MCP tool fails the turn. Muse cannot remove its own read-only tools (`read_file`, `search`, `tool_search`, `read_skill`, `write_todos`, `request_user_input`, `subagent_*`), so read-only sessions allow those; shell, writes, web tools and workflow launches are disabled at launch, and any other native tool fails the turn with `INTEGRATION_RESTRICTED`. See [Muse Code](../adapters/muse.md).
- **ACP:** ACP does not say which MCP server a tool call came from. A `tool_call` counts as a host tool only when it has no native `kind` (absent or `other`) and its `title` resolves to exactly one allowed tool (`mcp__s__t`, `s__t`, `s/t`, `s.t`, `s:t`, or an unambiguous bare tool name). Everything else keeps the text-only rule from `docs/adapters/acp.md`.

## Claude Code and safe mode

Claude Code 2.1 ignores `--mcp-config` servers in `--safe-mode` (verified: `mcp_status` returned no servers). Host-MCP sessions therefore omit `--safe-mode`. `--restricted` still ignores user/project/local settings files (hooks, plugins, permission rules), `--disable-slash-commands` keeps skills off, `--tools ""` removes built-in tools, and `--strict-mcp-config` excludes every other MCP server. What safe mode additionally suppressed and host-MCP sessions may load: CLAUDE.md instructions, custom agents and output styles. They add instructions, not tools, and the tool checks above still apply. `--bare` is not an alternative because it disables OAuth sign-in.

## Live checks

Codex models that defer tools to code mode need the host MCP namespaces exposed as direct tools. The adapter sets `features.code_mode.direct_only_tool_namespaces` for only those namespaces and disables `features.non_prefixed_mcp_tool_names` so they cannot overlap built-ins such as `functions`. An already-prefixed server name retains its prefix; server names that normalize to the same namespace fail with `INTEGRATION_RESTRICTED`. The code-mode host, shell/exec, unrelated integrations and existing tool-call guards stay unchanged. See the [official configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference).

On Windows with Codex 0.159.2, a separately owned managed ChatGPT session and a Claude Code session each called one fixed, read-only host tool and completed with the exact expected reply. The earlier managed configuration completed text turns but made no tool call and reported that the tool host was disabled. This qualifies the bounded host-tool round trip, not every host application's editor actions.

`crates/lai-adapters/tests/host_mcp_live.rs` is opt-in: set `LAI_LIVE_CLAUDE`, `LAI_LIVE_CLAUDE_HOME`, `LAI_LIVE_MCP_URL`, `LAI_LIVE_MCP_TOKEN` (and optionally `LAI_LIVE_CLAUDE_MODEL`, default `haiku`). `LAI_LIVE_MCP_STATUS_ONLY=1` stops after the no-inference connection check; without it the test runs one text turn that must call `echo_color` and nothing else.
