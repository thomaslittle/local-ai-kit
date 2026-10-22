use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_service::{ExecutionGrant, NativeService};
use lai_transports::local_ipc::{Broker, BrokerOptions};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
fn err() -> DomainError {
    DomainError::new(
        ErrorCode::ProviderUnavailable,
        "Broker control channel closed",
    )
}

#[derive(Debug, Clone)]
pub enum Lifetime {
    /// The trusted parent owns the broker. Closing its control input stops the
    /// service, which is the historical embedded/managed-helper behavior.
    Parent,
    /// The broker owns its own lifetime. Applications may come and go without
    /// terminating it, and a second launch converges on the existing listener.
    Independent {
        token_file: std::path::PathBuf,
        idle_timeout: Option<Duration>,
    },
}

fn private_write(path: &std::path::Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| {
            DomainError::new(
                ErrorCode::StorageUnavailable,
                "Broker token directory is unavailable",
            )
        })?;
    }
    let temporary = path.with_extension("json.tmp");
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
        .open(&temporary)
        .and_then(|mut file| {
            use std::io::Write;
            file.write_all(bytes)
        })
        .map_err(|_| {
            DomainError::new(
                ErrorCode::StorageUnavailable,
                "Broker token file could not be written",
            )
        })?;
    std::fs::rename(&temporary, path).map_err(|_| {
        DomainError::new(
            ErrorCode::StorageUnavailable,
            "Broker token file could not be published",
        )
    })?;
    #[cfg(windows)]
    {
        // The file contains an owner credential. Remove inherited ACEs and
        // grant only the current user. Fail closed if the ACL cannot be set.
        let user = std::env::var("USERNAME").map_err(|_| {
            DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Broker token owner is unavailable",
            )
        })?;
        let status = std::process::Command::new("icacls")
            .args([
                path.as_os_str(),
                std::ffi::OsStr::new("/inheritance:r"),
                std::ffi::OsStr::new("/grant:r"),
                std::ffi::OsStr::new(&format!("{user}:F")),
            ])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|_| {
                DomainError::new(
                    ErrorCode::StorageUnavailable,
                    "Broker token ACL could not be set",
                )
            })?;
        if !status.success() {
            let _ = std::fs::remove_file(path);
            return Err(DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Broker token ACL could not be restricted to the owner",
            ));
        }
    }
    Ok(())
}

fn descriptor(
    broker: &Broker,
    address: &str,
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
    reusable: bool,
) -> Result<serde_json::Value> {
    let token = if reusable {
        broker.authorize_shared(policy, execution)?
    } else {
        broker.authorize(policy, execution)?
    };
    Ok(serde_json::json!({
        "protocolVersion": "1.0",
        "address": address,
        "token": token,
        "expiresInSeconds": 300,
        "independent": reusable,
        "pid": std::process::id(),
    }))
}

pub async fn connect(address: &str) -> Result<()> {
    let stream = lai_transports::local_ipc::connect(address).await?;
    let (mut r, mut w) = tokio::io::split(stream);
    let (mut input, mut output) = (tokio::io::stdin(), tokio::io::stdout());
    tokio::select! {
     result=async {tokio::io::copy(&mut input,&mut w).await?;w.shutdown().await}=>{result.map_err(|_|err())?;},
     result=tokio::io::copy(&mut r,&mut output)=>{result.map_err(|_|err())?;}
    }
    Ok(())
}

pub async fn serve(
    service: Arc<NativeService>,
    address: &str,
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
    lifetime: Lifetime,
) -> Result<()> {
    match lifetime {
        Lifetime::Parent => serve_parent(service, address, policy, execution).await,
        Lifetime::Independent {
            token_file,
            idle_timeout,
        } => {
            serve_independent(
                service,
                address,
                policy,
                execution,
                token_file,
                idle_timeout,
            )
            .await
        }
    }
}

async fn serve_parent(
    service: Arc<NativeService>,
    address: &str,
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
) -> Result<()> {
    let broker = Broker::start(service, address).await?;
    // This stdout is a trusted parent control channel, not a diagnostic log.
    // Each bootstrap is single-use. No provider credential is involved.
    let send = || -> Result<()> {
        println!(
            "{}",
            descriptor(&broker, address, policy.clone(), execution.clone(), false)?
        );
        Ok(())
    };
    send()?;
    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut bytes = Vec::new();
        let n = (&mut reader)
            .take(1025)
            .read_until(b'\n', &mut bytes)
            .await
            .map_err(|_| err())?;
        if n == 0 {
            break;
        }
        if n > 1024 || !bytes.ends_with(b"\n") {
            break;
        }
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_default();
        if value == serde_json::json!({"action":"authorize"}) {
            send()?;
        } else {
            break;
        }
    }
    broker.close().await;
    Ok(())
}

async fn serve_independent(
    service: Arc<NativeService>,
    address: &str,
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
    token_file: std::path::PathBuf,
    idle_timeout: Option<Duration>,
) -> Result<()> {
    if idle_timeout.is_some_and(|timeout| timeout < Duration::from_secs(5)) {
        return Err(DomainError::new(
            ErrorCode::InvalidRequest,
            "Idle timeout must be at least five seconds",
        ));
    }
    // Endpoint binding is the race-free single-instance boundary. A second
    // launch gets Conflict and must connect to the existing service instead of
    // replacing it.
    let broker = Broker::start_with(service, address, BrokerOptions { idle_timeout: None }).await?;
    let value = descriptor(&broker, address, policy, execution, true)?;
    let bytes = serde_json::to_vec(&value).map_err(|_| err())?;
    private_write(&token_file, &bytes)?;
    println!("{value}");
    let started = std::time::Instant::now();
    let mut idle_since = std::time::Instant::now();
    let mut tick = tokio::time::interval(Duration::from_secs(1));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            _ = tick.tick() => {
                if broker.active_connections() == 0 {
                    if idle_timeout.is_some_and(|timeout| idle_since.elapsed() >= timeout) {
                        break;
                    }
                } else {
                    idle_since = std::time::Instant::now();
                }
                // A clock jump backwards must not extend the lease forever.
                if idle_timeout.is_some_and(|timeout| started.elapsed() > timeout.saturating_mul(2) && broker.active_connections() == 0) {
                    break;
                }
            }
        }
    }
    broker.close().await;
    let _ = std::fs::remove_file(&token_file);
    Ok(())
}
