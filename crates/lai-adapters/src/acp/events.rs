use super::{invalid, text, wire::Wire};
use lai_contracts::*;
use lai_runtime::TurnContext;
use serde_json::{Value, json};
impl Wire {
    pub async fn permission(
        &mut self,
        message: &Value,
        cx: &TurnContext,
        review: bool,
    ) -> Result<()> {
        if message["params"]["sessionId"] != self.session["sessionId"] {
            return Err(DomainError::new(
                ErrorCode::SessionRouteMismatch,
                "ACP permission belongs to another session",
            ));
        }
        if message["method"] != "session/request_permission" {
            self.deny_request(message).await?;
            return Err(DomainError::new(
                ErrorCode::IntegrationRestricted,
                "ACP requested an unadvertised client capability",
            ));
        }
        let p = &message["params"];
        let choices = p["options"]
            .as_array()
            .filter(|a| a.len() <= 32)
            .ok_or_else(invalid)?;
        let yes = choices
            .iter()
            .find(|c| c["kind"] == "allow_once")
            .map(|c| text(&c["optionId"]))
            .transpose()?;
        let no = choices
            .iter()
            .find(|c| c["kind"] == "reject_once" || c["kind"] == "reject_always")
            .map(|c| text(&c["optionId"]))
            .transpose()?;
        let allowed = if review && yes.is_some() {
            let title = p["toolCall"]["title"]
                .as_str()
                .unwrap_or("Native tool request")
                .chars()
                .take(200)
                .collect();
            cx.approve(title,"The native client requests one tool operation. Approval does not change future permissions.".into()).await?
        } else {
            false
        };
        let choice = if allowed { yes } else { no };
        let outcome = if cx.is_cancelled() {
            json!({"outcome":"cancelled"})
        } else if let Some(option) = choice {
            json!({"outcome":"selected","optionId":option})
        } else {
            json!({"outcome":"cancelled"})
        };
        self.peer
            .send(json!({"jsonrpc":"2.0","id":message["id"],"result":{"outcome":outcome}}))
            .await
    }
    pub fn update(&mut self, message: &Value, cx: &TurnContext, model: &Model) -> Result<()> {
        if message["params"]["sessionId"] != self.session["sessionId"] {
            return Err(DomainError::new(
                ErrorCode::SessionRouteMismatch,
                "ACP event belongs to another session",
            ));
        }
        let update = &message["params"]["update"];
        match update["sessionUpdate"].as_str() {
            Some("agent_message_chunk" | "agent_thought_chunk") => {
                let content = &update["content"];
                if content["type"] == "text" {
                    let text = content["text"].as_str().ok_or_else(invalid)?;
                    emit_text(cx, text, update["sessionUpdate"] == "agent_thought_chunk")?;
                } else {
                    cx.emit(EventPayload::Warning {
                        code: "UNSUPPORTED_CONTENT_BLOCK".into(),
                        message: "ACP returned a non-text content block".into(),
                    })?;
                }
            }
            Some("tool_call" | "tool_call_update") => {
                cx.emit(EventPayload::Tool {
                    name: update["title"]
                        .as_str()
                        .unwrap_or("Native tool")
                        .chars()
                        .take(200)
                        .collect(),
                    status: update["status"]
                        .as_str()
                        .unwrap_or("pending")
                        .chars()
                        .take(40)
                        .collect(),
                })?;
            }
            Some("config_option_update") => {
                self.session["configOptions"] = update["configOptions"].clone();
                let catalog = self.catalog()?;
                if !catalog
                    .models
                    .iter()
                    .any(|m| m.native_id == model.native_id && m.is_default)
                {
                    return Err(DomainError::new(
                        ErrorCode::SelectionStale,
                        "Native model changed during the turn",
                    ));
                }
                cx.emit(EventPayload::OptionsChanged {
                    revision: catalog
                        .models
                        .iter()
                        .find(|m| m.is_default)
                        .ok_or_else(invalid)?
                        .revision
                        .clone(),
                })?;
            }
            Some("usage_update") => {
                if let Some(used) = update["used"].as_u64() {
                    cx.emit(EventPayload::Usage {
                        unit: "context_tokens".into(),
                        used: used.to_string(),
                    })?;
                }
            }
            Some("current_mode_update") => {
                return Err(DomainError::new(
                    ErrorCode::SelectionStale,
                    "Native permission mode changed",
                ));
            }
            Some(
                "user_message_chunk" | "plan" | "available_commands_update" | "session_info_update",
            ) => {}
            Some(_) => {
                cx.emit(EventPayload::Warning {
                    code: "UNKNOWN_ACP_UPDATE".into(),
                    message: "An unsupported ACP update was omitted".into(),
                })?;
            }
            None => return Err(invalid()),
        }
        Ok(())
    }
}
fn emit_text(cx: &TurnContext, text: &str, thought: bool) -> Result<()> {
    let mut chunk = String::new();
    for ch in text.chars() {
        chunk.push(ch);
        if chunk.len() >= 16000 {
            let text = std::mem::take(&mut chunk);
            cx.emit(if thought {
                EventPayload::ReasoningSummary { text }
            } else {
                EventPayload::TextDelta { text }
            })?;
        }
    }
    if !chunk.is_empty() {
        cx.emit(if thought {
            EventPayload::ReasoningSummary { text: chunk }
        } else {
            EventPayload::TextDelta { text: chunk }
        })?;
    }
    Ok(())
}
