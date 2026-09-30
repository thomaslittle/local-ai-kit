use lai_adapters::http_provider::{
    self,
    config::{EndpointConfig, Registry},
};
use lai_adapters::{
    billing_declarations::BillingDeclarations,
    claude, codex,
    execution::{CodexExecution, open_codex},
    native_clients::config::NativeRegistry,
    process::LaunchSpec,
};
use lai_chatgpt_auth::AuthManager;
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
    build_service_with_declarations(
        source,
        execute,
        registry,
        native_registry,
        Arc::new(BillingDeclarations::default()),
    )
}
/// Like [`build_service_with_registries`], plus host billing declarations that
/// fill billing a provider leaves unknown (labeled user-supplied evidence). A
/// declared, discovered Grok connection may also run text-only sessions.
pub fn build_service_with_declarations(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
    native_registry: Arc<NativeRegistry>,
    declarations: Arc<BillingDeclarations>,
) -> Arc<NativeService> {
    build_service_with_state(
        source,
        execute,
        registry,
        native_registry,
        declarations,
        None,
    )
}
/// Providers whose adapters persist and reattach native conversations.
pub const PERSISTENT_PROVIDERS: [&str; 4] = ["codex", "codex_managed", "claude_code", "opencode"];
/// The kit's per-user state directory (holds `resume.key`). With a virtual
/// `home` it stays inside that home. Nothing is created here.
pub fn default_state_dir(home: Option<&std::path::Path>) -> Option<std::path::PathBuf> {
    if let Some(home) = home {
        return Some(home.join(".local-ai-kit"));
    }
    let var = |name: &str| {
        std::env::var_os(name)
            .map(std::path::PathBuf::from)
            .filter(|p| p.is_absolute())
    };
    if let Some(dir) = var("LOCAL_AI_KIT_STATE_DIR") {
        return Some(dir);
    }
    if cfg!(windows) {
        var("LOCALAPPDATA").map(|p| p.join("local-ai-kit"))
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|p| p.join("Library/Application Support/local-ai-kit"))
    } else {
        var("XDG_STATE_HOME")
            .or_else(|| var("HOME").map(|p| p.join(".local/state")))
            .map(|p| p.join("local-ai-kit"))
    }
}
/// Like [`build_service_with_declarations`]; `state_dir` enables durable
/// resume tokens that survive helper restarts.
pub fn build_service_with_state(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
    native_registry: Arc<NativeRegistry>,
    declarations: Arc<BillingDeclarations>,
    state_dir: Option<std::path::PathBuf>,
) -> Arc<NativeService> {
    build_service_with_chatgpt(
        source,
        execute,
        registry,
        native_registry,
        declarations,
        state_dir,
        None,
    )
}

