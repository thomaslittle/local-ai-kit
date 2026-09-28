mod args;
mod inventory;
use inventory::source;
use lai_service::InventorySource;
mod broker_cli;
mod endpoints;
mod host;
mod native_profiles;
use args::{Action, Args};
use lai_contracts::{DomainError, ErrorCode, Result};
use serde_json::json;
use std::{sync::Arc, time::Duration};

const HELP: &str = "Local AI Kit - native discovery and SDK helper
Usage:
  local-ai-kit serve --ipc LOCAL_ADDRESS [--allow-passive-discovery]
  local-ai-kit connect --ipc LOCAL_ADDRESS
  local-ai-kit scan --json [--home ABSOLUTE_DIRECTORY]
  local-ai-kit serve --stdio [--allow-passive-discovery] [--allow-codex-metadata] [--home ABSOLUTE_DIRECTORY]
  local-ai-kit serve --http --port 4499 --assets ABSOLUTE_DIST_DIRECTORY
      [--allow-passive-discovery] [--allow-codex-metadata]
      [--allow-codex-execution --workspace ABSOLUTE_DIRECTORY]
      [--allow-writes] [--review-native-requests] [--allow-paid-options]
  Reusable provider grants: --allow-metadata PROVIDER --allow-execution PROVIDER
  Execution providers: codex, claude_code, gemini_cli, antigravity, grok, ollama, lmstudio, openai_api, anthropic_api, gemini_api, xai_api, compatible_api, opencode.
  --endpoints ABSOLUTE_JSON registers local or API endpoints without contacting them.
  --native-profiles ABSOLUTE_JSON explicitly registers Gemini CLI, Antigravity ACP, or Grok native profiles; the file contains paths/labels, never tokens.
  --billing-declarations ABSOLUTE_JSON declares billing for discovered connections whose provider leaves it unknown; recorded as user-supplied evidence, never verification.
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

async fn execute(args: Args) -> Result<()> {
    if matches!(args.action, Action::Connect) {
        return broker_cli::connect(args.ipc.as_deref().unwrap()).await;
    }
    let registry = Arc::new(endpoints::load(args.endpoints.as_deref())?);
    let mut native = native_profiles::load(args.native_profiles.as_deref())?;
    native.browser_blocker = native_profiles::packaged_browser_blocker();
    let native = Arc::new(native);
    let declarations = Arc::new(native_profiles::load_declarations(
        args.billing_declarations.as_deref(),
    )?);
    let original = source(args.home.clone());
    let scan_registry = registry.clone();
    let scan_native = native.clone();
    let inventory: InventorySource = Arc::new(move |cancel| {
        let mut result = original(cancel)?;
        scan_registry.inject(&mut result);
        scan_native.inject(&mut result);
        Ok(result)
    });
    let service = host::build_service_with_state(
        inventory,
        args.allow_execution,
        registry,
        native,
        declarations,
        host::default_state_dir(args.home.as_deref()),
    );
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
        Action::Serve if args.ipc.is_some() => {
            broker_cli::serve(service, args.ipc.as_deref().unwrap(), policy, grant).await
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
