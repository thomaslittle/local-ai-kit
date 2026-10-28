# V1 dependency security review — 2026-10-28

Scope: integration branch `feat/v1-principal-review`. Reused the October 25
review for unchanged dependencies. GitHub's 15 open Dependabot alerts triggered
this focused follow-up; no account credentials were inspected.

| Dependency | Decision and official source |
|---|---|
| undici | 7.30.0 → 7.30.1, compatible security patch. [October 15 release](https://github.com/nodejs/undici/releases/tag/v7.30.1). Dev dependency through the JS tooling. |
| fast-uri | 3.1.8 → 3.1.9, compatible URI validation/security patch. [October 19 release](https://github.com/fastify/fast-uri/releases/tag/v3.1.9). |
| rustls | 0.23.45 → 0.23.46, compatible TLS security patch. [October 14 release](https://github.com/rustls/rustls/releases/tag/v/0.23.46). |
| setuptools | 84.0.0 → 84.1.0 for the build environment. [October 19 release](https://github.com/pypa/setuptools/releases/tag/v84.1.0), [Windows legacy-wheel advisory](https://github.com/pypa/setuptools/security/advisories/GHSA-xmj3-9gf7-h52w). |
| ratatui / lru | ratatui 0.29 pins vulnerable lru 0.12; directly selecting lru 0.16.3 was rejected by Cargo. Adopt stable ratatui 0.30.2 and its compatible lru 0.18.5, with crossterm 0.29. Enable only the used crossterm backend. [June 19 release](https://github.com/ratatui/ratatui/releases/tag/ratatui-v0.30.2), [upstream lru fix](https://github.com/jeromefroe/lru-rs/pull/224). Workspace MSRV becomes 1.88 to reflect this dependency; the pinned compiler is newer. |
| GPUI Kit | Keep 0.8.1, latest stable in the official registry and [October 26 release](https://github.com/longbridge/gpui-kit/releases/tag/v0.8.1). The October 25 audit predates its introduction; this records the missing review. |

No fork patches or Git-only snapshots were adopted. These are **implemented
lockfile/manifest updates**, not yet fixture-tested or platform-qualified.
The final verification pass must exercise the terminal UI, native transport/TLS
fixtures, JS tooling and clean Python packaging. No live provider operations
are needed. Windows and macOS results require their actual CI executions.

## Updater SemVer reuse

2026-10-28: adopted the existing lockfile's semver 1.0.28 as an exact direct
dependency of lai-update; no new version or fork patch. Official
[crate release](https://docs.rs/crate/semver/1.0.28) and
[Version precedence API](https://docs.rs/semver/1.0.28/semver/struct.Version.html#method.cmp_precedence)
support numeric prerelease ordering and build-metadata-independent precedence.
This removes the updater's incorrect string comparison and rejects invalid staging
version syntax. Focused build and signed fixture tests are pending final verification.
