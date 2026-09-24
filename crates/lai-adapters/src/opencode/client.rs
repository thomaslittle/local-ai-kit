use crate::http::Endpoint;
use lai_contracts::*;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    time::Duration,
};
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::ProtocolIncompatible,
        "OpenCode session response is invalid",
    )
}
fn sid(value: &Value) -> Result<String> {
    value
        .as_str()
        .filter(|s| {
            !s.is_empty()
                && s.len() < 200
                && s.bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
        })
        .map(str::to_owned)
        .ok_or_else(invalid)
}
pub async fn open(
    endpoint: Endpoint,
    model: Model,
    provider: String,
    workspace: PathBuf,
) -> Result<Box<dyn Agent>> {
    let directory = workspace.to_str().ok_or_else(invalid)?.to_owned();
    let permission = json!([{"permission":"*","pattern":"*","action":"deny"}]);
    let response=endpoint.json_query("session",&[("directory",&directory)],Some(json!({"title":"Local AI Kit text session","model":{"id":model.native_id,"providerID":provider},"permission":permission}))).await?;
    let id = sid(&response["id"])?;
    if response["permission"] != permission {
        let _ = endpoint
            .delete(&format!("session/{id}"), &[("directory", &directory)])
            .await;
        return Err(DomainError::new(
            ErrorCode::IntegrationRestricted,
            "OpenCode did not confirm deny-all tool permissions",
        ));
    }
    Ok(Box::new(Client {
        endpoint,
        model,
        provider,
        directory,
        id: Some(id),
        last_reply: None,
        poisoned: false,
    }))
}
struct Client {
    endpoint: Endpoint,
    model: Model,
    provider: String,
    directory: String,
    id: Option<String>,
    last_reply: Option<String>,
    poisoned: bool,
}
impl Client {
    async fn abort(&self, id: &str) {
        let _ = tokio::time::timeout(
            Duration::from_secs(2),
            self.endpoint.action(
                &format!("session/{id}/abort"),
                &[("directory", &self.directory)],
                json!({}),
            ),
        )
        .await;
    }
    async fn run(
        &mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        mut cx: TurnContext,
    ) -> Result<()> {
        if self.poisoned {
            return Err(invalid());
        }
        let id = self.id.clone().ok_or_else(invalid)?;
        for (key, value) in &options {
            self.model
                .options
                .iter()
                .find(|d| d.id == *key)
                .ok_or_else(invalid)?
                .validate(value)
                .map_err(|_| invalid())?;
        }
        let query = [("directory", self.directory.as_str())];
        let mut stream = tokio::select! {s=self.endpoint.event_stream("event",&query)=>s?,_=cx.cancelled()=>return Err(DomainError::new(ErrorCode::Cancelled,"Session cancelled"))};
        let mut body = json!({"model":{"providerID":self.provider,"modelID":self.model.native_id},"parts":[{"type":"text","text":prompt}]});
        if let Some(variant) = options.get("variant") {
            body["variant"] = variant.clone();
        }
        let prompt_path = format!("session/{id}/prompt_async");
        tokio::select! {r=self.endpoint.action(&prompt_path,&query,body)=>{r?;},_=cx.cancelled()=>{self.abort(&id).await;return Err(DomainError::new(ErrorCode::Cancelled,"Session cancelled"));}}
        let mut data = String::new();
        let mut emitted = String::new();
        let mut assistants = BTreeSet::new();
        let mut saw_unidentified_delta = false;
        loop {
            let line=tokio::select!{r=stream.line()=>r?,_=cx.cancelled()=>{self.abort(&id).await;return Err(DomainError::new(ErrorCode::Cancelled,"Session cancelled"));}}.ok_or_else(invalid)?;
            if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.trim_start());
                if data.len() > 1024 * 1024 {
                    return Err(invalid());
                }
                continue;
            }
            if !line.is_empty() || data.is_empty() {
                continue;
            }
            let event: Value =
                serde_json::from_str(&std::mem::take(&mut data)).map_err(|_| invalid())?;
            let p = &event["properties"];
            if event["type"] == "message.updated"
                && p["info"]["sessionID"] == id
                && p["info"]["role"] == "assistant"
            {
                if p["info"]
                    .get("modelID")
                    .is_some_and(|v| v != &self.model.native_id)
                    || p["info"]
                        .get("providerID")
                        .is_some_and(|v| v != &self.provider)
                {
                    return Err(invalid());
                }
                if assistants.len() >= 64 {
                    return Err(invalid());
                }
                assistants.insert(sid(&p["info"]["id"])?);
            }
            if p["sessionID"] != id {
                continue;
            }
            match event["type"].as_str() {
                Some("message.part.delta") if p["field"] == "text" => {
                    let message = sid(&p["messageID"])?;
                    if !assistants.contains(&message) {
                        saw_unidentified_delta = true;
                        continue;
                    }
                    if saw_unidentified_delta {
                        continue;
                    }
                    let text = p["delta"].as_str().ok_or_else(invalid)?;
                    if emitted.len() + text.len() > 1024 * 1024 {
                        return Err(invalid());
                    }
                    emit(&cx, text)?;
                    emitted.push_str(text);
                }
                Some("session.error") => {
                    return Err(DomainError::new(
                        ErrorCode::ProviderUnavailable,
                        "OpenCode reported a session error",
                    ));
                }
                Some("permission.asked" | "permission.updated") => {
                    return Err(DomainError::new(
                        ErrorCode::IntegrationRestricted,
                        "OpenCode requested a denied tool",
                    ));
                }
                Some("session.status") if p["status"]["type"] == "idle" => break,
                _ => {}
            }
        }
        let result = self
            .endpoint
            .json_query(
                &format!("session/{id}/message"),
                &[("directory", &self.directory), ("limit", "2")],
                None,
            )
            .await?;
        let messages = result.as_array().ok_or_else(invalid)?;
        let reply = messages
            .iter()
            .rev()
            .find(|m| m["info"]["role"] == "assistant")
            .ok_or_else(invalid)?;
        let reply_id = sid(&reply["info"]["id"])?;
        if self.last_reply.as_ref() == Some(&reply_id)
            || reply["info"].get("error").is_some()
            || reply["info"]["time"].get("completed").is_none()
            || reply["info"]["providerID"] != self.provider
            || reply["info"]["modelID"] != self.model.native_id
        {
            return Err(invalid());
        }
        let mut full = String::new();
        for part in reply["parts"].as_array().ok_or_else(invalid)? {
            if part["type"] == "tool" {
                return Err(DomainError::new(
                    ErrorCode::IntegrationRestricted,
                    "Unexpected tool result on text-only route",
                ));
            }
            if part["type"] == "text" {
                full.push_str(part["text"].as_str().ok_or_else(invalid)?);
                if full.len() > 1024 * 1024 {
                    return Err(invalid());
                }
            }
        }
        if !full.starts_with(&emitted) {
            return Err(invalid());
        }
        emit(&cx, &full[emitted.len()..])?;
        for (key, unit) in [("input", "input_tokens"), ("output", "output_tokens")] {
            if let Some(n) = reply["info"]["tokens"][key].as_u64() {
                cx.emit(EventPayload::Usage {
                    unit: unit.into(),
                    used: n.to_string(),
                })?;
            }
        }
        self.last_reply = Some(reply_id);
        Ok(())
    }
}
fn emit(cx: &TurnContext, text: &str) -> Result<()> {
    let mut chunk = String::new();
    for c in text.chars() {
        chunk.push(c);
        if chunk.len() >= 8192 {
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
impl Agent for Client {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let r = self.run(prompt, options, cx).await;
            if r.is_err() {
                self.poisoned = true;
                if let Some(id) = &self.id {
                    self.abort(id).await;
                }
            }
            r
        })
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            if let Some(id) = self.id.take() {
                self.endpoint
                    .delete(&format!("session/{id}"), &[("directory", &self.directory)])
                    .await?;
            }
            Ok(())
        })
    }
}
