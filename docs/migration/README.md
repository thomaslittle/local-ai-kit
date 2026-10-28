# Consumer migration notes

These notes are for the maintainers of each app. Migration happens in those
repositories, in their own branches and releases; nothing here edits them.

The shared move is the same everywhere: delete the hand-rolled helper client and
call the dependency instead.

```rust
// before: locate/start a helper, manage a socket and a token file
// after:
let mut ai = local_ai_kit::LocalAi::start()?;
let snapshot = ai.snapshot()?;
```

`LocalAi::start()` uses a granted shared broker, otherwise a private helper it
owns, otherwise the runtime in-process — so the app no longer needs the helper
installed separately. Show `ai.mode()` where the app reports connection state.

See [dependency-first clients](../integration/dependency-first.md).
