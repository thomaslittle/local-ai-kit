//! Bounded native protocol worker. The trusted host supplies the executable; no shell is used.
use lai_contracts::{ConnectionMetadata, DomainError, ErrorCode, Result};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    path::PathBuf,
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    task::JoinHandle,
};

const MAX_FRAME: u64 = 1_048_576;
#[derive(Clone)]
pub struct LaunchSpec {
    pub executable: PathBuf,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
    pub env: BTreeMap<OsString, OsString>,
}
struct Peer {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    stderr: JoinHandle<()>,
    sequence: u64,
    account_established: bool,
}
fn error(code: ErrorCode) -> DomainError {
    DomainError::new(code, "Native client metadata request failed")
}
fn validate(spec: &LaunchSpec) -> Result<()> {
    let extension = spec
        .executable
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !spec.executable.is_absolute()
        || !spec.cwd.is_absolute()
        || ["cmd", "bat", "ps1"].contains(&extension.as_str())
        || spec.args.len() > 128
    {
        return Err(error(ErrorCode::InvalidRequest));
    }
    if spec
        .args
        .iter()
        .any(|v| v.len() > 32768 || v.as_encoded_bytes().contains(&0))
    {
        return Err(error(ErrorCode::InvalidRequest));
    }
    for (key, value) in &spec.env {
        let name = key
            .to_str()
            .ok_or_else(|| error(ErrorCode::InvalidRequest))?;
        if name.is_empty()
            || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
            || value.as_encoded_bytes().contains(&0)
            || [
                "NODE_OPTIONS",
                "LD_PRELOAD",
                "DYLD_INSERT_LIBRARIES",
                "PYTHONPATH",
                "PYTHONHOME",
                "PYTHONSTARTUP",
            ]
            .contains(&name.to_ascii_uppercase().as_str())
        {
            return Err(error(ErrorCode::InvalidRequest));
        }
    }
    Ok(())
}
impl Peer {
    async fn spawn(spec: LaunchSpec) -> Result<Self> {
        validate(&spec)?;
        let mut command = Command::new(&spec.executable);
        command
            .args(spec.args)
            .current_dir(spec.cwd)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        for key in [
            "PATH",
            "Path",
            "SystemRoot",
            "SYSTEMROOT",
            "WINDIR",
            "HOME",
            "USERPROFILE",
            "APPDATA",
            "LOCALAPPDATA",
            "TMP",
            "TEMP",
            "TMPDIR",
            "LANG",
            "XDG_CONFIG_HOME",
            "XDG_DATA_HOME",
            "XDG_RUNTIME_DIR",
            "DBUS_SESSION_BUS_ADDRESS",
        ] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        command.envs(spec.env);
        #[cfg(windows)]
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW; no shell or terminal attachment.
        let mut child = command
            .spawn()
            .map_err(|_| error(ErrorCode::RuntimeMissing))?;
        let input = child
            .stdin
            .take()
            .ok_or_else(|| error(ErrorCode::ProviderUnavailable))?;
        let output = BufReader::new(
            child
                .stdout
                .take()
                .ok_or_else(|| error(ErrorCode::ProviderUnavailable))?,
        );
        let mut stderr = child
            .stderr
            .take()
            .ok_or_else(|| error(ErrorCode::ProviderUnavailable))?;
        let drain = tokio::spawn(async move {
            let _ = tokio::io::copy(&mut stderr, &mut tokio::io::sink()).await;
        });
        Ok(Self {
            child,
            input,
            output,
            stderr: drain,
            sequence: 0,
            account_established: false,
        })
    }
    async fn send(&mut self, value: Value) -> Result<()> {
        let mut bytes = serde_json::to_vec(&value).map_err(|_| error(ErrorCode::InvalidRequest))?;
        if bytes.len() as u64 >= MAX_FRAME {
            return Err(error(ErrorCode::FrameTooLarge));
        }
        bytes.push(b'\n');
        self.input
            .write_all(&bytes)
            .await
            .map_err(|_| error(ErrorCode::ProviderUnavailable))?;
        self.input
            .flush()
            .await
            .map_err(|_| error(ErrorCode::ProviderUnavailable))
    }
    async fn read(&mut self) -> Result<Value> {
        let mut bytes = Vec::new();
        let count = (&mut self.output)
            .take(MAX_FRAME + 1)
            .read_until(b'\n', &mut bytes)
            .await
            .map_err(|_| error(ErrorCode::ProviderUnavailable))?;
        if count == 0 {
            return Err(error(ErrorCode::ProviderUnavailable));
        }
        if count as u64 > MAX_FRAME {
            return Err(error(ErrorCode::FrameTooLarge));
        }
        if bytes.last() != Some(&b'\n') {
            return Err(error(ErrorCode::ProtocolIncompatible));
        }
        serde_json::from_slice(&bytes).map_err(|_| error(ErrorCode::ProtocolIncompatible))
    }
    async fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        self.sequence += 1;
        let id = self.sequence;
        self.send(json!({"id":id,"method":method,"params":params}))
            .await?;
        for _ in 0..512 {
            let message = self.read().await?;
            if !message.is_object() {
                return Err(error(ErrorCode::ProtocolIncompatible));
            }
            if message.get("method").is_some() {
                if message.get("id").is_some() {
                    return Err(error(ErrorCode::IntegrationRestricted));
                }
                if message["method"] == "account/updated" && self.account_established {
                    return Err(error(ErrorCode::SelectionStale));
                }
                continue;
            }
            if message["id"] != id {
                return Err(error(ErrorCode::ProtocolIncompatible));
            }
            match (message.get("result"), message.get("error")) {
                (Some(result), None) => return Ok(result.clone()),
                (None, Some(_)) => return Err(error(ErrorCode::ProviderUnavailable)),
                _ => return Err(error(ErrorCode::ProtocolIncompatible)),
            }
        }
        Err(error(ErrorCode::FrameTooLarge))
    }
    async fn shutdown(&mut self) {
        let _ = self.input.shutdown().await;
        if tokio::time::timeout(Duration::from_millis(250), self.child.wait())
            .await
            .is_err()
        {
            let _ = self.child.kill().await;
            let _ = self.child.wait().await;
        }
        self.stderr.abort();
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.stderr.abort();
        let _ = self.child.start_kill();
    }
}

