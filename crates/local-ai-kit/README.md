# local-ai-kit (Rust facade)

Dependency-first entry point for the local AI runtime. `LocalAi::start()`
resolves a granted shared broker, then a private helper child, then the kit in
this process. The standalone app is never required.

```rust
let mut ai = local_ai_kit::LocalAi::start()?;
let snapshot = ai.snapshot()?;
println!("{:?}: {} models", ai.mode(), snapshot.models.len());
```

See [dependency-first clients](../../docs/integration/dependency-first.md) for
modes, the helper lifetime, `LAI_HELPER_PATH` and the safety rules.
