use crate::args::{Action, Args};
use lai_adapters::http_provider::{
    self,
    config::{EndpointConfig, Registry},
};
use lai_adapters::{
    claude, codex,
    execution::{CodexExecution, open_codex},
    process::LaunchSpec,
};
use lai_contracts::{DomainError, ErrorCode};
use lai_core::ClientPolicy;
use lai_service::{ExecutionGrant, InventorySource, NativeService};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};
pub fn policy(args: &Args) -> ClientPolicy {
    let mut policy = if args.allow_passive || matches!(args.action, Action::Scan) {
        ClientPolicy::passive()
    } else {
        ClientPolicy::denied()
    };
    policy.probe_provider_ids = args.metadata_providers.clone();
    policy
}
pub fn execution_grant(args: &Args) -> Option<ExecutionGrant> {
    if !args.allow_execution {
        return None;
    }
    Some(ExecutionGrant {
        provider_ids: args.execution_providers.clone(),
        workspaces: BTreeMap::from([("default".into(), args.workspace.clone()?)]),
        allow_paid: args.allow_paid,
        allow_writes: args.allow_writes,
        review_native: args.review_requests,
        ..Default::default()
    })
}
pub fn build_service_with_endpoints(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
) -> Arc<NativeService> {
    let metadata_registry = registry.clone();
    let service = NativeService::new(source, Duration::from_secs(2)).with_metadata(Arc::new(
        move |located, connection, route| {
            let endpoint = metadata_registry.for_connection(&connection, &located.provider_id);
            Box::pin(async move {
                if let Some(config) = endpoint {
                    let client = configured_endpoint(&config)?;
                    return http_provider::probe(
                        config.kind,
                        client,
                        connection,
                        route,
                        config.text_models,
                    )
                    .await;
                }
                match located.provider_id.as_str() {
                    "codex" => codex::probe(located, connection, route).await,
                    "claude_code" => claude::probe(located, connection, route).await,
                    "grok" => lai_adapters::grok::probe(located, connection, route).await,
                    _ => Err(DomainError::new(
                        ErrorCode::CapabilityUnsupported,
                        "No metadata adapter registered",
                    )),
                }
            })
        },
    ));
    if !execute {
        return Arc::new(service);
    }
    Arc::new(
        service.with_execution(
            [
                "codex",
                "claude_code",
                "ollama",
                "lmstudio",
                "openai_api",
                "anthropic_api",
                "gemini_api",
                "xai_api",
                "compatible_api",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>(),
            Arc::new(move |input| {
                let endpoint =
                    registry.for_connection(&input.connection_id, &input.located.provider_id);
                Box::pin(async move {
                    if let Some(config) = endpoint {
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "HTTP endpoints expose text-only execution",
                            ));
                        }
                        let client = configured_endpoint(&config)?;
                        return Ok(http_provider::executor(config.kind, client, input.model));
                    }
                    if input.located.provider_id == "claude_code" {
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "This Claude adapter exposes text-only execution",
                            ));
                        }
                        let spec = claude::launch::launch(&input.located, input.workspace)?;
                        return Ok(claude::client::executor(
                            spec,
                            input.model,
                            input.connection_id,
                            input.route_id,
                        ));
                    }
                    if input.located.provider_id != "codex" {
                        return Err(DomainError::new(
                            ErrorCode::CapabilityUnsupported,
                            "Native executor unavailable",
                        ));
                    }
                    let candidate = input.located.executable.as_deref().ok_or_else(|| {
                        DomainError::new(ErrorCode::RuntimeMissing, "Native executable unavailable")
                    })?;
                    let executable = codex::resolve_executable(candidate)?;
                    let mut args = vec!["app-server".into(), "--stdio".into()];
                    for feature in [
                        "hooks",
                        "plugins",
                        "remote_plugin",
                        "apps",
                        "code_mode_host",
                        "shell_snapshot",
                    ] {
                        args.extend(["--disable".into(), feature.into()]);
                    }
                    let spec = LaunchSpec {
                        executable,
                        args,
                        cwd: input.workspace,
                        env: BTreeMap::from([(
                            "CODEX_HOME".into(),
                            input.located.profile_home.into_os_string(),
                        )]),
                    };
                    open_codex(
                        spec,
                        CodexExecution {
                            model: input.model,
                            connection: input.connection_id,
                            route: input.route_id,
                            writable: input.writable,
                            review_requests: input.review_requests,
                        },
                    )
                    .await
                })
            }),
        ),
    )
}

/// Only called after the service checked a provider-specific metadata/execution grant.
/// The CLI's explicit config names one credential variable; no environment enumeration.
fn configured_endpoint(
    config: &EndpointConfig,
) -> lai_contracts::Result<lai_adapters::http::Endpoint> {
    let credential = config
        .key_env
        .as_ref()
        .map(|name| {
            std::env::var(name).map_err(|_| {
                DomainError::new(
                    ErrorCode::AuthRequired,
                    "Configured endpoint credential is unavailable",
                )
            })
        })
        .transpose()?;
    config.endpoint_with_credential(credential)
}
