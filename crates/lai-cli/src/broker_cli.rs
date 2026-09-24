use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_service::{ExecutionGrant, NativeService};
use lai_transports::local_ipc::Broker;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
fn err() -> DomainError {
    DomainError::new(
        ErrorCode::ProviderUnavailable,
        "Broker control channel closed",
    )
}
pub async fn connect(address: &str) -> Result<()> {
    let stream = lai_transports::local_ipc::connect(address).await?;
    let (mut r, mut w) = tokio::io::split(stream);
    let (mut input, mut output) = (tokio::io::stdin(), tokio::io::stdout());
    tokio::select! {
     result=async {tokio::io::copy(&mut input,&mut w).await?;w.shutdown().await}=>{result.map_err(|_|err())?;},
     result=tokio::io::copy(&mut r,&mut output)=>{result.map_err(|_|err())?;}
    }
    Ok(())
}
pub async fn serve(
    service: Arc<NativeService>,
    address: &str,
    policy: ClientPolicy,
    execution: Option<ExecutionGrant>,
) -> Result<()> {
    let broker = Broker::start(service, address).await?;
    // This stdout is a trusted parent control channel, not a diagnostic log.
    // Each bootstrap is single-use. No provider credential is involved.
    let send = || -> Result<()> {
        println!(
            "{}",
            serde_json::json!({"protocolVersion":"1.0","address":address,"token":broker.authorize(policy.clone(),execution.clone())?,"expiresInSeconds":300})
        );
        Ok(())
    };
    send()?;
    let mut reader = BufReader::new(tokio::io::stdin());
    loop {
        let mut bytes = Vec::new();
        let n = (&mut reader)
            .take(1025)
            .read_until(b'\n', &mut bytes)
            .await
            .map_err(|_| err())?;
        if n == 0 {
            break;
        }
        if n > 1024 || !bytes.ends_with(b"\n") {
            break;
        }
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_default();
        if v == serde_json::json!({"action":"authorize"}) {
            send()?;
        } else {
            break;
        }
    }
    broker.close().await;
    Ok(())
}