pub async fn codex_metadata(
    spec: LaunchSpec,
    connection: &str,
    route: &str,
    deadline: Duration,
) -> Result<ConnectionMetadata> {
    if deadline.is_zero() || deadline > Duration::from_secs(60) {
        return Err(error(ErrorCode::InvalidRequest));
    }
    let mut peer = Peer::spawn(spec).await?;
    let result = tokio::time::timeout(deadline, exchange(&mut peer, connection, route))
        .await
        .unwrap_or_else(|_| Err(error(ErrorCode::Timeout)));
    peer.shutdown().await;
    result
}
async fn exchange(peer: &mut Peer, connection: &str, route: &str) -> Result<ConnectionMetadata> {
    let initialize=peer.request("initialize",json!({"clientInfo":{"name":"local_ai_kit","title":"Local AI Kit","version":env!("CARGO_PKG_VERSION")},"capabilities":{"experimentalApi":false}})).await?;
    peer.send(json!({"method":"initialized","params":{}}))
        .await?;
    let account = peer
        .request("account/read", json!({"refreshToken":false}))
        .await?;
    peer.account_established = true;
    if account["account"].is_null() {
        return crate::codex::parse_metadata(connection, route, &initialize, &account, &[], None);
    }
    let mut models = Vec::new();
    let mut cursors = BTreeSet::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut params = json!({"limit":100,"includeHidden":false});
        if let Some(value) = &cursor {
            params["cursor"] = json!(value);
        }
        let page = peer.request("model/list", params).await?;
        let values = page["data"]
            .as_array()
            .ok_or_else(|| error(ErrorCode::ProtocolIncompatible))?;
        if models.len() + values.len() > 2048 {
            return Err(error(ErrorCode::FrameTooLarge));
        }
        models.extend(values.iter().cloned());
        cursor = match page.get("nextCursor") {
            None | Some(Value::Null) => None,
            Some(Value::String(next)) if !next.is_empty() && next.len() <= 4096 => {
                Some(next.clone())
            }
            _ => return Err(error(ErrorCode::ProtocolIncompatible)),
        };
        let Some(next) = &cursor else {
            break;
        };
        if !cursors.insert(next.clone()) || cursors.len() > 32 {
            return Err(error(ErrorCode::ProtocolIncompatible));
        }
    }
    let limits = match peer.request("account/rateLimits/read", json!({})).await {
        Ok(value) => Some(value),
        Err(e) if e.code == ErrorCode::ProviderUnavailable => None,
        Err(error) => return Err(error),
    };
    crate::codex::parse_metadata(
        connection,
        route,
        &initialize,
        &account,
        &models,
        limits.as_ref(),
    )
}
