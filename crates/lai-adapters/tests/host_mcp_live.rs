//! Opt-in live check: one Claude Code turn that calls a host MCP tool.
//! Skipped unless `LAI_LIVE_CLAUDE=<claude executable>`,
//! `LAI_LIVE_CLAUDE_HOME=<Claude config dir>`, `LAI_LIVE_MCP_URL` (a loopback
//! server exposing `echo_color`) and `LAI_LIVE_MCP_TOKEN` are all set.
//! `LAI_LIVE_CLAUDE_MODEL` picks the native model by substring (default `haiku`).
//! `LAI_LIVE_MCP_STATUS_ONLY=1` stops after confirming the server connected
//! (no prompt, no inference).
use lai_adapters::{claude, host_mcp::HostTools};
use lai_contracts::*;
use lai_runtime::Runtime;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
#[tokio::test]
async fn live_claude_turn_calls_only_the_host_tool() {
    let (Some(exe), Some(home), Ok(url), Ok(token)) = (
        std::env::var_os("LAI_LIVE_CLAUDE"),
        std::env::var_os("LAI_LIVE_CLAUDE_HOME"),
        std::env::var("LAI_LIVE_MCP_URL"),
        std::env::var("LAI_LIVE_MCP_TOKEN"),
    ) else {
        return;
    };
    let wanted = std::env::var("LAI_LIVE_CLAUDE_MODEL").unwrap_or_else(|_| "haiku".into());
    let located = lai_discovery::LocatedClient {
        provider_id: "claude_code".into(),
        profile_home: home.into(),
        executable: Some(exe.into()),
    };
    let report = claude::launch::probe(located.clone(), "c".into(), "r".into())
        .await
        .unwrap();
    let model = report
        .models
        .iter()
        .find(|m| m.native_id.contains(&wanted) || m.id.contains(&wanted))
        .expect("model listed")
        .clone();
    let servers = vec![HostMcpServer {
        name: "design".into(),
        url,
        bearer_token: token,
        allowed_tools: vec!["echo_color".into()],
    }];
    let workspace = tempfile::tempdir().unwrap();
    let launch = claude::launch::launch_with_host_mcp(
        &located,
        workspace.path().canonicalize().unwrap(),
        &servers,
    )
    .unwrap();
    claude::client::probe_host_mcp(launch.spec.clone(), &HostTools::new(&servers))
        .await
        .unwrap();
    if std::env::var_os("LAI_LIVE_MCP_STATUS_ONLY").is_some() {
        return;
    }
    let agent = claude::client::host_executor(
        launch,
        HostTools::new(&servers),
        model.clone(),
        "c".into(),
        "r".into(),
    );
    let runtime = Runtime::new();
    let session = runtime
        .insert(
            "owner",
            "c".into(),
            Selection {
                route_id: "r".into(),
                model_id: model.id.clone(),
                revision: model.revision.clone(),
                options: BTreeMap::new(),
            },
            agent,
            Arc::new(|| Ok(())),
        )
        .unwrap();
    runtime
        .start(
            "owner",
            &session.id,
            "Call the echo_color tool with color \"teal\" exactly once, then reply with the tool's result and nothing else.".into(),
            "1".into(),
        )
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(120), async {
        while matches!(
            runtime.get("owner", &session.id).unwrap().state,
            SessionState::Running | SessionState::AwaitingApproval
        ) {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
    let events = runtime.events("owner", None, None).unwrap();
    println!("{events:#}");
    assert_eq!(
        runtime.get("owner", &session.id).unwrap().state,
        SessionState::Completed
    );
    let tools: Vec<_> = events["events"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["payload"]["type"] == "tool")
        .map(|e| {
            (
                e["payload"]["name"].as_str().unwrap().to_owned(),
                e["payload"]["status"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert!(tools.contains(&("mcp__design__echo_color".into(), "completed".into())));
    assert!(
        tools
            .iter()
            .all(|(name, _)| name == "mcp__design__echo_color")
    );
    runtime.close("owner", &session.id).await.unwrap();
}
