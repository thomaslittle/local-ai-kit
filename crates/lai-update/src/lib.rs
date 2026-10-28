//! Authenticated, staged executable updates. Checksums detect corruption;
//! Ed25519 verifies the publisher. This crate never downloads, publishes, or
//! creates signing credentials.
use lai_contracts::{DomainError, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const UPDATE_SCHEMA: u32 = 1;
pub const MAX_MANIFEST_BYTES: u64 = 256 * 1024;
pub const MAX_ARCHIVE_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateManifest {
    pub schema_version: u32,
    pub version: String,
    pub channel: String,
    pub platform: String,
    pub protocol_version: String,
    pub archive: String,
    pub archive_bytes: u64,
    pub archive_sha256: String,
    pub published_at: String,
    #[serde(default)]
    pub notes_url: Option<String>,
    /// Base64 Ed25519 signature over canonical JSON without this field.
    pub signature: String,
}

fn fail(code: ErrorCode, message: &'static str) -> DomainError {
    DomainError::new(code, message)
}

fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}

fn sha256_file(path: &Path) -> Result<String, DomainError> {
    let mut file = File::open(path).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update archive is unreadable",
        )
    })?;
    let mut digest = Sha256::new();
    let mut block = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut block).map_err(|_| {
            fail(
                ErrorCode::StorageUnavailable,
                "Update archive is unreadable",
            )
        })?;
        if count == 0 {
            break;
        }
        digest.update(&block[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub fn canonical_manifest(manifest: &UpdateManifest) -> Result<Vec<u8>, DomainError> {
    let value = serde_json::to_value(manifest)
        .map_err(|_| fail(ErrorCode::InvalidRequest, "Update manifest is invalid"))?;
    let Value::Object(mut map) = value else {
        return Err(fail(
            ErrorCode::InvalidRequest,
            "Update manifest is invalid",
        ));
    };
    map.remove("signature");
    // BTreeMap serializes keys in sorted order, matching the signer's contract.
    let ordered: BTreeMap<String, Value> = map.into_iter().collect();
    serde_json::to_vec(&ordered)
        .map_err(|_| fail(ErrorCode::InvalidRequest, "Update manifest is invalid"))
}

pub fn platform_id() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    let arch = std::env::consts::ARCH;
    format!("{os}-{arch}")
}

fn parse_version(value: &str) -> Option<semver::Version> {
    if value.len() > 128 { return None; }
    semver::Version::parse(value).ok()
}

pub fn verify_manifest_signature(
    manifest: &UpdateManifest,
    public_key: &[u8],
) -> Result<(), DomainError> {
    use base64::Engine;
    if manifest.schema_version != UPDATE_SCHEMA {
        return Err(fail(
            ErrorCode::ProtocolIncompatible,
            "Update manifest schema is not supported",
        ));
    }
    if public_key.len() != 32 {
        return Err(fail(
            ErrorCode::LocalAccessDenied,
            "Update publisher key is invalid",
        ));
    }
    let signature = base64::engine::general_purpose::STANDARD
        .decode(&manifest.signature)
        .map_err(|_| fail(ErrorCode::LocalAccessDenied, "Update signature is invalid"))?;
    let message = canonical_manifest(manifest)?;
    ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key)
        .verify(&message, &signature)
        .map_err(|_| {
            fail(
                ErrorCode::LocalAccessDenied,
                "Update publisher signature did not verify",
            )
        })
}

