# T3 Code sync state

This file is the baseline for a daily diff. Compare the pinned commit below with `pingdotgg/t3code` `main` and with the newest nightly tag. Do not move the pin just because `main` moved. Adopt a change only after it is normalized, tested, and attributed.

Released Local AI Kit builds do not clone or execute T3 Code.

## Pin

| Field | Value |
| --- | --- |
| Repository | https://github.com/pingdotgg/t3code |
| License | MIT, Copyright (c) 2026 T3 Tools Inc. |
| Synced commit | `cfafb4bb54d244e4612f5681aad3832ea64cb0fb` |
| Branch | `main` |
| Commit subject | `fix(codex): a slow but healthy Codex no longer shows as Unavailable (#15945)` |
| Nightly branch | none |
| Latest nightly tag | `v0.1.1-nightly.20261016.3203` |
| Nightly commit | `e3b35d1b2440b104f0bc2e357377c348121234dd` |
| Latest stable tag | `v0.1.0` |
| Stable commit | `81493efb71e8e8244f95d3e9f71d8369654ab151` |
| Synced on | 2026-10-16 |
| Extract | [upstream/t3code/snapshot.json](upstream/t3code/snapshot.json) |

`main` at the pin is one commit ahead of that nightly tag. The extra commit raises Codex's status-probe budget from 10 seconds to 20 seconds ([cfafb4bb](https://github.com/pingdotgg/t3code/commit/cfafb4bb54d244e4612f5681aad3832ea64cb0fb)). Local AI Kit's Codex metadata probe already waits 20 seconds, so no timeout change was required. `v0.1.0` is the stable tag and points at the same commit as `v0.0.46-nightly.20261015.3176`. Nightly builds are still scheduled prereleases from `main` (`.github/workflows/release.yml` every 30 minutes, or a manual `channel=nightly` dispatch). A manual stable release builds the latest nightly commit, not whatever `main` has moved to since. Preview tags are maintainer test builds and are not a sync source. The CLI npm publish in that workflow now ships a launcher instead of per-platform packages; Local AI Kit does not publish to npm from this workflow.

## Areas a daily pass must diff

Watch these paths at the pinned commit. A diff in any of them is in scope even when Local AI Kit does not adopt the behavior.

### Local model providers

Ollama, LM Studio, llama.cpp, and OpenAI-compatible local endpoints.

At this pin T3 Code has no Ollama, LM Studio, or llama.cpp provider. OpenAI-compatible base URLs show up only as OpenCode or Pi configuration, not as a network scanner. Local AI Kit keeps its own explicit Ollama, LM Studio, and compatible endpoint registry. Do not add llama.cpp from this upstream until T3 Code actually grows that integration.

### Cloud model providers

Built-in drivers at this pin: Codex, Claude, Cursor, Grok, OpenCode, Antigravity, Pi, Muse, and the ACP Registry (`apps/server/src/provider/builtInDrivers.ts`).

Cloud model access is the signed-in CLI or agent (ChatGPT plan, Claude, Grok, Cursor, OpenCode, Pi, Antigravity, a registry agent). T3 Code does not ship first-class OpenAI, Anthropic, Gemini, or xAI HTTP adapters. `apps/server/src/cloud` is the T3 Connect relay (remote access to a T3 server), not an LLM provider. Local AI Kit already has explicit metered API routes; do not treat the relay as one of them.

### Installed CLI and tool detection

| T3 Code provider | What it detects | Local AI Kit at this pin |
| --- | --- | --- |
| Codex | `codex`, `CODEX_HOME`, optional shadow home | `codex`, `CODEX_HOME`. Shadow homes not adopted. |
| Claude | `claude`, `CLAUDE_CONFIG_DIR` or `~/.claude` | `claude`, secure-storage dir then `CLAUDE_CONFIG_DIR`. |
| Gemini CLI | no built-in driver; ACP Registry can install `@google/gemini-cli` | `gemini` and `GEMINI_CLI_HOME` or `~/.gemini`. The installer is not adopted. |
| Antigravity | `antigravity` / ACP registry entry `antigravity-acp` | `antigravity` and `agy`, plus explicit ACP profile registration. |
| Grok | `grok`, `GROK_HOME` or `~/.grok` | same homes. Metadata probes (`grok models` and ACP initialize) set `GROK_DISABLE_AUTOUPDATER=1`. Text sessions do not. |
| OpenCode | `opencode`, native path `~/.opencode/bin/opencode` | PATH plus that installer directory. |
| Muse | `muse` | PATH, Windows install dir, `LOCAL_AI_KIT_MUSE_BINARY`. |
| Pi | `pi`, `PI_CODING_AGENT_DIR` or `~/.pi/agent` | passive inventory only. No Pi execution. |
| Cursor | bundled Cursor SDK; CLI path hidden | passive `cursor-agent` only. SDK not adopted. |
| Copilot | not a built-in driver; ACP registry can add it | passive `~/.copilot`. |
| ACP local command | user-typed executable, no scan | not auto-discovered. |
| Ollama / LM Studio / llama.cpp | not present | Ollama and LM Studio stay Local AI Kit's. |
| Shell resolution | `packages/shared/src/shell.ts`, `apps/desktop/src/shell/DesktopShellEnvironment.ts` | Watched for executable/PATH/platform-environment changes; no shell profile is read during passive discovery. |

