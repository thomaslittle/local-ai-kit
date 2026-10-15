# T3 Code as an upstream provider source

[T3 Code](https://github.com/pingdotgg/t3code) is a desktop coding product. Local AI Kit uses it as a reviewed source for how a host detects installed coding CLIs and how Sign in with ChatGPT is supposed to behave. It is not a dependency of a released build. The sync pin, nightly tag, and watched paths are in [UPSTREAM_T3CODE.md](../../UPSTREAM_T3CODE.md).

## What a nightly is

T3 Code has no `nightly` branch. `.github/workflows/release.yml` schedules a nightly check every 30 minutes and can also be started by hand with `channel=nightly`. Those runs publish a GitHub prerelease. A manual stable release builds the commit of the latest nightly, then tags that commit. Preview releases are maintainer test builds and are not a sync source.

## What this pin adopted

From `42da166d131017335a1158d0b379dd68b8985796`:

- Pi is a passive installation candidate. The command is `pi`. The profile is `PI_CODING_AGENT_DIR` or `~/.pi/agent`. Session files live under `sessions/` and are not opened. There is no Pi execution adapter.
- OpenCode's own installer directory `~/.opencode/bin` is searched for `opencode` in addition to `PATH`.
- Managed ChatGPT sign-in honors `earliest_refresh_at`, accepts RS256 and P-256 ES256 identity tokens with five seconds of clock skew, and sends `prompt=consent` after a sign-in that did not grant `chatgpt.tokens.use.direct`.

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

Authority is unchanged: an official client or a Local AI Kit qualification outranks this upstream. T3 Code is supporting evidence, recorded on the discovery rule as `upstream.t3code` at the pinned commit.

## Watcher

[`.github/workflows/upstream-discovery.yml`](../../.github/workflows/upstream-discovery.yml) also runs `python scripts/upstream_t3code.py --open-pr`. The script diffs the watched paths for local model providers, cloud CLI providers, installed CLI detection, and ChatGPT OAuth. When those paths differ on `main` or on a newer nightly tag, it opens a review pull request from `upstream/t3code-provider-review`. That pull request does not change adapter behavior.
