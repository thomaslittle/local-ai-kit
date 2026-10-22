# Alpha18 release security and build-budget follow-up

The refresh candidate `cf34b3dc8c9daf9a9684da32c9d49b5ca8871a4e` passed
the full Windows profile (638 Rust tests), all-feature tests, strict lint,
independent refresh review and clean release packaging. Its unsigned Windows
ZIP SHA-256 is `154e141a2ddf0290865f6d43129a27710c2ae7d90e5f3ff3ef6ba80059c0eae2`;
the standalone executable is
`3b8edd076547af65a53f4b128349bd5391ecc34ad2cf4e575d219134c7c1b660`.
Clean native, npm, Python and React SSR consumers passed. These receipts identify
that candidate; they do not qualify the later changes below. Release remains held.

## Observed dependency advisories

The push surfaced four default-branch Dependabot alerts, retained in
`00-dependabot-alerts.json`: two entries for the same JWT advisory, plus two
CSS build dependencies. Authoritative references:

- [JWT claim type validation](https://github.com/advisories/GHSA-h395-gr6q-cpjc):
  patched in jsonwebtoken 10.3.0.
- [Indexed source-map offsets](https://github.com/advisories/GHSA-68fv-2mgg-jv7q):
  patched in source-map-js 1.2.2.
- [Selector parser complexity](https://github.com/advisories/GHSA-rj75-hqrm-r3gf):
  patched in postcss-selector-parser 7.1.6. These parsers are build dependencies;
  the released helper does not parse untrusted CSS/source maps through them.

The existing signed-ID fixture was extended with absent, null, Boolean and
incorrect numeric standard claims. It failed on a missing issuer in `01`,
before any dependency change. Explicitly requiring issuer, audience, expiry and
subject passes all 21 auth tests on the old dependency in `02`. The flow still
checks its supported signature/key algorithms, configured issuer/audience,
expiry, subject and nonce. No OAuth scope, enablement or grant changed.

The candidate moves to jsonwebtoken 10.3.0 with the AWS-LC backend. Windows uses
the upstream `prebuilt-nasm` feature as a fallback when NASM is absent. The
[upstream Windows requirements](https://aws.github.io/aws-lc-rs/requirements/windows.html)
describe these checked-in objects and their source comparison checks. The
first build without that feature failed in `06`; it is retained. The subsequent
check passed, but the auth-test rebuild hit disk exhaustion in `08`, so that
attempt is not a test pass. A stale PowerShell exit value did not reflect the
logged compiler failure; qualification uses the actual logs and guarded exit.

Root npm overrides select the two patched CSS packages. The first lock update
left the old selector parser in place; audit `09` retained that finding. Targeted
lock regeneration `10` selects 7.1.6. Fresh installation `12` succeeds and final
audit `13` reports zero vulnerabilities. Independent round 6 approves the claim,
dependency and native-path changes with no Critical or Important findings.
Its single guarded Windows auth run passes 21 tests; `16-review6-*` retains the
log, source hashes, dependency provenance and 0.448 GiB peak-output receipt.
This is fixture execution, with full/UI and exact-source artifact gates separate.

## Retry-After fixture delivery

The first D: full profile `14` failed one of 638 native tests: the inventory
Retry-After fixture counted a third scan. The passing diagnostic `15` does not
erase that failure. A deliberately delayed blocking source in `18` reproduces
the same assertion and shows a committed `Timeout` while its `RateLimited`
result is still being delivered. Advancing fake time had tested the five-second
worker deadline instead of the intended provider backoff.

The fixture now holds virtual time until its public snapshot contains the
authoritative rate-limit issue, then tests background cadence and explicit
refresh against that installed failure. The initial synchronization attempt
`19` omitted Tokio's rounded initial tick and failed before launching the scan;
`20` adds only that one-millisecond tick and passes 25 library plus 20 refresh
tests. The intentional 50 ms source delay remains to exercise this boundary.
Production refresh/cache sources are unchanged from round 5. Full rerun `21`
passes all 24 commands: 638 Rust tests, 28 selectors, 68 TypeScript SDK, 27 Python
SDK, 13 React, generation/packaging/native integration, 16 Python and 16 Node
browser checks, 34 accessibility checks, and React Doctor without issues.
Its guard records 5.265 GiB peak active output and no pressure stop.
Round 7 approves the ordering by independent read-only inspection, as recorded
in `30`. The real mutation `28` replaces shared inventory Retry-After with a
one-second delay; the background assertion fails with exit 101. Source is then
restored byte-for-byte to the reviewed hash. Ten consecutive default combined
suites in `29` then pass 25 library plus 20 integration tests each: 450
executions, zero failures and no retries. The reviewed test and production
cache hashes remain unchanged. Formatting `22`, strict all-target/all-feature
Clippy `23`, all-feature tests `24` (638 passed, zero failed/ignored), all eight
adapter conformance groups `25` and the real native C# HTTP consumer `26` pass.
`27` and `35` summarize executed checks; `32` identifies qualified file bytes.
Clean-source artifacts and latest-source remote checks remain release gates.

The final offline default-rule source scan `34` covers 902 files / 6,533,312
bytes. Its nonzero exit is retained: 18 inspected matches comprise the existing
synthetic RSA fixture, two generated type declarations and 15 independently
verified checksums. Previously inspected matched files remain byte-identical;
new auth-source/log checksums were recomputed. Dispositions record zero unresolved
findings. No ignore rule, suppression, online validation or signing key was added.

## Bounded build ownership

Only the kit's generated PDB files were relocated: 664 files,
54,956,384,256 bytes (51.182 GiB), from this checkout's `target/debug/deps`
to `D:/lai-task-7a4b270a-20261022/retained-symbols`. Every file is retained;
none was deleted. Source, worktrees, evidence, credentials, live processes and
other agents' targets were untouched. The bulk deletion attempt was rejected
by automatic approval review as `blocked by policy`; relocation preserved data.

The unique D: task root has an ownership receipt. Its fixed retained-symbol
archive has a 54 GiB budget; active target plus temporary output has a 12 GiB
budget. The guard serializes invocations, uses two Cargo jobs, disables debug
symbols/incremental output per process, and stops only its captured process tree
at an 11 GiB threshold (1 GiB headroom), C: free below 16 GiB, or D: free below
64 GiB. One-second observations and before/after receipts record actual usage.
No global environment, disk quota, security setting or protected checkout changed.

The completed local gates peak at 8,622,620,476 bytes (8.03 GiB), with no pressure
stop. `31` verifies every relocated PDB destination's name and size against the
original receipt, and records current C:/D: space plus the preserved, frozen C:
target baseline. `33` retains all guard receipts, including failing attempts.

The first guarded full profile peaked at 2.892 GiB in its active target but
inherited the machine's Cargo `rustc-wrapper = "kache"`, which can generate
shared cache output outside that target. The guard now explicitly clears both
Rust wrapper variables per process. No shared cache entries were deleted or
claimed as task-owned recovery. The earlier receipt describes only its measured
target/temp scope, not a bound on that inherited wrapper's cache.

The active Remotif root, Afterlight and Crntly parity lanes were notified before
the D: rebuild. Their target directories remain independent. CI also disables
dev/test symbols so serial debug/all-feature/release steps do not accumulate
large PDB variants. Two native test harnesses now accept the existing
`LAI_NATIVE_BINARY` override, ensuring relocated-target checks exercise the
new executable rather than a prior executable in the default target directory.

All sources are synthetic or owned build tooling. No live provider account,
paid inference, OS input/audio, notification or signing credential was used.
