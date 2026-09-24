//! Installed native host. Registration configuration is owned by the installing application.
mod adapter_info;
mod args;
mod endpoints;
mod host;
mod inventory;
use lai_contracts::*;
use serde::Deserialize;
use std::{io::Read, time::Duration};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Registration {
    allowed_origins: Vec<String>,
    args: Vec<String>,
}
fn denied() -> DomainError {
    DomainError::new(
        ErrorCode::LocalAccessDenied,
        "Native host registration is unavailable or does not allow this extension",
    )
}
fn run() -> Result<()> {
    let mut input = std::env::args().skip(1);
    let origin = input.next().ok_or_else(denied)?;
    if !lai_transports::native_messaging::valid_origin(&origin)
        || input.any(|v| {
            !v.strip_prefix("--parent-window=")
                .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        })
    {
        return Err(denied());
    }
    let config = std::env::current_exe()
        .map_err(|_| denied())?
        .with_extension("json");
    let f = std::fs::File::open(config).map_err(|_| denied())?;
    let mut bytes = vec![];
    f.take(65537)
        .read_to_end(&mut bytes)
        .map_err(|_| denied())?;
    if bytes.len() > 65536 {
        return Err(denied());
    }
    let config: Registration = serde_json::from_slice(&bytes).map_err(|_| denied())?;
    if !config.allowed_origins.iter().any(|o| o == &origin)
        || config.allowed_origins.len() > 32
        || config.args.len() > 128
    {
        return Err(denied());
    }
    let args = args::Args::parse(
        ["serve".to_owned(), "--stdio".into()]
            .into_iter()
            .chain(config.args)
            .map(std::ffi::OsString::from),
    )?;
    if args.http || args.ipc.is_some() {
        return Err(denied());
    }
    let registry = std::sync::Arc::new(endpoints::load(args.endpoints.as_deref())?);
    let base = inventory::source(args.home.clone());
    let entries = registry.clone();
    let service = host::build_service_with_endpoints(
        std::sync::Arc::new(move |cancel| {
            let mut scan = base(cancel)?;
            entries.inject(&mut scan);
            Ok(scan)
        }),
        args.allow_execution,
        registry,
    );
    let client = stable_id("extension", &[&origin]);
    service.register_client(&client, host::policy(&args))?;
    if let Some(grant) = host::execution_grant(&args) {
        service.register_execution(&client, grant)?;
    }
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .map_err(|_| denied())?;
    let result = runtime.block_on(lai_transports::native_messaging::serve_native(
        tokio::io::stdin(),
        tokio::io::stdout(),
        service,
        client,
        &origin,
        &config.allowed_origins,
    ));
    runtime.shutdown_timeout(Duration::from_millis(100));
    result
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::from(2)
        }
    }
}
