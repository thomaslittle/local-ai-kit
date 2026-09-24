use super::{invalid, parse_metadata};
use crate::process::{LaunchSpec, Peer};
use lai_contracts::*;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::{Value, json};
use std::{collections::BTreeMap, time::Duration};
async fn control(peer: &mut Peer, request: Value) -> Result<Value> {
    peer.sequence += 1;
    let id = format!("local-ai-kit-{}", peer.sequence);
    peer.send(json!({"type":"control_request","request_id":id,"request":request}))
        .await?;
    for _ in 0..128 {
        let response = peer.read().await?;
        match response["type"].as_str() {
            Some("control_response") if response["response"]["request_id"] == id => {
                return if response["response"]["subtype"] == "success" {
                    Ok(response["response"]["response"].clone())
                } else {
                    Err(DomainError::new(
                        ErrorCode::CapabilityUnsupported,
                        "Native control operation unavailable",
                    ))
                };
            }
            Some("system" | "rate_limit_event") => {}
            _ => return Err(invalid()),
        }
    }
    Err(invalid())
}
async fn initialize(peer: &mut Peer) -> Result<Value> {
    let value = control(
        peer,
        json!({"subtype":"initialize","hooks":null,"skills":[]}),
    )
    .await?;
    if value["current_permission_mode"]
        .as_str()
        .is_some_and(|p| p != "dontAsk" && p != "plan")
    {
        return Err(DomainError::new(
            ErrorCode::IntegrationRestricted,
            "Native permission mode mismatch",
        ));
    }
    Ok(value)
}
pub async fn probe_spec(
    spec: LaunchSpec,
    connection: &str,
    route: &str,
) -> Result<ConnectionMetadata> {
    let mut peer = Peer::spawn(spec).await?;
    let outcome = tokio::time::timeout(Duration::from_secs(20), async {
        let init = initialize(&mut peer).await?;
        let usage = tokio::time::timeout(
            Duration::from_secs(4),
            control(&mut peer, json!({"subtype":"get_usage"})),
        )
        .await
        .ok()
        .and_then(Result::ok);
        parse_metadata(connection, route, &init, usage.as_ref())
    })
    .await
    .map_err(|_| DomainError::new(ErrorCode::Timeout, "Claude metadata timed out"))?;
    peer.shutdown().await;
    outcome
}
pub fn executor(
    spec: LaunchSpec,
    model: Model,
    connection: String,
    route: String,
) -> Box<dyn Agent> {
    Box::new(ClaudeAgent {
        spec: Some(spec),
        peer: None,
        model,
        connection,
        route,
        options: None,
        session: None,
    })
}
struct ClaudeAgent {
    spec: Option<LaunchSpec>,
    peer: Option<Peer>,
    model: Model,
    connection: String,
    route: String,
    options: Option<BTreeMap<String, Value>>,
    session: Option<String>,
}
impl ClaudeAgent {
    async fn execute(
        &mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        mut cx: TurnContext,
    ) -> Result<()> {
        for (id, value) in &options {
            self.model
                .options
                .iter()
                .find(|d| &d.id == id)
                .ok_or_else(invalid)?
                .validate(value)
                .map_err(|_| invalid())?;
        }
        if let Some(previous) = &self.options {
            if previous != &options {
                return Err(DomainError::new(
                    ErrorCode::OptionUnsupported,
                    "This client applies effort at session startup; create a new session",
                ));
            }
        }
        if let Some(mut spec) = self.spec.take() {
            spec.args
                .push(format!("--model={}", self.model.native_id).into());
            if let Some(effort) = options.get("reasoningEffort").and_then(Value::as_str) {
                spec.args.push(format!("--effort={effort}").into());
            }
            let mut peer = Peer::spawn(spec).await?;
            let init = tokio::time::timeout(Duration::from_secs(20), initialize(&mut peer))
                .await
                .map_err(|_| {
                    DomainError::new(ErrorCode::Timeout, "Claude initialization timed out")
                })??;
            let metadata = parse_metadata(&self.connection, &self.route, &init, None)?;
            if metadata.auth.fresh_value() != Some(&AuthState::Authenticated)
                || !metadata
                    .models
                    .iter()
                    .any(|m| m.id == self.model.id && m.revision == self.model.revision)
            {
                return Err(DomainError::new(
                    ErrorCode::SelectionStale,
                    "Native account or model selection changed",
                ));
            }
            self.peer = Some(peer);
            self.options = Some(options);
        }
        let peer = self.peer.as_mut().ok_or_else(invalid)?;
        peer.send(json!({"type":"user","message":{"role":"user","content":prompt},"parent_tool_use_id":null,"session_id":self.session.as_deref().unwrap_or(""),"client_composed":true})).await?;
        let mut streamed = false;
        loop {
            let response = tokio::select! {value=peer.read()=>value?,_=cx.cancelled()=>{peer.sequence+=1;let _=peer.send(json!({"type":"control_request","request_id":format!("cancel-{}",peer.sequence),"request":{"subtype":"interrupt"}})).await;return Err(DomainError::new(ErrorCode::Cancelled,"Native turn cancelled"));}};
            if let Some(id) = response["session_id"].as_str() {
                if id.len() > 256 || id.chars().any(char::is_control) {
                    return Err(invalid());
                }
                if let Some(expected) = &self.session {
                    if !id.is_empty() && id != expected {
                        return Err(DomainError::new(
                            ErrorCode::SessionRouteMismatch,
                            "Native session changed",
                        ));
                    }
                } else if !id.is_empty() {
                    self.session = Some(id.into());
                }
            }
            match response["type"].as_str() {
                Some("control_request") => {
                    let id = response["request_id"]
                        .as_str()
                        .filter(|s| s.len() <= 256)
                        .ok_or_else(invalid)?;
                    peer.send(json!({"type":"control_response","response":{"subtype":"success","request_id":id,"response":{"behavior":"deny","message":"Toolkit text-only mode does not authorize tools","interrupt":true}}})).await?;
                    return Err(DomainError::new(
                        ErrorCode::IntegrationRestricted,
                        "Unexpected native tool request",
                    ));
                }
                Some("system") => {
                    if response["subtype"] == "init"
                        && response["tools"].as_array().is_some_and(|t| !t.is_empty())
                    {
                        return Err(DomainError::new(
                            ErrorCode::IntegrationRestricted,
                            "Text-only native client advertised tools",
                        ));
                    }
                }
                Some("stream_event") => {
                    let event = &response["event"];
                    if event["type"] == "content_block_delta" {
                        let delta = &event["delta"];
                        if let Some(text) = delta["text"].as_str() {
                            streamed = true;
                            emit_text(&cx, text)?;
                        } else if let Some(text) = delta["thinking"].as_str() {
                            cx.emit(EventPayload::ReasoningSummary {
                                text: text.chars().take(16000).collect(),
                            })?;
                        }
                    }
                }
                Some("assistant") if !streamed => {
                    if let Some(parts) = response["message"]["content"].as_array() {
                        for part in parts {
                            if part["type"] == "text" {
                                if let Some(text) = part["text"].as_str() {
                                    emit_text(&cx, text)?;
                                }
                            } else if part["type"] == "tool_use" {
                                return Err(DomainError::new(
                                    ErrorCode::IntegrationRestricted,
                                    "Native tool use is not enabled",
                                ));
                            }
                        }
                    }
                }
                Some("result") => {
                    if response["is_error"] == true || response["subtype"] != "success" {
                        return Err(DomainError::new(
                            ErrorCode::ProviderUnavailable,
                            "Claude turn failed",
                        ));
                    }
                    for (key, unit) in [
                        ("input_tokens", "input_tokens"),
                        ("output_tokens", "output_tokens"),
                    ] {
                        if let Some(used) = response["usage"][key].as_u64() {
                            cx.emit(EventPayload::Usage {
                                unit: unit.into(),
                                used: used.to_string(),
                            })?;
                        }
                    }
                    return Ok(());
                }
                Some("rate_limit_event" | "user" | "assistant" | "control_cancel_request") => {}
                _ => return Err(invalid()),
            }
        }
    }
}
fn emit_text(cx: &TurnContext, text: &str) -> Result<()> {
    let mut chunk = String::new();
    for ch in text.chars() {
        chunk.push(ch);
        if chunk.len() >= 16000 {
            cx.emit(EventPayload::TextDelta {
                text: std::mem::take(&mut chunk),
            })?;
        }
    }
    if !chunk.is_empty() {
        cx.emit(EventPayload::TextDelta { text: chunk })?;
    }
    Ok(())
}
impl Agent for ClaudeAgent {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(self.execute(prompt, options, cx))
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            if let Some(mut peer) = self.peer.take() {
                peer.shutdown().await;
            }
            Ok(())
        })
    }
}
