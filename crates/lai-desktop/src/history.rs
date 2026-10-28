//! Bounded, local usage history.
//!
//! The broker exposes quota windows as point-in-time observations. Nothing in
//! the protocol fabricates a time series, so the desktop client keeps the
//! observations it actually saw, and labels them as locally collected.

use chrono::{DateTime, Duration, Utc};
use lai_contracts::{Observation, Snapshot, UsageReading};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub const MAX_SAMPLES: usize = 6000;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sample {
    pub t: i64,
    pub pool: String,
    pub pool_label: String,
    pub window: String,
    pub window_label: String,
    /// Remaining share of the window, 0..=1. Absent when the provider only
    /// reports a raw reading without a limit.
    pub remaining: Option<f64>,
    pub resets_at: Option<String>,
    pub unit: Option<String>,
    pub used: Option<String>,
    pub limit: Option<String>,
}

pub fn state_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("LAI_DESKTOP_STATE_DIR") {
        return PathBuf::from(dir);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/state/local-ai-kit/desktop")
}

pub struct History {
    pub samples: Vec<Sample>,
    path: PathBuf,
    last_written: Option<i64>,
}

impl History {
    pub fn load() -> Self {
        let path = state_dir().join("usage-history.jsonl");
        let mut samples = Vec::new();
        let cutoff = Utc::now() - Duration::days(30);
        if let Ok(file) = fs::File::open(&path) {
            for line in BufReader::new(file).lines().map_while(Result::ok) {
                if let Ok(sample) = serde_json::from_str::<Sample>(&line) {
                    let recent =
                        DateTime::from_timestamp(sample.t, 0).is_some_and(|stamp| stamp > cutoff);
                    if recent {
                        samples.push(sample);
                    }
                }
            }
        }
        if samples.len() > MAX_SAMPLES {
            let excess = samples.len() - MAX_SAMPLES;
            samples.drain(0..excess);
        }
        Self {
            samples,
            path,
            last_written: None,
        }
    }

    /// Record one observation per quota window. A sample is only appended when
    /// the value changed, or when the previous sample for that window is more
    /// than five minutes old, so idle polling does not inflate the file.
    pub fn observe(&mut self, snapshot: &Snapshot, now: i64) {
        if snapshot.quota_pools.is_empty() {
            return;
        }
        let mut appended = Vec::new();
        for pool in &snapshot.quota_pools {
            let pool_label = pool_label(snapshot, &pool.id, &pool.connection_ids);
            for window in &pool.windows {
                let Observation::Known {
                    value, freshness, ..
                } = &window.reading
                else {
                    continue;
                };
                let UsageReading {
                    unit,
                    used,
                    limit,
                    remaining: _,
                    used_fraction,
                } = value;
                let remaining_share =
                    used_fraction.map(|fraction| (1.0 - fraction).clamp(0.0, 1.0));
                let changed = self.samples.last().is_none_or(|last| {
                    last.pool != pool.id
                        || last.window != window.id
                        || last.remaining != remaining_share
                        || last.used.as_deref() != Some(used.as_str())
                });
                let fresh_enough = self
                    .samples
                    .iter()
                    .rev()
                    .find(|s| s.pool == pool.id && s.window == window.id)
                    .is_none_or(|s| now - s.t >= 300);
                if !changed && !fresh_enough {
                    continue;
                }
                let _ = freshness;
                let sample = Sample {
                    t: now,
                    pool: pool.id.clone(),
                    pool_label: pool_label.clone(),
                    window: window.id.clone(),
                    window_label: window.label.clone(),
                    remaining: remaining_share,
                    resets_at: window.resets_at.as_ref().map(|t| t.as_str().to_string()),
                    unit: Some(unit.clone()),
                    used: Some(used.as_str().to_string()),
                    limit: limit.as_ref().map(|l| l.as_str().to_string()),
                };
                self.samples.push(sample.clone());
                appended.push(sample);
            }
        }
        if self.samples.len() > MAX_SAMPLES {
            let excess = self.samples.len() - MAX_SAMPLES;
            self.samples.drain(0..excess);
        }
        if !appended.is_empty() {
            self.persist(&appended);
        }
    }

    fn persist(&mut self, appended: &[Sample]) {
        if self.last_written.is_none() {
            if let Some(parent) = self.path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            // One durable header rewrite is not needed: the file is JSON-lines.
        }
        let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        else {
            return;
        };
        for sample in appended {
            if let Ok(line) = serde_json::to_string(sample) {
                let _ = writeln!(file, "{line}");
            }
        }
        self.last_written = Some(Utc::now().timestamp());
    }
}

fn pool_label(snapshot: &Snapshot, pool_id: &str, connection_ids: &[String]) -> String {
    let provider = connection_ids
        .iter()
        .find_map(|id| snapshot.connections.iter().find(|c| &c.id == id))
        .and_then(|c| {
            snapshot
                .providers
                .iter()
                .find(|p| p.id == c.provider_id)
                .map(|p| p.name.clone())
                .or_else(|| Some(c.provider_id.clone()))
        });
    match provider {
        Some(name) => name,
        None => pool_id
            .strip_prefix("pool_")
            .map(|rest| format!("Pool {}", &rest[..rest.len().min(6)]))
            .unwrap_or_else(|| pool_id.to_string()),
    }
}
