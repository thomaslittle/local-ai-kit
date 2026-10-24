# Reboot resume handoff — Local AI Kit

This document preserves the essential state previously held in ignored local
handoffs. It contains no credentials, live account IDs, private endpoints or build
output. This preservation commit changes documentation only; do not interpret it
as a new implementation qualification or a deployment authorization.

## Branches and source checkpoints

The linked worktree is named `docs-full-api-reference`, but its actual current
branch is **`feat/cached-usage-host-bridge`**. Inspect Git rather than inferring
the branch from that directory or an old thread binding.

| Branch | Implementation checkpoint | Pull request |
|---|---|---|
| `docs/full-api-reference` | `21e032600983b5bcf40c8d78dc1550623a1f4085` | [PR29](https://github.com/thomaslittle/local-ai-kit/pull/29) |
| `fix/unix-executable-symlinks` | `5bd139d4f428ef8a41bb731315025d83c824ee89` | [PR31](https://github.com/thomaslittle/local-ai-kit/pull/31) |
| `feat/cached-usage-mcp` | `3c439561efb058658c8bc96666ca67d6bfde9074` | [PR32](https://github.com/thomaslittle/local-ai-kit/pull/32) |
| `feat/cached-usage-host-bridge` | `8355636855cea7e63e5e980f33e8e80fb36d3217`, followed by this preservation document | [PR33](https://github.com/thomaslittle/local-ai-kit/pull/33) |

All four checkpoint branch heads matched `git ls-remote` before this document was
added. Verify/push this documentation commit on the existing host-bridge branch;
PR33 remains stacked on `feat/cached-usage-mcp`. Never force-push, merge, release,
delete branches, or replace a shared helper as part of preservation.

## What exists and what was executed

- PR29 adds the human-readable API reference, generated method/schema details,
  synthetic examples, safe quickstart and drift checks. The bounded generation
  review passed 27 tests. It does not change canonical runtime behavior or close
  documented contract gaps. Its own branch retains the full documentation work.
- PR31 adds bounded Unix executable-symlink discovery and launch identity checks,
  preserving logical command and resolved executable identity. Credential/config
  symlink policy remains strict. A focused Linux/WSL2 synthetic discovery-to-probe
  journey passed; Ubuntu, Windows and macOS hosted checks passed on its checkpoint.
  A real user's standalone Codex account was not live-qualified. No released or
  shared helper was replaced.
- PR32 adds cache-only `kit_usage_cached`, explicit account/connection scope,
  operator mappings and conservative freshness. It also fixes SDK cache disposal
  and terminal-denial retirement. The refresh-fixture CI failure was reproduced
  with a delayed blocking source; the existing fixture now waits for publication
  before advancing paused time. No production scheduler/deadline changed.
  [Fixture evidence](../qualification/evidence/2026-10-23-cached-usage-mcp/README.md).
  All three platform jobs passed on `3c43956` in
  [run 40061359346](https://github.com/thomaslittle/local-ai-kit/actions/runs/40061359346).
- PR33 adds the inert in-process producer and explicit T3 environment/external
  MCP client-session binding, reusing PR32's projection and canonical quota types.
  It observes one existing SDK store locally; tool reads do not read that store,
  call a broker, renew interest, refresh a provider or perform inference.
  Stop/dispose/revocation clear access, and late callbacks cannot revive rows.
  [Implementation/evidence](../qualification/evidence/2026-10-24-cached-usage-host/README.md).
  Two focused journeys and the native Effect five-call in-memory demonstration
  passed. Ubuntu, Windows and macOS hosted checks passed on `8355636` in
  [run 40064116729](https://github.com/thomaslittle/local-ai-kit/actions/runs/40064116729).

The preservation commit comes after these exact-source CI checkpoints. Any checks
triggered by its push are new, pending evidence until their actual result is read;
do not report the older checks as checks on the preservation commit.

## Actual usage connection remains unconfigured

The complete source composition is in the
[T3 toolkit guide](../../examples/mcp/t3code/README.md) and the
[scope/interest/setup contract](../integration/cached-usage-mcp.md).
T3 source was inspected read-only at
`5bb54a0f77c6655fdb2ab551612ca1ca1fcd3667`; its checkout was unchanged.
The native Effect module was type-checked and exercised in memory against Effect
4.0.5, with zero broker/provider requests and zero listeners. T3's complete
HTTP/OAuth/layer composition and an actual cloud dot call remain unexecuted.

No approved compatible producer host is selected. T3 has no Kit SnapshotStore
service at the inspected source. A Rust app's private cache is not automatically
available to Node. A local stdio endpoint is not reachable by cloud dot.

Before activation, the operator must identify and approve:

1. The cache owner and feed policy. Reuse an existing approved store; a new
   T3-owned broker feed needs explicit single-feed/interest-policy approval.
   A separate process's cache requires an approved data channel.
2. Exact Kit account/connection scope and the operator-verified consumer/billing
   account mapping. Never infer mapping from email, provider or model names.
3. Destination T3 environment and the exact external MCP client session. Existing
   orchestration permission is not account-usage consent; thread credentials are
   denied by the binding. New OAuth sessions require deliberate rebinding.
4. Candidate build/install and host source composition/deployment/restart, any
   necessary persistent access or new feed, then one approved cached cloud read.

No credential, grant, persistent connection, endpoint/tunnel, service restart,
deployment, merge or release was performed by these source slices. Provider
allowance remains unknown; context capacity is not subscription quota.

## Preservation boundaries and local-only exceptions

Before this documentation edit, `git status --short --untracked-files=all` was
empty in this worktree and the original Kit main checkout (`9a40e50`). There is no
unfinished product-source WIP. No active owned child agent or child worktree is
needed to resume this slice. Old delegated reviews are completed/read-only work.

Other registered Kit worktrees belong to separate lanes: PR30's stdout fix and an
older detached managed-auth checkout. Both were clean at the preservation audit;
do not move, reset, clean or push them on this lane's behalf. PR30's branch was
independently verified at remote/local `e709645f5c7c4f2489a11ad10fa75b95f2bca535`.
The detached checkout has no branch publication claim from this lane.

The ignored `.verification` directory remains **local-only**. It includes raw
logs, machine-specific guard/staging scripts, superseded controlled-RED fixture
backups and historical pause/handoff receipts. Essential resume facts are now in
this document; committed qualification receipts retain their source/log hashes.
Useful local filenames are `OWNER-PAUSE-20261024.txt`, `RESUME-PR32-20261024.txt`,
`CACHED-USAGE-HOST-20261024.txt`, `pr32-final-ci-20261024.json`, and
`pr33-final-ci-20261024.json`. Older handoffs may still say CI is pending; use the
exact-head final receipts and source checkpoints above.

Generated D-volume targets/staging, raw logs, the retained PDB/incremental cleanup
archives and original-root verification records also remain local-only. Preserve
them through reboot; never upload them or treat unrelated directories as cache.
The host source demonstration stage was 95,738 bytes under its 2 MiB bound.

The earlier unsigned alpha.18 release and consumer integrations are completed
historical work. Do not restart their builds or qualification. Remotif and Haiwire
have separate owners; their current handoffs/publication must be verified in their
own lanes. Never edit/build/reset the protected interactive Crntly checkout.

## Next action

Read current Git status, branch/remote equality and the selected PR's actual
checks. Keep the same Sol/budget settings and avoid new workers, broad test
campaigns or live inference. No product implementation is pending locally.
Continue live-connection work only after the specific selection/approval gates
above are resolved. Do not enable a PR watch or automatic restart for reboot.
