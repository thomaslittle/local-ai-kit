# T3 Code

Local AI Kit reviewed [T3 Code](https://github.com/pingdotgg/t3code) as an upstream source for provider detection, local and cloud model access, and Sign in with ChatGPT. Released builds do not clone, download, or execute T3 Code, and they do not follow its `main` branch.

| Field | Value |
| --- | --- |
| Pinned commit | `cfafb4bb54d244e4612f5681aad3832ea64cb0fb` (`main`) |
| Latest nightly at pin time | `v0.1.1-nightly.20261016.3203` (`e3b35d1b2440b104f0bc2e357377c348121234dd`) |
| Latest stable at pin time | `v0.1.0` (`81493efb71e8e8244f95d3e9f71d8369654ab151`) |
| License | MIT |
| Copyright | Copyright (c) 2026 T3 Tools Inc. |

There is no `nightly` branch. Nightly builds are scheduled GitHub prereleases cut from `main` by `.github/workflows/release.yml`. A manual stable release builds the latest nightly's commit.

No T3 Code source file is copied into this build. The Pi profile, the OpenCode native binary directory, and the ChatGPT refresh behavior below were reimplemented in Rust from the pinned revision. The earlier Muse port and its copied transcripts are recorded separately in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

Adopted at this pin:

- Pi coding agent command `pi`, profile `PI_CODING_AGENT_DIR` or `~/.pi/agent`, session directory `sessions/` under that profile. Passive inventory only. First adopted at `42da166d131017335a1158d0b379dd68b8985796`.
- OpenCode installer binary directory `~/.opencode/bin`. First adopted at that same commit.
- ChatGPT OAuth: `earliest_refresh_at` is honored, ID tokens may be RS256 or ES256 with a five-second clock skew, and a sign-in that omitted plan sharing asks for consent on the next attempt. First adopted at that same commit.
- Grok metadata probes set `GROK_DISABLE_AUTOUPDATER=1`. Text sessions do not. Reimplemented from [2a7bd167](https://github.com/pingdotgg/t3code/commit/2a7bd1673c7bdad4304b7771944e73c29ecd3c01).

Reviewed and not adopted: Ollama, LM Studio, and llama.cpp (absent upstream), direct cloud HTTP APIs (absent upstream; Local AI Kit already has explicit API routes), ACP Registry downloads including the Windows ARM `windows-x86_64` fallback and Gemini CLI, multi-account ChatGPT handoff, Codex shadow homes, Codex release downloads (including the Windows system-tar extract), Windows Claude launcher-shim rewriting, managed Antigravity installs, and the bundled Cursor SDK (including Auto-review). Codex's 20-second status probe already matched Local AI Kit.

The daily diff list is [UPSTREAM_T3CODE.md](../../UPSTREAM_T3CODE.md). The factual extract is [snapshot.json](snapshot.json).
