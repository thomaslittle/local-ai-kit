# T3 Code sync state

This file is the baseline for a daily diff. Compare the pinned commit below with `pingdotgg/t3code` `main` and with the newest nightly tag. Do not move the pin just because `main` moved. Adopt a change only after it is normalized, tested, and attributed.

Released Local AI Kit builds do not clone or execute T3 Code.

## Pin

| Field | Value |
| --- | --- |
| Repository | https://github.com/pingdotgg/t3code |
| License | MIT, Copyright (c) 2026 T3 Tools Inc. |
| Synced commit | `42da166d131017335a1158d0b379dd68b8985796` |
| Branch | `main` |
| Commit subject | `fix(server): OTLP exports say which machine and build they came from (#19370)` |
| Nightly branch | none |
| Latest nightly tag | `v0.0.46-nightly.20261015.3158` |
| Nightly commit | `e6bbfdb9ffe8334bed1c9b992f828f04d30db7ea` |
| Latest stable tag | `v0.0.45` |
| Stable commit | `6c8fed35dded9ff71c5b46807125457acbb76be6` |
| Synced on | 2026-10-15 |
| Extract | [upstream/t3code/snapshot.json](upstream/t3code/snapshot.json) |

`main` at the pin is one commit ahead of that nightly tag. The extra commit is an OTLP label change and is outside the watched provider surface. Nightly builds are scheduled prereleases from `main` (`.github/workflows/release.yml` every 30 minutes, or a manual `channel=nightly` dispatch). A manual stable release builds the latest nightly commit, not whatever `main` has moved to since. Preview tags are maintainer test builds and are not a sync source.

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
| Grok | `grok`, `GROK_HOME` or `~/.grok` | same homes. |
| OpenCode | `opencode`, native path `~/.opencode/bin/opencode` | PATH plus that installer directory. |
| Muse | `muse` | PATH, Windows install dir, `LOCAL_AI_KIT_MUSE_BINARY`. |
| Pi | `pi`, `PI_CODING_AGENT_DIR` or `~/.pi/agent` | passive inventory only. No Pi execution. |
| Cursor | bundled Cursor SDK; CLI path hidden | passive `cursor-agent` only. SDK not adopted. |
| Copilot | not a built-in driver; ACP registry can add it | passive `~/.copilot`. |
| ACP local command | user-typed executable, no scan | not auto-discovered. |
| Ollama / LM Studio / llama.cpp | not present | Ollama and LM Studio stay Local AI Kit's. |

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
git diff 42da166d131017335a1158d0b379dd68b8985796..FETCH_HEAD -- \
  apps/server/src/provider/builtInDrivers.ts \
  apps/server/src/provider/builtInProviderCatalog.ts \
  apps/server/src/provider/CodexChatGptAuth.ts \
  apps/server/src/provider/CodexChatGptHandoff.ts \
  apps/server/src/provider/CodexChatGptSessionLock.ts \
  apps/server/src/provider/CodexInstallation.ts \
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

Also compare the newest `v*-nightly*` tag with `v0.0.46-nightly.20261015.3158`. If the watched paths are unchanged, leave this pin in place and record that the check found nothing to adopt.
