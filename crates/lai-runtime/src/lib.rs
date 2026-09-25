//! Provider-independent, owner-scoped sessions. Creating a session never runs inference.
mod artifact;
mod engine;
mod journal;
mod turn;
pub use engine::Runtime;
use lai_contracts::*;
use serde_json::Value;
use std::{collections::BTreeMap, future::Future, pin::Pin, sync::Arc};
pub use turn::TurnContext;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'a>>;
pub type Guard = Arc<dyn Fn() -> Result<()> + Send + Sync>;
/// Drivers own only the native resources they launch. close must release them.
pub trait Agent: Send {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        context: TurnContext,
    ) -> BoxFuture<'a, ()>;
    fn close(&mut self) -> BoxFuture<'_, ()>;
    fn native_resumption(&self) -> bool {
        false
    }
    fn suspend(&mut self) -> BoxFuture<'_, String> {
        Box::pin(async {
            Err(DomainError::new(
                ErrorCode::CapabilityUnsupported,
                "Native session resumption is unavailable",
            ))
        })
    }
}
pub(crate) fn err(code: ErrorCode) -> DomainError {
    DomainError::new(code, "Session operation failed")
}
