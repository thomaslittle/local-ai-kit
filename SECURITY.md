# Security and privacy

## Current status

This is a planning/bootstrap repository. No service, API, credential store, broker, or provider adapter is implemented or security-qualified yet. Security requirements below are acceptance criteria, not delivered protections.

## Required implementation boundaries

- Importing packages is side-effect-free. Passive discovery cannot run inference, launch agents, contact providers, or read secrets.
- Use revocable, per-application/per-connection grants. Detection permission does not permit execution or paid use.
- Keep native sign-ins with their owning clients where possible; never expose provider credentials in snapshots, events, UI, logs, URLs, or sample code.
- Authenticate local transports and enforce scopes on every path, including streams and artifacts. CORS is not authentication.
- Scope workspaces and operations; enforce approval denial and cancellation; terminate only owned processes.
- Use synthetic homes/credentials/clients for default tests. Avoid real private account data in fixtures.
- Validate protocol frames, provider responses, symlink containment, redirects, and credential-to-endpoint binding.
- Keep undocumented metadata mechanisms explicitly classified, reviewed, and opt-in.

See the architecture specification and implementation plan for the full threat model and required tests.

## Reporting

Report sensitive findings privately to the repository owner or through a private GitHub security advisory when enabled. Do not include real tokens, account credentials, private prompts, or personal logs in public issues or pull requests. Provide a minimal redacted reproduction and distinguish planned behavior from implemented behavior.
