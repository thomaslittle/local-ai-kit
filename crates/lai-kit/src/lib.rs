//! One Rust import for the same adapters/service used by the native helper.
//! Construction is inert. Scanning, account probes, and execution are explicit and separately granted.
mod adapter_info;
mod service_builder;
pub use lai_adapters::billing_declarations::BillingDeclarations;
pub use lai_adapters::http_provider::config::Registry as EndpointRegistry;
pub use lai_adapters::native_clients::config::NativeRegistry as NativeClientRegistry;
pub use lai_contracts::*;
pub use lai_core::ClientPolicy;
use lai_discovery::{HostPlatform, ScanEnvironment, Scanner};
use lai_platform::NativeFilesystem;
pub use lai_service::{ExecutionGrant, InventorySource, NativeService};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
pub use service_builder::{
    build_service_with_declarations, build_service_with_endpoints, build_service_with_registries,
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};
static CLIENT_SEQUENCE: AtomicU64 = AtomicU64::new(1);
pub struct Kit {
    service: Arc<NativeService>,
}
#[derive(Default)]
pub struct KitBuilder {
    home: Option<PathBuf>,
    source: Option<InventorySource>,
    registry: EndpointRegistry,
    native_registry: NativeClientRegistry,
    execute: bool,
}
pub struct EmbeddedClient {
    service: Arc<NativeService>,
    id: String,
    closed: AtomicBool,
}
impl Kit {
    pub fn builder() -> KitBuilder {
        KitBuilder::default()
    }
    pub fn service(&self) -> Arc<NativeService> {
        self.service.clone()
    }
    /// Trusted host API, not a remote permission request. IDs are fresh even when labels repeat.
    pub fn connect(&self, label: &str, policy: ClientPolicy) -> Result<EmbeddedClient> {
        if label.is_empty() || label.len() > 128 || label.chars().any(char::is_control) {
            return Err(DomainError::new(
                ErrorCode::InvalidRequest,
                "Invalid application label",
            ));
        }
        let id = stable_id(
            "embedded",
            &[
                label,
                &CLIENT_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string(),
            ],
        );
        self.service.register_client(&id, policy)?;
        Ok(EmbeddedClient {
            service: self.service.clone(),
            id,
            closed: AtomicBool::new(false),
        })
    }
}
impl KitBuilder {
    pub fn home(mut self, home: PathBuf) -> Self {
        self.home = Some(home);
        self
    }
    pub fn inventory(mut self, source: InventorySource) -> Self {
        self.source = Some(source);
        self
    }
    pub fn endpoints(mut self, registry: EndpointRegistry) -> Self {
        self.registry = registry;
        self
    }
    pub fn native_clients(mut self, registry: NativeClientRegistry) -> Self {
        self.native_registry = registry;
        self
    }
    /// Register executors, not execution permission. Each app still needs an explicit ExecutionGrant.
    pub fn execution(mut self, enabled: bool) -> Self {
        self.execute = enabled;
        self
    }
    pub fn build(self) -> Result<Kit> {
        if self
            .home
            .as_ref()
            .is_some_and(|p| !p.is_absolute() || p.to_string_lossy().chars().any(char::is_control))
        {
            return Err(DomainError::new(
                ErrorCode::InvalidRequest,
                "Home must be an absolute local directory",
            ));
        }
        let source = self.source.unwrap_or_else(|| inventory_source(self.home));
        let registry = Arc::new(self.registry);
        let native_registry = Arc::new(self.native_registry);
        let configs = registry.clone();
        let native_configs = native_registry.clone();
        let inventory = Arc::new(move |cancel: &AtomicBool| {
            let mut result = source(cancel)?;
            configs.inject(&mut result);
            native_configs.inject(&mut result);
            Ok(result)
        });
        Ok(Kit {
            service: build_service_with_registries(
                inventory,
                self.execute,
                registry,
                native_registry,
            ),
        })
    }
}
/// Lazy environment discovery: even reading HOME/PATH is deferred until an explicit scan.
pub fn inventory_source(home: Option<PathBuf>) -> InventorySource {
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
impl EmbeddedClient {
    fn open(&self) -> Result<()> {
        if self.closed.load(Ordering::Acquire) {
            Err(DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Application connection is closed",
            ))
        } else {
            Ok(())
        }
    }
    pub async fn request<T: DeserializeOwned>(&self, method: &str, params: Value) -> Result<T> {
        self.open()?;
        let value = self.service.handle(&self.id, method, params).await?;
        self.open()?;
        serde_json::from_value(value).map_err(|_| {
            DomainError::new(
                ErrorCode::ProtocolIncompatible,
                "Native response has an incompatible type",
            )
        })
    }
    pub async fn snapshot(&self) -> Result<Snapshot> {
        self.request("snapshot.get", json!({})).await
    }
    pub async fn scan(&self) -> Result<Snapshot> {
        self.request::<Value>("discovery.refresh", json!({"level":"passive"}))
            .await?;
        self.snapshot().await
    }
    pub async fn probe(&self, connection_id: &str) -> Result<Snapshot> {
        self.request::<Value>("connections.probe", json!({"connectionId":connection_id}))
            .await?;
        self.snapshot().await
    }
    /// Only call after the host obtained the user's approval for these exact scopes.
    pub fn authorize_execution(&self, grant: ExecutionGrant) -> Result<()> {
        self.open()?;
        self.service.register_execution(&self.id, grant)
    }
    pub fn close(&self) {
        if !self.closed.swap(true, Ordering::AcqRel) {
            self.service.close_client(&self.id);
        }
    }
}
impl Drop for EmbeddedClient {
    fn drop(&mut self) {
        self.close();
    }
}
