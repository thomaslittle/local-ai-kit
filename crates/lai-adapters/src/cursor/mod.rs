//! Immutable, allowlisted Cursor cache reads. Never exports or requests subscription tokens.
use lai_contracts::*;
use lai_discovery::LocatedClient;
use rusqlite::{Connection, OpenFlags, OptionalExtension, limits::Limit};
use std::time::Duration;
fn unavailable() -> DomainError {
    DomainError::new(
        ErrorCode::ProviderUnavailable,
        "Cursor's non-secret account cache is unavailable",
    )
}
fn cached<T>(value: T) -> Observation<T> {
    let mut o = Observation::reported(
        value,
        EvidenceSource::new(
            EvidenceKind::LocalConfig,
            "cursor",
            "Cursor ItemTable cached identity/plan; immutable on-disk snapshot",
        ),
    );
    o.mark_stale();
    o
}
fn value(db: &Connection, key: &str) -> Option<String> {
    db.query_row(
        "SELECT substr(value,1,1024) FROM ItemTable WHERE key=?1 LIMIT 1",
        [key],
        |r| r.get::<_, String>(0),
    )
    .optional()
    .ok()
    .flatten()
    .filter(|s| !s.trim().is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
}
pub fn read_cached(
    located: LocatedClient,
    connection: &str,
    route: &str,
) -> Result<ConnectionMetadata> {
    if located.provider_id != "cursor" || !located.profile_home.is_absolute() {
        return Err(unavailable());
    }
    let path = located.profile_home.join("User/globalStorage/state.vscdb");
    let info = path.symlink_metadata().map_err(|_| unavailable())?;
    if !info.is_file() || info.file_type().is_symlink() || info.len() > 64 * 1024 * 1024 {
        return Err(unavailable());
    }
    let mut url = reqwest::Url::from_file_path(&path).map_err(|_| unavailable())?;
    url.query_pairs_mut()
        .append_pair("mode", "ro")
        .append_pair("immutable", "1");
    let db = Connection::open_with_flags(
        url.as_str(),
        OpenFlags::SQLITE_OPEN_READ_ONLY
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|_| unavailable())?;
    db.busy_timeout(Duration::from_millis(200))
        .map_err(|_| unavailable())?;
    db.set_limit(Limit::SQLITE_LIMIT_LENGTH, 65536);
    db.set_limit(Limit::SQLITE_LIMIT_SQL_LENGTH, 4096);
    // Validate the fixed schema before treating absent cache fields as meaningful.
    db.prepare("SELECT key FROM ItemTable LIMIT 0")
        .map_err(|_| unavailable())?;
    let email = value(&db, "cursorAuth/cachedEmail");
    let plan = value(&db, "cursorAuth/stripeMembershipType");
    let account = email.map(|label| Account {
        id: stable_id("cached-account", &[connection, &label]),
        provider_id: "cursor".into(),
        display_label: cached(label),
        identity_source: cached("Cursor cached account".into()),
    });
    Ok(ConnectionMetadata {
        connection_id: connection.into(),
        route_id: route.into(),
        version: Observation::unknown("NATIVE_VERSION_NOT_PROBED"),
        auth: Observation::unknown("CACHED_ACCOUNT_DOES_NOT_VERIFY_AUTHENTICATION"),
        billing: Observation::unknown("CACHED_PLAN_DOES_NOT_VERIFY_BILLING"),
        plan: plan
            .map(|label| {
                cached(Plan {
                    native_id: label.clone(),
                    label,
                })
            })
            .unwrap_or_else(|| Observation::unknown("CACHED_PLAN_NOT_REPORTED")),
        account,
        models: vec![],
        providers: vec![],
        additional_routes: vec![],
        quota_pools: vec![],
        included_usage_allowed: Observation::unknown("CURSOR_QUOTA_NOT_READ"),
        diagnostics: vec![
            "UNVERIFIED_LOCAL_CACHE_ONLY".into(),
            "IMMUTABLE_DATABASE_EXCLUDES_LIVE_WAL_UPDATES".into(),
            "NO_SUBSCRIPTION_TOKEN_OR_PRIVATE_ENDPOINT_USED".into(),
        ],
    })
}