The watcher also reports provider-shaped files that are not yet in `watchedPaths`
and any watched path that disappeared at the tip. Those are review prompts, not
automatic coverage claims. Release/relay/docs changes are classified separately
and do not open a provider-review PR by themselves.

The daily workflow runs in **report-only** mode when the repository does not
permit GitHub Actions to create pull requests. The report and JSON metadata are
always uploaded as artifacts. The minimum setting to enable PR creation is
**Settings → Actions → General → Workflow permissions → Allow GitHub Actions to
create and approve pull requests**; no personal token is substituted.

### ChatGPT OAuth

Source: `apps/server/src/provider/CodexChatGptAuth.ts`, `CodexChatGptHandoff.ts`, `CodexChatGptSessionLock.ts`.

Adopted into `crates/lai-chatgpt-auth`:

- `earliest_refresh_at` delays refresh.
- ID tokens may be RS256 or P-256 ES256, with five seconds of clock skew.
- A sign-in that omits `chatgpt.tokens.use.direct` keeps the registration and the next attempt sends `prompt=consent`.

Not adopted:

- Multiple saved ChatGPT profiles and "use a different account".
- Primary-to-remote credential handoff (`CodexChatGptHandoff.ts`).
- Replacing a `localhost` redirect registration. This helper has always used `127.0.0.1`.
- Codex release downloads in `CodexInstallation.ts`.
- Following a Windows `claude.cmd` shim into the npm package so the Claude Agent SDK can spawn it.
- Windows ARM fallback to a `windows-x86_64` ACP Registry archive ([76cb251d](https://github.com/pingdotgg/t3code/commit/76cb251da129579d13be333ffe89a8767f7a257a)). Local AI Kit does not download registry agents.
- Windows Codex install extraction via system `tar` and a retried rename ([364f5084](https://github.com/pingdotgg/t3code/commit/364f5084d7418e1605220b09de5aaebd46305a73)). Local AI Kit does not download Codex.
- Managed Antigravity install validation on Windows ARM ([895354e9](https://github.com/pingdotgg/t3code/commit/895354e9fbf5be90cda4fdcd710dac96496181c4)). Local AI Kit does not install Antigravity.
- Cursor Auto threads using Cursor's Auto-review ([2d943723](https://github.com/pingdotgg/t3code/commit/2d943723769da0b675b56168d764d1ad96fc8d28)). The bundled Cursor SDK stays unadopted.
- Ending an ACP session on one schema-invalid notification ([559c4b1f](https://github.com/pingdotgg/t3code/commit/559c4b1f2192a17060c698f014074a223cbcf7bd)). An unrecognized `sessionUpdate` is already omitted here, and a `usage_update` that omits `used` does not end the turn.

Adopted from this pin, in addition to the ChatGPT, Pi, and OpenCode facts above: Grok metadata probes set `GROK_DISABLE_AUTOUPDATER=1` ([2a7bd167](https://github.com/pingdotgg/t3code/commit/2a7bd1673c7bdad4304b7771944e73c29ecd3c01)). Text sessions leave the variable unset.

One managed registration per state directory still rejects a changed subject. Claude config resolution keeps `CLAUDE_SECURESTORAGE_CONFIG_DIR` ahead of `CLAUDE_CONFIG_DIR`.

## How to diff the next pass

The daily workflow runs `python scripts/upstream_t3code.py --open-pr`. It diffs `main` and the newest nightly tag against this pin, but only the paths in [upstream/t3code/snapshot.json](upstream/t3code/snapshot.json). Those paths cover local model providers, cloud CLI providers, installed CLI detection (Codex, Claude Code, Gemini CLI via the ACP Registry, Grok, OpenCode, Pi, Muse, Cursor, Antigravity), and the ChatGPT OAuth sign-in flow. A difference opens a review pull request and does not change adapter behavior.

Check the pin without fetching:

```sh
python scripts/upstream_t3code.py --offline
```

The same comparison by hand:

```sh
git fetch https://github.com/pingdotgg/t3code.git main
git diff cfafb4bb54d244e4612f5681aad3832ea64cb0fb..FETCH_HEAD -- \
  apps/server/src/provider/builtInDrivers.ts \
  apps/server/src/provider/builtInProviderCatalog.ts \
  apps/server/src/provider/CodexChatGptAuth.ts \
  apps/server/src/provider/CodexChatGptHandoff.ts \
  apps/server/src/provider/CodexChatGptSessionLock.ts \
  apps/server/src/provider/CodexInstallation.ts \
  apps/server/src/provider/CodexProvider.ts \
  apps/server/src/provider/AntigravityInstallation.ts \
  apps/server/src/provider/Drivers \
  apps/server/src/cloud \
  packages/provider-pi \
  packages/provider-opencode/src/server/driver.ts \
  packages/provider-cursor \
  packages/provider-grok \
  packages/provider-muse \
  packages/provider-acp-registry \
  docs/internals/providers.md \
  docs/operations/release.md \
  .github/workflows/release.yml
```

Also compare the newest `v*-nightly*` tag with `v0.1.1-nightly.20261016.3203`. If the watched paths are unchanged, leave this pin in place and record that the check found nothing to adopt.
