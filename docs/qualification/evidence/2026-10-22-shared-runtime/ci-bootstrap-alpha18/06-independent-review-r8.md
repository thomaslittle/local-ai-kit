# Round 8 independent review

Task `kit-alpha18-bootstrap-fixture-review-20261022-r8`, Codex gpt-6.1-sol,
xhigh reasoning, default service tier. Read-only scope: the sole native SDK
test diff, relevant CLI/service/SDK paths, supplied receipts and fixture tests.
No edits, builds, native execution or independent test runs.

Verdict: accept; zero Critical, Important or Minor findings.

- `connect(home)` explicitly launches `--allow-passive-discovery`. CLI enables
  autonomous inventory and starts background tasks before serving requests.
  Either initial cache snapshot is valid, separate from SDK import semantics.
- `snapshot.get` reads the projected cache and renews an activity lease that
  changes cadence. It cannot discover or grant metadata permission.
- Initial auth unknown, empty models/accounts and sentinel exclusion strengthen
  existing security checks. Explicit refresh, credential equality, concurrency,
  revocation and default denial remain tested.
- File-content equality and sentinel absence cannot prove the file was never
  read. This inherited limit is covered separately by zero-call service fixtures.

The parent bridge receipt reports Node 5/5 and Python 4/4 passing. Reviewed test
SHA-256: `a35d227657f8de0b50d6d9a9def2d95b715c0a24b0cef8231aa776939ef0251b`.
Final-source CI/artifact qualification and release hold remain parent-owned.
This record transcribes the independent review delivered to the parent.