/// Verify a complete update before any executable is staged.
pub fn verify_update(
    manifest: &UpdateManifest,
    archive: &Path,
    public_key: &[u8],
    current_version: &str,
    protocol_version: &str,
    allow_downgrade: bool,
) -> Result<(), DomainError> {
    verify_manifest_signature(manifest, public_key)?;
    if manifest.platform != platform_id() {
        return Err(fail(
            ErrorCode::ProtocolIncompatible,
            "Update was built for a different platform",
        ));
    }
    if !manifest.protocol_version.starts_with("1.") || !protocol_version.starts_with("1.") {
        return Err(fail(
            ErrorCode::ProtocolIncompatible,
            "Update protocol is incompatible with this runtime",
        ));
    }
    let next = parse_version(&manifest.version)
        .ok_or_else(|| fail(ErrorCode::InvalidRequest, "Update version is invalid"))?;
    let alpha = next.pre.as_str().strip_prefix("alpha.");
    if (next.major, next.minor, next.patch) != (0, 1, 0)
        || !next.build.is_empty()
        || !alpha.is_some_and(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(fail(
            ErrorCode::ProtocolIncompatible,
            "Update version is outside the supported release family",
        ));
    }
    let current = parse_version(current_version)
        .ok_or_else(|| fail(ErrorCode::InvalidRequest, "Current version is invalid"))?;
    if !allow_downgrade && next.cmp_precedence(&current).is_lt() {
        return Err(fail(
            ErrorCode::Conflict,
            "Update would downgrade the shared runtime",
        ));
    }
    let metadata = std::fs::metadata(archive).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update archive is unavailable",
        )
    })?;
    if !metadata.is_file() || metadata.len() != manifest.archive_bytes {
        return Err(fail(
            ErrorCode::InvalidRequest,
            "Update archive size does not match",
        ));
    }
    if metadata.len() > MAX_ARCHIVE_BYTES {
        return Err(fail(
            ErrorCode::FrameTooLarge,
            "Update archive exceeds the bound",
        ));
    }
    if sha256_file(archive)? != manifest.archive_sha256.to_ascii_lowercase() {
        return Err(fail(
            ErrorCode::InvalidRequest,
            "Update archive checksum does not match",
        ));
    }
    Ok(())
}

pub struct StagedUpdate {
    pub directory: PathBuf,
    pub archive: PathBuf,
    pub manifest: PathBuf,
}

/// Copy a verified archive into a staging directory. No installed executable is
/// touched and no activation occurs.
pub fn stage_update(
    manifest: &UpdateManifest,
    archive: &Path,
    staging_root: &Path,
    public_key: &[u8],
    current_version: &str,
    protocol_version: &str,
) -> Result<StagedUpdate, DomainError> {
    verify_update(
        manifest,
        archive,
        public_key,
        current_version,
        protocol_version,
        false,
    )?;
    let directory = staging_root.join(&manifest.version);
    if directory.exists() {
        return Err(fail(
            ErrorCode::Conflict,
            "Update version is already staged",
        ));
    }
    std::fs::create_dir_all(&directory).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update staging is unavailable",
        )
    })?;
    let staged_archive = directory.join(
        Path::new(&manifest.archive)
            .file_name()
            .ok_or_else(|| fail(ErrorCode::InvalidRequest, "Update archive name is invalid"))?,
    );
    std::fs::copy(archive, &staged_archive).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update archive could not be staged",
        )
    })?;
    if sha256_file(&staged_archive)? != manifest.archive_sha256 {
        let _ = std::fs::remove_dir_all(&directory);
        return Err(fail(
            ErrorCode::InvalidRequest,
            "Staged update checksum does not match",
        ));
    }
    let manifest_path = directory.join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(manifest)
            .map_err(|_| fail(ErrorCode::InvalidRequest, "Update manifest is invalid"))?,
    )
    .map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update manifest could not be staged",
        )
    })?;
    std::fs::write(directory.join("READY"), b"verified\n").map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update staging is unavailable",
        )
    })?;
    Ok(StagedUpdate {
        directory,
        archive: staged_archive,
        manifest: manifest_path,
    })
}

pub struct SessionGuard {
    _file: File,
    marker: PathBuf,
}
impl SessionGuard {
    /// A running consumer/runtime holds a shared lease. Activation needs the
    /// exclusive lease, so it cannot replace a binary another session is using.
    /// A visible marker also lets an operator see why activation is blocked.
    pub fn begin(root: &Path) -> Result<Self, DomainError> {
        std::fs::create_dir_all(root)
            .map_err(|_| fail(ErrorCode::StorageUnavailable, "Update state is unavailable"))?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join("sessions.lock"))
            .map_err(|_| fail(ErrorCode::StorageUnavailable, "Update lease is unavailable"))?;
        fs4::fs_std::FileExt::lock_shared(&file).map_err(|_| {
            fail(
                ErrorCode::LocalAccessDenied,
                "Another update holds the session lease",
            )
        })?;
        let marker = root.join(format!("session-{}-{}.json", std::process::id(), unix_ms()));
        let _ = std::fs::write(
            &marker,
            serde_json::to_vec(&serde_json::json!({
                "pid": std::process::id(),
                "startedAt": unix_ms(),
            }))
            .unwrap_or_default(),
        );
        Ok(Self {
            _file: file,
            marker,
        })
    }
}
impl Drop for SessionGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.marker);
    }
}

