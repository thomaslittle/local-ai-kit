# Shared runtime acceptance ledger — 2026-10-22

- **Repository:** local-ai-kit (`C:\Users\User\Sites\local-ai-kit`)
- **Branch:** `feat/shared-runtime`. Code head `eed3a2c26d7c0930d7f6e6313eed29bb528e9df8`
  (per-app grants, OpenCode Go billing, opt-in notification sink, interactive
  control-center fixes); later docs/evidence commits may follow.
- **Pull requests:** local-ai-kit [#28](https://github.com/thomaslittle/local-ai-kit/pull/28);
  Remotif [#36](https://github.com/thomaslittle/remotif/pull/36);
  Crntly [#117](https://github.com/thomaslittle/crntly-desktop/pull/117).
- **Consumer heads:** Remotif `fc4a99e81b4a0da8ada8101c7469c6c6d5104bc6`
  (`feat/shared-local-ai-runtime`); Crntly `18f6dc52da9768c6d279708809c6b66abc8e2f4c`
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
| GUI lifecycle: first-app close/reopen, disconnect/reconnect, revoke | **partial** | broker lifecycle tests cover consumer disconnect, duplicate launch, idle shutdown, app revoke; Remotif picker proves discovery/refresh/error/recovery; Remotif bridge proves broker kill/stale-cache refusal/restart; Crntly GUI blocked; dual-app close/reopen not run |
| Mutation validation of critical guards | implemented, **platform-executed on Windows** | four real patch-restored faults: metadata base revision, journal retention, app-grant revocation, exact Go provider id; each selected test failed with a real panic (exit 101), source was restored byte-for-byte, same test passed (exit 0); logs and SHA-256 summary in `evidence/2026-10-22-shared-runtime/mutations/` |
| GUI failure injection | **partial** | synthetic gateway 503, broker grant revoke, auth/paid/unknown cases tested; full GUI error matrix and provider failure matrix not run |
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
