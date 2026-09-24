# Continuation — implementation to release checks

Base: b4437b81a7954ee74f2231c286eed8d4131e0ec6. Existing isolated worktree verified; 240-test native baseline passed.

Active: owned artifacts and missing public operations; remaining native provider evidence; complete installable packaging, contributor/support matrix, and test profiles; Windows and Ubuntu qualification.

Ruling: retain the existing tested module names instead of recreating the plan’s illustrative file layout. No paid inference, token export, main merge, or registry publication. macOS evidence must come from an actual runner, not a cross-compile claim.

Release manifest tests pass (six). The first clean consumer run found a harness assumption (`SnapshotStore.status`) not present in the public API; corrected it to check the actual `snapshot` and `loading` fields, preserving the no-automatic-I/O assertion.

Discovery regression: a Grok agent.exe plus Cursor configuration was incorrectly reported as a Cursor executable. Reproduced before narrowing executable attribution; configuration-only Cursor remains visible without claiming an authenticated/runnable client.
