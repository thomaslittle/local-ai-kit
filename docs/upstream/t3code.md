# T3 Code as an upstream provider source

[T3 Code](https://github.com/pingdotgg/t3code) is a desktop coding product. Local AI Kit uses it as a reviewed source for how a host detects installed coding CLIs and how Sign in with ChatGPT is supposed to behave. It is not a dependency of a released build. The sync pin, nightly tag, and watched paths are in [UPSTREAM_T3CODE.md](../../UPSTREAM_T3CODE.md).

## What a nightly is

T3 Code has no `nightly` branch. `.github/workflows/release.yml` schedules a nightly check every 30 minutes and can also be started by hand with `channel=nightly`. Those runs publish a GitHub prerelease. A manual stable release builds the commit of the latest nightly, then tags that commit. Preview releases are maintainer test builds and are not a sync source.

## What this pin adopted

From `f0dd86f16482595c6cfffe0ac64dc12bffb634f9`, including facts first adopted at `42da166d131017335a1158d0b379dd68b8985796` and `cfafb4bb54d244e4612f5681aad3832ea64cb0fb`:

- Pi is a passive installation candidate. The command is `pi`. The profile is `PI_CODING_AGENT_DIR` or `~/.pi/agent`. Session files live under `sessions/` and are not opened. There is no Pi execution adapter.
- OpenCode's own installer directory `~/.opencode/bin` is searched for `opencode` in addition to `PATH`.
- Managed ChatGPT sign-in honors `earliest_refresh_at`, accepts RS256 and P-256 ES256 identity tokens with five seconds of clock skew, and sends `prompt=consent` after a sign-in that did not grant `chatgpt.tokens.use.direct`.
- Grok metadata probes set `GROK_DISABLE_AUTOUPDATER=1` so `grok models` and the following ACP initialize do not start Grok's updater ([2a7bd167](https://github.com/pingdotgg/t3code/commit/2a7bd1673c7bdad4304b7771944e73c29ecd3c01)). Text sessions leave the variable unset.
- Codex metadata probes already wait 20 seconds, which matches the status-probe budget in [cfafb4bb](https://github.com/pingdotgg/t3code/commit/cfafb4bb54d244e4612f5681aad3832ea64cb0fb).
- A logged-out `grok models` probe is repeated once ([a2157ece](https://github.com/pingdotgg/t3code/commit/a2157eced708fd25fab7b616f5a68cb65312eaed)).
- A native Codex binary needs the adjacent `codex-code-mode-host` executable before launch ([348888a7](https://github.com/pingdotgg/t3code/commit/348888a767433464dd47e723ec6509dc5a8282db)).
- Known ChatGPT plan codes keep the raw id and use the current labels ([33f1fe5e](https://github.com/pingdotgg/t3code/commit/33f1fe5ebe53fdd8d18b6a5d63dfd29c336ba5a9)).
- A saved Codex `service_tier` selects an advertised choice (`fast` aliases `priority`) and does not add a tier ([3b844dab](https://github.com/pingdotgg/t3code/commit/3b844dab5ced7277ce4a2cf0e5e11d0f04e62241)).
- Codex individual monthly credits are a separate window ([4c18faf6](https://github.com/pingdotgg/t3code/commit/4c18faf62cd1685b0243dcacc1a94e333c2b93ff)).
- Claude monthly spend prefers an enabled budget with a matching currency ([fd8dd516](https://github.com/pingdotgg/t3code/commit/fd8dd51654f855dbf4e23c457701f5088c59b689)).

The behavior was reimplemented in Rust. No T3 Code source file was copied for this pin. See [upstream/t3code/NOTICE.md](../../upstream/t3code/NOTICE.md).

## What this pin did not adopt

- Ollama, LM Studio, and llama.cpp. They are not T3 Code providers at this commit. Local AI Kit's explicit local endpoint registry is unchanged.
- Direct OpenAI, Anthropic, Gemini, and xAI HTTP adapters. T3 Code reaches cloud models through the signed-in CLI. `apps/server/src/cloud` is the T3 Connect relay.
- ACP Registry downloads of third-party agents.
- Several ChatGPT accounts in one store, and transferring a refresh token to another machine.
- Codex shadow homes.
- The bundled Cursor SDK. Cursor remains passive `cursor-agent` inventory.
- Downloading a Codex release, or following a Windows `claude.cmd` shim into the npm package.
- Installing Gemini CLI from the ACP Registry. `gemini` on `PATH` and `GEMINI_CLI_HOME` stay Local AI Kit's own inventory.
- The Windows on ARM fallback that extracts a `windows-x86_64` ACP Registry archive ([76cb251d](https://github.com/pingdotgg/t3code/commit/76cb251da129579d13be333ffe89a8767f7a257a)).
- Windows Codex archive extraction with the system `tar` ([364f5084](https://github.com/pingdotgg/t3code/commit/364f5084d7418e1605220b09de5aaebd46305a73)) and managed Antigravity install validation on Windows ARM ([895354e9](https://github.com/pingdotgg/t3code/commit/895354e9fbf5be90cda4fdcd710dac96496181c4)).
- Cursor Auto-review for Auto threads ([2d943723](https://github.com/pingdotgg/t3code/commit/2d943723769da0b675b56168d764d1ad96fc8d28)).
- Grok billing that reads `auth.json` and calls a billing endpoint ([79f2bae6](https://github.com/pingdotgg/t3code/commit/79f2bae6)).
- Starting a Muse SDK host during usage lookup to learn `museHome` ([c8933536](https://github.com/pingdotgg/t3code/commit/c8933536)).
- Forcing reasoning effort `medium` for the `gpt-6-astra` family.
- Hashing a ChatGPT profile key or an OpenAI telemetry owner id.
- Reading Nushell or PowerShell profiles to hydrate `PATH`.
- T3 Code's Blacksmith release workflow and npm launcher publish.

Authority is unchanged: an official client or a Local AI Kit qualification outranks this upstream. T3 Code is supporting evidence, recorded on the discovery rule as `upstream.t3code` at the pinned commit.

## Watcher

[`.github/workflows/upstream-discovery.yml`](../../.github/workflows/upstream-discovery.yml) also runs `python scripts/upstream_t3code.py --open-pr`. The script diffs the watched paths for local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. When those paths differ on `main` or on a newer nightly tag, it opens a review pull request from `upstream/t3code-provider-review`. That pull request does not change adapter behavior.
