//! One native service shared by every transport. Host policies are not RPC-settable.
use std::{sync::{Arc,atomic::AtomicBool},time::Duration};
use lai_contracts::Result;
use lai_core::ClientPolicy;
use lai_discovery::ScanResult;
use serde_json::Value;
pub type InventorySource = Arc<dyn Fn(&AtomicBool)->Result<ScanResult> + Send + Sync>;
pub struct NativeService;
impl NativeService {
 pub fn new(_:InventorySource,_:Duration)->Self{Self}
 pub fn register_client(&self,_:&str,_:ClientPolicy)->Result<()>{unimplemented!("test-first host registration")}
 pub async fn handle(&self,_:&str,_:&str,_:Value)->Result<Value>{unimplemented!("test-first native service")}
}
