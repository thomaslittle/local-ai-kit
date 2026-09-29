use crate::{Result, error};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TokenRecord {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub expires_at: u64,
    pub scopes: Vec<String>,
}
impl std::fmt::Debug for TokenRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRecord")
            .field("tokens", &"[redacted]")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Registration {
    pub client_id: String,
    pub subject: String,
    pub email: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct StateFile {
    pub host_id: String,
    #[serde(default)]
    pub epoch: u64,
    pub registration: Option<Registration>,
    #[cfg(unix)]
    pub tokens: Option<TokenRecord>,
}
impl StateFile {
    pub fn fresh() -> Self {
        Self {
            host_id: format!("urn:uuid:{}", uuid::Uuid::new_v4()),
            epoch: 0,
            registration: None,
            #[cfg(unix)]
            tokens: None,
        }
    }
}
pub(crate) struct Store {
    root: PathBuf,
}
fn storage_error() -> crate::AuthError {
    error(
        "STORAGE_UNAVAILABLE",
        "ChatGPT credential storage is unavailable",
    )
}
impl Store {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub async fn lock_refresh(&self) -> Result<fs::File> {
        let root = self.root.clone();
        tokio::task::spawn_blocking(move || {
            secure_dir(&root)?;
            let file = fs::OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(root.join("chatgpt-refresh.lock"))
                .map_err(|_| storage_error())?;
            fs4::fs_std::FileExt::lock_exclusive(&file).map_err(|_| storage_error())?;
            Ok(file)
        })
        .await
        .map_err(|_| storage_error())?
    }
    pub fn load(&self) -> Result<StateFile> {
        let path = self.root.join("chatgpt-auth.json");
        if let Ok(meta) = path.symlink_metadata() {
            if !meta.file_type().is_file() {
                return Err(storage_error());
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if meta.permissions().mode() & 0o077 != 0 {
                    return Err(storage_error());
                }
            }
        }
        match fs::read(&path) {
            Ok(bytes) if bytes.len() <= 64 * 1024 => {
                serde_json::from_slice(&bytes).map_err(|_| storage_error())
            }
            Ok(_) => Err(storage_error()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(StateFile::fresh()),
            Err(_) => Err(storage_error()),
        }
    }
    pub fn save(&self, state: &StateFile) -> Result<()> {
        secure_dir(&self.root)?;
        let mut tmp = tempfile::Builder::new()
            .prefix(".chatgpt-auth-")
            .tempfile_in(&self.root)
            .map_err(|_| storage_error())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            tmp.as_file()
                .set_permissions(fs::Permissions::from_mode(0o600))
                .map_err(|_| storage_error())?;
        }
        serde_json::to_writer(tmp.as_file_mut(), state).map_err(|_| storage_error())?;
        tmp.as_file_mut().flush().map_err(|_| storage_error())?;
        tmp.as_file().sync_all().map_err(|_| storage_error())?;
        tmp.persist(self.root.join("chatgpt-auth.json"))
            .map_err(|_| storage_error())?;
        Ok(())
    }
    #[cfg(unix)]
    pub fn load_tokens(&self, state: &StateFile) -> Result<Option<TokenRecord>> {
        Ok(state.tokens.clone())
    }
    #[cfg(unix)]
    pub fn save_tokens(&self, state: &mut StateFile, tokens: Option<TokenRecord>) -> Result<()> {
        state.tokens = tokens;
        self.save(state)
    }
    #[cfg(windows)]
    pub fn load_tokens(&self, state: &StateFile) -> Result<Option<TokenRecord>> {
        let Some(reg) = &state.registration else {
            return Ok(None);
        };
        let path = credential_path(&self.root, &state.host_id, reg);
        let sealed = match fs::read(&path) {
            Ok(data) if data.len() <= 64 * 1024 => data,
            Ok(_) => return Err(storage_error()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(storage_error()),
        };
        if sealed.len() < 12 + 16 {
            return Err(storage_error());
        }
        let entry = keyring_entry(&state.host_id, reg)?;
        let key = entry.get_password().map_err(|_| storage_error())?;
        let key = decode_key(&key)?;
        let aead = ring::aead::LessSafeKey::new(
            ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key)
                .map_err(|_| storage_error())?,
        );
        let mut body = sealed[12..].to_vec();
        let plain = aead
            .open_in_place(
                ring::aead::Nonce::try_assume_unique_for_key(&sealed[..12])
                    .map_err(|_| storage_error())?,
                ring::aead::Aad::from(reg_key(&state.host_id, reg).as_bytes()),
                &mut body,
            )
            .map_err(|_| storage_error())?;
        serde_json::from_slice(plain)
            .map(Some)
            .map_err(|_| storage_error())
    }
    #[cfg(windows)]
    pub fn save_tokens(&self, state: &mut StateFile, tokens: Option<TokenRecord>) -> Result<()> {
        let reg = state.registration.as_ref().ok_or_else(storage_error)?;
        let entry = keyring_entry(&state.host_id, reg)?;
        let path = credential_path(&self.root, &state.host_id, reg);
        match tokens {
            Some(tokens) => {
                let key = match entry.get_password() {
                    Ok(value) => decode_key(&value)?,
                    Err(keyring::Error::NoEntry) => {
                        let mut bytes = [0u8; 32];
                        getrandom::fill(&mut bytes).map_err(|_| storage_error())?;
                        use base64::Engine as _;
                        entry
                            .set_password(
                                &base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes),
                            )
                            .map_err(|_| storage_error())?;
                        bytes
                    }
                    Err(_) => return Err(storage_error()),
                };
                let aead = ring::aead::LessSafeKey::new(
                    ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key)
                        .map_err(|_| storage_error())?,
                );
                let mut nonce = [0u8; 12];
                getrandom::fill(&mut nonce).map_err(|_| storage_error())?;
                let mut body = serde_json::to_vec(&tokens).map_err(|_| storage_error())?;
                aead.seal_in_place_append_tag(
                    ring::aead::Nonce::assume_unique_for_key(nonce),
                    ring::aead::Aad::from(reg_key(&state.host_id, reg).as_bytes()),
                    &mut body,
                )
                .map_err(|_| storage_error())?;
                let mut sealed = nonce.to_vec();
                sealed.extend(body);
                secure_dir(&self.root)?;
                let mut tmp = tempfile::Builder::new()
                    .prefix(".chatgpt-credentials-")
                    .tempfile_in(&self.root)
                    .map_err(|_| storage_error())?;
                tmp.write_all(&sealed).map_err(|_| storage_error())?;
                tmp.as_file().sync_all().map_err(|_| storage_error())?;
                tmp.persist(path).map_err(|_| storage_error())?;
            }
            None => {
                self.save(state)?; // Invalidate pending callbacks before deleting the ciphertext.
                match fs::remove_file(path) {
                    Ok(()) => (),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                    Err(_) => return Err(storage_error()),
                }
                let _ = entry.delete_credential(); // An orphan key cannot decrypt a deleted file.
                return Ok(());
            }
        }
        self.save(state)
    }
}
#[cfg(windows)]
fn keyring_entry(host_id: &str, reg: &Registration) -> Result<keyring::Entry> {
    keyring::Entry::new("local-ai-kit.chatgpt", &reg_key(host_id, reg)).map_err(|_| storage_error())
}
#[cfg(windows)]
fn reg_key(host_id: &str, reg: &Registration) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(format!("{}\0{}\0{}", host_id, reg.client_id, reg.subject));
    format!("{:x}", digest)
}
#[cfg(windows)]
fn credential_path(root: &Path, host_id: &str, reg: &Registration) -> PathBuf {
    root.join(format!("chatgpt-{}.credentials", reg_key(host_id, reg)))
}
#[cfg(windows)]
fn decode_key(value: &str) -> Result<[u8; 32]> {
    use base64::Engine as _;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| storage_error())?;
    bytes.try_into().map_err(|_| storage_error())
}
fn secure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path).map_err(|_| storage_error())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| storage_error())?;
    }
    Ok(())
}
