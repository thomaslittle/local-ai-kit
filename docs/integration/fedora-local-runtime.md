# Fedora local runtime

The local runtime is independent of the optional hosted cached-usage MCP route.
Local discovery needs no remote deployment or plugin activation.

## Build and install

Use the repository's pinned Rust toolchain and build the existing CLI:

```sh
cargo build --locked -p lai-cli --bin local-ai-kit
target/debug/local-ai-kit --version
```

The current consumer contract is runtime `0.1.0-alpha.18`, protocol `1.0`.
A source build is not the published release artifact: consumers that pin a
SHA256 must explicitly configure the hash of the approved source-built file,
while retaining version/protocol/hash validation. Do not bypass those checks.
Install the executable in a versioned user-local directory and expose
`$HOME/.local/bin/local-ai-kit`; give consumers an absolute path when their GUI
environment does not inherit that PATH. No root install is needed.

## One private shared broker

Create a mode-0700 directory under `$HOME/.local/state/local-ai-kit/fedora-shared`.
An owner-only `grants.json` contains separate app identities and token-file paths:

```json
{"schemaVersion":1,"apps":[
  {"id":"remotif","tokenFile":"/ABSOLUTE/PRIVATE/remotif-token.json","allowPassiveDiscovery":true},
  {"id":"crntly","tokenFile":"/ABSOLUTE/PRIVATE/crntly-token.json","allowPassiveDiscovery":true},
  {"id":"haiwire","tokenFile":"/ABSOLUTE/PRIVATE/haiwire-token.json","allowPassiveDiscovery":true}
]}
```

Replace the paths with owner-selected absolute local paths, then launch:

```sh
LOCAL_AI_KIT_STATE_DIR="$HOME/.local/state/local-ai-kit/fedora-shared/cache" \
  "$HOME/.local/bin/local-ai-kit" serve \
  --ipc "$HOME/.local/state/local-ai-kit/fedora-shared/broker.sock" \
  --independent \
  --grants-file "$HOME/.local/state/local-ai-kit/fedora-shared/grants.json" \
  --allow-passive-discovery
```

This listens only on a mode-0600 Unix socket. The supported broker lifecycle
creates distinct mode-0600 local app descriptors; these are not provider API keys
or OAuth grants. Never print their contents or commit them. No admin credential,
metadata-probe grant, execution permission, HTTP listener or remote exposure is
included. The broker uses the existing home for passive installation discovery;
its cache lives in the selected private state directory.

No startup service is required for this launch. Keep the owned process running
while applications connect. Consumer disconnects do not stop it; a second broker
on the same socket fails with `CONFLICT`. Ctrl+C (SIGINT) performs normal shutdown
and removes its socket/descriptors. Restart the same command to republish app
credentials; do not manually delete a live socket or start competing scanners.
This manual launch does not promise persistence across reboot.

## Consumer configuration

Remotif accepts these launch environment variables:

```sh
REMOTIF_LOCAL_AI_KIT="$HOME/.local/bin/local-ai-kit"
REMOTIF_AI_BROKER_TOKEN_FILE="$HOME/.local/state/local-ai-kit/fedora-shared/remotif-token.json"
```

Crntly's debug development override is `CRNTLY_LOCAL_AI_KIT`; its shared bridge
reads `CRNTLY_AI_BROKER_TOKEN_FILE`. Its owner must handle platform-specific
packaged helper discovery. Haiwire's owner selects its supported config with
`helper`, explicit `helperSha256`, and `transport` set to `mode: "broker"`, the
socket `address`, and its own `tokenFile`. Do not edit another application's
configuration behind its owner or share one application's descriptor with another.

Consumers spawn `local-ai-kit connect --ipc ADDRESS`, authenticate using the
descriptor internally, then negotiate `runtime.handshake`. Handshake-advertised
methods are authoritative: alpha.18 supports `snapshot.get`, `models.page`,
`models.list`, `capabilities.read`, `selection.resolve` and `health.get`;
there is no `models.describe` RPC. A configured but unavailable shared broker
must remain an explicit error, not trigger a second standalone scanner.

## Discovery is not authenticated readiness

Passive discovery can find installations and connections with zero models.
It does not establish sign-in, plan, subscription quota or generation readiness.
Authenticated metadata/catalog probes may contact account services or invoke a
native client's credential refresh; obtain the appropriate specific permission
before adding those app scopes. Do not run inference to inspect status or usage.
A passive shared consumer should read the broker cache without automatically
probing every connection. Actual generation remains untested until separately
requested and executed; model context capacity is not provider quota.

## Separately approved metadata access

After approval for existing Codex, Claude Code and OpenCode sign-ins, the owner
can add `"probeProviderIds": ["codex", "claude_code", "opencode"]` to each approved
app entry, gracefully stop the broker, then restart the same command with
`--allow-metadata codex --allow-metadata claude_code --allow-metadata opencode`.
Descriptor paths stay the same; their credentials rotate on restart. This allows
supported account/catalog/usage metadata and normal native credential refresh,
not session execution. Never add an execution grant merely to show a catalog.

The October 24 Fedora execution needed the existing Unix executable-symlink
fix (`5bd139d`, integrated as `2daa749`) to discover the installed native Codex
and Claude Code commands. The focused discovery-to-probe fixture passed, then
real authorized metadata checks reported authenticated Codex and primary Claude
profiles. A separate Claude profile remained `needs_auth`; its catalog aliases
must not be mistaken for authenticated readiness. OpenCode 2.1.4 was detected
but rejected as `PROTOCOL_INCOMPATIBLE`: discovered-process startup currently
requires 1.x. Do not remove that version check or invoke incompatible flags.
These results qualify metadata only, not generation or all consumer GUI flows.

## Separately approved app-scoped Claude execution

After approval to run Claude Code for specific applications, add an
`execution` block to each approved app entry. Keep it as narrow as the consumer
needs:

```json
"execution": {
  "connectionIds": ["<authenticated primary claude_code connection id>"],
  "workspaces": {"default": "/ABSOLUTE/PRIVATE/workspaces/<app>"},
  "allowPaid": false, "allowWrites": false,
  "reviewNative": false, "allowNativePolicy": false
}
```

Scope by connection id rather than `providerIds` so a `needs_auth` or
unknown-billing profile of the same provider is not granted. Each workspace is an
owner-only directory dedicated to that app. Consumers send
`workspaceId: "default"` and `permissionMode: "read_only"`. Gracefully stop the
broker, then restart the same command adding `--allow-execution claude_code` and
`--workspace /ABSOLUTE/PRIVATE/workspaces/host-unused`. The argument parser
requires a host workspace whenever execution is allowed. With only a grants
manifest (no `--token-file`/`--admin-token-file`), that host grant is never
attached to any descriptor; app grants come solely from the manifest. Execution
providers must also be metadata providers. Descriptor paths stay the same and
credentials rotate, so connected consumers must reconnect. An app's `grants.list`
reporting `execution: true` means the grant is installed. It is not consumer
connection or completed generation.
