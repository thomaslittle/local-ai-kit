# Dependency updates for consumers

Consumers should receive a new release as an ordinary dependency-bump pull
request, never through a bespoke updater. These registry configurations describe a future
published release; today use local artifacts or pinned source revisions. Copy the configuration for the
ecosystem you use into the consumer repository.

## npm (Renovate)

```json
{
  "extends": ["config:recommended"],
  "packageRules": [
    {
      "matchPackageNames": ["@local-ai-kit/sdk", "@local-ai-kit/cli-*"],
      "groupName": "local-ai-kit",
      "rangeStrategy": "bump"
    }
  ]
}
```

Keep `@local-ai-kit/sdk` and its `@local-ai-kit/cli-*` optional dependencies in
one group: they must move together.

## npm (Dependabot)

```yaml
version: 2
updates:
  - package-ecosystem: npm
    directory: /
    schedule:
      interval: weekly
    groups:
      local-ai-kit:
        patterns:
          - '@local-ai-kit/*'
```

## Cargo (Renovate)

```json
{
  "extends": ["config:recommended"],
  "packageRules": [
    {
      "matchManagers": ["cargo"],
      "matchPackageNames": ["local-ai-kit"],
      "groupName": "local-ai-kit"
    }
  ]
}
```

## Cargo (Dependabot)

```yaml
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: weekly
```

## pip (Renovate)

```json
{
  "extends": ["config:recommended"],
  "packageRules": [
    {
      "matchManagers": ["pep621", "pip_requirements"],
      "matchPackageNames": ["local-ai-kit"],
      "groupName": "local-ai-kit"
    }
  ]
}
```

## pip (Dependabot)

```yaml
version: 2
updates:
  - package-ecosystem: pip
    directory: /
    schedule:
      interval: weekly
```

## Policy for consumer updates

- Treat a helper bump like any dependency change: run the app's own tests and
  one real flow; the SDK's protocol handshake already negotiates versions.
- A version mismatch with a running shared broker falls back to the app's
  private helper when one is available. The active mode and `FALLBACK_PROTOCOL_MISMATCH` reason are exposed by automatic
  clients. Authorization denial never selects another runtime.
- Never pin a platform package manually. Install the SDK; the optional
  dependency resolves the right binary, or `LAI_HELPER_PATH` overrides it during
  development.
