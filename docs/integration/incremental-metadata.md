# Incremental metadata, usage, and backoff

Local AI Kit separates the mechanics for reading changing local evidence from the provider-specific interpretation of that evidence.

## Bounded log/activity tailing

`lai_discovery::log_tail::read_lines` is a provider-agnostic incremental newline-record reader. No current provider adapter uses it: Codex and Claude usage data come from their structured native interfaces, and no undocumented activity/log source is inferred. A future adapter may use the primitive only after identifying a specific nonsecret file and understanding its schema.

The caller owns the state; it stores an offset, a platform file identity when available, at most 1 MiB of an unfinished record, and a 64-byte anchor immediately before the current offset. It:

- reads at most 1 MiB per call;
- preserves partial records until a newline arrives;
- limits returned records to 4,096 per call;
- detects truncation when the current file is shorter than the saved offset;
- detects replacement/rotation by comparing file identity and verifying the previous byte anchor, including when the replacement file is larger or shares the same anchor bytes;
- restarts from byte zero after truncation/rotation without replaying records from the previous file;
- serves already-buffered complete records before reading more bytes;
- checks caller cancellation before each bounded filesystem operation and leaves saved state unchanged when cancelled;
- fails closed when one record exceeds the bounded buffer.
- reads only a single explicitly supplied path; it does not traverse a home directory, and the native filesystem rejects dot segments and symlink/reparse-point components anywhere in the file path.

Tests include append/partial records, truncation, replacement rotation with matching anchor bytes, cancellation, symlink rejection, line-window paging, an intentionally sparse 1 GiB file, and oversized records. The sparse-file test verifies that only a few kilobytes are actually read.

The toolkit does **not** infer quota, token cost, or plan access merely because a log exists. Codex/Claude quota data currently comes from structured native interfaces. A future adapter can connect a verified log schema to normalized usage records without changing the tailer.

## Metadata refresh and failure backoff

Connection probes remain single-flight per connection. Successful metadata is cached and field evidence becomes stale rather than silently refreshed in the background.

Retryable failures use bounded exponential backoff starting near 250 ms, capped at 30 seconds, with deterministic per-connection jitter. An upstream `Retry-After` value takes precedence and is bounded to 24 hours. Nonretryable failures retain a short negative cache to prevent tight loops.

Only safe retry information crosses the public API:

- `retryable: true|false`
- optional `retryAfterMs`

Provider response bodies, URLs, headers, tokens, local paths, and raw error messages are discarded before caching/serialization. TypeScript and Python SDK errors expose the same safe fields and never retry a request automatically. In particular, a generation/turn request is never replayed merely because an error is retryable.

OS metadata notifications invalidate cached evidence only. They do not launch a client, probe quota, or run inference; the host still explicitly chooses when to refresh.
