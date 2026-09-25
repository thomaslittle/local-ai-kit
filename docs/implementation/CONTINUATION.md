# Continuation — implementation to release checks

Base: b4437b81a7954ee74f2231c286eed8d4131e0ec6. Existing isolated worktree verified; 240-test native baseline passed.

Active: owned artifacts and missing public operations; remaining native provider evidence; complete installable packaging, contributor/support matrix, and test profiles; Windows and Ubuntu qualification.

Ruling: retain the existing tested module names instead of recreating the plan’s illustrative file layout. No paid inference, token export, main merge, or registry publication. macOS evidence must come from an actual runner, not a cross-compile claim.

Release manifest tests pass (six). The first clean consumer run found a harness assumption (`SnapshotStore.status`) not present in the public API; corrected it to check the actual `snapshot` and `loading` fields, preserving the no-automatic-I/O assertion.

Discovery regression: a Grok agent.exe plus Cursor configuration was incorrectly reported as a Cursor executable. Reproduced before narrowing executable attribution; configuration-only Cursor remains visible without claiming an authenticated/runnable client.

Ruling: provider-native tool policy is a distinct, explicit execution mode, not an assertion of a toolkit OS sandbox. It requires a separate host grant and a session request; ordinary read-only/workspace-write modes remain unchanged. Providers that cannot enforce a requested mode must reject it. No native-policy live inference is used for implementation verification. Cursor uses only immutable non-secret cached metadata and never token/cookie reuse.

Native Google/Grok client-to-execution integration write was blocked by the tool safety boundary. Not retried/obfuscated or redirected through another tool. Provider-specific integration remains unsupported; common ACP/auth/recipe tests are not live qualification. Only the incomplete new client glue/test sketches were preserved under ignored verification scratch and removed from product sources. Release status must name this unresolved scope.

Final Windows UI profile exposed two previously container-specific harness assumptions: default text decoding was cp1252 and Chromium was hard-coded to /usr/bin/chromium. Fixed explicit UTF-8 and selected the installed Playwright browser by default; custom CHROMIUM_EXECUTABLE remains explicit. These were real reported failures, not skipped tests.
