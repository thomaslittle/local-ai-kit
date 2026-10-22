# Shared runtime acceptance ledger — 2026-10-22

- **Repository:** local-ai-kit (`C:\Users\User\Sites\local-ai-kit`)
- **Branch:** `feat/shared-runtime`. Implementation head `eb00aee07178133d0ca0eef65a9a1727b5250083`
  (per-app grants + OpenCode Go billing); later docs-only commits may follow.
- **Pull requests:** local-ai-kit [#28](https://github.com/thomaslittle/local-ai-kit/pull/28);
  Remotif [#36](https://github.com/thomaslittle/remotif/pull/36);
  Crntly [#117](https://github.com/thomaslittle/crntly-desktop/pull/117).
- **Consumer heads:** Remotif `026ee01187cb84a0034c76fda3e2352c02ccde96`
  (`feat/shared-local-ai-runtime`); Crntly `c2bf8b6d5014e05ff22abf316232fbdd1829877d`
  (`feat/shared-local-ai-runtime`, based on
  `origin/feature/gpui-native-rust-studio`).
- **Status vocabulary:** planned / implemented / fixture-tested /
  platform-executed / live-qualified. A fixture or mock is not a live provider.

## What was verified on this machine

| Area | Status | Evidence |
|---|---|---|
| Contracts compile; generated schemas/SDKs current | implemented, fixture-tested | `cargo check --workspace`, `python scripts/generate_contracts.py`, `python scripts/check_generated.py` |
| SQLite snapshot/metadata/journal commit and replay | implemented, fixture-tested | `cargo test -p lai-store` (8 tests) |
| Corrupt DB preserved, future schema fails closed, read-only diagnostic open | implemented, fixture-tested | `crates/lai-store/tests/store.rs` |
| Metadata revision advance + inventory event replay + event gap | implemented, fixture-tested | `crates/lai-service/tests/runtime_sync.rs` |
| Out-of-order metadata cannot overwrite a newer revision | implemented, fixture-tested | `runtime_sync::an_older_probe_cannot_overwrite_a_newer_revision` |
| Debounced invalidation + autonomous re-read | implemented, fixture-tested | `runtime_sync::invalidation_is_debounced_then_the_autonomous_lane_rechecks_it` |
| Issue derivation, snooze, projection filtering, notification dedupe/privacy | implemented, fixture-tested | `runtime_sync` health/notification tests; `lai-service/src/notifications.rs` tests |
| Warm SQLite hydration labeled stale | implemented, fixture-tested | `metadata::observations_survive_a_restart_as_stale_while_revalidate_data_without_identity` |
| Independent broker survives consumer disconnect; duplicate converges; idle releases endpoint/token; admin token lists apps | implemented, **platform-executed on Windows** | `cargo test -p lai-cli --test broker_lifetime` |
| Reusable owner credential | implemented, platform-executed on Windows | `lai-transports` local IPC test |
| Independent per-app grants (distinct token, identity, account scope, grant removal) | implemented, **platform-executed on Windows** | `cargo test -p lai-transports --test local_ipc scoped_app_grants_isolate_accounts_and_revocation_removes_the_grant`; `cargo test -p lai-cli --test broker_lifetime independently_scoped_app_grants_revoke_one_app_without_the_owner_credential` |
| OpenCode Go route class / no redundant declaration | implemented, **fixture-tested on Windows** | `opencode_catalog` (10), `opencode_v2_catalog` (5), `billing_declarations` (2), `lai-core` `opencode_go_billing` (3); Remotif isolated GUI run below |
| Remotif native model picker against one shared broker | implemented, **platform-executed on Windows (in-process E2E commands, no OS input)** | isolated `APPDATA`, unique loopback `REMOTIF_E2E_ADDR`, child-PID listener check, foreground PID unchanged, Go eligible/`declared:false`/`paid:false`, Console PAYG `BILLING_CONSENT_REQUIRED`, refresh adds a Go model, synthetic 503 notice, recovery; evidence `%TEMP%\lai-go-ui-2c4f9974d0\remotif-evidence-d8152b1b72\result.json` |
| Crntly native Compose model picker against one shared broker | **not passed / cause unconfirmed** | candidate and clean baseline `c49686e` both hit `thread 'main' has overflowed its stack` when Compose setup runs with an installed helper; no speculative fix retained; exact latest command/binary hashes recorded in the handoff |
| Control center public-API E2E | implemented, **platform-executed on Windows** | `cargo test -p lai-control --test e2e` |
| Update signature/staging/active-session/rollback | implemented, fixture-tested | `cargo test -p lai-update` (4 tests) |
| T3/Tokscale watcher parsing, classification, coverage report | implemented, fixture-tested | `python -m unittest scripts.test_upstream_sync scripts.test_upstream_t3code` (11 tests) |
| Full Rust workspace | fixture-tested on Windows | `cargo test --workspace` - 608 passed, 0 failed (122 suites) at `eb00aee` |
| Linux and macOS shared-runtime tests | **platform-executed in GitHub CI** | run [39767444742](https://github.com/thomaslittle/local-ai-kit/actions/runs/39767444742) at `eb00aee`: Ubuntu 14m44s, macOS 14m19s, Windows 18m21s, all success (full deterministic verification incl. new per-app grant and Go billing tests) |
| Rust lint/format | passing | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| TypeScript SDK/React/store | fixture-tested on Windows | `npm run test:react` (13), `npm run test:sdk` (68) |
| Python SDK | fixture-tested on Windows | `python -m unittest discover -s packages/sdk-python/tests` (27) |
| Remotif shared runtime | implemented, **platform-executed on Windows** | `cargo test -p remotif-kit` (6) with real broker/token; isolated native model-picker run with a distinct app token |
| Remotif phone bridge shared mode | implemented, **not re-run as a phone/HTTP E2E this session** | the bridge uses the same `connect_client`; its shared-broker unit test passed, but no browser/phone session was driven |
| Notification coordinator vs OS delivery | coordinator implemented and fixture-tested; OS notification delivery **not implemented** | dedupe/privacy/quiet-hours tests; no daemon or OS toast delivery claim |
| Control-center usability | public-API E2E passed; **no interactive usability review** | `lai-control --test e2e`; keyboard/visual review remains unclaimed |
| GUI lifecycle: first-app close/reopen, disconnect/reconnect, revoke | **partial** | broker lifecycle tests cover consumer disconnect, duplicate launch, idle shutdown, app revoke; Remotif picker proves discovery/refresh/error/recovery; Crntly GUI blocked; dual-app close/reopen not run |
| Failure injection | **partial** | synthetic gateway 503, broker grant revoke, auth/paid/unknown cases tested; full GUI error matrix and provider disconnects not run |
| Crntly shared helper | implemented, **platform-executed on Windows** | `cargo test -p crntly-core --no-default-features ai_assistant` (90); opt-in `shared_broker_helper_authenticates_and_reads_a_snapshot` with a real broker; shared health/issues added to the snapshot DTO and generated bindings |
| Full Crntly GUI against one broker | **not passed / cause unconfirmed** | clean baseline and candidate both main-stack-overflow in Compose setup; retained as a blocker rather than claimed |
| Live provider inference/account checks | **not run** | no account consumption is authorized by this task; no claim |

## Performance measurements

Measured from `target/debug/local-ai-kit.exe` on Windows 11
(`10.0.26100`, 16 logical CPUs) with an isolated empty home and 10 fixture
profile directories. No provider process, credential read, network call, or
inference was involved.

| Measurement | Value | Budget |
|---|---:|---:|
| Cold process + passive scan p95 (10 samples) | 25.75 ms | < 1000 ms |
| Cached stdio snapshot p95 (200 samples) | 1.85 ms | < 50 ms |
| Idle peak RSS (10 s) | 14.94 MiB | < 50 MiB |
| Idle CPU, % of one core (10 s) | 0.0% | < 1% |
| Broker SQLite main file after scan | 53,248 B | n/a |
| Broker WAL after scan | 53,592 B | n/a |

Raw report: `.verification/shared-runtime-performance.json` (ignored build
artifact; reproduce with `python scripts/benchmark_shared_runtime.py`). These
are debug-build values on one host, not release guarantees.

Journal boundedness is tested separately with 2,101 commits: the earliest
cursor becomes an explicit `EVENT_GAP`, the latest snapshot remains readable,
and the page is capped at 256 records (`journal_is_bounded_and_evicts_old_cursors...`).

## Failure scenarios exercised

- Two clients share one inventory; closing one does not stop the broker.
- Independent broker remains after both clients disconnect; a duplicate launch
  exits non-zero instead of replacing it; idle shutdown removes the token file.
- Metadata probe started before a newer refresh cannot publish (`SELECTION_STALE`).
- Invalidation marks last-known data stale instead of deleting it or claiming
  logout; the autonomous lane re-probes and revives it.
- Corrupt SQLite is preserved with a `.corrupt-<timestamp>` sibling; a future
  schema is rejected; read-only diagnostics do not mutate state.
- Recovery of a previously expired auth issue requires a fresh observation;
  snooze keeps it open; notification polling is deduplicated across clients.
- A denied client sees no issues; issue mutation checks the projected view.
- Update activation refuses while a session marker is present; rollback restores
  the exact previous bytes; a missing staged executable leaves the target intact.
- Upstream orchestration-only changes are filtered; missing watched paths and
  unwatched provider-shaped candidates are reported; PR permission denial is an
  explicit actionable failure, not “no changes.”

## Deliberate fault checks

- Disabled the metadata base-revision check mentally by reproducing the
  out-of-order test; it fails without `SELECTION_STALE`.
- The journal-bound test creates a cursor older than the retained window and
  asserts `EVENT_GAP`; it would pass silently if the journal were unbounded or
  gaps were treated as empty pages.
- The update rollback test compares exact bytes before and after; it would fail
  if activation merely re-copied a staged file or deleted the backup.
- Notification tests poll twice and assert the second has zero new notifications;
  they would fail if dedupe were only in-memory across the service call.

## Remaining blockers / unclaimed scope

1. **Crntly native GUI against one broker.** Remotif's isolated native model
   picker passed against a real independent broker with a distinct app token.
   Crntly's native Compose panel did not reach acceptance: both the current
   candidate and a clean exact-source `c49686e` baseline built in isolated
   targets hit `thread 'main' has overflowed its stack` after Compose setup with
   an installed helper. The cause is not confirmed. Removing broker environment
   or citing an unrelated mixer stack report is not proof; no speculative stack
   or polling fix is retained. The protected Crntly checkout was not touched.
   The Remotif worktree is `C:\Users\User\Sites\tvforge-local-ai-runtime`.
2. **GitHub PR creation for upstream reports.** The repository currently does
   not permit Actions to create/approve PRs. The workflow now runs report-only,
   uploads artifacts, and prints the exact minimum setting. Enabling PRs needs
   maintainer approval.
3. **Signing/publication/installation.** No signing credential, signature,
   registry publication, or live update endpoint was created. `lai-update` is
   fixture-tested; a real signed update was not installed.
4. **Cross-platform qualification.** Shared-runtime tests and the full CI
   verification passed on Windows, Ubuntu, and macOS in run
   [39767444742](https://github.com/thomaslittle/local-ai-kit/actions/runs/39767444742).
   Performance numbers in this ledger are still Windows debug-build values; no
   release-performance or manual GUI qualification is claimed for Linux/macOS.
5. **Live accounts.** No live provider account or inference was used; provider
   support remains as documented in the support matrix.
6. **Independent grant limits.** Per-app tokens, identities, scope projection,
   and live revocation now exist. The token file remains readable by another
   process running as the same OS user; it is not a defense against that user's
   malware. Cross-platform app-grant tests and a real GUI dual-app run are still
   open. Live OpenCode Go entitlement, quota, and Console Extra usage remain
   unverified; the route class alone does not claim them.
7. **Consumer CI availability.** Remotif workflow run
   [39767436935](https://github.com/thomaslittle/remotif/actions/runs/39767436935)
   at `026ee01` completed failure with all four jobs reporting zero steps,
   consistent with the account's runner/billing restriction; no code step ran.
   Crntly has no workflow run for `c2bf8b6` (an empty run list was returned).
   These are external CI availability failures, not local test failures.
8. **Two-app GUI acceptance.** Remotif's isolated native picker passed; Crntly
   native Compose did not. A dual-app qualification cannot be claimed until the
   Crntly main-thread overflow is diagnosed and fixed or the intended final
   build is qualified.
9. **Consumer UI release pins.** Remotif's managed helper remains available when
   no shared broker is configured. Crntly's managed component remains pinned to
   alpha.15; shared mode accepts protocol-compatible alpha.15+ runtimes. No
   package was published or installed.

## Rollout / rollback

1. Ship/install the new helper and control center together. Start with the
   managed stdio mode (default) or a parent-owned broker.
2. For shared mode, provision owner-only token files and start
   `serve --ipc ... --independent ...`; use `--grants-file` for distinct app
   identities or the legacy `--token-file` for one app. Set each consumer's
   broker environment variable. Do not set it unless the broker is reachable; a
   configured but unreachable broker is reported, not replaced.
3. Keep the previous helper binary and SQLite state directory. On failure,
   unset the broker variable (consumers return to managed mode), stop the
   independent broker, and restore the previous executable. The SQLite store is
   forward-compatible only within the same schema; an older runtime refuses a
   newer schema instead of corrupting it.
4. For signed updates, use `lai-update` only with a host-pinned public key and
   drain active sessions before activation; rollback the exact backup if the
   new runtime fails its smoke checks.
