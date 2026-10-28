//! Broker I/O. Both workers use the public authenticated JSON-RPC surface and
//! never touch the broker store or provider credentials directly.

use crate::history::{History, Sample};
use lai_contracts::Snapshot;
use lai_control::BrokerClient;
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread,
    time::{Duration, Instant},
};

pub struct Bundle {
    pub snapshot: Snapshot,
    pub runtime: Value,
    pub apps: Vec<Value>,
    pub apps_error: Option<String>,
    pub adapters: Vec<Value>,
    pub history: Vec<Sample>,
    pub fetched_at: i64,
    pub error: Option<String>,
}

pub enum Command {
    Refresh,
    Probe(String),
    RevokeApp(String),
}

pub struct BrokerHandle {
    pub commands: Sender<Command>,
    pub updates: Receiver<Bundle>,
}

pub fn spawn(token_file: PathBuf, refresh: Duration) -> BrokerHandle {
    let (command_tx, command_rx) = std::sync::mpsc::channel();
    let (update_tx, update_rx) = std::sync::mpsc::channel();
    thread::Builder::new()
        .name("lai-broker".into())
        .spawn(move || worker(token_file, refresh, command_rx, update_tx))
        .expect("broker worker");
    BrokerHandle {
        commands: command_tx,
        updates: update_rx,
    }
}

fn connect(token_file: &PathBuf) -> Result<BrokerClient, String> {
    if std::env::var_os("LAI_DEBUG").is_some() {
        eprintln!("[debug] connecting with {}", token_file.display());
    }
    let result = BrokerClient::from_token_file(token_file, None);
    if std::env::var_os("LAI_DEBUG").is_some() {
        eprintln!(
            "[debug] connect result: {}",
            if result.is_ok() { "ok" } else { "err" }
        );
    }
    result
}

fn fetch(client: &mut BrokerClient, history: &mut History) -> Result<Bundle, String> {
    let snapshot = client.snapshot()?;
    let runtime = client
        .call("runtime.health", json!({}))
        .unwrap_or(Value::Null);
    let (apps, apps_error) = match client.apps() {
        Ok(apps) => (apps, None),
        Err(error) => (Vec::new(), Some(error)),
    };
    let adapters = client
        .call("adapters.list", json!({}))
        .ok()
        .and_then(|value| value.as_array().cloned())
        .unwrap_or_default();
    let now = chrono::Utc::now().timestamp();
    history.observe(&snapshot, now);
    Ok(Bundle {
        snapshot,
        runtime,
        apps,
        apps_error,
        adapters,
        history: history.samples.clone(),
        fetched_at: now,
        error: None,
    })
}

