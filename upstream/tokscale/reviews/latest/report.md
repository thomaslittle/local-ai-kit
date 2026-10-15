Tokscale upstream discovery changes

Tokscale revision: 1d9a9395418efc6952944b794097935d7d6fa1e8 -> af8ec3efd674b27864231b7c5a158acdf644e7a0
Local parity: PASS

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

Path literals:
  added: .omo/agent/sessions

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
  Downloaded autocfg v1.5.1
  Downloaded cfg-if v1.0.5
  Downloaded bitflags v2.13.2
  Downloaded bigdecimal v0.4.10
  Downloaded crypto-common v0.1.7
  Downloaded itoa v1.0.18
  Downloaded cpufeatures v0.2.17
  Downloaded generic-array v0.14.7
  Downloaded inotify-sys v0.1.8
  Downloaded ref-cast v1.0.27
  Downloaded ref-cast-impl v1.0.27
  Downloaded version_check v0.9.5
  Downloaded file-id v0.2.3
  Downloaded errno v0.3.14
  Downloaded notify-types v2.1.0
  Downloaded same-file v1.0.6
  Downloaded digest v0.10.7
  Downloaded dyn-clone v1.0.20
  Downloaded fastrand v2.5.0
  Downloaded thiserror-impl v2.0.21
  Downloaded serde_derive_internals v0.30.0
  Downloaded zmij v1.0.23
  Downloaded quote v1.0.47
  Downloaded inotify v0.11.5
  Downloaded walkdir v2.5.0
  Downloaded sha2 v0.10.9
  Downloaded once_cell v1.21.4
  Downloaded thiserror v2.0.21
  Downloaded num-integer v0.1.47
  Downloaded notify v8.2.0
  Downloaded schemars_derive v1.2.2
  Downloaded unicode-ident v1.0.26
  Downloaded iana-time-zone v0.1.65
  Downloaded tempfile v3.27.0
  Downloaded num-traits v0.2.19
  Downloaded log v0.4.34
  Downloaded getrandom v0.4.3
  Downloaded serde_derive v1.0.229
  Downloaded proc-macro2 v1.0.107
  Downloaded serde_core v1.0.229
  Downloaded schemars v1.2.2
  Downloaded serde v1.0.229
  Downloaded memchr v2.8.3
  Downloaded num-bigint v0.4.8
  Downloaded typenum v1.20.1
  Downloaded mio v1.2.3
  Downloaded serde_json v1.0.151
  Downloaded libm v0.2.16
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
   Compiling zmij v1.0.23
   Compiling bitflags v2.13.2
   Compiling serde v1.0.229
   Compiling cfg-if v1.0.5
   Compiling libm v0.2.16
   Compiling ref-cast v1.0.27
   Compiling inotify-sys v0.1.8
   Compiling serde_json v1.0.151
   Compiling serde_derive_internals v0.30.0
   Compiling num-integer v0.1.47
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling bigdecimal v0.4.10
   Compiling memchr v2.8.3
   Compiling thiserror v2.0.21
   Compiling serde_derive v1.0.229
   Compiling ref-cast-impl v1.0.27
   Compiling itoa v1.0.18
   Compiling schemars_derive v1.2.2
   Compiling thiserror-impl v2.0.21
   Compiling num-bigint v0.4.8
   Compiling digest v0.10.7
   Compiling same-file v1.0.6
   Compiling dyn-clone v1.0.20
   Compiling cpufeatures v0.2.17
   Compiling log v0.4.34
   Compiling iana-time-zone v0.1.65
   Compiling mio v1.2.3
   Compiling sha2 v0.10.9
   Compiling walkdir v2.5.0
   Compiling inotify v0.11.5
   Compiling schemars v1.2.2
   Compiling chrono v0.4.45
   Compiling notify-types v2.1.0
   Compiling rustix v1.1.5
   Compiling getrandom v0.4.3
   Compiling notify v8.2.0
   Compiling lai-contracts v0.1.0-alpha.15 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-contracts)
   Compiling file-id v0.2.3
   Compiling linux-raw-sys v0.12.1
   Compiling fastrand v2.5.0
   Compiling once_cell v1.21.4
   Compiling tempfile v3.27.0
   Compiling lai-platform v0.1.0-alpha.15 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-platform)
   Compiling lai-discovery v0.1.0-alpha.15 (/home/runner/work/local-ai-kit/local-ai-kit/crates/lai-discovery)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.62s
     Running tests/upstream_parity.rs (target/debug/deps/upstream_parity-228e972fbcecad19)

This report does not change Local AI Kit discovery. Review the diff, update the pin and adapters deliberately, and do not merge it automatically.
