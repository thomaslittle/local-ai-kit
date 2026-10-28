# Publishing

Nothing in this repository publishes on its own. A release is a maintainer
action, gated by the `release` environment approval in GitHub, and every
dry run uploads nothing.

## Dry run first

```sh
gh workflow run release.yml -f dry_run=true
```

A dry run builds every target, strips Linux binaries, enforces the size budget,
stages the npm platform packages and wheels, and prints the checksum manifest.
It uploads no registry artifact. Review that manifest before publishing.

## Publishing

```sh
gh workflow run release.yml -f dry_run=false
```

or push a `v*` tag. The `publish` job runs only when the plan says the run is not
a dry run, and only after the `release` environment approval. It publishes in
dependency order: crates, then npm platform packages and the SDK, then wheels,
then the signed GitHub bundle with `SHA256SUMS`.

## Required secrets and certificates

| Name | Used for |
| --- | --- |
| `CARGO_REGISTRY_TOKEN` | `cargo publish` for `lai-contracts`, `lai-core`, `lai-transports`, `lai-kit`, `local-ai-kit` |
| `NPM_TOKEN` | npm platform packages and the SDK (provenance uses OIDC) |
| `PYPI_API_TOKEN` | wheel upload; Trusted Publishing is preferred when configured |
| `APPLE_CERTIFICATE_P12`, `APPLE_CERTIFICATE_PASSWORD` | Developer ID signing of the macOS helper |
| `APPLE_ID`, `APPLE_APP_SPECIFIC_PASSWORD`, `APPLE_TEAM_ID` | notarization |
| `WINDOWS_CERTIFICATE_PFX`, `WINDOWS_CERTIFICATE_PASSWORD` | Authenticode signing of the Windows helper and installer |
| `MINISIGN_SECRET_KEY` (or `COSIGN_PRIVATE_KEY`) | Linux checksums and signatures |

This document lists them; it does not buy or generate anything. Unsigned
artifacts are dry-run only: Gatekeeper and SmartScreen block them.

## Size budget

`build` computes the stripped helper size per target and fails the run above the
budget (40 MB for desktop targets, 60 MB for musl). The measured size is printed
in the job summary and belongs in the release notes, because the helper ships
inside every consumer's installer.

## Provenance

- npm: `npm publish --provenance` with the workflow's OIDC identity.
- crates: `cargo publish` records the source revision.
- wheels and bundles: built in this workflow, checksummed with `SHA256SUMS`,
  and signed on macOS/Windows/Linux as described above.

## Staged package shapes

- `scripts/npm-platform-packages.mjs` writes `@local-ai-kit/cli-<platform>`
  packages, one binary each, with `os`/`cpu` and a `bin` entry.
- `scripts/npm-add-platform-deps.mjs` copies the SDK and adds those packages as
  `optionalDependencies`; the workspace copy stays platform-free so `npm ci` is
  identical everywhere.
- `scripts/stage_python_package.py` copies `packages/sdk-python`, drops the
  private classifier, bundles the helper under `local_ai_kit/_bin/`, and forces
  a platform-specific wheel.
