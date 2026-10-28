# Background service

The one-line install is optional. Apps installed as dependencies already start
their own private helper; the service is for people who want the dashboard and
one shared runtime across applications.

## Install

```sh
curl -fsSL https://<site>/install.sh | sh
```

```powershell
irm https://<site>/install.ps1 | iex
```

No admin rights. The script:

1. detects the platform and downloads the matching release archive;
2. verifies it against the release `SHA256SUMS` (and a minisign signature when
   `minisign` and `LAI_MINISIGN_PUBKEY` are available);
3. installs one binary into `~/.local/bin` (macOS/Linux) or
   `%LOCALAPPDATA%\OriginLM\bin` (Windows);
4. registers a **user** service:
   - macOS: `~/Library/LaunchAgents/dev.originlm.helper.plist` (`KeepAlive`)
   - Linux: `~/.config/systemd/user/originlm-helper.service` (`Restart=on-failure`)
   - Windows: the `OriginLM Helper` Task Scheduler logon task
5. starts an independent broker on the well-known address with
   `~/.local-ai-kit/broker-token.json`, so dependency-first SDKs attach to it
   automatically.

Overrides: `LAI_RELEASE_BASE`, `LAI_BIN_DIR`, `LAI_STATE_DIR`,
`LAI_BROKER_SOCKET`, `LAI_MINISIGN_PUBKEY`.

## Uninstall

```sh
curl -fsSL https://<site>/uninstall.sh | sh
```

```powershell
irm https://<site>/uninstall.ps1 | iex
```

Removes the user service, tray autostart entry, Native Messaging manifests for
every browser it may have registered, and the binary. User data (state, history,
grants) is kept unless `--purge` / `-Purge` is passed. The command is idempotent
and never needs admin rights.

## Behaviour

- Idle cost is the broker's slow inventory/metadata cadence; no polling loop
  runs in the tray.
- The service restarts on crash and starts at logon.
- On-demand start still works: an app with no running broker starts its own
  private helper, so the service is a convenience, not a dependency.
- Self-update runs in the helper with checksum and signature verification; the
  tray only reports.

## Status surfaces

| Surface | State |
| --- | --- |
| `local-ai-kit status` | Planned in this phase; prints the same mode/providers/apps as the SDK status |
| SDK status | Phase 1 (`LocalAi::status`, `ai.mode`) |
| Tray / menu bar | Planned: optional, lists connected apps and models, Pause and Open Control Center; no popups |
| Notifications | Existing broker notification decisions; only user-actionable items |

## Tests for this phase

Install, enable, crash-restart, upgrade verification and uninstall are exercised
per platform at the end of the project. Scripts parse-check on Linux
(`sh -n`); Windows/macOS execution is not yet qualified.
