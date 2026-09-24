use crate::{
    engine::{Cell, Pending, id},
    err,
};
use lai_contracts::*;
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use tokio::sync::{oneshot, watch};
pub struct TurnContext {
    pub(crate) cell: Arc<Cell>,
    pub(crate) turn: String,
    pub(crate) cancel: watch::Receiver<bool>,
}
impl TurnContext {
    pub fn emit(&self, payload: EventPayload) -> Result<()> {
        (self.cell.guard)()?;
        if *self.cancel.borrow() || !self.cell.active.load(Ordering::Acquire) {
            return Err(err(ErrorCode::Cancelled));
        }
        if matches!(
            payload,
            EventPayload::Final { .. }
                | EventPayload::State { .. }
                | EventPayload::ApprovalRequest { .. }
                | EventPayload::ApprovalResolved { .. }
        ) {
            return Err(err(ErrorCode::InvalidRequest));
        }
        self.cell.emit(Some(self.turn.clone()), payload)
    }
    pub async fn cancelled(&mut self) {
        if !*self.cancel.borrow() {
            let _ = self.cancel.changed().await;
        }
    }
    pub fn is_cancelled(&self) -> bool {
        *self.cancel.borrow() || !self.cell.active.load(Ordering::Acquire)
    }
    pub async fn approve(&self, action: String, detail: String) -> Result<bool> {
        (self.cell.guard)()?;
        if self.is_cancelled() {
            return Err(err(ErrorCode::Cancelled));
        }
        if action.len() > 256 || detail.len() > 8192 {
            return Err(err(ErrorCode::FrameTooLarge));
        }
        let approval = id("approval");
        let (tx, rx) = oneshot::channel();
        {
            let mut state = self
                .cell
                .state
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
            state.view.state = SessionState::AwaitingApproval;
            let mut approvals = self
                .cell
                .approvals
                .lock()
                .map_err(|_| err(ErrorCode::ProviderUnavailable))?;
            if approvals.len() >= 16 {
                return Err(err(ErrorCode::RateLimited));
            }
            approvals.insert(
                approval.clone(),
                Pending {
                    session: state.view.id.clone(),
                    sender: tx,
                },
            );
        }
        self.cell.emit(
            Some(self.turn.clone()),
            EventPayload::ApprovalRequest {
                approval_id: approval.clone(),
                action,
                detail,
            },
        )?;
        let mut cancel = self.cancel.clone();
        let allowed = tokio::select! {
         answer=rx=>answer.unwrap_or(false),
         _=async{if !*cancel.borrow(){let _=cancel.changed().await;}}=>false,
         _=tokio::time::sleep(Duration::from_secs(120))=>false
        };
        self.cell
            .approvals
            .lock()
            .map_err(|_| err(ErrorCode::ProviderUnavailable))?
            .remove(&approval);
        if let Ok(mut state) = self.cell.state.lock() {
            if state.view.state == SessionState::AwaitingApproval {
                state.view.state = SessionState::Running;
            }
        }
        let allowed = allowed && !self.is_cancelled() && (self.cell.guard)().is_ok();
        self.cell.emit(
            Some(self.turn.clone()),
            EventPayload::ApprovalResolved {
                approval_id: approval,
                allowed,
            },
        )?;
        Ok(allowed)
    }
}
