//! Bounded, handshaken JSON-RPC over native streams. Caller identity is supplied by the host.
mod wire;
use lai_contracts::{ErrorCode, Result};
use lai_service::NativeService;
use serde_json::Value;
use std::{collections::BTreeSet, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    task::JoinSet,
};
use wire::{error, reply};

#[derive(Clone)]
pub struct StdioOptions {
    pub max_frame_bytes: usize,
    pub max_pending_requests: usize,
    pub write_timeout: Duration,
}
impl Default for StdioOptions {
    fn default() -> Self {
        Self {
            max_frame_bytes: 1_048_576,
            max_pending_requests: 64,
            write_timeout: Duration::from_secs(5),
        }
    }
}
async fn send<W: AsyncWrite + Unpin>(
    writer: &mut W,
    value: Value,
    options: &StdioOptions,
) -> Result<()> {
    let mut bytes = serde_json::to_vec(&value).map_err(|_| error(ErrorCode::InvalidRequest))?;
    if bytes.len() + 1 > options.max_frame_bytes {
        bytes = serde_json::to_vec(&reply(
            value.get("id").cloned().unwrap_or(Value::Null),
            Err(error(ErrorCode::FrameTooLarge)),
        ))
        .map_err(|_| error(ErrorCode::InvalidRequest))?;
    }
    bytes.push(b'\n');
    tokio::time::timeout(options.write_timeout, async {
        writer.write_all(&bytes).await?;
        writer.flush().await
    })
    .await
    .map_err(|_| error(ErrorCode::Timeout))?
    .map_err(|_| error(ErrorCode::ProviderUnavailable))
}

pub async fn serve_io<R: AsyncRead + Unpin, W: AsyncWrite + Unpin>(
    mut reader: R,
    mut writer: W,
    service: Arc<NativeService>,
    client: String,
    options: StdioOptions,
) -> Result<()> {
    if !(1024..=16_777_216).contains(&options.max_frame_bytes)
        || !(1..=256).contains(&options.max_pending_requests)
        || options.write_timeout.is_zero()
        || options.write_timeout > Duration::from_secs(60)
    {
        return Err(error(ErrorCode::InvalidRequest));
    }
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 8192];
    let mut ready = false;
    let mut eof = false;
    let mut pending = BTreeSet::new();
    let mut tasks: JoinSet<(String, Value, Result<Value>)> = JoinSet::new();
    while !eof || !tasks.is_empty() {
        tokio::select! {
            biased;
            joined = tasks.join_next(), if !tasks.is_empty() => {
                let (key, id, outcome) = joined.ok_or_else(|| error(ErrorCode::ProviderUnavailable))?.map_err(|_| error(ErrorCode::ProviderUnavailable))?;
                pending.remove(&key);
                send(&mut writer, reply(id, outcome), &options).await?;
            }
            read = reader.read(&mut chunk), if !eof => {
                let count = read.map_err(|_| error(ErrorCode::ProviderUnavailable))?;
                if count == 0 {
                    eof = true;
                    if !buffer.is_empty() { send(&mut writer, wire::invalid(-32700), &options).await?; buffer.clear(); }
                    continue;
                }
                for byte in &chunk[..count] {
                    if *byte != b'\n' {
                        if buffer.len() >= options.max_frame_bytes {
                            send(&mut writer, reply(Value::Null, Err(error(ErrorCode::FrameTooLarge))), &options).await?;
                            return Err(error(ErrorCode::FrameTooLarge));
                        }
                        buffer.push(*byte); continue;
                    }
                    if buffer.is_empty() { continue; }
                    let request = wire::parse(&buffer); buffer.clear();
                    let request = match request {
                        Ok(Some(request)) => request,
                        Ok(None) => continue, // Notifications never initiate work in this protocol.
                        Err(value) => { send(&mut writer, value, &options).await?; continue; }
                    };
                    if request.method == "runtime.handshake" {
                        if ready { send(&mut writer, reply(request.id, Err(error(ErrorCode::Conflict))), &options).await?; continue; }
                        let outcome = service.handle(&client, &request.method, request.params).await;
                        ready = outcome.is_ok(); send(&mut writer, reply(request.id, outcome), &options).await?; continue;
                    }
                    if !ready { send(&mut writer, reply(request.id, Err(error(ErrorCode::ProtocolIncompatible))), &options).await?; continue; }
                    let key = request.id.to_string();
                    if pending.contains(&key) {
                        send(&mut writer, reply(request.id, Err(error(ErrorCode::Conflict))), &options).await?;
                        return Err(error(ErrorCode::Conflict));
                    }
                    if pending.len() >= options.max_pending_requests {
                        send(&mut writer, reply(request.id, Err(error(ErrorCode::RateLimited))), &options).await?; continue;
                    }
                    pending.insert(key.clone());
                    let service = service.clone(); let client = client.clone();
                    tasks.spawn(async move { let outcome = service.handle(&client, &request.method, request.params).await; (key, request.id, outcome) });
                }
            }
        }
    }
    Ok(())
}
