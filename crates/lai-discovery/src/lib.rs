//! Explicit passive inventory. No client launches, secret reads, or network calls.
mod environment;
pub mod log_tail;
mod registry;
mod scanner;
pub use environment::{HostPlatform, ScanEnvironment};
use lai_contracts::Snapshot;
use lai_platform::LocalFilesystem;
pub use registry::{ProviderDefinition, definitions};
use std::{collections::BTreeMap, path::PathBuf};
#[derive(Debug, Clone)]
pub struct LocatedClient {
    pub provider_id: String,
    pub profile_home: PathBuf,
    pub executable: Option<PathBuf>,
}
/// Locations stay inside the native process; only `snapshot` is a public wire record.
pub struct ScanResult {
    pub snapshot: Snapshot,
    pub locations: BTreeMap<String, LocatedClient>,
    pub metadata_probes: usize,
}
pub struct Scanner<F: LocalFilesystem> {
    filesystem: F,
}
impl<F: LocalFilesystem> Scanner<F> {
    pub fn new(filesystem: F) -> Self {
        Self { filesystem }
    }
}
