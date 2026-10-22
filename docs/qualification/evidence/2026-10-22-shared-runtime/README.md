# 2026-10-22 shared runtime evidence

All evidence here was produced on Windows 11 in test-owned homes, loopback
endpoints, and synthetic fixtures. No live provider account, inference, TV,
LAN listener, desktop notification, or global keyboard/mouse input was used.

- `mutations/`: four real patch-restored faults. For each one, a selected
  acceptance test failed with a panic, the exact source was restored, and the
  same test passed. `summary.json` records source SHA-256, commands, exit codes,
  and test names. The faulty sources were never committed or pushed.
- `control-center/`: real ratatui TUI runs captured through a test-owned
  Windows ConPTY. `80x24-prompt-clipped` is a retained failure; the final 80x24
  and 100x30 runs pass. `ATTEMPTS.md` discloses earlier artifacts that were
  overwritten before the unique-directory rule was imposed; those are not
  claimed as retained.
- `bridge/`: Remotif cross-process tests. One retained log is an opt-in skip
  (no test binary configured); the other is the real broker run proving app
  scope, revocation, peer survival, broker-kill failure, and restart recovery.
- `tests/`: the transient Windows loopback socket-buffer failure, its targeted
  passing retry, the subsequent full 611-test pass, and control-center tests.

Token files and broker databases were deliberately excluded from this tree.
The working copies outside the repository had their test-only token values
redacted. No raw memory dump or provider credential is present.

Source state:

- Local AI Kit control fix: `eed3a2c26d7c0930d7f6e6313eed29bb528e9df8`
  (mutation run was against its parent `76df73d`; the mutated guards were
  unchanged by the later control-center commit).
- Remotif bridge test/fix: `fc4a99e81b4a0da8ada8101c7469c6c6d5104bc6`.
