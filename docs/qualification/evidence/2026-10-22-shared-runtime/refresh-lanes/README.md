# Refresh-lane release-readiness evidence - 2026-10-22

The original interrupted audit below is historical. Sol continued the preserved
partial cache/metadata edits and implemented the scheduler. Behavioral tests,
mutation failures and restorations are retained under `sol-20261022/`. Round 5 independent review approves the fixture behavior; exact-source CI,
clean artifacts and release qualification remain pending.

Run:

```powershell
python docs\qualification\evidence\2026-10-22-shared-runtime\refresh-lanes\run_refresh_lane_audit.py
```

`audit-output.txt` retains the failing result (exit 1).

| Requirement | Current implementation | Verdict |
|---|---|---|
| Scheduled inventory reconciliation | Only explicit `discovery.refresh`, startup warm scan, or caller-triggered discovery updates the inventory. `NativeService::start_background_tasks` contains one fixed 5-second loop that calls `refresh_due_metadata` only (`lib.rs:108-112`). | **missing** |
| Separately paced usage/reset lane | Usage/reset fields are carried inside metadata/catalog reports. There is no separate lane, cadence, or `usage_due` scheduler. ADR D4 explicitly says usage remains part of metadata and "a timer never declares a reset" (`ADR-001-shared-runtime.md:90`). | **missing** |
| Client-interest cadence | The background loop runs whenever `enable_autonomous_refresh` is configured; no subscriber/interest count adjusts cadence or stops work when clients disappear. The only event subscription apply is transport-level `events.read`. | **missing** |

Existing coverage is narrower: `runtime_sync::invalidation_is_debounced_then_the_autonomous_lane_rechecks_it`
proves metadata invalidation/re-read, not inventory scheduling, separate usage
pacing, or client-interest cadence. The service-level `discovery.watch` sets a
filesystem metadata watcher; it does not schedule inventory reconciliation.

Ruling: the original refresh requirements are not implemented elsewhere. The
release is held until the lanes are implemented and behavior-tested with
bounded cadence, or the owner explicitly accepts an alpha limitation. A green
GUI/reconnect suite does not cover this system-level requirement.


## Sol continuation: implementation and behavioral coverage

`run_refresh_lane_audit.py` now runs the actual Rust service tests. The original
source-only script is retained as `legacy_source_audit.py`; its old missing-lane
failure is `audit-output.txt`. Compiler/setup failures and fixture timing
corrections are retained alongside assertion failures. They are not all product
failures. No prior failed artifact was replaced on retry.

The supervisor in `crates/lai-service/src/refresh.rs` has separate inventory,
metadata and usage admission, rotating cursors, alternating lane priority,
64-item pages, two shared probe slots, and one coalesced inventory flight.
Active read/activity interest lasts 120 seconds; disconnect removes it.

| Lane | Active cadence | Idle cadence | Authority |
|---|---|---|---|
| Passive inventory | 5 minutes | 30 minutes | Separate host opt-in; CLI `--allow-passive-discovery` |
| Full metadata | 15 minutes | 60 minutes | Existing host provider metadata grant |
| Supported usage | 60 seconds | 5 minutes | Provider reports windows/included-usage evidence |
| Targeted invalidation | Next 5-second admission tick | Same | Debounced watcher; existing metadata grant and shared backoff |

Usage uses the existing profile-bound metadata operation where that is the only
provider interface; a successful full report can satisfy both logical lanes.
Unknown/unsupported usage does not create a fast polling loop. A provider reset
is converted once to a monotonic recheck deadline; timers only mark readings
stale and request authoritative data. They never lower usage or declare reset.
Sleep resumes with missed ticks skipped and one bounded batch.

Coalescing boundary: Codex (`codex/usage.rs`), Claude (`claude/usage.rs`) and Muse
(`muse/catalog.rs`) currently emit connection-scoped pool IDs with
`sharing: "unknown"`. Their native operations also return profile-specific auth
and catalogs. No verified cross-profile quota equivalence or independent usage
operation exists in these adapters. The safe shared execution scope is therefore
one provider/profile/connection, including all its verified routes/pools and all
interested apps. The duplicate-route test exercises one reported account with
two routes to one verified pool; a separate account with the same display email
still performs its own probe. Cross-profile equivalence is not guessed from an
email, display label, model ID, or a similar reset window. A future adapter that
reports verified cross-connection scope requires an explicit scoped operation;
this checkpoint does not claim that unsupported provider behavior.

