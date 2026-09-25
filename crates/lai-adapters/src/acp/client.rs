use super::{
    AcpCatalog, invalid,
    wire::{Wire, response_result},
};
use crate::process::LaunchSpec;
use lai_contracts::*;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::{Value, json};
use std::{collections::BTreeMap, time::Duration};
pub async fn inspect(spec: LaunchSpec, provider: &str, route: &str) -> Result<AcpCatalog> {
    let mut wire = Wire::connect(spec, provider.into(), route.into()).await?;
    let result = wire.catalog();
    wire.close().await;
    result
}
/// The host must select a trusted native client and explicitly own its permission policy.
/// ACP callbacks alone are not an OS sandbox for the native agent's built-in tools.
pub async fn open(
    spec: LaunchSpec,
    provider: String,
    route: String,
    model: Model,
    review: bool,
) -> Result<Box<dyn Agent>> {
    open_with_auth(spec, provider, route, model, review, None).await
}
pub async fn open_with_auth(
    spec: LaunchSpec,
    provider: String,
    route: String,
    model: Model,
    review: bool,
    auth: Option<&str>,
) -> Result<Box<dyn Agent>> {
    let mut wire = Wire::connect_with_auth(spec, provider, route, auth).await?;
    if !wire
        .catalog()?
        .models
        .iter()
        .any(|m| m.id == model.id && m.revision == model.revision)
    {
        wire.close().await;
        return Err(DomainError::new(
            ErrorCode::SelectionStale,
            "ACP model configuration changed",
        ));
    }
    Ok(Box::new(AcpAgent {
        wire,
        model,
        review,
    }))
}
struct AcpAgent {
    wire: Wire,
    model: Model,
    review: bool,
}
impl AcpAgent {
    async fn configure(&mut self, options: &BTreeMap<String, Value>) -> Result<()> {
        let catalog = self.wire.catalog()?;
        let sid = catalog.session_id;
        if !catalog
            .models
            .iter()
            .any(|m| m.native_id == self.model.native_id && m.is_default)
        {
            if let Some(model) = catalog
                .config_options
                .iter()
                .find(|d| d.category.as_deref() == Some("model") || d.id == "model")
            {
                let response = self
                    .wire
                    .call(
                        "session/set_config_option",
                        json!({"sessionId":sid,"configId":model.id,"value":self.model.native_id}),
                    )
                    .await?;
                self.wire.session["configOptions"] = response["configOptions"].clone();
            } else {
                self.wire
                    .call(
                        "session/set_model",
                        json!({"sessionId":sid,"modelId":self.model.native_id}),
                    )
                    .await?;
                self.wire.session["models"]["currentModelId"] = json!(self.model.native_id);
            }
            self.wire.apply_notices()?;
        }
        for (id, value) in options {
            let config = self.wire.catalog()?;
            let descriptor = config
                .config_options
                .iter()
                .find(|d| d.id == *id)
                .ok_or_else(invalid)?;
            descriptor.validate(value).map_err(|_| {
                DomainError::new(ErrorCode::OptionUnsupported, "Native option changed")
            })?;
            if descriptor.permission_sensitive {
                return Err(DomainError::new(
                    ErrorCode::ApprovalRequired,
                    "Permission changes require a dedicated host policy",
                ));
            }
            if descriptor.category.as_deref() == Some("model") {
                return Err(invalid());
            }
            if descriptor.current_value.as_ref() == Some(value) {
                continue;
            }
            let mut params = json!({"sessionId":sid,"configId":id,"value":value});
            if matches!(descriptor.control, OptionControl::Boolean) {
                params["type"] = json!("boolean");
            }
            let response = self.wire.call("session/set_config_option", params).await?;
            self.wire.session["configOptions"] = response["configOptions"].clone();
            self.wire.apply_notices()?;
        }
        let current = self.wire.catalog()?;
        if !current
            .models
            .iter()
            .any(|m| m.native_id == self.model.native_id && m.is_default)
        {
            return Err(DomainError::new(
                ErrorCode::SelectionStale,
                "ACP model switch was not accepted",
            ));
        }
        for (id, value) in options {
            if !current
                .config_options
                .iter()
                .any(|d| d.id == *id && d.current_value.as_ref() == Some(value))
            {
                return Err(DomainError::new(
                    ErrorCode::SelectionStale,
                    "ACP configuration was not accepted",
                ));
            }
        }
        Ok(())
    }
    async fn run(
        &mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        mut cx: TurnContext,
    ) -> Result<()> {
        tokio::select! {
         result=tokio::time::timeout(Duration::from_secs(20),self.configure(&options))=>result.map_err(|_|DomainError::new(ErrorCode::Timeout,"ACP configuration timed out"))??,
         _=cx.cancelled()=>return Err(DomainError::new(ErrorCode::Cancelled,"ACP turn cancelled"))
        }
        let sid = self.wire.session["sessionId"].clone();
        let id = self
            .wire
            .send_request(
                "session/prompt",
                json!({"sessionId":sid,"prompt":[{"type":"text","text":prompt}]}),
            )
            .await?;
        loop {
            let response = tokio::select! {value=self.wire.read()=>value?,_=cx.cancelled()=>{
             let _=tokio::time::timeout(Duration::from_millis(300),self.wire.peer.send(json!({"jsonrpc":"2.0","method":"session/cancel","params":{"sessionId":sid}}))).await;
             return Err(DomainError::new(ErrorCode::Cancelled,"ACP turn cancelled"));
            }};
            if response.get("method").is_some() {
                if response.get("id").is_some() {
                    self.wire.permission(&response, &cx, self.review).await?;
                } else if response["method"] == "session/update" {
                    self.wire.update(&response, &cx, &self.model)?;
                }
                continue;
            }
            if response["id"] != id {
                return Err(invalid());
            }
            let result = response_result(&response)?;
            return match result["stopReason"].as_str() {
                Some("end_turn") => Ok(()),
                Some("max_tokens" | "max_turn_requests") => {
                    cx.emit(EventPayload::Warning {
                        code: "NATIVE_TURN_LIMIT".into(),
                        message: "Native client stopped at its turn limit".into(),
                    })?;
                    Ok(())
                }
                Some("cancelled") => Err(DomainError::new(
                    ErrorCode::Cancelled,
                    "Native turn cancelled",
                )),
                Some("refusal") => Err(DomainError::new(
                    ErrorCode::IntegrationRestricted,
                    "Native client declined the request",
                )),
                _ => Err(invalid()),
            };
        }
    }
}
impl Agent for AcpAgent {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(self.run(prompt, options, cx))
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.wire.close().await;
            Ok(())
        })
    }
}

/// Explicit native auth negotiation; no token is sent by the host.
pub async fn inspect_authenticated(
    spec: LaunchSpec,
    provider: &str,
    route: &str,
    auth: Option<&str>,
) -> Result<AcpCatalog> {
    let mut wire = Wire::connect_with_auth(spec, provider.into(), route.into(), auth).await?;
    let result = wire.catalog();
    wire.close().await;
    result
}
