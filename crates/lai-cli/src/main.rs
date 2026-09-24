mod args;
mod host;
use args::{Action, Args};
use lai_contracts::{DomainError, ErrorCode, Result};
use lai_discovery::{HostPlatform, ScanEnvironment, Scanner};
use lai_platform::NativeFilesystem;
use lai_service::InventorySource;
use serde_json::json;
use std::{path::PathBuf, sync::Arc, time::Duration};

const HELP: &str = "Local AI Kit - native discovery and SDK helper
Usage:
  local-ai-kit scan --json [--home ABSOLUTE_DIRECTORY]
  local-ai-kit serve --stdio [--allow-passive-discovery] [--allow-codex-metadata] [--home ABSOLUTE_DIRECTORY]
  local-ai-kit serve --http --port 4499 --assets ABSOLUTE_DIST_DIRECTORY
      [--allow-passive-discovery] [--allow-codex-metadata]
      [--allow-codex-execution --workspace ABSOLUTE_DIRECTORY]
      [--allow-writes] [--review-native-requests] [--allow-paid-options]
  Reusable provider grants: --allow-metadata PROVIDER --allow-execution PROVIDER
  Execution providers: codex, claude_code. Metadata also supports grok. Repeated grants are supported.
  local-ai-kit --version

Serving never scans on startup. --allow-passive-discovery explicitly authorizes
metadata-only scans requested by the owning SDK. No credentials or inference.
--allow-codex-metadata separately permits explicit connections.probe requests
through the installed Codex client. This may read its login and contact its
account/catalog/usage services, but never starts threads or model generation.
--allow-codex-execution authorizes explicit session/turn requests, never generation
at startup. A --workspace is required; its public name is default. Writes, native
approval review, and paid options each require their own flag. HTTP is loopback-only
and requires explicit single-use browser pairing. No tokens are printed.
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
    let service = host::build_service(source(args.home.clone()), args.allow_execution);
    let policy = host::policy(&args);
    let grant = host::execution_grant(&args);
    match args.action {
        Action::Scan => {
            service.register_client("stdio-owner", policy)?;
            service
                .handle(
                    "stdio-owner",
                    "discovery.refresh",
                    json!({"level":"passive"}),
                )
                .await?;
            println!(
                "{}",
                service
                    .handle("stdio-owner", "snapshot.get", json!({}))
                    .await?
            );
            Ok(())
        }
        Action::Serve if args.http => {
            let server = lai_transports::http::start(
                service,
                lai_transports::http::HttpOptions {
                    port: args.port,
                    policy,
                    execution: grant,
                    assets: args.assets,
                },
            )
            .await?;
            println!("Local AI Kit: {}", server.url);
            println!(
                "Pairing code (single use, expires in five minutes): {}",
                server.pairing_code
            );
            tokio::signal::ctrl_c().await.map_err(|_| {
                DomainError::new(
                    ErrorCode::ProviderUnavailable,
                    "Shutdown signal unavailable",
                )
            })?;
            server.close().await;
            Ok(())
        }
        Action::Serve => {
            service.register_client("stdio-owner", policy)?;
            if let Some(grant) = grant {
                service.register_execution("stdio-owner", grant)?;
            }
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
