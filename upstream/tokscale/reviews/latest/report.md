Tokscale upstream discovery changes

Tokscale revision: 1d9a9395418efc6952944b794097935d7d6fa1e8 -> 54de31f96bec962e8a696e7421f927369c26e113
Local parity: PASS

Provider: amp
Type: modified
Previous:
  xdg-data / amp/threads
Current:
  config / amp-cache/threads
  windows: C:\Users\User\.local\share\amp\threads -> C:\Users\User\.config\tokscale\amp-cache\threads
  macos: /Users/user/.local/share/amp/threads -> /Users/user/.config/tokscale/amp-cache/threads
  linux: /home/user/.local/share/amp/threads -> /home/user/.config/tokscale/amp-cache/threads
Affected Local AI Kit adapter: (no Local AI Kit adapter)
Mapping parity: NEEDS REVIEW (unmapped upstream client)

Provider: aside
Type: added
Current:
  home / .aside/u
  windows: C:\Users\User\.aside\u
  macos: /Users/user/.aside/u
  linux: /home/user/.aside/u
Affected Local AI Kit adapter: (no Local AI Kit adapter)
Mapping parity: NEEDS REVIEW (unmapped upstream client)

Provider: codex
Type: modified
Previous:
  CODEX_HOME || ~/.codex / sessions
Current:
  CODEX_HOME || ~/.codex / sessions
  windows: C:\Users\User\.codex\sessions -> C:\Users\User\.codex\sessions
  macos: /Users/user/.codex/sessions -> /Users/user/.codex/sessions
  linux: /home/user/.codex/sessions -> /home/user/.codex/sessions
Affected Local AI Kit adapter: codex
Mapping parity: contained
Note: Profile home is CODEX_HOME or ~/.codex. Tokscale reads sessions/ under that home.

Provider: vibe
Type: added
Current:
  VIBE_HOME || ~/.vibe / logs/session
  windows: C:\Users\User\.vibe\logs\session
  macos: /Users/user/.vibe/logs/session
  linux: /home/user/.vibe/logs/session
Affected Local AI Kit adapter: (no Local AI Kit adapter)
Mapping parity: NEEDS REVIEW (unmapped upstream client)

Environment variables:
  added: VIBE_HOME
  removed: XDG_CONFIG_HOME

Path literals:
  added: .aside/u, .json.tmp, .omo/agent/sessions, .vibe, .vscode-server/data, AppData/Roaming/Block/goose/data/sessions/sessions.db, AppData/Roaming/Code, Block/goose/data/sessions/sessions.db, Library/Application Support/Code, User/globalStorage/{extension_id}/tasks, amp-cache/threads, logs/session
  removed: .vscode-server/data/User/globalStorage/saoudrizwan.claude-dev/tasks, AppData/Roaming/Code/User/globalStorage/saoudrizwan.claude-dev/tasks, Code/User/globalStorage/saoudrizwan.claude-dev/tasks, Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/tasks, {xdg_config_home}/tokscale, {}/.vscode-server/data/User/globalStorage/kilocode.kilo-code/tasks, {}/.vscode-server/data/User/globalStorage/rooveterinaryinc.roo-cline/tasks

Parity check:
running 5 tests
test platform_profile_components_stay_split ... ok
test secure_storage_override_stays_ahead_of_the_tokscale_claude_home ... ok
test windows_redirect_ignores_posix_and_drive_relative_home ... ok
test upstream_parity_matches_pinned_tokscale_discovery ... ok
test adopted_homes_are_discovered_without_reading_session_files ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


