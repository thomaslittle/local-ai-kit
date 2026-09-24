use lai_discovery::{HostPlatform, ScanEnvironment, Scanner};
use lai_platform::NativeFilesystem;
use lai_service::InventorySource;
use std::{path::PathBuf, sync::Arc};
pub fn source(home: Option<PathBuf>) -> InventorySource {
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