fn worker(
    token_file: PathBuf,
    refresh: Duration,
    commands: Receiver<Command>,
    updates: Sender<Bundle>,
) {
    let mut history = History::load();
    let mut client: Option<BrokerClient> = None;
    let mut last_error: Option<String> = None;

    loop {
        if client.is_none() {
            match connect(&token_file) {
                Ok(mut connected) => {
                    match fetch(&mut connected, &mut history) {
                        Ok(bundle) => {
                            let _ = updates.send(bundle);
                        }
                        Err(error) => {
                            last_error = Some(error);
                        }
                    }
                    client = Some(connected);
                }
                Err(error) => {
                    if std::env::var_os("LAI_DEBUG").is_some() {
                        eprintln!("[debug] connect failed: {error}");
                    }
                    if last_error.as_deref() != Some(error.as_str()) {
                        let _ = updates.send(Bundle {
                            snapshot: Snapshot::empty(lai_contracts::DataMode::Live),
                            runtime: Value::Null,
                            apps: Vec::new(),
                            apps_error: Some(
                                "No administrative credential; app management is unavailable."
                                    .into(),
                            ),
                            adapters: Vec::new(),
                            history: history.samples.clone(),
                            fetched_at: chrono::Utc::now().timestamp(),
                            error: Some(error.clone()),
                        });
                        last_error = Some(error);
                    }
                    thread::sleep(Duration::from_secs(3));
                    if commands.try_recv().is_ok() {
                        continue;
                    }
                    continue;
                }
            }
        }

        match commands.recv_timeout(refresh) {
            Ok(Command::Refresh) | Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if let Some(active) = client.as_mut() {
                    match fetch(active, &mut history) {
                        Ok(bundle) => {
                            if std::env::var_os("LAI_DEBUG").is_some() {
                                eprintln!(
                                    "[debug] bundle sent: {} connections",
                                    bundle.snapshot.connections.len()
                                );
                            }
                            let _ = updates.send(bundle);
                        }
                        Err(error) => {
                            let _ = updates.send(Bundle {
                                snapshot: Snapshot::empty(lai_contracts::DataMode::Live),
                                runtime: Value::Null,
                                apps: Vec::new(),
                                apps_error: Some("Broker became unavailable; reconnecting.".into()),
                                adapters: Vec::new(),
                                history: history.samples.clone(),
                                fetched_at: chrono::Utc::now().timestamp(),
                                error: Some(error),
                            });
                            client = None;
                        }
                    }
                }
            }
            Ok(Command::Probe(connection_id)) => {
                if let Some(active) = client.as_mut() {
                    let _ =
                        active.call("connections.probe", json!({"connectionId": connection_id}));
                    if let Ok(bundle) = fetch(active, &mut history) {
                        let _ = updates.send(bundle);
                    }
                }
            }
            Ok(Command::RevokeApp(target)) => {
                if let Some(active) = client.as_mut() {
                    let _ = active.call("apps.revoke", json!({"targetClientId": target}));
                    if let Ok(bundle) = fetch(active, &mut history) {
                        let _ = updates.send(bundle);
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    if let Some(client) = client {
        client.close();
    }
}

// ---------------------------------------------------------------------------
// Playground execution worker
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ModelChoice {
    pub route_id: String,
    pub model_id: String,
    pub revision: String,
    pub label: String,
    pub connection: String,
    pub provider: String,
    pub provider_id: String,
    pub billing: String,
}

pub enum ExecCommand {
    Run { prompt: String, choice: ModelChoice },
    Cancel,
    Close(String),
}

#[derive(Clone, Debug)]
pub enum ExecEvent {
    Started {
        session_id: String,
        turn_id: String,
        label: String,
    },
    Delta {
        session_id: String,
        text: String,
    },
    Usage {
        session_id: String,
        description: String,
    },
    Finished {
        session_id: String,
        status: String,
    },
    Failed {
        error: String,
    },
}

pub struct ExecHandle {
    pub commands: Sender<ExecCommand>,
    pub events: Receiver<ExecEvent>,
}

pub fn spawn_executor(token_file: PathBuf) -> ExecHandle {
    let (command_tx, command_rx) = std::sync::mpsc::channel();
    let (event_tx, event_rx) = std::sync::mpsc::channel();
    thread::Builder::new()
        .name("lai-playground".into())
        .spawn(move || executor(token_file, command_rx, event_tx))
        .expect("execution worker");
    ExecHandle {
        commands: command_tx,
        events: event_rx,
    }
}

fn executor(token_file: PathBuf, commands: Receiver<ExecCommand>, events: Sender<ExecEvent>) {
    let mut client: Option<BrokerClient> = None;
    #[allow(unused_assignments)]
    let mut current_session: Option<String> = None;
    #[allow(unused_assignments)]
    let mut current_turn: Option<String> = None;

    while let Ok(command) = commands.recv() {
        match command {
            ExecCommand::Run { prompt, choice } => {
                if current_session.is_some() {
                    let _ = events.send(ExecEvent::Failed {
                        error: "A turn is already running.".into(),
                    });
                    continue;
                }
                if client.is_none() {
                    match connect(&token_file) {
                        Ok(connected) => client = Some(connected),
                        Err(error) => {
                            let _ = events.send(ExecEvent::Failed { error });
                            continue;
                        }
                    }
                }
                let Some(active) = client.as_mut() else {
                    continue;
                };
                let selection = json!({
                    "routeId": choice.route_id,
                    "modelId": choice.model_id,
                    "revision": choice.revision,
                    "options": {},
                });
                let created = active.call(
                    "sessions.create",
                    json!({
                        "selection": selection,
                        "workspaceId": "default",
                        "permissionMode": "read_only",
                        "reviewRequests": false,
                        "allowPaid": false,
                    }),
                );
                let session_id = match created {
                    Ok(value) => value
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    Err(error) => {
                        let _ = events.send(ExecEvent::Failed {
                            error: format!("Session could not start: {error}"),
                        });
                        continue;
                    }
                };
                let started = active.call(
                    "turns.start",
                    json!({
                        "sessionId": session_id,
                        "prompt": prompt,
                        "requestId": format!("desktop-{}", chrono::Utc::now().timestamp_millis()),
                    }),
                );
                let turn_id = match started {
                    Ok(value) => value
                        .get("turnId")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    Err(error) => {
                        let _ = active.call("sessions.close", json!({"sessionId": session_id}));
                        let _ = events.send(ExecEvent::Failed {
                            error: format!("Turn could not start: {error}"),
                        });
                        continue;
                    }
                };
                current_session = Some(session_id.clone());
                current_turn = Some(turn_id.clone());
                let _ = (&current_session, &current_turn);
                let _ = events.send(ExecEvent::Started {
                    session_id: session_id.clone(),
                    turn_id: turn_id.clone(),
                    label: choice.label.clone(),
                });

                let mut cursor: Option<String> = None;
                let deadline = Instant::now() + Duration::from_secs(180);
                let mut idle = 0u32;
                let mut finished = false;
                while Instant::now() < deadline && !finished {
                    thread::sleep(Duration::from_millis(500));
                    let mut params = json!({"sessionId": session_id});
                    if let Some(after) = &cursor {
                        params["after"] = json!(after);
                    }
                    let page = match active.call("events.read", params) {
                        Ok(page) => page,
                        Err(error) => {
                            let _ = events.send(ExecEvent::Failed {
                                error: format!("Event stream stopped: {error}"),
                            });
                            break;
                        }
                    };
                    let empty = page
                        .get("events")
                        .and_then(Value::as_array)
                        .is_none_or(Vec::is_empty);
                    if empty {
                        idle += 1;
                        if idle >= 40 {
                            break;
                        }
                        continue;
                    }
                    idle = 0;
                    if let Some(next) = page.get("cursor").and_then(Value::as_str) {
                        cursor = Some(next.to_string());
                    }
                    for event in page
                        .get("events")
                        .and_then(Value::as_array)
                        .cloned()
                        .unwrap_or_default()
                    {
                        let payload = event.get("payload").cloned().unwrap_or(Value::Null);
                        match payload.get("type").and_then(Value::as_str) {
                            Some("text_delta") => {
                                if let Some(text) = payload.get("text").and_then(Value::as_str) {
                                    let _ = events.send(ExecEvent::Delta {
                                        session_id: session_id.clone(),
                                        text: text.to_string(),
                                    });
                                }
                            }
                            Some("usage") => {
                                let unit = payload
                                    .get("unit")
                                    .and_then(Value::as_str)
                                    .unwrap_or("tokens");
                                let used =
                                    payload.get("used").and_then(Value::as_str).unwrap_or("?");
                                let _ = events.send(ExecEvent::Usage {
                                    session_id: session_id.clone(),
                                    description: format!("{used} {unit}"),
                                });
                            }
                            Some("final") => {
                                let status = payload
                                    .get("status")
                                    .and_then(Value::as_str)
                                    .unwrap_or("completed");
                                let _ = events.send(ExecEvent::Finished {
                                    session_id: session_id.clone(),
                                    status: status.to_string(),
                                });
                                finished = true;
                            }
                            Some("error") => {
                                let message = payload
                                    .get("message")
                                    .and_then(Value::as_str)
                                    .unwrap_or("The provider reported an error.");
                                let _ = events.send(ExecEvent::Failed {
                                    error: message.to_string(),
                                });
                                finished = true;
                            }
                            _ => {}
                        }
                    }
                }
                if !finished {
                    let _ = events.send(ExecEvent::Finished {
                        session_id: session_id.clone(),
                        status: "idle".into(),
                    });
                }
                current_session = None;
                current_turn = None;
            }
            ExecCommand::Cancel => {
                if let (Some(active), Some(session), Some(turn)) = (
                    client.as_mut(),
                    current_session.clone(),
                    current_turn.clone(),
                ) {
                    let _ = active.call(
                        "turns.cancel",
                        json!({"sessionId": session, "turnId": turn}),
                    );
                }
            }
            ExecCommand::Close(session_id) => {
                if let Some(active) = client.as_mut() {
                    let _ = active.call("sessions.close", json!({"sessionId": session_id}));
                }
            }
        }
    }
    if let Some(client) = client {
        client.close();
    }
    let _ = TryRecvError::Empty;
}