fn session_markers(root: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(root)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter(|entry| {
            entry.file_name().to_string_lossy().starts_with("session-")
                && entry.file_name().to_string_lossy().ends_with(".json")
        })
        .map(|entry| entry.path())
        .filter_map(|path| {
            let started = std::fs::read(&path)
                .ok()
                .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
                .and_then(|value| value["startedAt"].as_u64());
            if started
                .is_some_and(|started| unix_ms().saturating_sub(started) > 24 * 60 * 60 * 1000)
            {
                // A crashed consumer cannot release its marker. Preserve it as
                // evidence but do not block updates forever.
                let stale = path.with_extension("stale.json");
                let _ = std::fs::rename(&path, stale);
                None
            } else {
                Some(path)
            }
        })
        .collect()
}

pub struct Activation {
    pub backup: PathBuf,
    pub installed: PathBuf,
}

/// Atomically replace a target executable after verifying the staged archive.
/// `active_sessions` is represented by a shared lease; callers that need live
/// session coordination should also drain sessions before invoking this.
pub fn activate(
    state_root: &Path,
    staged: &StagedUpdate,
    manifest: &UpdateManifest,
    target: &Path,
    public_key: &[u8],
) -> Result<Activation, DomainError> {
    std::fs::create_dir_all(state_root)
        .map_err(|_| fail(ErrorCode::StorageUnavailable, "Update state is unavailable"))?;
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(state_root.join("activation.lock"))
        .map_err(|_| fail(ErrorCode::StorageUnavailable, "Update lock is unavailable"))?;
    fs4::fs_std::FileExt::try_lock_exclusive(&lock).map_err(|_| {
        fail(
            ErrorCode::Conflict,
            "Another update activation is already in progress",
        )
    })?;
    let sessions = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(state_root.join("sessions.lock"))
        .map_err(|_| fail(ErrorCode::StorageUnavailable, "Update lease is unavailable"))?;
    fs4::fs_std::FileExt::try_lock_exclusive(&sessions).map_err(|_| {
        fail(
            ErrorCode::Conflict,
            "An active consumer/runtime session prevents activation",
        )
    })?;
    if !session_markers(state_root).is_empty() {
        return Err(fail(
            ErrorCode::Conflict,
            "An active consumer/runtime session marker prevents activation",
        ));
    }
    verify_update(
        manifest,
        &staged.archive,
        public_key,
        env!("CARGO_PKG_VERSION"),
        "1.0",
        true,
    )?;
    let backup_dir = state_root.join("backups");
    std::fs::create_dir_all(&backup_dir).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update backup is unavailable",
        )
    })?;
    let backup = backup_dir.join(format!("{}-{}.bak", manifest.version, unix_ms()));
    if target.exists() {
        std::fs::copy(target, &backup).map_err(|_| {
            fail(
                ErrorCode::StorageUnavailable,
                "Current executable could not be backed up",
            )
        })?;
    }
    let staged_binary = staged.directory.join(if cfg!(windows) {
        "local-ai-kit.exe"
    } else {
        "local-ai-kit"
    });
    if !staged_binary.is_file() {
        return Err(fail(
            ErrorCode::InvalidRequest,
            "Staged update has no executable",
        ));
    }
    let replacement = target.with_extension(format!("new-{}", unix_ms()));
    std::fs::copy(&staged_binary, &replacement).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Staged executable could not be installed",
        )
    })?;
    if std::fs::rename(&replacement, target).is_err() {
        let _ = std::fs::remove_file(&replacement);
        return Err(fail(
            ErrorCode::StorageUnavailable,
            "Executable replacement failed; the previous binary is intact",
        ));
    }
    let _ = std::fs::write(
        state_root.join("state.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "installedVersion": manifest.version,
            "protocolVersion": manifest.protocol_version,
            "backup": backup,
            "installedAt": manifest.published_at,
        }))
        .unwrap_or_default(),
    );
    Ok(Activation {
        backup,
        installed: target.to_path_buf(),
    })
}

/// Restore the exact backup recorded by `activate`. The failed version is kept
/// in the backup directory rather than deleted.
pub fn rollback(target: &Path, backup: &Path) -> Result<(), DomainError> {
    if !backup.is_file() {
        return Err(fail(ErrorCode::NotFound, "Update backup is unavailable"));
    }
    let replacement = target.with_extension(format!("rollback-{}", unix_ms()));
    std::fs::copy(backup, &replacement)
        .map_err(|_| fail(ErrorCode::StorageUnavailable, "Rollback copy failed"))?;
    std::fs::rename(&replacement, target).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Rollback replacement failed; no file was deleted",
        )
    })?;
    Ok(())
}

