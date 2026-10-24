# Unix executable symlink candidate qualification

Base: `9a40e503c82e3efc15991382322ccf30bc00f806` (alpha.18 main).
Implementation: [supported behavior and limits](../../../adapters/unix-executable-links.md).
Machine-readable [run receipt and source hashes](receipt.json).

**Implemented; fixture-tested and platform-executed on Linux/WSL2 x86_64**
(kernel 6.6.87.2-microsoft-standard-WSL2, Rust 1.97.1). One focused test:

```sh
cargo test --offline --locked -p lai-kit --test unix_executable_symlink -- --nocapture
```

The baseline failed: the registered `codex` symlink was configuration-only. The
final source passed one isolated discovery-to-probe/launch fixture. A synthetic
Python Codex peer returned account/model/usage fixture data; its log had metadata
methods and no turn/thread methods. Passive discovery did not launch it. The
fixture also checked the managed command's identity, rejection of another valid
retarget/replacement, broken/cyclic/deep/non-executable/directory targets, and
continued credential/configuration data-read rejection. Fixture homes were
temporary; no actual provider account or inference was used.

The first offline attempt could not compile because a locked dependency was not
cached. The bounded retry downloaded public build dependencies into this task's
private D: Cargo home. All compilation stages used the same target, two jobs,
symbols/incremental disabled and empty per-process Rust wrappers. Final stage:
38.14 seconds including compilation; fixture execution 0.38 seconds; observed
task output peak 897,613,102 bytes. Budget 4 GiB, sampled stop 3.5 GiB, C: reserve
25 GiB, D: reserve 64 GiB; no pressure stop. These sampled thresholds are not a
filesystem quota. The native Unix fixture directory had an 8 MiB bound.

Formatting passed. No broad unit-suite campaign or extra worker was launched.
Existing native/fixture `LocatedClient` initializers receive the absent optional
identity field; no provider grants or wire contracts change.

macOS execution, Windows execution of this delta, real standalone Codex account
metadata, strict atomic check-to-exec protection, and hosted exact-head CI are
**unqualified here**. The common launcher checks are not handle-based execution
or a same-user adversary sandbox. No shared binary, service, release, credentials,
security settings, or PR29/PR30 implementation was replaced.
