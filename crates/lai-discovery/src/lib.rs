//! Passive, explicitly invoked inventory. It never launches clients or reads secrets.
use std::{collections::BTreeMap, path::PathBuf, sync::atomic::AtomicBool};
use lai_contracts::{Result, Snapshot};
use lai_platform::LocalFilesystem;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPlatform { Windows, Macos, Linux }
#[derive(Debug, Clone)]
pub struct ScanEnvironment {
    pub home: PathBuf, pub config_home: PathBuf, pub data_home: PathBuf,
    pub app_data: PathBuf, pub local_app_data: PathBuf, pub search_path: Vec<PathBuf>,
    pub overrides: BTreeMap<String, PathBuf>, pub platform: HostPlatform,
}
impl ScanEnvironment {
    pub fn for_home(home: PathBuf, platform: HostPlatform) -> Self {
        Self { config_home:home.join(".config"), data_home:home.join(".local/share"), app_data:home.join("AppData/Roaming"), local_app_data:home.join("AppData/Local"), home, search_path:vec![], overrides:BTreeMap::new(), platform }
    }
}
#[derive(Debug, Clone)]
pub struct LocatedClient { pub provider_id:String, pub profile_home:PathBuf, pub executable:Option<PathBuf> }
pub struct ScanResult { pub snapshot:Snapshot, pub locations:BTreeMap<String,LocatedClient>, pub metadata_probes:usize }
pub struct Scanner<F:LocalFilesystem> { filesystem:F }
impl<F:LocalFilesystem> Scanner<F> {
    pub fn new(filesystem:F)->Self { Self { filesystem } }
    pub fn scan(&self,_env:&ScanEnvironment,_cancel:&AtomicBool,_progress:impl FnMut(&Snapshot))->Result<ScanResult> {
        let _=&self.filesystem;
        unimplemented!("test-first native inventory")
    }
}