Tests include a healthy source beyond 68 continuously failing prefix sources,
usage while that metadata backlog remains, atomic late-error rejection after
scan/invalidation, forward/backward ten-year wall-clock jumps, two-hour sleep,
expiry journal events, stale quota warnings, inventory Retry-After/recovery,
scoped events, host grant withdrawal, immediate stop/restart before the old
source unwinds, cancellation, and a foreground joiner's shared scan ownership.

`20-mutations.json` records three real isolated faults: failure-generation guard,
queue rotation and restart progress. Each fails a behavioral assertion (101),
is restored byte-for-byte with SHA-256 verification, and passes again (0).
The first mutation setup's pattern mismatch occurred before any edit/test; it
is disclosed in `20-mutation-setup-note.txt`.

The original cache/metadata files and dirty patch were preserved locally in
`.verification/refresh-lanes-sol-20261022/`; full source copies are excluded from
published evidence. All sources used in tests are synthetic. No live account,
credential, inference, notification, OS input or GUI test was used.

Shared GUI coordination: `C:/Users/User/.codex/gui-test-locks/crntly-desktop.lock`
is an atomically created exclusive directory with owner/thread/PID/time/evidence
metadata. An existing lock fails closed; only its owner releases it after child
GUI processes exit. Kit held no GUI lock during this continuation; Remotif and
the separate Crntly parity thread were informed. Release and consumer branch
boundaries remain in the main qualification ledger.

## Independent review and final candidate checks

Five read-only review rounds are tracked in `sol-20261022/review-rounds.json`.
Rounds 1–3 found scoped journal, generation, fairness, stale warning and scan
atomicity/cancellation defects; failure-first regressions and fixes are retained.
Round 4 accepted the runtime fixes but retained another nondeterministic restart
test failure. Earlier passing repeats do not supersede it. The test now lives
in the private cache unit suite and waits for publication/flight retirement
before advancing the fake clock. Round 5 approved its ordering and ran ten
combined default suites: 25 library + 20 integration tests each, 450 executions,
zero failures or retries. See `sol-20261022/review5/` for hashes and logs.
Production code is unchanged from round 4.

Alpha18 preparation updates the Rust/npm packages together and corrects the
previously lagging Python wheel version from `0.1.0a11` to `0.1.0a18`.
The initial full check48 stopped because the local build venv lacked Playwright;
installed Vite also differed from the lockfile. Pinned Python requirements and
`npm ci --ignore-scripts` corrected the local environment. The offline npm attempt50
could not find one cached archive; the normal public-registry attempt51 passed.
A later full run54 hit a Windows sharing violation in an unchanged temporary
workspace rename. Focused rerun55 passed; the responsible handle was not captured.
No product failure is inferred from that environmental error. Every attempt is
retained; a failed full profile is not reported as passing.

The offline Gitleaks positive control45 detects a synthetic token. Default source
scan 46 reports three matches: an existing synthetic OAuth test RSA key introduced
by `4bfae8f`, and two generated TypeScript type declarations. Dispositions record
source hashes and inspection reasons; the scan exit remains 1 and no ignore rule
was added. No signing key was generated or used, no token was tested online, and
raw secret-scan reports remain ignored. This is a bounded repository check, not
proof that every artifact or historical commit is secret-free.

The final full pinned-dependency profile56 passes all 24 commands, including
638 Rust tests, SDK/generation/native bridge/packaging checks, headless demos,
34 accessibility checks and React Doctor with no issues. Format52 and strict
all-target/all-feature Clippy53 pass. Optimized compilation57 passes. C#58 first
found no installed SDK; Microsoft .NET8 SDK8.0.426 was checksum-verified and
extracted only under ignored workspace tools59, with no global install/PATH or
security setting change. C#61 passes using an empty NuGet package-source config.
The parent accidentally overlapped build60 with that check's running Windows
executable; the retained file-lock failure predates tests and is rerun serially
in62. Source hashes in `final-source-sha256.json` match the accepted R5 source.
No later source change is implied by documentation/evidence updates.

The serialized all-target/all-feature workspace gate 62 passes 638 tests, zero
failed/ignored. This final result does not overwrite the failed build60.

Final source scan 64 includes the added review/hash evidence and reports 15
matches: the previous 3 plus 12 source SHA-256 values in auth-named file entries.
Each of those 12 values was recomputed from the corresponding reviewed source
and matched exactly. See `64-gitleaks-final-dispositions.json`; unresolved
findings: 0, scanner exit: 1, no suppression added. The copied corpus contains 833
files/5,868,356bytes; its file-hash manifest digest is retained in summary 64.
