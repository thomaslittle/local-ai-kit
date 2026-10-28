# Linux installation pause — 2026-10-24

Owner requested preservation and safe pause, not implementation continuation.

- Current branch: `feat/direct-cached-usage-remote`, PR34. Product implementation is `0ed1649474e2e5107d509f3caf6e2eba71ebfd37`; incoming owner guidance `6fb4954a39abf4580b5886f40ce3514984006721` was preserved by fast-forward.
- PR29 API docs, PR31 Unix discovery, PR32 cached usage and PR33 producer integration remain separate, pushed branches. No merge or release is authorized by this pause.
- PR34 is source-only. Direct dot usage is NOT connected. Hosting selection, exact account mapping, cache access, principal/credential, deployment and private plugin registration still require explicit decisions/approvals. Do not activate from this handoff.
- Last bounded cache read at 10:22:02 UTC reported Codex weekly 28% remaining, observation 10:17:08.144 UTC marked stale; reset October 30 at 20:08:42 UTC. Five-hour allowance absent. This is historical, not current quota or verified T3 billing mapping.
- Resume only when Thomas explicitly asks. First reconcile current remote changes and outstanding PR feedback; do not automatically build, test, refresh providers or resolve the hosting question.

Migration recovery inventory is on `checkpoint/windows-migration-20261024/inventory` at `0629c2ab658e96882d015326b09b855d056890c3`, under `docs/handoffs/`. It maps historical branches and all three sanitized stash snapshots. Stash index coverage found no unique staged versions. Original stashes remain intact.

At pause inspection all four registered worktrees had clean normal Git status. No owned build/test/helper process matched this task's workspace/staging paths; no GUI/device lease or pending clicks are held. PR watches were all disabled. User applications and the Remotif-owned running Kit helper were left untouched.

Ignored verification logs, machine-specific scripts, runtime databases/credentials, generated caches and retained cleanup archives remain local-only; they must not be uploaded as source. Essential source/config templates, inert-check receipt and setup limits are committed in PR34. Historical local branches with differing/missing same-name remotes are preserved by named checkpoint refs, not force-pushed over existing history.
