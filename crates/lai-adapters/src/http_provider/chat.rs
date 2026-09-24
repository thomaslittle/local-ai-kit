use super::{HttpKind, frames};
use crate::http::Endpoint;
use lai_contracts::*;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::{Value, json};
use std::collections::BTreeMap;
struct Chat {
    kind: HttpKind,
    endpoint: Endpoint,
    model: Model,
    history: Vec<Value>,
    closed: bool,
}
pub fn executor(kind: HttpKind, endpoint: Endpoint, model: Model) -> Box<dyn Agent> {
    Box::new(Chat {
        kind,
        endpoint: if kind == HttpKind::Anthropic {
            endpoint.anthropic_version()
        } else {
            endpoint
        },
        model,
        history: vec![],
        closed: false,
    })
}
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::ProtocolIncompatible,
        "HTTP text stream was not completed",
    )
}
impl Chat {
    async fn run(
        &mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        mut cx: TurnContext,
    ) -> Result<()> {
        if self.closed {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "HTTP conversation is closed",
            ));
        }
        for (key, value) in &options {
            self.model
                .options
                .iter()
                .find(|o| o.id == *key)
                .ok_or_else(invalid)?
                .validate(value)
                .map_err(|_| invalid())?;
        }
        if self.history.len() > 62 {
            return Err(DomainError::new(
                ErrorCode::FrameTooLarge,
                "Conversation history limit reached",
            ));
        }
        let mut history = self.history.clone();
        history.push(json!({"role":"user","content":prompt}));
        if serde_json::to_vec(&history).map_err(|_| invalid())?.len() > 1024 * 1024 {
            return Err(DomainError::new(
                ErrorCode::FrameTooLarge,
                "Conversation byte limit reached",
            ));
        }
        let (path, body) = match self.kind {
            HttpKind::Ollama => (
                "api/chat".into(),
                json!({"model":self.model.native_id,"messages":history,"stream":true}),
            ),
            HttpKind::Openai | HttpKind::Xai => (
                "responses".into(),
                json!({"model":self.model.native_id,"input":history,"stream":true,"store":false,"max_output_tokens":1024}),
            ),
            HttpKind::Anthropic => {
                let mut body = json!({"model":self.model.native_id,"messages":history,"stream":true,"max_tokens":1024});
                if let Some(effort) = options.get("effort") {
                    body["output_config"] = json!({"effort":effort});
                }
                ("messages".into(), body)
            }
            HttpKind::Gemini => {
                let name = self
                    .model
                    .native_id
                    .strip_prefix("models/")
                    .unwrap_or(&self.model.native_id);
                if name.is_empty()
                    || name
                        .bytes()
                        .any(|c| !c.is_ascii_alphanumeric() && !b"._-".contains(&c))
                {
                    return Err(invalid());
                }
                let contents=history.iter().map(|v|json!({"role":if v["role"]=="assistant"{"model"}else{"user"},"parts":[{"text":v["content"]}]})).collect::<Vec<_>>();
                (
                    format!("models/{name}:streamGenerateContent"),
                    json!({"contents":contents,"generationConfig":{"maxOutputTokens":1024}}),
                )
            }
            _ => (
                "v1/chat/completions".into(),
                json!({"model":self.model.native_id,"messages":history,"stream":true}),
            ),
        };
        // Compatible endpoints use a base ending in /v1/, LM Studio uses its root for native catalogs.
        let path = if self.kind == HttpKind::Compatible {
            "chat/completions".to_owned()
        } else {
            path
        };
        let query = if self.kind == HttpKind::Gemini {
            vec![("alt", "sse")]
        } else {
            vec![]
        };
        let mut stream = tokio::select! {r=self.endpoint.stream_query(&path,&query,body)=>r?,_=cx.cancelled()=>return Err(DomainError::new(ErrorCode::Cancelled,"HTTP request cancelled"))};
        let mut reply = String::new();
        let mut terminal = false;
        let mut data = String::new();
        loop {
            let line = tokio::select! {r=stream.line()=>r?,_=cx.cancelled()=>return Err(DomainError::new(ErrorCode::Cancelled,"HTTP request cancelled; remote compute may already have occurred"))};
            let Some(line) = line else {
                break;
            };
            let packet = if self.kind == HttpKind::Ollama {
                Some(line)
            } else if line.is_empty() {
                if data.is_empty() {
                    None
                } else {
                    Some(std::mem::take(&mut data))
                }
            } else if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.trim_start());
                if data.len() > 1024 * 1024 {
                    return Err(invalid());
                }
                None
            } else {
                None
            };
            let Some(packet) = packet else {
                continue;
            };
            if packet == "[DONE]" {
                if !terminal {
                    return Err(invalid());
                }
                break;
            }
            let value = serde_json::from_str(&packet).map_err(|_| invalid())?;
            let frame = frames::decode(self.kind, &value)?;
            if terminal && !frame.text.is_empty() {
                return Err(invalid());
            }
            if reply.len() + frame.text.len() > 1024 * 1024 {
                return Err(invalid());
            }
            reply.push_str(&frame.text);
            let mut chunk = String::new();
            for ch in frame.text.chars() {
                chunk.push(ch);
                if chunk.len() >= 8192 {
                    cx.emit(EventPayload::TextDelta {
                        text: std::mem::take(&mut chunk),
                    })?;
                }
            }
            if !chunk.is_empty() {
                cx.emit(EventPayload::TextDelta { text: chunk })?;
            }
            for (unit, used) in frame.usage {
                cx.emit(EventPayload::Usage { unit, used })?;
            }
            terminal |= frame.done;
            if frame.done
                && matches!(
                    self.kind,
                    HttpKind::Ollama
                        | HttpKind::Openai
                        | HttpKind::Xai
                        | HttpKind::Anthropic
                        | HttpKind::Gemini
                )
            {
                break;
            }
        }
        if !terminal || !data.is_empty() {
            return Err(invalid());
        }
        history.push(json!({"role":"assistant","content":reply}));
        self.history = history;
        Ok(())
    }
}
impl Agent for Chat {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let result = self.run(prompt, options, cx).await;
            if result.is_err() {
                self.closed = true;
            }
            result
        })
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.history.clear();
            self.closed = true;
            Ok(())
        })
    }
}
