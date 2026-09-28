use super::{CodexExecution, PathBuf, failure, native_id};
use crate::process::Peer;
use lai_contracts::*;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    time::Duration,
};
pub(super) struct CodexAgent {
    pub peer: Peer,
    pub config: CodexExecution,
    pub workspace: PathBuf,
    pub thread: Option<String>,
}
/// Thread config. Read-only sessions are text-only: besides the always-off
/// integrations, Codex's command, exec and generation tools are disabled so
/// the model cannot run anything on the host (the read-only sandbox alone
/// still permits reading commands).
fn thread_config(writable: bool) -> Value {
    let mut config = json!({"features.apps":false,"features.plugins":false,"features.hooks":false,"features.multi_agent":false,"features.browser_use":false,"features.computer_use":false,"web_search":"disabled"});
    if !writable {
        for feature in [
            "features.shell_tool",
            "features.unified_exec",
            "features.image_generation",
            "features.tool_suggest",
            "features.goals",
            "features.sleep_tool",
        ] {
            config[feature] = Value::Bool(false);
        }
    }
    config
}

impl CodexAgent {
    async fn ensure_thread(&mut self) -> Result<String> {
        if let Some(thread) = &self.thread {
            return Ok(thread.clone());
        }
        // Codex 0.157+ takes the thread sandbox mode in kebab case and reports
        // it back in camel case.
        let (mode, reported) = if self.config.writable {
            ("workspace-write", "workspaceWrite")
        } else {
            ("read-only", "readOnly")
        };
        let response=tokio::time::timeout(Duration::from_secs(20),self.peer.request("thread/start",json!({
   "model":self.config.model.native_id,"cwd":self.workspace,"ephemeral":true,
   "sandbox":mode,"approvalPolicy":if self.config.review_requests{"onRequest"}else{"never"},
   "approvalsReviewer":"user","serviceTier":"default","serviceName":"local-ai-kit",
   "config":thread_config(self.config.writable)
  }))).await.map_err(|_|failure(ErrorCode::Timeout))??;
        if response["model"] != self.config.model.native_id
            || response["sandbox"]["type"] != reported
        {
            return Err(failure(ErrorCode::IntegrationRestricted));
        }
        let thread = native_id(&response["thread"]["id"])?;
        self.thread = Some(thread.clone());
        Ok(thread)
    }
    async fn execute(
        &mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        mut cx: TurnContext,
    ) -> Result<()> {
        if cx.is_cancelled() {
            return Err(failure(ErrorCode::Cancelled));
        }
        for (id, value) in &options {
            let d = self
                .config
                .model
                .options
                .iter()
                .find(|d| &d.id == id)
                .ok_or_else(|| failure(ErrorCode::OptionUnsupported))?;
            d.validate(value)
                .map_err(|_| failure(ErrorCode::OptionUnsupported))?;
        }
        tokio::time::timeout(Duration::from_secs(10), self.peer.verify_account())
            .await
            .map_err(|_| failure(ErrorCode::Timeout))??;
        let thread = self.ensure_thread().await?;
        // Codex 0.157+ rejects per-turn read access (`readOnly.access`,
        // `readOnlyAccess`); read scope now comes from permission profiles.
        let sandbox = if self.config.writable {
            json!({"type":"workspaceWrite","writableRoots":[self.workspace],"networkAccess":false})
        } else {
            json!({"type":"readOnly"})
        };
        let params = json!({"threadId":thread,"model":self.config.model.native_id,"input":[{"type":"text","text":prompt}],"effort":options.get("reasoningEffort"),"serviceTier":options.get("serviceTier").cloned().unwrap_or(json!("default")),"sandboxPolicy":sandbox,"approvalPolicy":if self.config.review_requests{"onRequest"}else{"never"}});
        self.peer.sequence += 1;
        let request = self.peer.sequence;
        self.peer
            .send(json!({"id":request,"method":"turn/start","params":params}))
            .await?;
        let mut turn: Option<String> = None;
        let mut early = VecDeque::new();
        let mut delivered = BTreeSet::new();
        loop {
            let message = if turn.is_some() && !early.is_empty() {
                early.pop_front().unwrap()
            } else {
                tokio::select! {
                 value=self.peer.read()=>value?,
                 _=cx.cancelled()=>{
                  if let Some(turn)=&turn{self.peer.sequence+=1;let _=self.peer.send(json!({"id":self.peer.sequence,"method":"turn/interrupt","params":{"threadId":thread,"turnId":turn}})).await;}
                  return Err(failure(ErrorCode::Cancelled));
                 }
                }
            };
            if !message.is_object() {
                return Err(failure(ErrorCode::ProtocolIncompatible));
            }
            if message.get("method").is_none() {
                if message["id"] != request || message.get("error").is_some() {
                    return Err(failure(ErrorCode::ProviderUnavailable));
                }
                if turn.is_some() {
                    return Err(failure(ErrorCode::ProtocolIncompatible));
                }
                turn = Some(native_id(&message["result"]["turn"]["id"])?);
                continue;
            }
            let method = message["method"]
                .as_str()
                .ok_or_else(|| failure(ErrorCode::ProtocolIncompatible))?;
            // A token refresh or plan-label update keeps the turn; a sign-out or
            // model reroute does not. Account switches are caught by the
            // identity re-read before every turn.
            if method == "model/rerouted"
                || (method == "account/updated"
                    && crate::process::account_update_signs_out(&message["params"]))
            {
                return Err(failure(ErrorCode::SelectionStale));
            }
            if method == "account/updated" {
                self.peer.account_update_seen = true;
                continue;
            }
            if turn.is_none() {
                if early.len() >= 64 {
                    return Err(failure(ErrorCode::FrameTooLarge));
                }
                early.push_back(message);
                continue;
            }
            let p = &message["params"];
            if p.get("threadId").is_some_and(|id| id != &thread)
                || p.get("turnId")
                    .is_some_and(|id| Some(id.as_str().unwrap_or("")) != turn.as_deref())
            {
                return Err(failure(ErrorCode::SessionRouteMismatch));
            }
            if message.get("id").is_some() {
                if ![
                    "item/commandExecution/requestApproval",
                    "item/fileChange/requestApproval",
                ]
                .contains(&method)
                {
                    self.peer.send(json!({"id":message["id"],"error":{"code":-32601,"message":"Unsupported client request"}})).await?;
                    return Err(failure(ErrorCode::IntegrationRestricted));
                }
                if p["threadId"] != thread || p["turnId"] != turn.as_deref().unwrap_or("") {
                    return Err(failure(ErrorCode::SessionRouteMismatch));
                }
                let detail = if method.contains("commandExecution") {
                    p["command"].as_str().unwrap_or("Native command request")
                } else {
                    "Native file change request"
                };
                let escalation = [
                    "additionalPermissions",
                    "networkApprovalContext",
                    "proposedExecpolicyAmendment",
                    "grantRoot",
                ]
                .iter()
                .any(|k| p.get(k).is_some_and(|v| !v.is_null()));
                let allowed = if self.config.review_requests && !escalation {
                    cx.approve(
                        "Review native operation".into(),
                        detail.chars().take(8192).collect(),
                    )
                    .await?
                        && self.config.writable
                } else {
                    false
                };
                self.peer.send(json!({"id":message["id"],"result":{"decision":if allowed{"accept"}else{"decline"}}})).await?;
                continue;
            }
            match method {
                "item/agentMessage/delta" => {
                    let text = p["delta"]
                        .as_str()
                        .ok_or_else(|| failure(ErrorCode::ProtocolIncompatible))?;
                    delivered.insert(p["itemId"].as_str().unwrap_or("").to_owned());
                    emit_text(&cx, text, false)?;
                }
                "item/reasoning/summaryTextDelta" => {
                    if let Some(text) = p["delta"].as_str() {
                        emit_text(&cx, text, true)?;
                    }
                }
                "item/started" | "item/completed" => {
                    let item = &p["item"];
                    match item["type"].as_str().unwrap_or("") {
                        "agentMessage"
                            if method == "item/completed"
                                && !delivered.contains(item["id"].as_str().unwrap_or("")) =>
                        {
                            if let Some(text) = item["text"].as_str() {
                                emit_text(&cx, text, false)?;
                            }
                        }
                        "commandExecution" | "fileChange" | "mcpToolCall" => {
                            cx.emit(EventPayload::Tool {
                                name: item["type"].as_str().unwrap().into(),
                                status: if method == "item/started" {
                                    "started"
                                } else {
                                    "completed"
                                }
                                .into(),
                            })?;
                        }
                        _ => {}
                    }
                }
                "thread/tokenUsage/updated" => {
                    if let Some(total) = p["tokenUsage"]["total"]["totalTokens"].as_u64() {
                        cx.emit(EventPayload::Usage {
                            unit: "session_tokens".into(),
                            used: total.to_string(),
                        })?;
                    }
                }
                "turn/completed" => {
                    if p["turn"]["id"].as_str() != turn.as_deref() {
                        return Err(failure(ErrorCode::SessionRouteMismatch));
                    }
                    return match p["turn"]["status"].as_str() {
                        Some("completed") => Ok(()),
                        Some("interrupted") => Err(failure(ErrorCode::Cancelled)),
                        _ => {
                            let detail = p["turn"]["error"]["message"]
                                .as_str()
                                .or_else(|| p["turn"]["error"].as_str())
                                .filter(|message| !message.trim().is_empty())
                                .map(redact_diagnostic_detail)
                                .filter(|message| !message.is_empty())
                                .unwrap_or_else(|| "Codex app-server turn did not complete".into());
                            Err(DomainError::new(
                                ErrorCode::ProviderUnavailable,
                                format!("Codex app-server turn: {detail}"),
                            ))
                        }
                    };
                }
                "error" => {
                    let detail = p["error"]["message"]
                        .as_str()
                        .or_else(|| p["message"].as_str())
                        .filter(|message| !message.trim().is_empty())
                        .map(redact_diagnostic_detail)
                        .filter(|message| !message.is_empty())
                        .unwrap_or_else(|| "Codex app-server reported a turn error".into());
                    return Err(DomainError::new(
                        ErrorCode::ProviderUnavailable,
                        format!("Codex app-server turn: {detail}"),
                    ));
                }
                _ => {}
            }
        }
    }
}
fn emit_text(cx: &TurnContext, text: &str, reasoning: bool) -> Result<()> {
    let mut chunk = String::new();
    for ch in text.chars() {
        chunk.push(ch);
        if chunk.len() >= 16_000 {
            cx.emit(if reasoning {
                EventPayload::ReasoningSummary {
                    text: std::mem::take(&mut chunk),
                }
            } else {
                EventPayload::TextDelta {
                    text: std::mem::take(&mut chunk),
                }
            })?;
        }
    }
    if !chunk.is_empty() {
        cx.emit(if reasoning {
            EventPayload::ReasoningSummary { text: chunk }
        } else {
            EventPayload::TextDelta { text: chunk }
        })?;
    }
    Ok(())
}
impl Agent for CodexAgent {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        context: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(self.execute(prompt, options, context))
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.peer.shutdown().await;
            Ok(())
        })
    }
}
