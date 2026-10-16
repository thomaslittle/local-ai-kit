# T3 Code

Local AI Kit reviewed [T3 Code](https://github.com/pingdotgg/t3code) as an upstream source for provider detection, local and cloud model access, and Sign in with ChatGPT. Released builds do not clone, download, or execute T3 Code, and they do not follow its `main` branch.

| Field | Value |
| --- | --- |
| Pinned commit | `42da166d131017335a1158d0b379dd68b8985796` (`main`) |
| Latest nightly at pin time | `v0.0.46-nightly.20261015.3158` (`e6bbfdb9ffe8334bed1c9b992f828f04d30db7ea`) |
| Latest stable at pin time | `v0.0.45` (`6c8fed35dded9ff71c5b46807125457acbb76be6`) |
| License | MIT |
| Copyright | Copyright (c) 2026 T3 Tools Inc. |

There is no `nightly` branch. Nightly builds are scheduled GitHub prereleases cut from `main` by `.github/workflows/release.yml`. A manual stable release builds the latest nightly's commit.

No T3 Code source file is copied into this build. The Pi profile, the OpenCode native binary directory, and the ChatGPT refresh behavior below were reimplemented in Rust from the pinned revision. The earlier Muse port and its copied transcripts are recorded separately in [THIRD_PARTY_NOTICES.md](../../THIRD_PARTY_NOTICES.md).

Adopted at this pin:

- Pi coding agent command `pi`, profile `PI_CODING_AGENT_DIR` or `~/.pi/agent`, session directory `sessions/` under that profile. Passive inventory only.
- OpenCode installer binary directory `~/.opencode/bin`.
- ChatGPT OAuth: `earliest_refresh_at` is honored, ID tokens may be RS256 or ES256 with a five-second clock skew, and a sign-in that omitted plan sharing asks for consent on the next attempt.

Reviewed and not adopted: Ollama, LM Studio, and llama.cpp (absent upstream), direct cloud HTTP APIs (absent upstream; Local AI Kit already has explicit API routes), ACP Registry downloads including Gemini CLI, multi-account ChatGPT handoff, Codex shadow homes, Codex release downloads, Windows Claude launcher-shim rewriting, and the bundled Cursor SDK.

The daily diff list is [UPSTREAM_T3CODE.md](../../UPSTREAM_T3CODE.md). The factual extract is [snapshot.json](snapshot.json).
