# Shared runtime acceptance ledger — 2026-10-22

- **Repository:** local-ai-kit (`C:\Users\User\Sites\local-ai-kit`)
- **Branch:** `feat/shared-runtime`. Code head `eed3a2c26d7c0930d7f6e6313eed29bb528e9df8`
  (per-app grants, OpenCode Go billing, opt-in notification sink, interactive
  control-center fixes); the synthetic gateway fixture is
  `2ddd0e27ff1ea6f120020088545bfb1d0d793c51` (`recovered`/`Kimi K4` mode);
  later docs/evidence commits may follow.
- **Pull requests:** local-ai-kit [#28](https://github.com/thomaslittle/local-ai-kit/pull/28);
  Remotif [#36](https://github.com/thomaslittle/remotif/pull/36);
  Crntly [#117](https://github.com/thomaslittle/crntly-desktop/pull/117).
- **Consumer heads:** Remotif `fc4a99e81b4a0da8ada8101c7469c6c6d5104bc6`
  (`feat/shared-local-ai-runtime`); Crntly `b05ebb3002f9c63d0ec3ef802a956e0256e36d2e`
  (`feat/shared-local-ai-runtime`, based on
  `origin/feature/gpui-native-rust-studio`). The final Crntly binary was built
  from that source: SHA-256
  `30c1259c2d0a2aea22c280c73157416abe34714bda6858eb9f55b51cf7b58c4b`.
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
| Crntly native Compose model picker against one shared broker | implemented, **platform-executed on Windows (in-process E2E input)** | stack overflow diagnosed with live cdb and fixed in `f3cb5688`; final classification fix `b05ebb30`; `cargo test -p crntly-compose-core` at final source: 114 passed, 0 failed, 1 ignored; full combined gate below lists `Kimi K3`, `GLM-5.3`, and `Kimi K4` and proves app-scoped revocation |
| Control center public-API E2E | implemented, **platform-executed on Windows** | `cargo test -p lai-control --test e2e` |
| Update signature/staging/active-session/rollback | implemented, fixture-tested | `cargo test -p lai-update` (4 tests) |
| T3/Tokscale watcher parsing, classification, coverage report | implemented, fixture-tested | `python -m unittest scripts.test_upstream_sync scripts.test_upstream_t3code` (11 tests) |
| Full Rust workspace | fixture-tested on Windows | `cargo test --workspace` - 611 passed, 0 failed (122 suites) at code head `eed3a2c`; a prior run hit a transitive Windows loopback socket-buffer error in one HTTP test, retained with the passing targeted retry and full rerun under `evidence/2026-10-22-shared-runtime/tests/` |
| Linux and macOS shared-runtime tests | **platform-executed in GitHub CI** | run [39775047109](https://github.com/thomaslittle/local-ai-kit/actions/runs/39775047109) at prior code head `db240c0`: Ubuntu, macOS, Windows all success; final `eed3a2c` VM verification is recorded in the PR checks, not claimed here until it completes |
| Rust lint/format | passing | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| TypeScript SDK/React/store | fixture-tested on Windows | `npm run test:react` (13), `npm run test:sdk` (68) |
| Python SDK | fixture-tested on Windows | `python -m unittest discover -s packages/sdk-python/tests` (27) |
| Remotif shared runtime | implemented, **platform-executed on Windows** | `cargo test -p remotif-kit` (6) with real broker/token; isolated native model-picker run with a distinct app token |
| Remotif phone bridge shared mode | implemented, **platform-executed on Windows (loopback HTTP only)** | isolated `remotifd --demo --bind 127.0.0.1`, unique port, child PID listener ownership, `/api/v1/motif/models?refresh=true`: Go eligible/`declared:false`/`paid:false`, PAYG consent, cached state, updated model, synthetic 503 notice, recovery; no LAN bind, firewall change, TV, or inference |
| Remotif bridge scope/revocation/recovery | implemented, **platform-executed on Windows (real broker, test-owned IPC)** | `cargo test -p remotifd --test shared_bridge` at Remotif `fc4a99e`: two app-scoped tokens see only their codex or claude connection; `apps.revoke` invalidates the app and its cached view; peer survives; broker kill returns explicit unavailable rather than stale catalog; restart/reconnect recovers; log retained under `evidence/2026-10-22-shared-runtime/bridge/` |
| Notification coordinator vs OS delivery | coordinator implemented; **optional OS sink implemented behind `os-notifications`/`--os-notifications`; not enabled in qualification** | injected sink test proves due decisions reach a host sink and duplicate polls do not; no real desktop notification was emitted; OS toast delivery is not claimed in the default build |
| Control-center usability | implemented, **platform-executed in a test-owned Windows ConPTY** | keyboard/visual pass at 100x30 and 80x24 through real TUI rendering, no `SendInput` or global input: tab switching, row selection, confirmation prompt and cancel, redacted export, diagnostics scrolling, clean exit; a retained 80x24 failure showed the old prompt clipping `y/n`; later fixes and lost-attempt disclosure are in `evidence/2026-10-22-shared-runtime/control-center/` |
| GUI lifecycle: first-app close/reopen, disconnect/reconnect, revoke | implemented, **platform-executed on Windows, with one explicit gap** | combined gate at Crntly `b05ebb30` / Remotif `fc4a99e`: distinct live clients, shared `GLM-5.3`, actual Crntly process close/reopen, explicit outage, broker restart, both live after **both app process restarts**, fresh `Kimi K4` seen by both, fresh Crntly `LOCAL_ACCESS_DENIED` denial with null baseline, and Remotif changed post-revoke catalog (`Kimi K4` removed) + `ready`. **Automatic reconnect was not implemented or qualified**; the gate used manual process restart. |
| Mutation validation of critical guards | implemented, **platform-executed on Windows** | four real patch-restored faults: metadata base revision, journal retention, app-grant revocation, exact Go provider id; each selected test failed with a real panic (exit 101), source was restored byte-for-byte, same test passed (exit 0); logs and SHA-256 summary in `evidence/2026-10-22-shared-runtime/mutations/` |
| GUI failure injection | **partial** | synthetic gateway 503, broker grant revoke, auth/paid/unknown cases tested; full GUI error matrix and provider failure matrix not run |
| Crntly shared helper | implemented, **platform-executed on Windows** | `cargo test -p crntly-core --no-default-features ai_assistant` (90); opt-in `shared_broker_helper_authenticates_and_reads_a_snapshot` with a real broker; shared health/issues added to the snapshot DTO and generated bindings |
| Full Crntly + Remotif combined-client gate | implemented, **platform-executed on Windows (synthetic loopback, sequential injected input)** | accepted result `docs/developer/gpui-native-migration/evidence/2026-10-22-compose-stack/combined/full-gate-result.json` at Crntly `b05ebb30`, Remotif `fc4a99e`, kit `eed3a2c`, gateway fixture `2ddd0e2`: distinct client IDs on one broker; shared `GLM-5.3`; actual Crntly process close/reopen; Crntly outage `The AI helper stopped` (transport, not protocol) and Remotif explicit error with cached rows not treated as recovery; broker restart plus both app processes restarted; both live and observe fresh `Kimi K4`; app-scoped revoke yields fresh specific denial, while Remotif observes changed post-revoke catalog, reports `ready`, and remains registered. Manual-restart recovery only. |
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

## Deliberate fault validation

This replaces the earlier hypothetical notes. A disposable patch-restored
checkout at `76df73d` (`rustc 1.97.1`, Windows) changed one guard at a time,
ran only the selected acceptance test, restored the exact original bytes, and
ran the same test again. No faulty variant was committed or pushed. The
repository was verified clean after every restore and at the end.

| Fault introduced | Selected test | Observed failure | Restored result |
|---|---|---|---|
| `if false && revision != base` disables the metadata base-revision guard (`lai-service/src/cache.rs`) | `an_older_probe_cannot_overwrite_a_newer_revision` | panic at `runtime_sync.rs:226`: `unwrap_err()` on `Ok {"status":"updated"}`; exit 101 | 1 passed; exit 0 |
| `MAX_JOURNAL_ROWS` raised from 2048 to 4096 (`lai-store/src/lib.rs`) | `journal_is_bounded_and_evicts_old_cursors_without_losing_the_snapshot` | assertion at `store.rs:105`: earliest cursor was replayed instead of `EventGap`; exit 101 | 1 passed; exit 0 |
| `Shared::revoke_app` no longer removes the reusable grant (`lai-transports/src/local_ipc.rs`) | `scoped_app_grants_isolate_accounts_and_revocation_removes_the_grant` | panic at `local_ipc.rs:428`: revoked token authenticated again (`"authenticated":true`); exit 101 | 1 passed; exit 0 |
| Exact `provider_id == "opencode-go"` changed to `starts_with` (`lai-adapters/src/opencode/mod.rs`) | `lookalike_provider_ids_do_not_receive_the_go_subscription_class` | assertion at `opencode_catalog.rs:134`: lookalike inherited Go subscription; exit 101 | 1 passed; exit 0 |

Artifacts: [mutation summary](evidence/2026-10-22-shared-runtime/mutations/summary.json),
the four `*-fault.log` and `*-restored.log` pairs in the same directory. Each
summary entry records the source SHA-256 before and after, the exact command,
exit codes, and the selected test name. The logs are sanitized of 64-hex values;
the summary retains the source hashes. Tests not selected here, such as update
rollback and notification dedupe, remain fixture coverage rather than mutation
evidence.

## Remaining blockers / unclaimed scope

1. **Automatic broker reconnection is not implemented or qualified.** The
   accepted combined gate recovered only after **both native app processes were
   restarted** following the broker restart; the existing workers did not
   re-authenticate on their own. The crate/bridge tests still prove explicit
   stale-cache refusal and recovery in their own paths, but no automatic
   native-client reconnect is claimed. The prior stack overflow was diagnosed
   with live cdb and fixed; it is no longer a blocker for the synthetic path.
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
   malware. Cross-platform app-grant tests remain open. The synthetic GUI
   dual-app gate passed; live OpenCode Go entitlement, quota, and Console Extra
   usage remain unverified, and the route class alone does not claim them.
7. **Consumer CI availability.** Remotif runs
   [39787048161](https://github.com/thomaslittle/remotif/actions/runs/39787048161)
   and [39787048643](https://github.com/thomaslittle/remotif/actions/runs/39787048643)
   at `fc4a99e` completed failure with all four jobs reporting zero steps,
   consistent with the account's runner/billing restriction; no code step ran.
   Crntly's `check.yml` is manual-only and no workflow run exists for
   `b05ebb30`/`a8957095`; local targeted tests are the available evidence.
   These are external CI availability limitations, not local test failures.
8. **Two-app GUI acceptance boundaries.** The synthetic dual-app lifecycle
   gate passed, but recovery required both app process restarts. This is not an
   automatic-reconnect qualification, a live provider/account result, or a
   release sign-off. A real notification, paid inference, or new provider test
   was not run.
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
