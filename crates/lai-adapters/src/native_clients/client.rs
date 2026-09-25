use super::{
    config::NativeProfile,
    launch::{prepare, prepare_execution},
};
use crate::acp::{
    client as acp_client,
    metadata::{self, AccountEvidence},
};
use lai_contracts::*;
use lai_discovery::LocatedClient;
use lai_runtime::{Agent, BoxFuture, TurnContext};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

pub fn profile_evidence(profile: &NativeProfile) -> AccountEvidence {
    let provider = profile.provider.id();
    AccountEvidence {
        auth: Observation::reported(
            AuthState::Authenticated,
            EvidenceSource::new(
                EvidenceKind::OfficialProtocol,
                provider,
                "authenticated native ACP session established",
            ),
        ),
        billing: match &profile.billing {
            Some(mode) => Observation::reported(
                mode.clone(),
                EvidenceSource::new(
                    EvidenceKind::UserSupplied,
                    provider,
                    "host-declared billing mode for this native profile",
                ),
            ),
            None => Observation::unknown("BILLING_NOT_REPORTED"),
        },
        plan: Observation::unknown("PLAN_NOT_REPORTED"),
    }
}

pub async fn probe(
    profile: &NativeProfile,
    located: &LocatedClient,
    connection: &str,
    route: &str,
    browser: Option<&Path>,
) -> Result<ConnectionMetadata> {
    if profile.provider.id() != located.provider_id
        || profile.profile_home != located.profile_home
        || Some(&profile.executable) != located.executable.as_ref()
    {
        return Err(DomainError::new(
            ErrorCode::InvalidRequest,
            "Native profile no longer matches discovered client",
        ));
    }
    let workspace = tempfile::Builder::new()
        .prefix("lai-native-probe-")
        .tempdir()
        .map_err(|_| {
            DomainError::new(
                ErrorCode::ProviderUnavailable,
                "Native metadata workspace unavailable",
            )
        })?;
    let prepared = prepare(profile, workspace.path(), browser)?;
    let auth = prepared.auth_method.clone();
    let result = metadata::probe_with_auth(
        prepared.spec,
        profile.provider.id(),
        connection,
        route,
        profile_evidence(profile),
        auth.as_deref(),
    )
    .await;
    drop(prepared.scratch);
    result
}

pub async fn open(
    profile: &NativeProfile,
    located: &LocatedClient,
    workspace: &Path,
    browser: Option<&Path>,
    route: String,
    model: Model,
    review: bool,
) -> Result<Box<dyn Agent>> {
    open_internal(
        profile, located, workspace, browser, route, model, review, None,
    )
    .await
}
pub async fn open_resume(
    profile: &NativeProfile,
    located: &LocatedClient,
    workspace: &Path,
    browser: Option<&Path>,
    route: String,
    model: Model,
    review: bool,
    native_session_id: &str,
) -> Result<Box<dyn Agent>> {
    open_internal(
        profile,
        located,
        workspace,
        browser,
        route,
        model,
        review,
        Some(native_session_id),
    )
    .await
}
async fn open_internal(
    profile: &NativeProfile,
    located: &LocatedClient,
    workspace: &Path,
    browser: Option<&Path>,
    route: String,
    model: Model,
    review: bool,
    resume: Option<&str>,
) -> Result<Box<dyn Agent>> {
    if profile.provider.id() != located.provider_id
        || profile.profile_home != located.profile_home
        || Some(&profile.executable) != located.executable.as_ref()
    {
        return Err(DomainError::new(
            ErrorCode::InvalidRequest,
            "Native profile no longer matches discovered client",
        ));
    }
    let prepared = prepare_execution(profile, workspace, browser)?;
    let auth = prepared.auth_method.clone();
    let inner = match resume {
        Some(native_session_id) => {
            acp_client::open_resume(
                prepared.spec,
                profile.provider.id().into(),
                route,
                model,
                review,
                auth.as_deref(),
                native_session_id,
            )
            .await?
        }
        None => {
            acp_client::open_with_auth(
                prepared.spec,
                profile.provider.id().into(),
                route,
                model,
                review,
                auth.as_deref(),
            )
            .await?
        }
    };
    Ok(Box::new(OwnedNativeAgent {
        inner,
        _scratch: prepared.scratch,
    }))
}

struct OwnedNativeAgent {
    inner: Box<dyn Agent>,
    _scratch: tempfile::TempDir,
}
impl Agent for OwnedNativeAgent {
    fn turn<'a>(
        &'a mut self,
        prompt: String,
        options: BTreeMap<String, Value>,
        context: TurnContext,
    ) -> BoxFuture<'a, ()> {
        self.inner.turn(prompt, options, context)
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        self.inner.close()
    }
    fn native_resumption(&self) -> bool {
        self.inner.native_resumption()
    }
    fn suspend(&mut self) -> BoxFuture<'_, String> {
        self.inner.suspend()
    }
}
