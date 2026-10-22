# Sibling SDK bootstrap assumptions — alpha18

After the Node first-read correction in `e679215`, sibling inspection found the
same Python assertion. A diagnostic 100 ms delay reproduces the old empty-cache
assertion failure (`bootstrap-python-red-01.txt`); the corrected security
invariants pass with that delay, which is removed from final source.

The first combined rerun retained a different failure: Node's empty catalog
page compared its revision with a later independent snapshot while bootstrap
published between them (`bootstrap-both-bridge-green-01.txt`). The test now
brackets the page with snapshot reads and requires its revision to match one
boundary. This fixture has at most one bootstrap publication: hydration precedes
serving, no explicit refresh or metadata grant occurs, and the 180-second command
limit precedes the minimum 300-second inventory cadence. Stable generations
still require equality. This is not a general multi-publication pagination proof.

The final combined bridge passes Node 5/5 and Python 4/4 (`green-02`). Ten
consecutive direct combined runs pass 90 executions, zero failures/retries.
All 30 command logs and results remain retained. Initial auth-unknown,
empty-model/account and sentinel checks preserve both valid startup orderings;
credential equality, explicit refresh, 24 reads, scope revocation, default denial,
terminal cursor, empty page, invalid limit and method advertisement remain tested.
Content equality/non-disclosure do not prove absence of credential reads; separate
service fixtures enforce zero calls without permission.

R9 independently accepts both diffs with zero Critical, Important or Minor
findings. Read-only review included the four sibling native integration files;
no further unsupported startup/cross-request stability assumption was found.
The reviewer ran no tests or builds and changed no files. Reviewed SHA-256:

- Node: `5989bbf952bafade8cce1d2e7adde0dfea4adef52ea9eb19198214bc5f2a54f0`
- Python: `6b540eadd0111b79e9976a190089c33f5cf64d86936cb1b1a9ee2a90ef9b4502`

`source-and-tests.json` verifies that production and R6/R7 bytes remain unchanged.
No product scheduling, SDK implementation, permission, sleep or retry changed.
Prior clean `e679215` artifact, package and enforced performance receipts are
retained separately; the optimized helper remains identical to `aeecd15`.
Final-source manifest/artifact and CI readiness remain release gates. No release,
consumer merge, live account, inference, OS input/audio, signing or new grant
was performed. Build output remains in the same bounded, serialized D: target.