pub fn read_manifest(path: &Path) -> Result<UpdateManifest, DomainError> {
    let metadata = std::fs::metadata(path).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update manifest is unavailable",
        )
    })?;
    if !metadata.is_file() || metadata.len() > MAX_MANIFEST_BYTES {
        return Err(fail(
            ErrorCode::FrameTooLarge,
            "Update manifest exceeds the bound",
        ));
    }
    let bytes = std::fs::read(path).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update manifest is unreadable",
        )
    })?;
    serde_json::from_slice(&bytes)
        .map_err(|_| fail(ErrorCode::InvalidRequest, "Update manifest is malformed"))
}

pub fn write_unsigned_manifest(
    path: &Path,
    version: &str,
    channel: &str,
    protocol_version: &str,
    archive: &Path,
    archive_name: &str,
    published_at: &str,
) -> Result<UpdateManifest, DomainError> {
    let bytes = std::fs::metadata(archive)
        .map_err(|_| {
            fail(
                ErrorCode::StorageUnavailable,
                "Release archive is unavailable",
            )
        })?
        .len();
    if bytes > MAX_ARCHIVE_BYTES {
        return Err(fail(
            ErrorCode::FrameTooLarge,
            "Release archive exceeds the bound",
        ));
    }
    let manifest = UpdateManifest {
        schema_version: UPDATE_SCHEMA,
        version: version.into(),
        channel: channel.into(),
        platform: platform_id(),
        protocol_version: protocol_version.into(),
        archive: archive_name.into(),
        archive_bytes: bytes,
        archive_sha256: sha256_file(archive)?,
        published_at: published_at.into(),
        notes_url: None,
        signature: String::new(),
    };
    let mut file = File::create(path).map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update manifest could not be written",
        )
    })?;
    file.write_all(
        &serde_json::to_vec_pretty(&manifest)
            .map_err(|_| fail(ErrorCode::InvalidRequest, "Update manifest is invalid"))?,
    )
    .map_err(|_| {
        fail(
            ErrorCode::StorageUnavailable,
            "Update manifest could not be written",
        )
    })?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::rand::SystemRandom;
    use ring::signature::{Ed25519KeyPair, KeyPair};

    struct Signed {
        manifest: UpdateManifest,
        public_key: Vec<u8>,
        archive: PathBuf,
        _temp: tempfile::TempDir,
    }

    fn signed(version: &str) -> Signed {
        let temp = tempfile::tempdir().unwrap();
        let archive = temp.path().join("bundle.zip");
        std::fs::write(&archive, b"release-bytes").unwrap();
        let mut manifest = write_unsigned_manifest(
            &temp.path().join("manifest.json"),
            version,
            "alpha",
            "1.0",
            &archive,
            "bundle.zip",
            "2026-10-22T00:00:00.000Z",
        )
        .unwrap();
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let message = canonical_manifest(&manifest).unwrap();
        use base64::Engine;
        manifest.signature =
            base64::engine::general_purpose::STANDARD.encode(pair.sign(&message).as_ref());
        Signed {
            manifest,
            public_key: pair.public_key().as_ref().to_vec(),
            archive,
            _temp: temp,
        }
    }

    #[test]
    fn prerelease_numbers_and_staging_names_follow_semver() {
        let update = signed("0.1.0-alpha.20");
        verify_update(&update.manifest, &update.archive, &update.public_key, "0.1.0-alpha.9", "1.0", false).unwrap();
        let older = signed("0.1.0-alpha.9");
        assert_eq!(verify_update(&older.manifest, &older.archive, &older.public_key, "0.1.0-alpha.20", "1.0", false).unwrap_err().code, ErrorCode::Conflict);
        for version in ["0.1.0-alpha/../../escape", "0.1.0-alphabet", "0.1.0-alpha.020", "0.1.0-alpha.20+metadata"] {
            let update = signed(version);
            assert!(verify_update(&update.manifest, &update.archive, &update.public_key, "0.1.0-alpha.1", "1.0", false).is_err());
        }
    }

    #[test]
    fn signature_checksum_platform_and_downgrade_are_all_required() {
        let signed = signed("0.1.0-alpha.18");
        verify_update(
            &signed.manifest,
            &signed.archive,
            &signed.public_key,
            "0.1.0-alpha.17",
            "1.0",
            false,
        )
        .unwrap();
        let mut tampered = signed.manifest.clone();
        tampered.archive_sha256 = "0".repeat(64);
        assert!(
            verify_update(
                &tampered,
                &signed.archive,
                &signed.public_key,
                "0.1.0-alpha.17",
                "1.0",
                false
            )
            .is_err()
        );
        let mut downgrade = signed.manifest.clone();
        let message = canonical_manifest(&downgrade).unwrap();
        downgrade.signature = signed.manifest.signature.clone();
        assert!(
            verify_update(
                &downgrade,
                &signed.archive,
                &signed.public_key,
                "0.1.0-alpha.20",
                "1.0",
                false
            )
            .is_err()
        );
        let _ = message;
        let mut wrong_platform = signed.manifest.clone();
        wrong_platform.platform = "plan9-mips".into();
        assert!(
            verify_update(
                &wrong_platform,
                &signed.archive,
                &signed.public_key,
                "0.1.0-alpha.17",
                "1.0",
                false
            )
            .is_err()
        );
    }

    #[test]
    fn staged_activation_refuses_a_live_session_and_rolls_back() {
        let signed = signed("0.1.0-alpha.18");
        let root = tempfile::tempdir().unwrap();
        let staging = root.path().join("staged");
        let staged = stage_update(
            &signed.manifest,
            &signed.archive,
            &staging,
            &signed.public_key,
            "0.1.0-alpha.17",
            "1.0",
        )
        .unwrap();
        let executable = staged.directory.join(if cfg!(windows) {
            "local-ai-kit.exe"
        } else {
            "local-ai-kit"
        });
        std::fs::write(&executable, b"new-binary").unwrap();
        let target = root.path().join(if cfg!(windows) {
            "current.exe"
        } else {
            "current"
        });
        std::fs::write(&target, b"old-binary").unwrap();
        {
            let _session = SessionGuard::begin(root.path()).unwrap();
            assert!(
                activate(
                    root.path(),
                    &staged,
                    &signed.manifest,
                    &target,
                    &signed.public_key
                )
                .is_err()
            );
            assert_eq!(std::fs::read(&target).unwrap(), b"old-binary");
        }
        let activation = activate(
            root.path(),
            &staged,
            &signed.manifest,
            &target,
            &signed.public_key,
        )
        .unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"new-binary");
        rollback(&target, &activation.backup).unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"old-binary");
    }

    #[test]
    fn an_old_crash_marker_is_preserved_but_does_not_block_activation() {
        let signed = signed("0.1.0-alpha.18");
        let root = tempfile::tempdir().unwrap();
        let staged = stage_update(
            &signed.manifest,
            &signed.archive,
            &root.path().join("staged"),
            &signed.public_key,
            "0.1.0-alpha.17",
            "1.0",
        )
        .unwrap();
        let executable = staged.directory.join(if cfg!(windows) {
            "local-ai-kit.exe"
        } else {
            "local-ai-kit"
        });
        std::fs::write(&executable, b"new-binary").unwrap();
        let target = root.path().join("current");
        std::fs::write(&target, b"old-binary").unwrap();
        let marker = root.path().join("session-999.json");
        std::fs::write(&marker, br#"{"pid":999,"startedAt":0}"#).unwrap();
        activate(
            root.path(),
            &staged,
            &signed.manifest,
            &target,
            &signed.public_key,
        )
        .unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"new-binary");
        assert!(!marker.exists());
        assert!(
            root.path()
                .read_dir()
                .unwrap()
                .flatten()
                .any(|entry| entry.file_name().to_string_lossy().ends_with(".stale.json"))
        );
    }

    #[test]
    fn a_failed_staging_copy_never_touches_the_installed_binary() {
        let signed = signed("0.1.0-alpha.18");
        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("current");
        std::fs::write(&target, b"old-binary").unwrap();
        let staging = root.path().join("staged");
        let staged = stage_update(
            &signed.manifest,
            &signed.archive,
            &staging,
            &signed.public_key,
            "0.1.0-alpha.17",
            "1.0",
        )
        .unwrap();
        std::fs::remove_file(staged.directory.join(if cfg!(windows) {
            "local-ai-kit.exe"
        } else {
            "local-ai-kit"
        }))
        .ok();
        assert!(
            activate(
                root.path(),
                &staged,
                &signed.manifest,
                &target,
                &signed.public_key
            )
            .is_err()
        );
        assert_eq!(std::fs::read(&target).unwrap(), b"old-binary");
    }
}
