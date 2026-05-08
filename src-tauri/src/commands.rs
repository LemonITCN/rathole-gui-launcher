use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::process::Command;

use crate::error::{AppError, AppResult};
use crate::models::{ClientConfig, ServerConfig};
use crate::paths::{self, app_data_dir, client_conf_dir, config_file, server_conf_dir, Mode};
use crate::runtime::{
    self, ExternalRathole, LogLine, PortStatus, ProcessManager, RunStatus,
};
use crate::store::{self, Settings, SettingsHandle};

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub platform: String,
    pub app_data_dir: String,
    pub server_conf_dir: String,
    pub client_conf_dir: String,
    pub rathole_path: String,
    pub rathole_exists: bool,
    pub rathole_version: Option<String>,
    pub auto_resume: bool,
    pub language: Option<String>,
}

#[tauri::command]
pub async fn get_app_info(settings: State<'_, SettingsHandle>) -> AppResult<AppInfo> {
    let snapshot = settings.read().clone();
    let rathole_path = snapshot.resolved_rathole_path();
    let exists = rathole_path.exists();
    let version = if exists {
        detect_version(&rathole_path).await
    } else {
        None
    };
    Ok(AppInfo {
        platform: std::env::consts::OS.to_string(),
        app_data_dir: app_data_dir().display().to_string(),
        server_conf_dir: server_conf_dir().display().to_string(),
        client_conf_dir: client_conf_dir().display().to_string(),
        rathole_path: rathole_path.display().to_string(),
        rathole_exists: exists,
        rathole_version: version,
        auto_resume: snapshot.auto_resume,
        language: snapshot.language,
    })
}

async fn detect_version(path: &std::path::Path) -> Option<String> {
    let output = Command::new(path)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            None
        } else {
            Some(stderr)
        }
    } else {
        Some(stdout)
    }
}

#[derive(Deserialize)]
pub struct UpdateSettings {
    pub rathole_path: Option<String>,
    pub auto_resume: Option<bool>,
    pub language: Option<String>,
}

#[tauri::command]
pub async fn update_settings(
    settings: State<'_, SettingsHandle>,
    payload: UpdateSettings,
) -> AppResult<Settings> {
    let mut s = settings.write();
    if let Some(p) = payload.rathole_path {
        s.rathole_path = if p.is_empty() { None } else { Some(p) };
    }
    if let Some(a) = payload.auto_resume {
        s.auto_resume = a;
    }
    if let Some(lang) = payload.language {
        s.language = if lang.is_empty() { None } else { Some(lang) };
    }
    s.save()?;
    Ok(s.clone())
}

#[tauri::command]
pub async fn list_configs(mode: String) -> AppResult<Vec<store::ConfigSummary>> {
    let m = Mode::from_str(&mode)?;
    store::list_configs(m)
}

#[tauri::command]
pub async fn get_server_config(name: String) -> AppResult<ServerConfig> {
    store::load_server(&name)
}

#[tauri::command]
pub async fn get_client_config(name: String) -> AppResult<ClientConfig> {
    store::load_client(&name)
}

#[tauri::command]
pub async fn save_server_config(name: String, config: ServerConfig) -> AppResult<()> {
    store::save_server(&name, &config)
}

#[tauri::command]
pub async fn save_client_config(name: String, config: ClientConfig) -> AppResult<()> {
    store::save_client(&name, &config)
}

#[tauri::command]
pub async fn delete_config(mode: String, name: String) -> AppResult<()> {
    let m = Mode::from_str(&mode)?;
    store::delete_config(m, &name)
}

#[tauri::command]
pub async fn rename_config(mode: String, old_name: String, new_name: String) -> AppResult<()> {
    let m = Mode::from_str(&mode)?;
    store::rename_config(m, &old_name, &new_name)
}

