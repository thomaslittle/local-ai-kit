use crate::{Agent, Guard, TurnContext, err, journal::Journal};
use lai_contracts::*;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::sync::{Mutex as AsyncMutex, oneshot, watch};
static NEXT: AtomicU64 = AtomicU64::new(1);
pub(crate) fn id(prefix: &str) -> String {
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}
pub(crate) struct Pending {
    pub session: String,
    pub sender: oneshot::Sender<bool>,
}
pub(crate) struct State {
    pub view: Session,
    pub requests: BTreeSet<String>,
    pub turn: Option<String>,
    pub cancel: Option<watch::Sender<bool>>,
    pub usable: bool,
}
pub(crate) struct Cell {
    pub owner: String,
    pub state: Mutex<State>,
    pub agent: AsyncMutex<Option<Box<dyn Agent>>>,
    pub guard: Guard,
    pub active: AtomicBool,
    pub approvals: Mutex<BTreeMap<String, Pending>>,
    pub journal: Arc<Mutex<Journal>>,
}
impl Cell {
    pub fn emit(&self, turn: Option<String>, payload: EventPayload) -> Result<()> {
        if !self.active.load(Ordering::Acquire) {
            return Err(err(ErrorCode::LocalAccessDenied));
        }
        let state = self
            .state
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
        self.journal
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .append(Event {
                session_id: state.view.id.clone(),
                turn_id: turn,
                sequence: String::new(),
                timestamp: Timestamp::now(),
                adapter_id: "native".into(),
                schema_version: "1.0".into(),
                payload,
            })
    }
}
#[derive(Default)]
pub struct Runtime {
    sessions: Mutex<BTreeMap<String, Arc<Cell>>>,
    journals: Mutex<BTreeMap<String, Arc<Mutex<Journal>>>>,
}
impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(
        &self,
        owner: &str,
        connection: String,
        selection: Selection,
        agent: Box<dyn Agent>,
        guard: Guard,
    ) -> Result<Session> {
        if owner.is_empty() || owner.len() > 128 {
            return Err(err(ErrorCode::InvalidRequest));
        }
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
        sessions.retain(|_, cell| {
            cell.state
                .lock()
                .map(|s| s.view.state != SessionState::Closed)
                .unwrap_or(true)
        });
        if sessions.len() >= 64 || sessions.values().filter(|s| s.owner == owner).count() >= 16 {
            return Err(err(ErrorCode::RateLimited));
        }
        let view = Session {
            id: id("session"),
            connection_id: connection,
            selection,
            state: SessionState::Ready,
            created_at: Timestamp::now(),
            native_resumption: false,
        };
        let journal = self
            .journals
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .entry(owner.into())
            .or_default()
            .clone();
        sessions.insert(
            view.id.clone(),
            Arc::new(Cell {
                owner: owner.into(),
                state: Mutex::new(State {
                    view: view.clone(),
                    requests: BTreeSet::new(),
                    turn: None,
                    cancel: None,
                    usable: true,
                }),
                agent: AsyncMutex::new(Some(agent)),
                guard,
                active: AtomicBool::new(true),
                approvals: Mutex::new(BTreeMap::new()),
                journal,
            }),
        );
        Ok(view)
    }
    fn cell(&self, owner: &str, session: &str) -> Result<Arc<Cell>> {
        self.sessions
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .get(session)
            .filter(|s| s.owner == owner && s.active.load(Ordering::Acquire))
            .cloned()
            .ok_or_else(|| err(ErrorCode::NotFound))
    }
    pub fn get(&self, owner: &str, session: &str) -> Result<Session> {
        Ok(self
            .cell(owner, session)?
            .state
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .view
            .clone())
    }
    pub fn update_options(
        &self,
        owner: &str,
        session: &str,
        options: BTreeMap<String, Value>,
    ) -> Result<Session> {
        let cell = self.cell(owner, session)?;
        (cell.guard)()?;
        let mut state = cell
            .state
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
        if !matches!(
            state.view.state,
            SessionState::Ready | SessionState::Completed
        ) {
            return Err(err(ErrorCode::Conflict));
        }
        state.view.selection.options = options;
        Ok(state.view.clone())
    }
    pub async fn start(
        &self,
        owner: &str,
        session: &str,
        prompt: String,
        request: String,
    ) -> Result<Value> {
        if prompt.is_empty()
            || prompt.len() > 65_536
            || prompt.contains('\0')
            || request.is_empty()
            || request.len() > 128
        {
            return Err(err(ErrorCode::InvalidRequest));
        }
        let cell = self.cell(owner, session)?;
        (cell.guard)()?;
        let (turn, options, cancel) = {
            let mut state = cell
                .state
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
            if !state.usable
                || !matches!(
                    state.view.state,
                    SessionState::Ready | SessionState::Completed
                )
                || state.requests.len() >= 256
                || !state.requests.insert(request)
            {
                return Err(err(ErrorCode::Conflict));
            }
            let turn = id("turn");
            let (tx, rx) = watch::channel(false);
            state.turn = Some(turn.clone());
            state.cancel = Some(tx);
            state.view.state = SessionState::Running;
            (turn, state.view.selection.options.clone(), rx)
        };
        cell.emit(
            Some(turn.clone()),
            EventPayload::State {
                state: SessionState::Running,
            },
        )?;
        let run_id = turn.clone();
        tokio::spawn(async move {
            run(cell, run_id, prompt, options, cancel).await;
        });
        Ok(json!({"turnId":turn}))
    }
    pub async fn cancel(&self, owner: &str, turn: &str) -> Result<Value> {
        let cells: Vec<_> = self
            .sessions
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .values()
            .filter(|s| s.owner == owner)
            .cloned()
            .collect();
        for cell in cells {
            let state = cell
                .state
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
            if state.turn.as_deref() == Some(turn) {
                if let Some(cancel) = &state.cancel {
                    cancel.send_replace(true);
                }
                return Ok(json!({"cancelled":true}));
            }
        }
        Err(err(ErrorCode::NotFound))
    }
    pub async fn close(&self, owner: &str, session: &str) -> Result<Value> {
        let cell = self.cell(owner, session)?;
        {
            let mut state = cell
                .state
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
            state.view.state = SessionState::Closed;
            state.usable = false;
            if let Some(cancel) = &state.cancel {
                cancel.send_replace(true);
            }
        }
        cell.approvals
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .clear();
        let mut agent = tokio::time::timeout(Duration::from_secs(5), cell.agent.lock())
            .await
            .map_err(|_| err(ErrorCode::Timeout))?;
        if let Some(mut driver) = agent.take() {
            let _ = tokio::time::timeout(Duration::from_secs(2), driver.close()).await;
        }
        Ok(json!({"closed":true}))
    }
    pub fn approve(&self, owner: &str, approval: &str, allow: bool) -> Result<Value> {
        let cells: Vec<_> = self
            .sessions
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .values()
            .filter(|s| s.owner == owner && s.active.load(Ordering::Acquire))
            .cloned()
            .collect();
        for cell in cells {
            let pending = cell
                .approvals
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?
                .remove(approval);
            if let Some(p) = pending {
                (cell.guard)()?;
                if p.session
                    != cell
                        .state
                        .lock()
                        .map_err(|_| err(ErrorCode::ProviderUnavailable))?
                        .view
                        .id
                {
                    return Err(err(ErrorCode::NotFound));
                }
                p.sender.send(allow).map_err(|_| err(ErrorCode::NotFound))?;
                return Ok(json!({"allowed":allow}));
            }
        }
        Err(err(ErrorCode::NotFound))
    }
    pub fn events(&self, owner: &str, session: Option<&str>, after: Option<u64>) -> Result<Value> {
        if let Some(session) = session {
            self.cell(owner, session)?;
        }
        let journals = self
            .journals
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
        match journals.get(owner) {
            Some(j) => j
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?
                .read(session, after),
            None => Ok(json!({"events":[],"cursor":"0"})),
        }
    }
    pub fn revoke(&self, owner: &str) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.retain(|_, cell| {
                if cell.owner != owner {
                    return true;
                }
                cell.active.store(false, Ordering::Release);
                if let Ok(state) = cell.state.lock() {
                    if let Some(cancel) = &state.cancel {
                        cancel.send_replace(true);
                    }
                }
                let cell = cell.clone();
                tokio::spawn(async move {
                    let mut agent = cell.agent.lock().await;
                    if let Some(mut driver) = agent.take() {
                        let _ = tokio::time::timeout(Duration::from_secs(2), driver.close()).await;
                    }
                });
                false
            });
        }
        if let Ok(mut journals) = self.journals.lock() {
            journals.remove(owner);
        }
    }
}
async fn run(
    cell: Arc<Cell>,
    turn: String,
    prompt: String,
    options: BTreeMap<String, Value>,
    mut cancel: watch::Receiver<bool>,
) {
    let mut agent = cell.agent.lock().await;
    let result = if let Some(driver) = agent.as_mut() {
        let cx = TurnContext {
            cell: cell.clone(),
            turn: turn.clone(),
            cancel: cancel.clone(),
        };
        let operation = driver.turn(prompt, options, cx);
        tokio::pin!(operation);
        tokio::select! {
         result=&mut operation=>result,
         _=async{if !*cancel.borrow(){let _=cancel.changed().await;}}=>{
          let _=tokio::time::timeout(Duration::from_secs(2),&mut operation).await;Err(err(ErrorCode::Cancelled))
         },
         _=tokio::time::sleep(Duration::from_secs(600))=>Err(err(ErrorCode::Timeout))
        }
    } else {
        Err(err(ErrorCode::ProviderUnavailable))
    };
    let result = if *cancel.borrow() {
        Err(err(ErrorCode::Cancelled))
    } else {
        result
    };
    if result.is_err() {
        if let Some(mut driver) = agent.take() {
            let _ = tokio::time::timeout(Duration::from_secs(2), driver.close()).await;
        }
    }
    if let Ok(mut approvals) = cell.approvals.lock() {
        approvals.clear();
    }
    let state = match &result {
        Ok(()) => SessionState::Completed,
        Err(e) if e.code == ErrorCode::Cancelled => SessionState::Cancelled,
        _ => SessionState::Error,
    };
    if let Ok(mut record) = cell.state.lock() {
        record.usable = result.is_ok();
        record.cancel = None;
        if record.view.state != SessionState::Closed {
            record.view.state = state.clone();
        }
    }
    if let Err(error) = &result {
        let _ = cell.emit(
            Some(turn.clone()),
            EventPayload::Error {
                code: serde_json::to_value(&error.code)
                    .ok()
                    .and_then(|v| v.as_str().map(str::to_owned))
                    .unwrap_or_default(),
                message: "Native turn did not complete".into(),
            },
        );
    }
    let _ = cell.emit(Some(turn.clone()), EventPayload::State { state });
    let _ = cell.emit(
        Some(turn),
        EventPayload::Final {
            status: if result.is_ok() {
                "completed"
            } else if *cancel.borrow() {
                "cancelled"
            } else {
                "error"
            }
            .into(),
        },
    );
}
