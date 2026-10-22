# Shared runtime implementation plan

## October 22 original section 5 remediation

- [x] Preserve partial files and recover original requirements/branch boundaries.
- [x] Implement separate host-authorized inventory and metadata/usage scheduling.
- [x] Test fair admission past 64 failing sources, active/idle cadence, reset and
  wall-clock/sleep behavior, coalescing boundaries, backoff and cancellation.
- [x] Reproduce and repair initial independent-review findings in scoped events,
  atomic failures, quota-warning freshness, fairness and restart progress.
- [x] Complete fresh independent review and full local profile/strict lint.
  C# and all-feature executions pass too. Exact-head CI and artifacts are tracked
  in the qualification evidence; local success alone does not lift the release hold.
- [ ] Commit/push the qualified candidate; verify exact-head remote checks.
- [ ] Prepare alpha.18 under release conventions; qualify Windows artifacts and
  provenance/clean-consumer checks before merge/publication.
- [ ] Update released helper consumers and integrate Remotif into main, Crntly
  into feature/gpui-native-rust-studio only. Preserve all existing GUI receipts.

- **Started:** 2026-10-22
- **Baseline:** `556e35e` (`main`, before this change)
- **Decision record:** [ADR-001](../design/ADR-001-shared-runtime.md)
- **Acceptance ledger:** [2026-10-22-shared-runtime](../qualification/2026-10-22-shared-runtime.md)

This plan is a status tracker, unlike the historical
[implementation-plan](../design/implementation-plan.md). Status words are used
in the repository's strict sense: **planned**, **implemented**,
**fixture-tested**, **platform-executed**, **live-qualified**.

## Phase 0 — preserve the baseline

- [x] Read repository instructions, architecture, contracts, handoffs, support
  matrix, upstream tracking, SDKs, broker, cache, tests, and release scripts.
- [x] Record that Remotif's working checkout is `C:\Users\User\Sites\tvforge`
  with unrelated uncommitted landing/design changes; work on a new worktree
  from local `main` (`feat/shared-local-ai-runtime`).
- [x] Record that Crntly's protected checkout is
  `C:\Users\User\Sites\crntly-desktop`; work on a new worktree from
  `origin/feature/gpui-native-rust-studio` (`feat/shared-local-ai-runtime`).
- [x] Do not reset, force-push, merge into `main`, or overwrite unrelated work.

## Phase 1 — contracts and durable state

- [x] Additive `Snapshot.cursor`, `health`, `issues`; account scopes; typed
  issue/health/notification/remediation records; inventory change/page types;
  `apps.list`/`apps.revoke`; `STORAGE_UNAVAILABLE`.
- [x] `lai-store`: SQLite WAL, schema version 1, atomic snapshot + metadata +
  journal commit, bounded journal, epoch cursors, issue and delivery state,
  sanitized identity persistence, corruption preservation, read-only
  diagnostics, future-schema fail-closed.
- [x] Regenerate Rust-authored schema/SDK artifacts and check drift.
- [x] Store tests: replay, gap, bounded journal, corruption backup, future
  schema, read-only, identity redaction, notification dedupe.

## Phase 2 — service synchronization

- [x] Hydrate the ready snapshot and issues before accepting clients, labeled
  stale; do not bump a revision merely because the process restarted.
- [x] Metadata publication advances the revision and writes an inventory event.
- [x] Invalidation publishes stale state and schedules an autonomous probe.
- [x] Debounced/coalesced filesystem invalidation; bounded autonomous lane with
  host-authorized providers and backoff.
- [x] Snapshot cursor + inventory event replay + gap detection + epoch.
- [x] Out-of-order metadata publication fails with `SELECTION_STALE`.
- [x] Health derivation, snooze/acknowledge, issue projection, notification
  coordinator, quiet hours/privacy/reminder dedupe.
- [x] Runtime health exposes revision, cursor, hydration, storage, and lane
  status.

## Phase 3 — shared lifecycle and transports

- [x] Parent-owned lifetime remains the default.
- [x] Independent lifetime, reusable owner token file, idle timeout, conflict
  on a live endpoint, no implicit takeover.
- [x] On-demand `openSharedBrokerTransport` in the TypeScript SDK.
- [x] SDK inventory subscription and live snapshot store; Python inventory
  subscription; notification/app families.
- [x] Admin-only application listing/revocation; revocation affects active
  calls and sweeps owned work.
- [x] Process-level independent-broker test (disconnect survives, duplicate
  converges, idle releases endpoint/token).

## Phase 4 — control center

- [x] Native Rust TUI (`lai-control`) over the public authenticated API.
- [x] Overview, accounts, models, usage, issues, apps, diagnostics.
- [x] Refresh, probe, acknowledge, snooze, revoke, redacted export preview.
- [x] Public-API-only E2E test with a real broker and token file.

## Phase 5 — upstream and release pipeline

- [x] T3 watcher watches shell resolution paths, classifies provider vs
  orchestration noise, detects missing covered paths and unwatched candidates,
  reports structural changes, and separates report-only from PR creation.
- [x] Tokscale watcher has captured errors, durable report artifacts, explicit
  permission-denied remediation, and report-only mode.
- [x] Workflow preflight reports the minimum repository setting; reports are
  always uploaded; PR creation is attempted only when permitted.
- [x] `lai-update`: Ed25519 verification over canonical JSON, platform/protocol/
  version/checksum checks, staging, session lease, atomic activation, rollback.
- [x] Release build emits an unsigned update manifest and `latest.json` pointer;
  signing/publication remain approval gates.

## Phase 6 — consumer integration

- [x] Remotif: `remotif-kit` shared-broker mode via the Kit `connect` bridge,
  health notices, explicit no-silent-fallback errors, cross-process test.
- [x] Crntly: Tauri helper shared-broker mode via `connect`, owner token file,
  protocol-compatible shared version check (managed mode remains exactly
  pinned), health/issue data available through the existing snapshot contract.
- [x] Targeted Crntly AI tests and Rust check.
- [ ] Launch both full GUI apps against one broker and capture one GUI-input
  run. Blocked by the environment: no authorized current Crntly GPUI build
  target is qualified in this session; document exact blocker rather than
  claim it.

## Phase 7 — qualification and handoff

- [x] Failure-oriented tests and deliberate fault checks.
- [x] Performance/measurement script; values recorded in the ledger after the
  release/debug build used for the run.
- [x] Security review notes: owner file/pipe boundaries, secret redaction,
  no token export, permission filtering on errors, update trust.
- [x] Rollout/rollback instructions.
- [x] Exact SHAs, commands, remaining blockers.