#[tauri::command]
pub async fn start_config(
    manager: State<'_, Arc<ProcessManager>>,
    settings: State<'_, SettingsHandle>,
    mode: String,
    name: String,
) -> AppResult<RunStatus> {
    let m = Mode::from_str(&mode)?;
    let config_path: PathBuf = config_file(m, &name)?;

    if !config_path.exists() {
        return Err(AppError::NotFound(name));
    }

    let rathole_path = settings.read().resolved_rathole_path();
    if !rathole_path.exists() {
        return Err(AppError::BinaryMissing(rathole_path.display().to_string()));
    }

    let required_ports = collect_ports_for(m, &name)?;
    let conflicts: Vec<PortStatus> = runtime::scan_ports(required_ports)
        .await
        .into_iter()
        .filter(|p| !p.available)
        .collect();
    if !conflicts.is_empty() {
        let summary = conflicts
            .iter()
            .map(|p| p.addr.clone())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(AppError::PortInUse { addr: summary });
    }

    let mgr = manager.inner().clone();
    mgr.start(m, name, config_path, rathole_path).await
}

#[tauri::command]
pub async fn stop_config(
    manager: State<'_, Arc<ProcessManager>>,
    mode: String,
    name: String,
) -> AppResult<()> {
    let m = Mode::from_str(&mode)?;
    manager.stop(m, &name).await
}

#[tauri::command]
pub async fn list_running(manager: State<'_, Arc<ProcessManager>>) -> AppResult<Vec<RunStatus>> {
    Ok(manager.list())
}

#[tauri::command]
pub async fn get_run_status(
    manager: State<'_, Arc<ProcessManager>>,
    mode: String,
    name: String,
) -> AppResult<Option<RunStatus>> {
    let m = Mode::from_str(&mode)?;
    Ok(manager.status(m, &name))
}

#[tauri::command]
pub async fn get_recent_logs(
    manager: State<'_, Arc<ProcessManager>>,
    mode: String,
    name: String,
    limit: usize,
) -> AppResult<Vec<LogLine>> {
    let m = Mode::from_str(&mode)?;
    Ok(manager.recent_logs(m, &name, limit))
}

#[tauri::command]
pub async fn check_ports(addrs: Vec<String>) -> AppResult<Vec<PortStatus>> {
    Ok(runtime::scan_ports(addrs).await)
}

#[tauri::command]
pub async fn find_external_rathole(
    manager: State<'_, Arc<ProcessManager>>,
) -> AppResult<Vec<ExternalRathole>> {
    let pids = manager.managed_pids();
    Ok(runtime::find_external_rathole(&pids))
}

#[tauri::command]
pub async fn open_conf_dir(mode: String) -> AppResult<String> {
    let m = Mode::from_str(&mode)?;
    let dir = paths::conf_dir_for(m);
    std::fs::create_dir_all(&dir)?;

    #[cfg(target_os = "macos")]
    let opener = "open";
    #[cfg(target_os = "windows")]
    let opener = "explorer";
    #[cfg(target_os = "linux")]
    let opener = "xdg-open";

    let _ = std::process::Command::new(opener)
        .arg(&dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    Ok(dir.display().to_string())
}

#[tauri::command]
pub async fn duplicate_config(
    mode: String,
    source: String,
    target: String,
) -> AppResult<()> {
    let m = Mode::from_str(&mode)?;
    let from = config_file(m, &source)?;
    let to = config_file(m, &target)?;
    if !from.exists() {
        return Err(AppError::NotFound(source));
    }
    if to.exists() {
        return Err(AppError::AlreadyExists(target));
    }
    std::fs::copy(from, to)?;
    Ok(())
}

fn collect_ports_for(mode: Mode, name: &str) -> AppResult<Vec<String>> {
    match mode {
        Mode::Server => {
            let cfg = store::load_server(name)?;
            let mut ports = Vec::new();
            if !cfg.server.bind_addr.is_empty() {
                ports.push(cfg.server.bind_addr.clone());
            }
            for svc in cfg.server.services.values() {
                if !svc.bind_addr.is_empty() {
                    ports.push(svc.bind_addr.clone());
                }
            }
            Ok(ports)
        }
        Mode::Client => Ok(Vec::new()),
    }
}
