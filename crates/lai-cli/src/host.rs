use crate::args::{Action, Args};
use lai_core::ClientPolicy;
pub use lai_kit::{build_service_with_state, default_state_dir};
use lai_service::ExecutionGrant;
use std::collections::BTreeMap;
pub fn policy(args: &Args) -> ClientPolicy {
    let mut policy = if args.allow_passive || matches!(args.action, Action::Scan) {
        ClientPolicy::passive()
    } else {
        ClientPolicy::denied()
    };
    policy.probe_provider_ids = args.metadata_providers.clone();
    policy
}
pub fn execution_grant(args: &Args) -> Option<ExecutionGrant> {
    if !args.allow_execution {
        return None;
    }
    Some(ExecutionGrant {
        provider_ids: args.execution_providers.clone(),
        workspaces: BTreeMap::from([("default".into(), args.workspace.clone()?)]),
        allow_paid: args.allow_paid,
        allow_writes: args.allow_writes,
        review_native: args.review_requests,
        ..Default::default()
    })
}
