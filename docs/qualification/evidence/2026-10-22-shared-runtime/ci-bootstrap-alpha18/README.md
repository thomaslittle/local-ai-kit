# Native SDK bootstrap ordering — alpha18

At `aeecd15f9a5a909452c11da0ae396c08cc2799c0`, Ubuntu verification and React
Doctor passed. macOS job `120092756667`, run `39855565654`, executed and failed
the Node native SDK assertion that its first snapshot had no connections.
The host had explicitly authorized passive autonomous inventory, which already
published a configuration-only Codex connection with unknown auth. Windows was
still running at that readiness snapshot; no later result is inferred here.

`00` retains the failed job's metadata and relevant log excerpt; the complete
raw log remains ignored with its SHA-256 recorded. This is an actual test failure,
not an account-level runner limitation.

A diagnostic 100 ms delay before the old assertion reproduces the same failure
on Windows (`01`). The corrected assertion accepts either pre/post inventory
publication, requiring live mode, unknown auth, empty models/accounts and no
credential sentinel. It keeps explicit refresh, unchanged credential content,
24 concurrent cached reads, revocation and default-denied behavior. The delayed
variant passes (`02`); the delay is removed from the final tree, and the full
native bridge passes Node 5/5 plus Python 4/4 (`03`). No sleep, retry, product
schedule or permission change remains in the diff.

R8 independently accepts the bounded test-only diff with no Critical, Important
or Minor finding (`06`). It verifies that host authorization precedes serving,
while importing the SDK and `snapshot.get` cannot authorize or initiate a scan.
Credential content equality cannot prove the file was never read; separate
service fixtures count zero discovery/metadata calls without permission.

Production and R6/R7 hashes remain unchanged (`05`). The clean `aeecd15` artifact
passed manifest/license/SBOM/native/npm/Python/React checks and all enforced
60-second benchmark targets; its receipts are preserved as `04`. Its ZIP hash
is `b0b4b02e41b4459e9a2326582d915ebc7a2f6406ec5ce1b92f9f57433492d8a7`;
helper hash is `9b3f6d2ea14b259b449145e6c15732c714f45b708ccdc560c193172d06bf42a6`.
These identify that source, not a later manifest. Final-source artifact and CI
readiness remain release gates. The existing D: output is reused under the
same guard; no cold rebuild, extra target or shared cache cleanup is needed
for this test/docs change.

No live accounts, paid inference, GUI/OS input, notification, signing or new
access grant was used. Release, consumer merges and Haiwire remain pending.
