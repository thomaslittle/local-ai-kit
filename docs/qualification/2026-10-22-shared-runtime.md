# Shared runtime acceptance ledger — 2026-10-22

- **Repository:** local-ai-kit (`C:\Users\User\Sites\local-ai-kit`)
- **Branch:** `feat/shared-runtime`. Implementation head before the final
  docs-only refresh: `c16696d12a8f1e39bafb996b33c9800cf87bd72d`; runtime commit
  `f98a8c6`, merged with `origin/main` before the docs/runtime follow-ups. The
  pull request contains the exact current head.
- **Pull requests:** local-ai-kit [#28](https://github.com/thomaslittle/local-ai-kit/pull/28);
  Remotif [#36](https://github.com/thomaslittle/remotif/pull/36);
  Crntly [#117](https://github.com/thomaslittle/crntly-desktop/pull/117).
- **Consumer heads:** Remotif `f4329fe96546e38b218b4218b24f924f8808f75d`
  (`feat/shared-local-ai-runtime`); Crntly `c49686eae68773cd215824788e49df5783a15539`
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
| Control center public-API E2E | implemented, **platform-executed on Windows** | `cargo test -p lai-control --test e2e` |
| Update signature/staging/active-session/rollback | implemented, fixture-tested | `cargo test -p lai-update` (4 tests) |
| T3/Tokscale watcher parsing, classification, coverage report | implemented, fixture-tested | `python -m unittest scripts.test_upstream_sync scripts.test_upstream_t3code` (11 tests) |
| Full Rust workspace | fixture-tested on Windows | `cargo test --workspace` — 598 passed, 0 failed |
| Linux and macOS shared-runtime tests | **platform-executed in GitHub CI** | run [39736523716](https://github.com/thomaslittle/local-ai-kit/actions/runs/39736523716) at code commit `e809afd`: Ubuntu 13m41s, macOS 13m10s, Windows 27m35s, all pass |
| Rust lint/format | passing | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| TypeScript SDK/React/store | fixture-tested on Windows | `npm run test:react` (13), `npm run test:sdk` (68) |
| Python SDK | fixture-tested on Windows | `python -m unittest discover -s packages/sdk-python/tests` (27) |
| Remotif shared runtime | implemented, **platform-executed on Windows** | `cargo test -p remotif-kit --test shared_broker` with a real broker and token file |
| Crntly shared helper | implemented, **platform-executed on Windows** | `cargo test -p crntly-core --no-default-features ai_assistant` (90); opt-in `shared_broker_helper_authenticates_and_reads_a_snapshot` with a real broker; shared health/issues added to the snapshot DTO and generated bindings |
| Full Crntly GUI against one broker | **not run** | environment/authorization blocker; do not claim it |
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

1. **Full Crntly GUI + Remotif GUI against one broker.** Targeted consumer
   crates and real process/auth paths passed, but no authorized current GPUI
   desktop launch was performed in this session. The Crntly protected checkout
   was not touched; work is in `C:\Users\User\Sites\crntly-local-ai-runtime`
   from `origin/feature/gpui-native-rust-studio`. The Remotif worktree is
   `C:\Users\User\Sites\tvforge-local-ai-runtime` from local `main`.
2. **GitHub PR creation for upstream reports.** The repository currently does
   not permit Actions to create/approve PRs. The workflow now runs report-only,
   uploads artifacts, and prints the exact minimum setting. Enabling PRs needs
   maintainer approval.
3. **Signing/publication/installation.** No signing credential, signature,
   registry publication, or live update endpoint was created. `lai-update` is
   fixture-tested; a real signed update was not installed.
4. **Cross-platform qualification.** Shared-runtime tests and the full CI
   verification passed on Windows, Ubuntu, and macOS in run `39736523716`.
   Performance numbers in this ledger are still Windows debug-build values; no
   release-performance or manual GUI qualification is claimed for Linux/macOS.
5. **Live accounts.** No live provider account or inference was used; provider
   support remains as documented in the support matrix.
6. **Per-app grant semantics in independent mode.** The reusable owner token is
   owner-scoped, not per-app. Hosts needing distinct app grants should use the
   parent/embedded broker. This is documented in the ADR and security review.
7. **Consumer UI release pins.** Remotif's managed helper remains available when
   no shared broker is configured. Crntly's managed component remains pinned to
   alpha.15; shared mode accepts protocol-compatible alpha.15+ runtimes. No
   package was published or installed.

## Rollout / rollback

1. Ship/install the new helper and control center together. Start with the
   managed stdio mode (default) or a parent-owned broker.
2. For shared mode, provision an owner-only token descriptor and start
   `serve --ipc ... --independent ...`; set the consumer's broker environment
   variable. Do not set it unless the broker is reachable; a configured but
   unreachable broker is reported, not replaced.
3. Keep the previous helper binary and SQLite state directory. On failure,
   unset the broker variable (consumers return to managed mode), stop the
   independent broker, and restore the previous executable. The SQLite store is
   forward-compatible only within the same schema; an older runtime refuses a
   newer schema instead of corrupting it.
4. For signed updates, use `lai-update` only with a host-pinned public key and
   drain active sessions before activation; rollback the exact backup if the
   new runtime fails its smoke checks.