info: syncing channel updates for 1.97.1-x86_64-unknown-linux-gnu
info: latest update on 2026-07-16 for version 1.97.1 (8bab26f4f 2026-07-14)
info: downloading 2 components
    Updating crates.io index
 Downloading crates ...
  Downloaded block-buffer v0.10.4
  Downloaded cfg-if v1.0.5
  Downloaded dyn-clone v1.0.20
  Downloaded crypto-common v0.1.7
  Downloaded autocfg v1.5.1
  Downloaded bitflags v2.13.2
  Downloaded errno v0.3.14
  Downloaded inotify-sys v0.1.8
  Downloaded version_check v0.9.5
  Downloaded ref-cast v1.0.27
  Downloaded ref-cast-impl v1.0.27
  Downloaded generic-array v0.14.7
  Downloaded file-id v0.2.3
  Downloaded cpufeatures v0.2.17
  Downloaded itoa v1.0.18
  Downloaded same-file v1.0.6
  Downloaded sha2 v0.10.9
  Downloaded schemars_derive v1.2.2
  Downloaded walkdir v2.5.0
  Downloaded inotify v0.11.5
  Downloaded fastrand v2.5.0
  Downloaded num-integer v0.1.47
  Downloaded zmij v1.0.23
  Downloaded iana-time-zone v0.1.65
  Downloaded serde_derive_internals v0.30.0
  Downloaded digest v0.10.7
  Downloaded thiserror-impl v2.0.21
  Downloaded quote v1.0.47
  Downloaded notify-types v2.1.0
  Downloaded tempfile v3.27.0
  Downloaded unicode-ident v1.0.26
  Downloaded thiserror v2.0.21
  Downloaded notify v8.2.0
  Downloaded once_cell v1.21.4
  Downloaded getrandom v0.4.3
  Downloaded log v0.4.34
  Downloaded num-traits v0.2.19
  Downloaded serde_core v1.0.229
  Downloaded proc-macro2 v1.0.107
  Downloaded bigdecimal v0.4.10
  Downloaded serde_derive v1.0.229
  Downloaded schemars v1.2.2
  Downloaded serde v1.0.229
  Downloaded memchr v2.8.3
  Downloaded num-bigint v0.4.8
  Downloaded mio v1.2.3
  Downloaded typenum v1.20.1
  Downloaded libm v0.2.16
  Downloaded serde_json v1.0.151
  Downloaded chrono v0.4.45
  Downloaded syn v3.0.6
  Downloaded rustix v1.1.5
  Downloaded libc v0.2.189
  Downloaded linux-raw-sys v0.12.1
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling autocfg v1.5.1
   Compiling version_check v0.9.5
   Compiling libc v0.2.189
   Compiling generic-array v0.14.7
   Compiling num-traits v0.2.19
   Compiling serde_core v1.0.229
   Compiling typenum v1.20.1
   Compiling syn v3.0.6
   Compiling serde v1.0.229
   Compiling bitflags v2.13.2
   Compiling zmij v1.0.23
   Compiling cfg-if v1.0.5
   Compiling inotify-sys v0.1.8
   Compiling libm v0.2.16
   Compiling serde_json v1.0.151
   Compiling ref-cast v1.0.27
   Compiling serde_derive_internals v0.30.0
   Compiling num-integer v0.1.47
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling bigdecimal v0.4.10
   Compiling itoa v1.0.18
   Compiling memchr v2.8.3
   Compiling thiserror v2.0.21
   Compiling serde_derive v1.0.229
   Compiling ref-cast-impl v1.0.27
   Compiling schemars_derive v1.2.2
   Compiling thiserror-impl v2.0.21
   Compiling digest v0.10.7
   Compiling num-bigint v0.4.8
   Compiling log v0.4.34
   Compiling cpufeatures v0.2.17
   Compiling dyn-clone v1.0.20
   Compiling same-file v1.0.6
   Compiling iana-time-zone v0.1.65
   Compiling walkdir v2.5.0
   Compiling sha2 v0.10.9
   Compiling mio v1.2.3
   Compiling schemars v1.2.2
   Compiling chrono v0.4.45
   Compiling inotify v0.11.5
   Compiling notify-types v2.1.0
   Compiling rustix v1.1.5
   Compiling getrandom v0.4.3
   Compiling lai-contracts v0.1.0-alpha.17 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-contracts)
   Compiling notify v8.2.0
   Compiling file-id v0.2.3
   Compiling linux-raw-sys v0.12.1
   Compiling once_cell v1.21.4
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling lai-platform v0.1.0-alpha.17 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-platform)
   Compiling lai-discovery v0.1.0-alpha.17 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-discovery)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.99s
     Running tests/upstream_parity.rs (target/debug/deps/upstream_parity-ca6da7c31c4ed94c)

This report does not change Local AI Kit discovery. Review the diff, update the pin and adapters deliberately, and do not merge it automatically.
