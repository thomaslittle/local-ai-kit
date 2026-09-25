# Cursor cached metadata

The Cursor adapter reads only `cursorAuth/cachedEmail` and `cursorAuth/stripeMembershipType` from the editor's on-disk `User/globalStorage/state.vscdb` cache. These keys are documented in CodeNotch's implementation at `vinzdg/codenotch`, commit `642d329c52ba127adec6eb04e6e2614495b47f19`, `windows/codenotch/src/cursor.rs` and `Sources/Providers/CursorCredentials.swift`.

The implementation uses an immutable read-only SQLite URI, does not write sidecars, and queries no credential keys. It never reads or repurposes Cursor access tokens, builds browser cookies, or calls private usage endpoints. The cache may omit newer WAL data; plan and identity records are always explicitly stale/unverified. Cached plan presence never becomes authentication or billing proof. Identity is masked by the service unless the embedding host separately grants identity visibility.

Models, current allowances, and native execution are not advertised by this cache reader. Cursor's generic `agent` command is ambiguous because other clients ship that filename; an unrelated executable is not attributed to Cursor. Applications can use separately configured API routes, but that is not a conversion of a Cursor subscription.

`--allow-metadata cursor` enables an explicit probe of a passively detected Cursor profile. No credential reads or SQLite access occur during import or passive inventory. File size, SQLite value size, query text, and busy timeout are bounded. Native database corruption or absence returns a sanitized error.

Tests cover unchanged source bytes/no sidecar creation, token exclusion, separate profiles, stale cache records, corrupted databases, and invalid plan values. These are local-cache tests, not a live subscription verification.
