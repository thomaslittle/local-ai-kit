# T3 Code sync state

This file is the baseline for a daily diff. Compare the pinned commit below with `pingdotgg/t3code` `main` and with the newest nightly tag. Do not move the pin just because `main` moved. Adopt a change only after it is normalized, tested, and attributed.

Released Local AI Kit builds do not clone or execute T3 Code.

## Pin

| Field | Value |
| --- | --- |
| Repository | https://github.com/pingdotgg/t3code |
| License | MIT, Copyright (c) 2026 T3 Tools Inc. |
| Synced commit | `f0dd86f16482595c6cfffe0ac64dc12bffb634f9` |
| Branch | `main` |
| Commit subject | `fix(web): stable callbacks refresh before layout effects read them (#25489)` |
| Nightly branch | none |
| Latest nightly tag | `v0.1.3-nightly.20261026.3717` |
| Nightly commit | `1316d48b0c68151afd8b47eb0c11a114cf0b4daa` |
| Latest stable tag | `v0.1.2` |
| Stable commit | `044f07257c9684ae4543bd1e1ebf9c394e9e3830` |
| Synced on | 2026-10-26 |
| Extract | [upstream/t3code/snapshot.json](upstream/t3code/snapshot.json) |

`main` at the pin is 16 commits ahead of nightly `v0.1.3-nightly.20261026.3717` (`1316d48b0c68151afd8b47eb0c11a114cf0b4daa`). Stable `v0.1.2` (`044f07257c9684ae4543bd1e1ebf9c394e9e3830`) and `v0.1.1` both landed after the previous pin (`cfafb4bb54d244e4612f5681aad3832ea64cb0fb`, nightly `v0.1.1-nightly.20261016.3203`, stable `v0.1.0`). There is still no `nightly` branch. Nightly builds remain scheduled prereleases from `main` (`.github/workflows/release.yml` every 30 minutes, or a manual `channel=nightly` dispatch). A manual stable release builds the latest nightly commit, not whatever `main` has moved to since. Preview tags are maintainer test builds and are not a sync source. T3's release workflow has moved packaging between GitHub and Blacksmith runners and stopped publishing per-platform npm packages. That pipeline is not Local AI Kit's CI.

Since `cfafb4bb`, the watched provider paths changed. The facts reimplemented in Rust, with no T3 Code source copied, are:

- A logged-out `grok models` probe is checked once more. A definite recheck replaces the first result. An unknown recheck stays logged out ([a2157ece](https://github.com/pingdotgg/t3code/commit/a2157eced708fd25fab7b616f5a68cb65312eaed)).
- A native Codex binary is launchable only when the adjacent `codex-code-mode-host` executable exists and is non-empty. Shebang scripts, Volta/mise/aqua shims, Windows non-`.exe` launchers, and `.bunx`/`.shim` markers stay exempt ([348888a7](https://github.com/pingdotgg/t3code/commit/348888a767433464dd47e723ec6509dc5a8282db)).
- Known ChatGPT plan codes keep the raw `planType` and use T3's current display labels, including Pro 200, Pro Lite 100, Pro Max 500, Business Premium, Business, Enterprise, and Edu. An unrecognized code is shown as itself. An unknown plan is still omitted ([33f1fe5e](https://github.com/pingdotgg/t3code/commit/33f1fe5ebe53fdd8d18b6a5d63dfd29c336ba5a9)).
- Codex `config/read` `service_tier` selects an advertised choice. `fast` aliases `priority` when that exact id is absent. The probe does not add a tier, does not change the catalog default, and ignores a missing `config/read` ([3b844dab](https://github.com/pingdotgg/t3code/commit/3b844dab5ced7277ce4a2cf0e5e11d0f04e62241)).
- Codex `individualLimit.remainingPercent` on the `codex` bucket becomes a window labeled "Monthly credit limit". Used is `100 - remaining` only when remaining is finite and in `0..=100` ([4c18faf6](https://github.com/pingdotgg/t3code/commit/4c18faf62cd1685b0243dcacc1a94e333c2b93ff)).
- Claude monthly spend prefers an enabled spend budget when currency and exponent match, then enabled extra-usage amounts or utilization. The window id is `monthly_spend`. Disabled budgets do not invent a window, and the generic utilization loop no longer double-counts `spend` and `extra_usage` ([fd8dd516](https://github.com/pingdotgg/t3code/commit/fd8dd51654f855dbf4e23c457701f5088c59b689)).

Reviewed and not adopted from this range: Ollama, LM Studio, and llama.cpp (still absent), Grok billing that reads `auth.json` ([79f2bae6](https://github.com/pingdotgg/t3code/commit/79f2bae6)), spawning a Muse SDK host to learn `museHome` ([c8933536](https://github.com/pingdotgg/t3code/commit/c8933536)), forcing `gpt-6-astra` reasoning effort to medium, ChatGPT profile-key hashing and hashed telemetry owner ids, Nushell PATH hydration and the shorter PowerShell probe, and T3's release CI. ChatGPT OAuth endpoints, algorithms, clock skew, `earliest_refresh_at`, and `prompt=consent` did not change. The previous non-adoptions (ACP downloads, Codex downloads and shadow homes, Antigravity installs, the Cursor SDK, Windows `claude.cmd` rewriting, multi-account handoff) stay in place. Codex's 20-second probe budget already matched.

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
| Codex | `codex`, `CODEX_HOME`, optional shadow home, adjacent `codex-code-mode-host` | `codex`, `CODEX_HOME`. Launch resolution requires the adjacent host for a native binary. Shadow homes not adopted. |
| Claude | `claude`, `CLAUDE_CONFIG_DIR` or `~/.claude` | `claude`, secure-storage dir then `CLAUDE_CONFIG_DIR`. |
| Gemini CLI | no built-in driver; ACP Registry can install `@google/gemini-cli` | `gemini` and `GEMINI_CLI_HOME` or `~/.gemini`. The installer is not adopted. |
| Antigravity | `antigravity` / ACP registry entry `antigravity-acp` | `antigravity` and `agy`, plus explicit ACP profile registration. |
| Grok | `grok`, `GROK_HOME` or `~/.grok` | same homes. Metadata probes (`grok models` and ACP initialize) set `GROK_DISABLE_AUTOUPDATER=1`. A logged-out `grok models` result is probed once more. Text sessions do not set the updater variable. |
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

Still adopted from earlier pins: Grok metadata probes set `GROK_DISABLE_AUTOUPDATER=1` ([2a7bd167](https://github.com/pingdotgg/t3code/commit/2a7bd1673c7bdad4304b7771944e73c29ecd3c01)). Text sessions leave the variable unset. The 2026-10-26 additions are the Grok login recheck, Codex code-mode host, plan labels, configured service tier, individual credit window, and Claude monthly spend listed in the pin section. ChatGPT OAuth protocol fields in `adopted.chatgpt` are unchanged.

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
git diff f0dd86f16482595c6cfffe0ac64dc12bffb634f9..FETCH_HEAD -- \
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

Also compare the newest `v*-nightly*` tag with `v0.1.3-nightly.20261026.3717`. If the watched paths are unchanged, leave this pin in place and record that the check found nothing to adopt.
