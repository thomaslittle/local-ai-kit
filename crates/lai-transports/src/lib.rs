//! Bounded, handshaken JSON-RPC over native streams. Caller identity is supplied by the host.
pub mod http;
pub mod local_ipc;
pub mod native_messaging;
mod stdio_output;
mod wire;
use lai_contracts::{ErrorCode, Result};
use lai_service::NativeService;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};
pub use stdio_output::stdout as stdio_stdout;
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
            // Sized for one staged base64 image (inputs.put); responses remain service-bounded.
            max_frame_bytes: lai_contracts::MAX_REQUEST_FRAME_BYTES,
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
    let started = std::time::Instant::now();
    let mut stage = "write";
    tokio::time::timeout(options.write_timeout, async {
        writer.write_all(&bytes).await?;
        stage="flush";
        writer.flush().await
    })
    .await
    .map_err(|_| lai_contracts::DomainError::new(ErrorCode::Timeout, format!(
        "Native transport output deadline exceeded: stage={stage} response_bytes={} elapsed_ms={}",bytes.len(),started.elapsed().as_millis()
    )))?
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
    let _lifetime = ClientLifetime {
        service: service.clone(),
        client: client.clone(),
    };
    let mut subscriptions: BTreeMap<String, Value> = BTreeMap::new();
    let mut subscription_sequence = 0_u64;
    let mut ticks = tokio::time::interval(Duration::from_millis(50));
    ticks.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
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
            _=ticks.tick(), if !subscriptions.is_empty()&&!eof => {
                for (id,query) in subscriptions.clone() {
                    match service.handle(&client,"events.read",query).await {
                        Ok(page)=>{
                            if let Some(events)=page["events"].as_array(){for event in events {
                                send(&mut writer,serde_json::json!({"jsonrpc":"2.0","method":"events.event","params":{"subscriptionId":id,"event":event}}),&options).await?;
                            }}
                            if let Some(query)=subscriptions.get_mut(&id){query["after"]=page["cursor"].clone();}
                        },
                        Err(problem)=>{
                            subscriptions.remove(&id);
                            send(&mut writer,serde_json::json!({"jsonrpc":"2.0","method":"events.error","params":{"subscriptionId":id,"code":problem.code}}),&options).await?;
                        }
                    }
                }
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
                    if request.method=="events.subscribe" {
                        if subscriptions.len()>=16 {send(&mut writer,reply(request.id,Err(error(ErrorCode::RateLimited))),&options).await?;continue;}
                        match service.handle(&client,"events.read",request.params.clone()).await {
                            Ok(page)=>{
                                subscription_sequence+=1;
                                let id=format!("subscription-{subscription_sequence}");let mut query=request.params;
                                if query.get("after").is_none(){query["after"]=page["tailCursor"].clone();}
                                subscriptions.insert(id.clone(),query);
                                send(&mut writer,reply(request.id,Ok(serde_json::json!({"subscriptionId":id}))),&options).await?;
                            },Err(e)=>send(&mut writer,reply(request.id,Err(e)),&options).await?
                        }
                        continue;
                    }
                    if request.method=="events.unsubscribe" {
                        let result=if request.params.as_object().is_some_and(|p|p.len()==1){request.params["subscriptionId"].as_str().and_then(|id|subscriptions.remove(id)).map(|_|serde_json::json!({"unsubscribed":true})).ok_or_else(||error(ErrorCode::NotFound))}else{Err(error(ErrorCode::InvalidRequest))};
                        send(&mut writer,reply(request.id,result),&options).await?;continue;
                    }
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

struct ClientLifetime {
    service: Arc<NativeService>,
    client: String,
}
impl Drop for ClientLifetime {
    fn drop(&mut self) {
        self.service.close_client(&self.client);
    }
}

#[cfg(test)]
mod stdio_write_tests {
    use super::*;
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };
    struct BlockedWriter {
        flush_only: bool,
    }
    impl AsyncWrite for BlockedWriter {
        fn poll_write(
            self: Pin<&mut Self>,
            _: &mut Context<'_>,
            bytes: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            if self.flush_only {
                Poll::Ready(Ok(bytes.len()))
            } else {
                Poll::Pending
            }
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Pending
        }
        fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }
    #[tokio::test]
    async fn timeout_identifies_write_or_flush_without_payload_contents() {
        for (flush_only, stage) in [(false, "write"), (true, "flush")] {
            let value = serde_json::json!({"id":"PRIVATE_SENTINEL","result":"PRIVATE_SENTINEL"});
            let expected = serde_json::to_vec(&value).unwrap().len() + 1;
            let options = StdioOptions {
                write_timeout: Duration::from_millis(10),
                ..Default::default()
            };
            let problem = send(&mut BlockedWriter { flush_only }, value, &options)
                .await
                .unwrap_err();
            assert_eq!(problem.code, ErrorCode::Timeout);
            assert!(
                problem.message.contains(&format!("stage={stage}")),
                "{}",
                problem.message
            );
            assert!(
                problem
                    .message
                    .contains(&format!("response_bytes={expected}"))
            );
            assert!(!problem.message.contains("PRIVATE_SENTINEL"));
        }
    }
    #[test]
    fn stdout_progresses_while_provider_blocking_pool_is_busy() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .max_blocking_threads(1)
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let (started, ready) = std::sync::mpsc::channel();
            let busy = tokio::task::spawn_blocking(move || {
                started.send(()).unwrap();
                std::thread::sleep(Duration::from_millis(200));
            });
            ready.recv_timeout(Duration::from_secs(1)).unwrap();
            let mut writer = stdio_stdout().unwrap();
            let result = tokio::time::timeout(Duration::from_millis(50), async {
                writer.write_all(b"\n").await.unwrap();
                writer.flush().await.unwrap();
            })
            .await;
            busy.await.unwrap();
            assert!(
                result.is_ok(),
                "stdout was queued behind provider blocking work despite a draining consumer"
            );
        });
    }
    #[tokio::test]
    async fn large_response_drains_through_small_pipe() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        let value = serde_json::json!({"id":1,"result":"x".repeat(262144)});
        let expected = serde_json::to_vec(&value).unwrap().len() + 1;
        let drain = tokio::spawn(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            bytes.len()
        });
        send(&mut writer, value, &StdioOptions::default())
            .await
            .unwrap();
        drop(writer);
        assert_eq!(drain.await.unwrap(), expected);
    }
}
