use super::{AcpCatalog, invalid, parse_catalog, text};
use crate::process::{LaunchSpec, Peer};
use lai_contracts::*;
use serde_json::{Value, json};
use std::time::Duration;
pub(super) struct Wire {
    pub peer: Peer,
    pub init: Value,
    pub session: Value,
    pub notices: Vec<Value>,
    pub provider: String,
    pub route: String,
}
impl Wire {
    pub async fn connect(spec: LaunchSpec, provider: String, route: String) -> Result<Self> {
        let cwd = spec.cwd.clone();
        let mut this = Self {
            peer: Peer::spawn(spec).await?,
            init: Value::Null,
            session: Value::Null,
            notices: vec![],
            provider,
            route,
        };
        let outcome=tokio::time::timeout(Duration::from_secs(20),async {
   this.init=this.call("initialize",json!({"protocolVersion":1,"clientInfo":{"name":"local-ai-kit","version":env!("CARGO_PKG_VERSION")},"clientCapabilities":{"fs":{"readTextFile":false,"writeTextFile":false},"terminal":false,"session":{"configOptions":{"boolean":{}}}}})).await?;
   if this.init["protocolVersion"].as_u64()!=Some(1){return Err(invalid());}
   this.session=this.call("session/new",json!({"cwd":cwd,"mcpServers":[]})).await?;
   this.catalog()?;
   this.apply_notices()?;
   Ok(())
  }).await.unwrap_or_else(|_|Err(DomainError::new(ErrorCode::Timeout,"ACP initialization timed out")));
        if let Err(error) = outcome {
            this.peer.shutdown().await;
            return Err(error);
        }
        Ok(this)
    }
    pub fn catalog(&self) -> Result<AcpCatalog> {
        parse_catalog(&self.provider, &self.route, &self.init, &self.session)
    }
    pub async fn send_request(&mut self, method: &str, params: Value) -> Result<u64> {
        self.peer.sequence += 1;
        let id = self.peer.sequence;
        self.peer
            .send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .await?;
        Ok(id)
    }
    pub async fn read(&mut self) -> Result<Value> {
        let v = self.peer.read().await?;
        if v["jsonrpc"] != "2.0" || !v.is_object() {
            return Err(invalid());
        }
        Ok(v)
    }
    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.send_request(method, params).await?;
        for _ in 0..512 {
            let response = self.read().await?;
            if response.get("method").is_some() {
                if response.get("id").is_some() {
                    self.deny_request(&response).await?;
                    return Err(DomainError::new(
                        ErrorCode::IntegrationRestricted,
                        "ACP requested an unadvertised setup operation",
                    ));
                }
                if response["method"] == "session/update" {
                    if self.notices.len() >= 128 {
                        return Err(DomainError::new(
                            ErrorCode::FrameTooLarge,
                            "Too many ACP updates",
                        ));
                    }
                    self.notices.push(response);
                }
                continue;
            }
            if response["id"] != id {
                return Err(invalid());
            }
            return response_result(&response);
        }
        Err(invalid())
    }
    pub fn apply_notices(&mut self) -> Result<()> {
        let session = text(&self.session["sessionId"])?;
        for notice in std::mem::take(&mut self.notices) {
            if notice["params"]["sessionId"] != session {
                return Err(DomainError::new(
                    ErrorCode::SessionRouteMismatch,
                    "ACP session changed",
                ));
            }
            let update = &notice["params"]["update"];
            if update["sessionUpdate"] == "config_option_update" {
                self.session["configOptions"] = update["configOptions"].clone();
            }
        }
        self.catalog()?;
        Ok(())
    }
    pub async fn deny_request(&mut self, request: &Value) -> Result<()> {
        let id = request
            .get("id")
            .filter(|id| id.is_string() || id.is_u64())
            .ok_or_else(invalid)?
            .clone();
        if request["method"] == "session/request_permission" {
            self.peer
                .send(json!({"jsonrpc":"2.0","id":id,"result":{"outcome":{"outcome":"cancelled"}}}))
                .await
        } else {
            self.peer.send(json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Client capability not available"}})).await
        }
    }
    pub async fn close(&mut self) {
        if self.init["agentCapabilities"]["sessionCapabilities"]
            .get("close")
            .is_some()
        {
            let id = self.session["sessionId"].clone();
            let _ = tokio::time::timeout(
                Duration::from_millis(500),
                self.call("session/close", json!({"sessionId":id})),
            )
            .await;
        }
        self.peer.shutdown().await;
    }
}
pub(super) fn response_result(value: &Value) -> Result<Value> {
    match (value.get("result"), value.get("error")) {
        (Some(result), None) if result.is_object() => Ok(result.clone()),
        (None, Some(error)) => Err(DomainError::new(
            if error["code"].as_i64() == Some(-32601) {
                ErrorCode::CapabilityUnsupported
            } else {
                ErrorCode::ProviderUnavailable
            },
            "ACP request was rejected",
        )),
        _ => Err(invalid()),
    }
}
