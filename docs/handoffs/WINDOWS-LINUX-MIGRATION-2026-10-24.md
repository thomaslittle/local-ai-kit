# Windows to Linux migration inventory — 2026-10-24

Preservation only. No new implementation, tests, inference, release or deployment. All original worktrees, branches and three stashes remain intact. Personal Windows directory prefixes below are placeholders; no credentials/runtime configuration are included.

[Machine-readable full inventory](WINDOWS-LINUX-MIGRATION-2026-10-24.json) includes each source path, original stash/base SHA, exact remote destination SHA, redactions and exclusions. This inventory is published on `checkpoint/windows-migration-20261024/inventory`. Its own commit SHA is reported by the publication receipt rather than embedded recursively in its own contents.

## All 15 original local branches

| Original local branch | Recovery remote branch | Exact local/recovery SHA |
|---|---|---|
| `chore/open-source-readiness` | `chore/open-source-readiness` | `f2cc34db2b7a90c29703749f8ce0c56c504e77d2` |
| `codex/chatgpt-managed-auth` | `codex/chatgpt-managed-auth` | `470dee4569c049777e21b58392964292808e077c` |
| `docs/full-api-reference` | `docs/full-api-reference` | `21e032600983b5bcf40c8d78dc1550623a1f4085` |
| `feat/cached-usage-host-bridge` | `feat/cached-usage-host-bridge` | `97053661461b991762d0606a1b5da1a01b1661ce` |
| `feat/cached-usage-mcp` | `feat/cached-usage-mcp` | `3c439561efb058658c8bc96666ca67d6bfde9074` |
| `feat/demo-react-vite` | `feat/demo-react-vite` | `da8a20ca7f893f32729e42f04d49b218f90798b6` |
| `feat/host-mcp-native-policy` | `feat/host-mcp-native-policy` | `55fa0542d9d016ff49f4fc6c9c855e74a3f64069` |
| `feat/shared-runtime` | `feat/shared-runtime` | `649b2a8db00d4d316b5c2489dc56ecef928dbd2a` |
| `feature/initial-implementation` | `checkpoint/windows-migration-20261024/initial-implementation-local` | `b0f2793489b223f6d7fd555f87bd0acd4da53a2e` |
| `feature/muse-provider` | `feature/muse-provider` | `8ec7123e19b16c35c0370a123d8170730517181d` |
| `fix/remotif-stdio-diagnostic` | `fix/remotif-stdio-diagnostic` | `e709645f5c7c4f2489a11ad10fa75b95f2bca535` |
| `fix/unix-executable-symlinks` | `fix/unix-executable-symlinks` | `5bd139d4f428ef8a41bb731315025d83c824ee89` |
| `grok-tokscale-upstream-discovery` | `checkpoint/windows-migration-20261024/grok-tokscale-base` | `22f4423d3bcdd1fdef214e40b6017df9fb6749d3` |
| `main` | `main` | `9a40e503c82e3efc15991382322ccf30bc00f806` |
| `merge/muse-into-main` | `merge/muse-into-main` | `dee22a7a605d6d0d6b9bb30aa1a10900c76cb748` |

Each listed local head equals its mapped remote head. The historical initial-implementation local branch is two commits ahead of its old same-name remote; that remote was not overwritten. The Grok/Tokscale local branch had no same-name remote. Both exact heads, plus the detached alpha10 head, now have dedicated checkpoint branches even though their commits were already reachable from remote main.

## All four registered worktrees

