# Shared runtime security review

- **Scope:** broker lifetime, SQLite state, snapshot/event permissions, issue
  and notification data, control center, update verifier, consumer bridges.
- **Date:** 2026-10-22
- **Status:** implementation review with fixture and Windows platform evidence;
  not an external audit.

## Assets and boundaries

| Asset | Storage/owner | Boundary |
|---|---|---|
| Provider credentials | Native client / supported keyring | Never read into snapshots, SQLite, logs, exports, or the control center |
| Normalized snapshot | Broker memory + broker-owned SQLite | Per-client `ClientPolicy` projection at the service boundary |
| Inventory journal | Broker-owned SQLite | Same projection; cursor is opaque and epoch-bound |
| Issue evidence | Broker-owned SQLite + projected snapshot | Sanitized; no raw provider bodies or credential material |
| Notification delivery state | Broker-owned SQLite | Deduplicated; private-content mode omits issue text |
| Broker token | Parent descriptor or owner-only independent token file | Not argv, not logs, not snapshots; independent mode is owner-scoped, not per-app |
| Native endpoint | Windows pipe owner-only DACL / Unix socket 0600 | One-use grant in parent mode; owner token in independent mode |
| Update manifest/archive | Release pipeline + host-pinned key | Ed25519 publisher signature plus checksum; checksum alone is insufficient |

## Findings and controls

1. **Independent token file is not a per-app grant.** It is deliberately
   reusable so a second application can connect after the first exits. Any
   process running as the same OS user that can read the file can connect.
   Mitigation: owner-only file permissions, endpoint permissions, explicit
   configuration, and per-app grants remain available in parent/embedded mode.
   This is documented in the ADR and broker docs. A future per-app secret
   exchange should be added before claiming per-app revocation in independent
   mode.
2. **Windows token-file ACLs.** The descriptor is created and then restricted
   with `icacls /inheritance:r /grant:r <current-user>:F`; if that fails, the
   file is removed and the broker fails closed. The native endpoint remains
   owner-only. This is platform-executed by the independent-broker test on
   Windows. Unix files are written 0600 and the SDK rejects group/other access.
3. **Out-of-order publication.** Metadata base revisions are checked before
   commit. A stale probe cannot overwrite a newer inventory/identity state.
4. **Persistence sanitization.** Account labels and identity sources are
   replaced with `NOT_PERSISTED` before SQLite writes. Metadata reports are
   sanitized separately. Snapshot serialization is sanitized in the store, not
   only in the in-memory public projection.
5. **Corruption.** A failed integrity check preserves the database and WAL
   beside it, creates a new store, and records a visible storage issue. It does
   not silently delete account or issue state.
6. **Permission filtering on errors.** Access checks precede path lookup; issue
   listing and mutation use the projected snapshot; error messages are stable
   and do not echo adapter/OS paths or raw provider strings.
7. **Revocation.** `apps.revoke` checks the admin policy, refuses self-revoke,
   removes the client policy, sweeps sessions/inputs/resume records, and
   invalidates subsequent calls. Existing transports are not trusted to
   re-authorize themselves.
8. **Notification privacy.** Private-content mode replaces title/body; quiet
   hours suppress delivery; snooze does not resolve; inline health remains.
9. **Control center.** It uses only the authenticated JSON-RPC API and never
   opens SQLite. Export redacts secret-shaped keys recursively, though the
   protocol already never returns provider tokens.
10. **Update trust.** Signature verification is mandatory before staging.
    Activation refuses while a session marker/lease is present, uses exclusive
    locks, backs up the current binary, and preserves the failed version.
    Signing/publication/installation remain approval gates.
11. **Threat-model limits.** The runtime is not a defense against malware that
    already controls the user's OS account. CORS is not authentication.
    Loopback HTTP still requires pairing and per-client grants.

## Required follow-ups before a production claim

- Apply an explicit owner-only Windows DACL to the independent token file, or
  require a host-supplied credential with the same guarantee.
- Add per-app credentials/revocation to independent mode if independent mode is
  marketed as a multi-app security boundary.
- Run the shared-runtime tests on Linux and macOS, including socket permissions,
  file locking, and SQLite WAL behavior.
- External security review of the update signer/publisher process and the
  independent token distribution.
- Live-account qualification for each provider remains separate from this
  review.
