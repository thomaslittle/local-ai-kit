mod args;
use args::{Action, Args};
use lai_contracts::{DomainError, ErrorCode, Result};
use lai_core::ClientPolicy;
use lai_discovery::{HostPlatform, ScanEnvironment, Scanner};
use lai_platform::NativeFilesystem;
use lai_service::{InventorySource, NativeService};
use serde_json::json;
use std::{path::PathBuf, sync::Arc, time::Duration};

const HELP: &str = "Local AI Kit - native discovery and SDK helper
Usage:
  local-ai-kit scan --json [--home ABSOLUTE_DIRECTORY]
  local-ai-kit serve --stdio [--allow-passive-discovery] [--home ABSOLUTE_DIRECTORY]
  local-ai-kit --version

Serving never scans on startup. --allow-passive-discovery explicitly authorizes
metadata-only scans requested by the owning SDK. No credentials or inference.
--home isolates all discovery roots and excludes the inherited PATH/overrides.";

fn source(home: Option<PathBuf>) -> InventorySource {
    Arc::new(move |cancel| {
        let environment = if let Some(home) = &home {
            let platform = if cfg!(windows) {
                HostPlatform::Windows
            } else if cfg!(target_os = "macos") {
                HostPlatform::Macos
            } else {
                HostPlatform::Linux
            };
            ScanEnvironment::for_home(home.clone(), platform)
        } else {
            ScanEnvironment::from_process()?
        };
        Scanner::new(NativeFilesystem).scan(&environment, cancel, |_| {})
    })
}
async fn execute(args: Args) -> Result<()> {
    let service = Arc::new(NativeService::new(
        source(args.home),
        Duration::from_secs(2),
    ));
    let policy = if args.allow_passive || matches!(args.action, Action::Scan) {
        ClientPolicy::passive()
    } else {
        ClientPolicy::denied()
    };
    service.register_client("stdio-owner", policy)?;
    match args.action {
        Action::Scan => {
            service
                .handle(
                    "stdio-owner",
                    "discovery.refresh",
                    json!({"level":"passive"}),
                )
                .await?;
            let data = service
                .handle("stdio-owner", "snapshot.get", json!({}))
                .await?;
            println!("{}", data);
            Ok(())
        }
        Action::Serve => {
            lai_transports::serve_io(
                tokio::io::stdin(),
                tokio::io::stdout(),
                service,
                "stdio-owner".into(),
                Default::default(),
            )
            .await
        }
        _ => Ok(()),
    }
}
fn main() -> std::process::ExitCode {
    let result = (|| {
        let args = Args::parse(std::env::args_os().skip(1))?;
        match args.action {
            Action::Help => {
                println!("{HELP}");
                return Ok(());
            }
            Action::Version => {
                println!("local-ai-kit {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {}
        }
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .max_blocking_threads(4)
            .enable_all()
            .build()
            .map_err(|_| DomainError::new(ErrorCode::ProviderUnavailable, "Runtime unavailable"))?;
        let result = runtime.block_on(execute(args));
        // Tokio stdin uses a blocking thread; an idle pipe must not hold shutdown forever.
        runtime.shutdown_timeout(Duration::from_millis(100));
        result
    })();
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::from(2)
        }
    }
}