| Windows worktree alias | Local state | Recovery remote branch/SHA |
|---|---|---|
| `<WINDOWS_SITES>/local-ai-kit` | `main`; clean | `main` / `9a40e503c82e3efc15991382322ccf30bc00f806` |
| `<WINDOWS_HOME>/.codex/worktrees/chatgpt-managed-auth/crntly-desktop/tmp/timer-wire-kit-alpha10` | Detached; clean | `checkpoint/windows-migration-20261024/detached-alpha10` / `794584735152c16f10a9d256cea2d4b681a7cda9` |
| `<WINDOWS_HOME>/.t3/worktrees/local-ai-kit/docs-full-api-reference` | `feat/cached-usage-host-bridge`; clean | `feat/cached-usage-host-bridge` / `97053661461b991762d0606a1b5da1a01b1661ce` |
| `<WINDOWS_D>/remotif-cinematic-20261023/kit-stdout-worktree` | `fix/remotif-stdio-diagnostic`; clean | `fix/remotif-stdio-diagnostic` / `e709645f5c7c4f2489a11ad10fa75b95f2bca535` |

## Three stashes preserved as separate source checkpoints

| Original stash and SHA | Source snapshot remote branch | Exact remote SHA |
|---|---|---|
| `stash@{0}` / `0dcbdb8ca6e5f9a8f04b50e3dcf98e5097be2dea` | `checkpoint/windows-migration-20261024/stash-0-grok-freebuff` | `ef6d454a916c6dd775206b370b559e8965fcead6` |
| `stash@{1}` / `1c5761e7d86c3552ab1dc0bba192c7a2064f663c` | `checkpoint/windows-migration-20261024/stash-1-platform-contracts` | `a5a13e5180fa7533c496043a06d7f3e11fd3b480` |
| `stash@{2}` / `97c4266fe3fc59584b2302e74be4acb6a9c5103e` | `checkpoint/windows-migration-20261024/stash-2-offline-scaffolds` | `1179deeae19932a1d188a5f715ef37944eb7752d` |

The checkpoints preserve 45, 15 and 7 changed paths respectively, including relevant untracked source, lockfiles and canonical checked-in schema outputs. Each is labelled **UNQUALIFIED historical WIP** and has a checkpoint note. Nothing was applied/dropped, reformatted, regenerated or claimed tested. Raw stash merge/index objects are not published; use the source snapshot branch in Linux rather than expecting `git stash list` to recreate them.

Secret/private-data scan found no blocking credential files in the 67 changed source/document paths. One stashed document, `docs/upstream/tokscale.md`, has personal paths replaced with `<WINDOWS_OR_USER_LOCAL_PATH>`. Source fixtures/comments initially flagged by the path detector were unchanged already-public base lines. No other stashed source path was excluded.

## Stash index completeness proof

[Exact tree IDs and comparisons](WINDOWS-STASH-INDEX-COVERAGE-2026-10-24.json) were recorded in one bounded preservation audit. All three second-parent index trees exactly equal their respective base trees. Comparison includes path, mode, object type and blob ID, including staged deletions and executable-mode changes.

| Stash | Index/base tree (exactly equal) | Entries | Staged changes |
|---|---|---:|---:|
| `stash@{0}` | `c50de6cf80003386c00bf0aad92aa2ad59c94e09` | 562 | 0 |
| `stash@{1}` | `5709903c7f3eae79158ad20e1aaef35cfc9d9400` | 92 | 0 |
| `stash@{2}` | `03ce90857b705b33d385013b3253ced97cbc9c47` | 17 | 0 |

There are **zero uniquely staged source versions**, so no additional staged-source checkpoint refs are necessary. Every indexed entry is inherited from its already-recoverable base; the separate working/untracked source checkpoints retain the stashed changes. Raw index-object/stagedness omission therefore loses no distinct staged source bytes. Original stashes and worktrees remain untouched. No source content, credential/runtime data, feature tests or builds were added by this check.

## Exclusions and unresolved items

- `.verification/**`: Local-only raw receipts/logs, one-off already-applied transformation/recording helpers, superseded RED fixture and machine-specific guards; essential resume facts consolidated in committed handoffs.
- `target/**; dist/**; node_modules/**; .venv*/**; __pycache__/**`: Generated targets/packages/dependencies/caches; not source migration data.
- `retained-symbols/**; retained-incremental-20261022-2335/**`: Local retained build/debug caches; not uploaded.
- `actual OAuth/broker/provider credentials, private runtime databases and machine configuration`: Excluded; not read or provisioned as part of this operation.
- `raw stash merge/index/untracked objects`: Original stash plumbing remains local; only reviewed source snapshots published. Original stagedness is not reproduced..

