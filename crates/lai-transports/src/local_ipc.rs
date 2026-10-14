//! Shared local broker. OS endpoint permissions and one-use host grants are separate boundaries.
#[cfg(unix)]
use interprocess::local_socket::GenericFilePath;
#[cfg(windows)]
use interprocess::local_socket::GenericNamespaced;
use interprocess::local_socket::{
    ListenerOptions,
    tokio::{Stream, prelude::*},
};
use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_service::{ExecutionGrant, NativeService};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    sync::{Semaphore, oneshot},
    task::JoinSet,
};
struct Grant {
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
    created: Instant,
}
struct Shared {
    service: Arc<NativeService>,
    pending: Mutex<BTreeMap<[u8; 32], Grant>>,
}
pub struct Broker {
    shared: Arc<Shared>,
    stop: Option<oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<()>>,
}
fn fail(code: ErrorCode) -> DomainError {
    DomainError::new(code, "Local broker operation failed")
}
fn hash(s: &str) -> [u8; 32] {
    Sha256::digest(s.as_bytes()).into()
}
fn key() -> Result<String> {
    let mut b = [0u8; 32];
    getrandom::fill(&mut b).map_err(|_| fail(ErrorCode::ProviderUnavailable))?;
    Ok(b.iter().map(|b| format!("{b:02x}")).collect())
}
fn name(value: &str) -> Result<interprocess::local_socket::Name<'_>> {
    if value.is_empty()
        || value.len() > 200
        || value.chars().any(char::is_control)
        || value.contains("://")
        || value.starts_with("//")
        || value.starts_with(r"\\")
    {
        return Err(fail(ErrorCode::InvalidRequest));
    }
    #[cfg(windows)]
    {
        if !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        {
            return Err(fail(ErrorCode::InvalidRequest));
        }
        value
            .to_ns_name::<GenericNamespaced>()
            .map_err(|_| fail(ErrorCode::InvalidRequest))
    }
    #[cfg(unix)]
    {
        if !std::path::Path::new(value).is_absolute() {
            return Err(fail(ErrorCode::InvalidRequest));
        }
        value
            .to_fs_name::<GenericFilePath>()
            .map_err(|_| fail(ErrorCode::InvalidRequest))
    }
}
/// The listener builder cannot set a socket mode on every Unix (macOS reports it unsupported), so
/// those platforms restrict the endpoint to its owner right after binding and fail closed if they
/// cannot. A connection also needs a one-use host grant, so the instant before this call exposes
/// no usable access.
#[cfg(all(unix, not(any(target_os = "linux", target_os = "android"))))]
fn restrict_endpoint(address: &str) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(address, std::fs::Permissions::from_mode(0o600)).map_err(|_| {
        let _ = std::fs::remove_file(address);
        fail(ErrorCode::LocalAccessDenied)
    })
}
impl Broker {
    pub async fn start(service: Arc<NativeService>, address: &str) -> Result<Self> {
        let options = ListenerOptions::new().name(name(address)?);
        #[cfg(windows)]
        let options = {
            use interprocess::os::windows::{
                local_socket::ListenerOptionsExt, security_descriptor::SecurityDescriptor,
            };
            let sddl = widestring::U16CString::from_str("D:P(A;;GA;;;OW)")
                .map_err(|_| fail(ErrorCode::LocalAccessDenied))?;
            options.security_descriptor(
                SecurityDescriptor::deserialize(&sddl)
                    .map_err(|_| fail(ErrorCode::LocalAccessDenied))?,
            )
        };
        #[cfg(any(target_os = "linux", target_os = "android"))]
        let options = {
            use interprocess::os::unix::local_socket::ListenerOptionsExt;
            options.mode(0o600)
        };
        let listener = options
            .create_tokio()
            .map_err(|_| fail(ErrorCode::Conflict))?;
        #[cfg(all(unix, not(any(target_os = "linux", target_os = "android"))))]
        restrict_endpoint(address)?;
        let shared = Arc::new(Shared {
            service,
            pending: Mutex::new(BTreeMap::new()),
        });
        let state = shared.clone();
        let (stop, mut stopped) = oneshot::channel();
        let task = tokio::spawn(async move {
            let permits = Arc::new(Semaphore::new(32));
            let mut jobs = JoinSet::new();
            loop {
                tokio::select! {
                 _=&mut stopped=>break,
                 _=jobs.join_next(),if !jobs.is_empty()=>{},
                 incoming=listener.accept()=>match incoming{
                  Ok(stream)=>{if let Ok(permit)=permits.clone().try_acquire_owned(){let state=state.clone();jobs.spawn(async move{let _p=permit;let _=serve_connection(stream,state).await;});}},
                  Err(_)=>break,
                 }
                }
            }
            jobs.abort_all();
            while jobs.join_next().await.is_some() {}
        });
        Ok(Self {
            shared,
            stop: Some(stop),
            task: Some(task),
        })
    }
    /// Pass this one-use credential over the embedding application's trusted channel, never logs or argv.
    pub fn authorize(
        &self,
        policy: ClientPolicy,
        execution: Option<ExecutionGrant>,
    ) -> Result<String> {
        let token = key()?;
        let mut map = self
            .shared
            .pending
            .lock()
            .map_err(|_| fail(ErrorCode::ProviderUnavailable))?;
        map.retain(|_, g| g.created.elapsed() < Duration::from_secs(300));
        if map.len() >= 64 {
            return Err(fail(ErrorCode::RateLimited));
        }
        map.insert(
            hash(&token),
            Grant {
                policy,
                execution,
                created: Instant::now(),
            },
        );
        Ok(token)
    }
    pub async fn close(mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }
}
impl Drop for Broker {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
    }
}
struct Owner {
    service: Arc<NativeService>,
    client: String,
}
impl Drop for Owner {
    fn drop(&mut self) {
        self.service.unregister_client(&self.client);
    }
}
async fn serve_connection(stream: Stream, state: Arc<Shared>) -> Result<()> {
    let mut stream = BufReader::new(stream);
    let mut bytes = Vec::new();
    tokio::time::timeout(
        Duration::from_secs(5),
        (&mut stream).take(4097).read_until(b'\n', &mut bytes),
    )
    .await
    .map_err(|_| fail(ErrorCode::Timeout))?
    .map_err(|_| fail(ErrorCode::ProviderUnavailable))?;
    let request = crate::wire::parse(&bytes).ok().flatten();
    let id = request
        .as_ref()
        .map(|r| r.id.clone())
        .unwrap_or(Value::Null);
    let token = request
        .as_ref()
        .filter(|r| {
            r.method == "broker.authenticate" && r.params.as_object().is_some_and(|p| p.len() == 1)
        })
        .and_then(|r| r.params["token"].as_str())
        .filter(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit()));
    let grant = token
        .and_then(|token| state.pending.lock().ok()?.remove(&hash(token)))
        .filter(|g| g.created.elapsed() < Duration::from_secs(300));
    let Some(grant) = grant else {
        let value = crate::wire::reply(id, Err(fail(ErrorCode::LocalAccessDenied)));
        let _ = stream
            .get_mut()
            .write_all(format!("{value}\n").as_bytes())
            .await;
        return Err(fail(ErrorCode::LocalAccessDenied));
    };
    if bytes.len() > 4096 || !bytes.ends_with(b"\n") {
        return Err(fail(ErrorCode::InvalidRequest));
    }
    let client = format!("ipc-{}", &key()?[..32]);
    state.service.register_client(&client, grant.policy)?;
    state.service.set_app_identity(&client, "ipc-host")?;
    let _owner = Owner {
        service: state.service.clone(),
        client: client.clone(),
    };
    if let Some(grant) = grant.execution {
        state.service.register_execution(&client, grant)?;
    }
    stream
        .get_mut()
        .write_all(
            format!(
                "{}\n",
                json!({"jsonrpc":"2.0","id":id,"result":{"authenticated":true}})
            )
            .as_bytes(),
        )
        .await
        .map_err(|_| fail(ErrorCode::ProviderUnavailable))?;
    let (reader, writer) = tokio::io::split(stream);
    crate::serve_io(
        reader,
        writer,
        state.service.clone(),
        client,
        Default::default(),
    )
    .await
}
/// Explicit connector used by SDK bridge processes. No daemon is implicitly started.
pub async fn connect(address: &str) -> Result<Stream> {
    Stream::connect(name(address)?)
        .await
        .map_err(|_| fail(ErrorCode::RuntimeMissing))
}