/// The managed SIWC route is opt-in. The manager owns protected credentials;
/// this service retains only an Arc and never serializes tokens into catalog data.
pub fn build_service_with_chatgpt(
    source: InventorySource,
    execute: bool,
    registry: Arc<Registry>,
    native_registry: Arc<NativeRegistry>,
    declarations: Arc<BillingDeclarations>,
    state_dir: Option<std::path::PathBuf>,
    chatgpt: Option<(Arc<AuthManager>, String)>,
) -> Arc<NativeService> {
    let metadata_registry = registry.clone();
    let metadata_native = native_registry.clone();
    let metadata_declarations = declarations.clone();
    let metadata_chatgpt = chatgpt.clone();
    let mut service = NativeService::new(source, Duration::from_secs(2))
        .with_adapters(crate::adapter_info::descriptors())
        .with_metadata(Arc::new(move |located, connection, route| {
            let endpoint = metadata_registry.for_connection(&connection, &located.provider_id);
            let native_profile = metadata_native.profiles.get(&connection).cloned();
            let native_browser = metadata_native.browser_blocker.clone();
            let declarations = metadata_declarations.clone();
            let provider = located.provider_id.clone();
            let chatgpt = metadata_chatgpt.clone();
            Box::pin(async move {
                let mut report = probe_metadata(
                    located,
                    connection,
                    route,
                    endpoint,
                    native_profile,
                    native_browser,
                    chatgpt,
                )
                .await?;
                declarations.apply(&provider, &mut report);
                Ok(report)
            })
        }));
    if let Some((auth, _)) = &chatgpt {
        service = service.with_chatgpt_auth(auth.clone());
    }
    if !execute {
        return Arc::new(service);
    }
    build_execution(
        service,
        registry,
        native_registry,
        declarations,
        state_dir,
        chatgpt,
    )
}
async fn probe_metadata(
    located: lai_discovery::LocatedClient,
    connection: String,
    route: String,
    endpoint: Option<EndpointConfig>,
    native_profile: Option<lai_adapters::native_clients::config::NativeProfile>,
    native_browser: Option<std::path::PathBuf>,
    chatgpt: Option<(Arc<AuthManager>, String)>,
) -> lai_contracts::Result<lai_contracts::ConnectionMetadata> {
    if located.provider_id == "codex_managed" {
        let (auth, app_name) = chatgpt.ok_or_else(|| {
            DomainError::new(
                ErrorCode::CapabilityUnsupported,
                "Managed ChatGPT integration is unavailable",
            )
        })?;
        let credentials = auth.credentials().await.map_err(auth_failure)?;
        let executable = located.executable.as_deref().ok_or_else(|| {
            DomainError::new(
                ErrorCode::RuntimeMissing,
                "Native Codex executable unavailable",
            )
        })?;
        std::fs::create_dir_all(&located.profile_home).map_err(|_| {
            DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Managed Codex profile is unavailable",
            )
        })?;
        let workspace = tempfile::Builder::new()
            .prefix("lai-managed-codex-metadata-")
            .tempdir()
            .map_err(|_| {
                DomainError::new(
                    ErrorCode::LocalAccessDenied,
                    "Neutral metadata workspace unavailable",
                )
            })?;
        let spec = codex::managed_launch_spec(
            executable,
            &located.profile_home,
            workspace.path(),
            &credentials.access_token,
        )?;
        return codex::managed_metadata(
            spec,
            &connection,
            &route,
            codex::ManagedIdentity {
                app_name,
                subject: credentials.subject,
                client_id: credentials.client_id,
                email: credentials.email,
            },
            Duration::from_secs(20),
        )
        .await;
    }
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
        return http_provider::probe(config.kind, client, connection, route, config.text_models)
            .await;
    }
    match located.provider_id.as_str() {
        "codex" => codex::probe(located, connection, route).await,
        "claude_code" => claude::probe(located, connection, route).await,
        "grok" => lai_adapters::grok::probe(located, connection, route).await,
        "opencode" => {
            let executable = located.executable.clone().ok_or_else(|| {
                DomainError::new(ErrorCode::RuntimeMissing, "Native executable unavailable")
            })?;
            lai_adapters::opencode::server::probe(&executable, &connection, &route).await
        }
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
}
fn build_execution(
    service: NativeService,
    registry: Arc<Registry>,
    native_registry: Arc<NativeRegistry>,
    declarations: Arc<BillingDeclarations>,
    state_dir: Option<std::path::PathBuf>,
    chatgpt: Option<(Arc<AuthManager>, String)>,
) -> Arc<NativeService> {
    let mut native_connections = native_registry
        .profiles
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    native_connections.extend(declarations.native_execution_connections());
    let service = match state_dir {
        Some(dir) => service.with_persistent_resume(
            dir,
            PERSISTENT_PROVIDERS
                .into_iter()
                .map(str::to_owned)
                .collect(),
            native_connections.clone(),
        ),
        None => service,
    };
    let execution_native = native_registry.clone();
    let execution_chatgpt = chatgpt.clone();
    Arc::new(
        service.with_execution_scoped(
            [
                "codex",
                "codex_managed",
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
                let chatgpt = execution_chatgpt.clone();
                let endpoint =
                    registry.for_connection(&input.connection_id, &input.located.provider_id);
                let native_profile = declared_native_profile(
                    &execution_native,
                    &declarations,
                    &input.connection_id,
                    &input.located,
                );
                let native_browser = execution_native.browser_blocker.clone();
                Box::pin(async move {
                    if input.located.provider_id == "codex_managed" {
                        let (auth, app_name) = chatgpt.ok_or_else(|| DomainError::new(
                            ErrorCode::CapabilityUnsupported, "Managed ChatGPT integration is unavailable",
                        ))?;
                        let credentials = auth.credentials().await.map_err(auth_failure)?;
                        let executable = input.located.executable.as_deref().ok_or_else(|| DomainError::new(
                            ErrorCode::RuntimeMissing, "Native Codex executable unavailable",
                        ))?;
                        std::fs::create_dir_all(&input.located.profile_home).map_err(|_| DomainError::new(
                            ErrorCode::LocalAccessDenied, "Managed Codex profile is unavailable",
                        ))?;
                        let spec = codex::managed_launch_spec(executable, &input.located.profile_home,
                            &input.workspace, &credentials.access_token)?;
                        let source: lai_adapters::execution::ManagedCredentialSource = Arc::new(move || {
                            let auth = auth.clone();
                            Box::pin(async move {
                                let value = auth.credentials().await.map_err(auth_failure)?;
                                Ok(lai_adapters::execution::ManagedCredential {
                                    access_token: value.access_token,
                                    subject: value.subject,
                                    client_id: value.client_id,
                                    email: value.email,
                                })
                            })
                        });
                        return lai_adapters::execution::open_managed_codex(
                            spec,
                            CodexExecution {
                                model: input.model, connection: input.connection_id,
                                route: input.route_id, writable: input.writable,
                                review_requests: input.review_requests,
                                host_mcp_servers: input.host_mcp_servers,
                                persistent: input.persistent, resume_thread: input.native_resume,
                            },
                            codex::ManagedIdentity {
                                app_name,
                                subject: credentials.subject,
                                client_id: credentials.client_id,
                                email: credentials.email,
                            },
                            source,
                        ).await;
                    }
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
                            host_mcp_servers: input.host_mcp_servers,
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
                    if input.native_resume.is_some() && !input.persistent {
                        return Err(DomainError::new(
                            ErrorCode::CapabilityUnsupported,
                            "Selected integration does not support native session resumption",
                        ));
                    }
                    if let Some(config) = endpoint {
                        if !input.host_mcp_servers.is_empty() {
                            return Err(lai_adapters::host_mcp::unsupported());
                        }
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "HTTP endpoints expose text-only execution",
                            ));
                        }
                        let client = configured_endpoint(&config)?;
                        if config.kind == http_provider::HttpKind::Opencode {
                            return lai_adapters::opencode::client::open_session(
                                client,
                                input.model,
                                input.route_provider_id,
                                input.workspace,
                                input.persistent,
                                input.native_resume,
                            )
                            .await;
                        }
                        if input.native_resume.is_some() {
                            return Err(DomainError::new(
                                ErrorCode::CapabilityUnsupported,
                                "Selected integration does not support native session resumption",
                            ));
                        }
                        return Ok(http_provider::executor(config.kind, client, input.model));
                    }
                    if input.located.provider_id == "opencode" {
                        if !input.host_mcp_servers.is_empty() {
                            return Err(lai_adapters::host_mcp::unsupported());
                        }
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "OpenCode sessions are text-only with every tool denied",
                            ));
                        }
                        let executable = input.located.executable.clone().ok_or_else(|| {
                            DomainError::new(
                                ErrorCode::RuntimeMissing,
                                "Native executable unavailable",
                            )
                        })?;
                        return lai_adapters::opencode::server::open_session(
                            &executable,
                            input.model,
                            input.route_provider_id,
                            input.workspace,
                            input.persistent,
                            input.native_resume,
                        )
                        .await;
                    }
                    if input.located.provider_id == "claude_code" {
                        if input.writable || input.review_requests {
                            return Err(DomainError::new(
                                ErrorCode::IntegrationRestricted,
                                "This Claude adapter exposes text-only execution",
                            ));
                        }
                        let mut launch = claude::launch::launch_with_host_mcp(
                            &input.located,
                            input.workspace,
                            &input.host_mcp_servers,
                        )?;
                        let host =
                            lai_adapters::host_mcp::HostTools::new(&input.host_mcp_servers);
                        if !input.persistent {
                            return Ok(claude::client::host_executor(
                                launch,
                                host,
                                input.model,
                                input.connection_id,
                                input.route_id,
                            ));
                        }
                        let resumed = input.native_resume.is_some();
                        let session_id = match input.native_resume {
                            Some(id) => {
                                if !claude::launch::persisted_session_exists(
                                    &input.located.profile_home,
                                    &id,
                                ) {
                                    return Err(DomainError::new(
                                        ErrorCode::ResumeUnavailable,
                                        "Claude Code no longer has this conversation",
                                    ));
                                }
                                id
                            }
                            None => claude::launch::new_session_id()?,
                        };
                        claude::launch::persist(&mut launch.spec, &session_id, resumed)?;
                        return Ok(claude::client::persistent_executor(
                            launch,
                            host,
                            input.model,
                            input.connection_id,
                            input.route_id,
                            session_id,
                            resumed,
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
                            host_mcp_servers: input.host_mcp_servers,
                            persistent: input.persistent,
                            resume_thread: input.native_resume,
                        },
                    )
                    .await
                })
            }),
        ),
    )
}

fn auth_failure(_: lai_chatgpt_auth::AuthError) -> DomainError {
    DomainError::new(
        ErrorCode::AuthRequired,
        "ChatGPT sign-in or refresh is required",
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

/// A registered native profile, or for a discovered client whose host
/// declared its billing, the profile derived from the discovered location.
fn declared_native_profile(
    native: &NativeRegistry,
    declarations: &BillingDeclarations,
    connection: &str,
    located: &lai_discovery::LocatedClient,
) -> Option<lai_adapters::native_clients::config::NativeProfile> {
    if let Some(profile) = native.profiles.get(connection) {
        return Some(profile.clone());
    }
    if !declarations
        .native_execution_connections()
        .contains(connection)
    {
        return None;
    }
    let declaration = declarations.get(connection, &located.provider_id)?;
    lai_adapters::grok::supported_executable(located).ok()?;
    let mut profile = native.profile(connection, located).ok()?;
    profile.billing = Some(declaration.billing.clone());
    Some(profile)
}
