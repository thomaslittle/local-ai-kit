# Native passive inventory

An explicit `Scanner::scan` probes allowlisted client/configuration locations and bounded directory entries. It never reads file content, unlocks a credential store, launches clients, or contacts a provider. The injected `LocalFilesystem` boundary allows tests to fail immediately on accidental file reads.

Initial pattern registry: Codex, Claude Code, Gemini CLI, Antigravity, Grok, OpenCode, Cursor, Ollama, LM Studio. These are candidate location patterns, not certification of authenticated access. Provider directories and command names are based on the project source audit; installed-version authentication/execution remains separate work.

Windows shims are detected but not run. Symlink executables are reported as not followed; explicit approved shim/symlink resolution remains pending. Relative/network roots are rejected. Known home overrides and XDG locations are respected. Personal/work Codex/Claude directories have independent stable identities; no account or shared allowance is inferred. OpenCode's configuration and data roots remain separate candidates until later authenticated identity linking.

Public snapshots expose normalized hints and opaque path-derived IDs, not full private paths. Actual executable/profile paths remain in the native `ScanResult.locations` map for later authorized adapters. Snapshot models, authenticated accounts, plans, and quotas remain empty/unknown until explicitly enriched.

Limits: at most 128 home entries and PATH entries, 16 profile roots and 8 executable candidates per provider. Partial enumeration is explicitly diagnosed. Progress publishes one partial snapshot per provider; cancellation is checked between metadata probes. A filesystem metadata call can still block at the OS level; this synchronous library is not a deadline-enforced sandbox. Native service integration must keep it off request/UI threads.

No automatic WSL/container/network inventory, no claims of inactive subscription detection, and no credentials are returned.