The full inventory lists local-only ignored helper filenames explicitly. These are operational scratch/reproduction material, not unpublished product implementation; essential current handoff facts are in [the committed resume handoff](../implementation/REBOOT-HANDOFF-2026-10-24.md). Preserve Windows/local archives separately if forensic raw logs or PDB caches are wanted; they were not uploaded. Historical source checkpoints may conflict with newer implementation and must not be blindly merged. Actual account/connection mapping, a compatible cache host, persistent access and deployment remain unapproved.

## Linux recovery in a fresh clone

```sh
git clone --no-single-branch https://github.com/thomaslittle/local-ai-kit.git
cd local-ai-kit
git fetch origin
git show origin/checkpoint/windows-migration-20261024/inventory:docs/handoffs/WINDOWS-LINUX-MIGRATION-2026-10-24.json
# Continue the current source integration:
git switch --track -c resume-cached-usage origin/feat/cached-usage-host-bridge
# Or recover ONE historical line in a separate checkout/clean clone:
git switch --track -c recover-freebuff origin/checkpoint/windows-migration-20261024/stash-0-grok-freebuff
git diff --stat 22f4423d3bcdd1fdef214e40b6017df9fb6749d3..HEAD
# Other snapshots use the exact remote branch names in the tables above.
```

Do not run these switch commands over unrelated dirty work. Fetch and inspect before choosing a line; do not reset, drop stashes or merge unqualified historical WIP just to recreate the Windows directory layout. No Windows private runtime tokens/databases are needed to retrieve source.

## Ready-to-paste Claude launch prompt for tomorrow

```text
Continue Local AI Kit from its preserved Windows-to-Linux state. Read AGENTS.md, CONTRIBUTING.md, docs/implementation/REBOOT-HANDOFF-2026-10-24.md and the migration inventory on origin/checkpoint/windows-migration-20261024/inventory first. Verify current branch/status and exact remote SHA before writing. Do not restart completed release or consumer work.
The product outcome is cloud dot reading truthful per-account provider quota windows from one approved shared Kit cache. PR29 is API documentation, PR31 is Unix executable-symlink discovery, PR32 is cache-only MCP, and PR33 is the local producer/T3 source binding stacked on PR32. Reuse these lines and the independent PR30 stdout fix; do not duplicate them or silently merge unrelated branches.
Source integration is implemented and fixture/platform-tested, but no actual cache host/account mapping, T3 registration or deployment is approved. First reconcile the source and choose the smallest compatible cache-host arrangement. Present exact account/connection scope, operator-confirmed consumer mapping, T3 environment/external MCP client binding, feed/interest policy and build/deployment/restart actions for approval. Existing orchestration permission does not grant account-data access. Local stdio is not cloud connectivity.
Do not read credentials, infer account mapping from names/models, create persistent grants/listeners/tunnels, start/restart a shared service or deploy without the required approval. Tool reads must add zero inference/probe/refresh/RPC/polling work; reuse one cache/subscription and preserve unknown/stale/unsupported evidence, reset observations and distinct account/route/pool identity. Context counters are not plan quota.
Use one worker and the owner-selected model/budget; no costlier fallback, broad validation loops, cleanup or build campaign. The first useful deliverable is a concrete approved path to one real cached cloud read with honest provenance/freshness and zero new provider work. If approval/environment is missing, make only independently authorized source/docs progress and state the exact blocker. Historical stash checkpoints are preservation snapshots, not product-ready branches. Preserve user apps/data and stop at the requested checkpoint.
```

No Claude thread is started tonight. Remotif owns its separate comprehensive Claude/UI handoff; this inventory does not certify another project's migration.
