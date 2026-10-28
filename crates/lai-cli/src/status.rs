//! `local-ai-kit status`: the same facts the dashboard badge shows.
//!
//! Exit codes: 0 running and granted, 3 no broker is listening, 4 the broker
//! rejected this application's credential. Other failures are 1.
use lai_contracts::ErrorCode;
use lai_control::BrokerClient;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::time::Duration;

fn home() -> Option<PathBuf> {
    let key = if cfg!(windows) {
        "USERPROFILE"
    } else {
        "HOME"
    };
    std::env::var_os(key)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

fn default_address() -> String {
    if let Some(value) = std::env::var_os("LAI_BROKER_ADDRESS") {
        if let Some(value) = value.to_str().filter(|value| !value.is_empty()) {
            return value.to_string();
        }
    }
    if cfg!(windows) {
        return "local-ai-kit-user".into();
    }
    if let Some(runtime) = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from) {
        if runtime.is_absolute() {
            return runtime
                .join("local-ai-kit/user.sock")
                .to_string_lossy()
                .into();
        }
    }
    home()
        .map(|home| home.join(".local-ai-kit/user.sock"))
        .unwrap_or_else(|| PathBuf::from("/tmp/local-ai-kit-user.sock"))
        .to_string_lossy()
        .into()
}

fn default_token_file() -> Option<PathBuf> {
    if let Some(value) = std::env::var_os("LAI_BROKER_TOKEN_FILE") {
        return Some(PathBuf::from(value));
    }
    home().map(|home| home.join(".local-ai-kit/broker-token.json"))
}

fn token_from_file(path: &std::path::Path) -> Option<String> {
    let info = path.symlink_metadata().ok()?;
    if !info.is_file() || info.file_type().is_symlink() || info.len() > 16 * 1024 {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if info.permissions().mode() & 0o077 != 0 {
            return None;
        }
    }
    let value: Value = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
    let token = value.get("token")?.as_str()?;
    (token.len() == 64 && token.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| token.to_string())
}

fn listening(address: &str) -> bool {
    if !cfg!(windows)
        && std::path::Path::new(address).is_absolute()
        && !std::path::Path::new(address).exists()
    {
        return false;
    }
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    else {
        return false;
    };
    let connected = runtime.block_on(async {
        tokio::time::timeout(
            Duration::from_secs(2),
            lai_transports::local_ipc::connect(address),
        )
        .await
    });
    drop(runtime);
    connected.map(|result| result.is_ok()).unwrap_or(false)
}

pub fn run(
    address: Option<String>,
    token_file: Option<PathBuf>,
    json: bool,
) -> lai_contracts::Result<u8> {
    let address = address.unwrap_or_else(default_address);
    let token = token_file
        .or_else(default_token_file)
        .as_deref()
        .and_then(token_from_file);
    let Some(token) = token else {
        if json {
            println!(
                "{}",
                json!({"running": false, "reason": "NO_CREDENTIAL", "address": address})
            );
        } else {
            println!("OriginLM helper: not configured (no owner-only token descriptor)");
            println!("  expected: ~/.local-ai-kit/broker-token.json");
        }
        return Ok(3);
    };
    if !listening(&address) {
        if json {
            println!(
                "{}",
                json!({"running": false, "reason": "NOT_RUNNING", "address": address})
            );
        } else {
            println!("OriginLM helper: not running");
            println!("  address: {address}");
        }
        return Ok(3);
    }
    let client = BrokerClient::connect(&address, &token).ok();
    let Some(mut client) = client else {
        if json {
            println!(
                "{}",
                json!({"running": true, "authorized": false, "reason": "LOCAL_ACCESS_DENIED", "address": address})
            );
        } else {
            eprintln!("OriginLM helper: running, but this credential was rejected");
        }
        return Ok(4);
    };
    let snapshot = client
        .snapshot()
        .map_err(|_| lai_contracts::DomainError::new(ErrorCode::ProviderUnavailable, "snapshot"))?;
    let health = client.call("runtime.health", json!({})).unwrap_or(Value::Null);
    let apps = client.apps().ok();
    let providers: Vec<String> = snapshot
        .providers
        .iter()
        .map(|provider| provider.name.clone())
        .collect();
    if json {
        println!(
            "{}",
            json!({
                "running": true,
                "authorized": true,
                "mode": "shared",
                "address": address,
                "version": env!("CARGO_PKG_VERSION"),
                "protocolVersion": snapshot.protocol_version,
                "revision": snapshot.revision,
                "observedAt": snapshot.observed_at,
                "providers": providers,
                "models": snapshot.models.len(),
                "connectedApps": apps.as_ref().map(|apps| apps.len()),
                "health": health,
            })
        );
    } else {
        println!(
            "OriginLM helper: running (protocol {}, revision {})",
            snapshot.protocol_version, snapshot.revision
        );
        println!("  version: {}", env!("CARGO_PKG_VERSION"));
        println!(
            "  providers: {}",
            if providers.is_empty() {
                "none observed".to_string()
            } else {
                providers.join(", ")
            }
        );
        println!("  models: {}", snapshot.models.len());
        match apps {
            Some(apps) => println!("  connected apps: {}", apps.len()),
            None => println!("  connected apps: requires the administrator credential"),
        }
    }
    Ok(0)
}
