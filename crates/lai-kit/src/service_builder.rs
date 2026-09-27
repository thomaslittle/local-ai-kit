use lai_adapters::http_provider::{
    self,
    config::{EndpointConfig, Registry},
};
use lai_adapters::{
    claude, codex,
    execution::{CodexExecution, open_codex},
    native_clients::config::NativeRegistry,
    process::LaunchSpec,
};
use lai_contracts::{DomainError, ErrorCode};
use lai_service::{InventorySource, NativeService};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};
pub fn build_service_with_endpoints(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
) -> Arc<NativeService> {
    build_service_with_registries(
        source,
        execute,
        registry,
        Arc::new(NativeRegistry::default()),
    )
}
pub fn build_service_with_registries(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
    native_registry: Arc<NativeRegistry>,
) -> Arc<NativeService> {
    let metadata_registry = registry.clone();
    let metadata_native = native_registry.clone();
    let service = NativeService::new(source, Duration::from_secs(2))
        .with_adapters(crate::adapter_info::descriptors())
        .with_metadata(Arc::new(move |located, connection, route| {
            let endpoint = metadata_registry.for_connection(&connection, &located.provider_id);
            let native_profile = metadata_native.profiles.get(&connection).cloned();
            let native_browser = metadata_native.browser_blocker.clone();
            Box::pin(async move {
                if let Some(profile) = native_profile {
                    return lai_adapters::native_clients::client::probe(
                        &profile,
                        &located,
                        &connection,
                        &route,
                        native_browser.as_deref(),
                    )
                    .await;
                }
                if let Some(config) = endpoint {
                    let client = configured_endpoint(&config)?;
                    if config.kind == http_provider::HttpKind::Opencode {
                        let raw = client
                            .json_query_limited("provider", &[], None, 16 * 1024 * 1024)
                            .await?;
                        return lai_adapters::opencode::catalog(
                            &connection,
                            &route,
                            &raw,
                            &config.provider_billing,
                        );
                    }
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
                    "cursor" => tokio::task::spawn_blocking(move || {
                        lai_adapters::cursor::read_cached(located, &connection, &route)
                    })
                    .await
                    .map_err(|_| {
                        DomainError::new(
                            ErrorCode::ProviderUnavailable,
                            "Cursor metadata worker unavailable",
                        )
                    })?,
                    _ => Err(DomainError::new(
                        ErrorCode::CapabilityUnsupported,
                        "No metadata adapter registered",
                    )),
                }
            })
        }));
    if !execute {
        return Arc::new(service);
    }
    let native_connections = native_registry
        .profiles
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let execution_native = native_registry.clone();
    Arc::new(
        service.with_execution_scoped(
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
                "opencode",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>(),
            native_connections,
            Arc::new(move |input| {
                let endpoint =
                    registry.for_connection(&input.connection_id, &input.located.provider_id);
                let native_profile = execution_native.profiles.get(&input.connection_id).cloned();
                let native_browser = execution_native.browser_blocker.clone();
                Box::pin(async move {
                    if let Some(profile) = native_profile {
                        if input.writable || input.native_policy {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "Native ACP subscription routes expose text sessions with mediated approvals only",
                            ));
                        }
                        let request = lai_adapters::native_clients::client::NativeOpen {
                            route: input.route_id,
                            model: input.model,
                            review: input.review_requests,
                        };
                        return match input.native_resume.as_deref() {
                            Some(native_session_id) => {
                                lai_adapters::native_clients::client::open_resume(
                                    &profile,
                                    &input.located,
                                    &input.workspace,
                                    native_browser.as_deref(),
                                    request,
                                    native_session_id,
                                )
                                .await
                            }
                            None => {
                                lai_adapters::native_clients::client::open(
                                    &profile,
                                    &input.located,
                                    &input.workspace,
                                    native_browser.as_deref(),
                                    request,
                                )
                                .await
                            }
                        };
                    }
                    if input.native_resume.is_some() {
                        return Err(DomainError::new(
                            ErrorCode::CapabilityUnsupported,
                            "Selected integration does not support native session resumption",
                        ));
                    }
                    if let Some(config) = endpoint {
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "HTTP endpoints expose text-only execution",
                            ));
                        }
                        let client = configured_endpoint(&config)?;
                        if config.kind == http_provider::HttpKind::Opencode {
                            return lai_adapters::opencode::client::open(
                                client,
                                input.model,
                                input.route_provider_id,
                                input.workspace,
                            )
                            .await;
                        }
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
