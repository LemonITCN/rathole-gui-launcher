//! Persists the PIDs of rathole instances spawned (or adopted) by the
//! launcher so that, if the launcher itself dies unexpectedly, the next
//! launch can detect those still-alive children and re-attach to them.

use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::paths::app_data_dir;

const STATE_FILE: &str = ".runtime-state.json";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RuntimeState {
    #[serde(default)]
    pub instances: Vec<RuntimeInstance>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeInstance {
    pub mode: String,
    pub name: String,
    pub pid: u32,
    pub started_at: DateTime<Utc>,
}

fn state_path() -> PathBuf {
    app_data_dir().join(STATE_FILE)
}

pub fn load() -> RuntimeState {
    let path = state_path();
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
        Err(_) => RuntimeState::default(),
    }
}

pub fn save(state: &RuntimeState) {
    let path = state_path();
    let Ok(serialized) = serde_json::to_string_pretty(state) else {
        return;
    };
    let tmp = path.with_extension("json.tmp");
    if fs::write(&tmp, serialized).is_ok() {
        let _ = fs::rename(&tmp, &path);
    }
}

pub fn add(mode: &str, name: &str, pid: u32, started_at: DateTime<Utc>) {
    let mut state = load();
    state.instances.retain(|i| !(i.mode == mode && i.name == name));
    state.instances.push(RuntimeInstance {
        mode: mode.to_string(),
        name: name.to_string(),
        pid,
        started_at,
    });
    save(&state);
}

pub fn remove(mode: &str, name: &str) {
    let mut state = load();
    let before = state.instances.len();
    state.instances.retain(|i| !(i.mode == mode && i.name == name));
    if state.instances.len() != before {
        save(&state);
    }
}
