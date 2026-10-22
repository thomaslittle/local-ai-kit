# Control-center evidence attempts

Every retained run below has its own directory. The harness now refuses to
start when the target directory is non-empty, so a later retry cannot overwrite
an earlier artifact.

## Retained

- `100x30-before-retention-rule`: the first passing 100x30 run after the
  selection-reset, scrolling, and footer fixes, but before `y/n` was moved to
  the front of the confirmation prompt.
- `80x24-prompt-clipped`: retained failure. The confirmation prompt was
  truncated before `y/n` at 80 columns, so the required confirmation keys were
  not visible.
- `80x24-final`: passing run after the prompt fix, including scrolling
  Diagnostics.
- `100x30-final`: passing run after the prompt fix.

## Lost before the retention rule was imposed

Earlier harness attempts were deleted by the command that recreated the
evidence directory. They are not claimed as retained artifacts:

- ctypes ConPTY attempt: blank capture / `AssertionError('\n')` after a
  malformed `UpdateProcThreadAttribute` handle argument.
- 100x30 after the first ConPTY fix: only the tab bar was captured; the check
  for `Revision` failed before the body was drawn.
- 100x30 after the refresh fix: Accounts showed "No connections" because
  discovery had not yet been refreshed.
- 100x30 after the refresh was added: the selection marker check used `>`
  while the rendered marker was `›`.
- 100x30 after switching with digit keys: Apps did not show a selected row
  because the Accounts selection index was carried into Apps; `x` therefore
  did nothing. This exposed the digit-tab selection bug that was fixed.
- 100x30 after the selection fix: the Diagnostics disclaimer was below the
  viewport and the non-selectable page could not scroll. This exposed the
  scroll bug that was fixed.
- The first mutation-harness run stopped after its first mutation because the
  log-path variable was undefined. That non-acceptance harness crash was
  overwritten; the complete four-mutation run is retained under `mutations/`.

The terminal output of these lost attempts remains in the agent transcript,
but no original file or screenshot is claimed. The final pass, the retained
80x24 failure, and the four mutation fault/restore logs are the durable
evidence.
