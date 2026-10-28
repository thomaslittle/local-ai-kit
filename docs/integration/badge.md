# Status badge

The same facts power the dashboard, `local-ai-kit status` and a small badge on a
website. Nothing in the badge reads a credential; it shows what a granted client
is allowed to see.

## The data

`local-ai-kit status --json` prints one object and exits:

| exit | meaning |
| --- | --- |
| 0 | running and this credential is accepted |
| 3 | no helper is listening (or no descriptor is configured) |
| 4 | the helper is running but rejected this credential |

```json
{
  "running": true,
  "authorized": true,
  "mode": "shared",
  "protocolVersion": "1.0",
  "revision": "projection_…",
  "providers": ["Codex", "Claude Code", "Ollama"],
  "models": 371,
  "connectedApps": 2
}
```

## The badge

```ts
// Uses the web bridge (extension first, pairing second).
const status = await detect(); // 'extension' | 'pairing' | 'none'

const badge =
  status === 'none'
    ? {label: 'Not detected', action: 'Install OriginLM'}
    : {label: `Local AI connected · ${providers.join(', ')}`, action: null};
```

Rules:

- Render at most three provider names, then "+N".
- "Not detected" always offers the install page; it never says "no providers
  found", because that would hide a sandbox or a missing helper.
- Sandboxed pages (Flatpak/Snap/App Store, or a browser without the extension)
  show "Sandboxed" with a short explanation, not an error.
- The badge is read-only: it cannot start a scan, grant access or run a turn.
  Those still require an explicit call and the normal grant.

## Terminal

```sh
local-ai-kit status
local-ai-kit status --json
```

`status` reads an existing broker only. It never starts, scans or probes one, so
it is safe in a shell prompt or a health check.
