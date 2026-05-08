use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::error::{AppError, AppResult};
use crate::paths::Mode;

const LOG_BUFFER_CAP: usize = 1000;
const STOP_GRACE_SECS: u64 = 5;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)] // Starting is reserved for future use; the frontend already handles it.
pub enum InstanceState {
    Starting,
    Running,
    Stopping,
    Exited,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    pub mode: String,
    pub name: String,
    pub stream: String,
    pub line: String,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunStatus {
    pub mode: String,
    pub name: String,
    pub pid: Option<u32>,
    pub state: InstanceState,
    pub started_at: Option<String>,
    pub last_exit_code: Option<i32>,
}

struct Instance {
    mode: Mode,
    name: String,
    pid: u32,
    started_at: chrono::DateTime<chrono::Utc>,
    state: InstanceState,
    last_exit_code: Option<i32>,
    stop_tx: mpsc::Sender<()>,
    log_buffer: Arc<Mutex<VecDeque<LogLine>>>,
}

impl Instance {
    fn snapshot(&self) -> RunStatus {
        RunStatus {
            mode: self.mode.as_str().to_string(),
            name: self.name.clone(),
            pid: Some(self.pid),
            state: self.state,
            started_at: Some(self.started_at.to_rfc3339()),
            last_exit_code: self.last_exit_code,
        }
    }
}

pub struct ProcessManager {
    app: AppHandle,
    instances: Mutex<HashMap<String, Instance>>,
}

impl ProcessManager {
    pub fn new(app: AppHandle) -> Self {
        Self {
            app,
            instances: Mutex::new(HashMap::new()),
        }
    }

    fn key(mode: Mode, name: &str) -> String {
        format!("{}:{}", mode.as_str(), name)
    }

    pub fn list(&self) -> Vec<RunStatus> {
        self.instances.lock().values().map(Instance::snapshot).collect()
    }

    pub fn status(&self, mode: Mode, name: &str) -> Option<RunStatus> {
        self.instances
            .lock()
            .get(&Self::key(mode, name))
            .map(Instance::snapshot)
    }

    pub fn managed_pids(&self) -> Vec<u32> {
        self.instances
            .lock()
            .values()
            .filter(|i| !matches!(i.state, InstanceState::Exited))
            .map(|i| i.pid)
            .collect()
    }

    pub fn recent_logs(&self, mode: Mode, name: &str, limit: usize) -> Vec<LogLine> {
        let lock = self.instances.lock();
        let Some(inst) = lock.get(&Self::key(mode, name)) else {
            return Vec::new();
        };
        let buf = inst.log_buffer.lock();
        let len = buf.len();
        let take = limit.min(len);
        buf.iter().skip(len - take).cloned().collect()
    }

    pub async fn start(
        self: Arc<Self>,
        mode: Mode,
        name: String,
        config_path: PathBuf,
        rathole_path: PathBuf,
    ) -> AppResult<RunStatus> {
        let key = Self::key(mode, &name);
        {
            let lock = self.instances.lock();
            if let Some(inst) = lock.get(&key) {
                if matches!(inst.state, InstanceState::Running | InstanceState::Starting) {
                    return Err(AppError::AlreadyRunning);
                }
            }
        }

        if !rathole_path.exists() {
            return Err(AppError::BinaryMissing(rathole_path.display().to_string()));
        }
        if !config_path.exists() {
            return Err(AppError::NotFound(name));
        }

        let mut cmd = Command::new(&rathole_path);
        cmd.arg(mode.cli_flag()).arg(&config_path);
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::null())
            .env("RUST_LOG", "info")
            .kill_on_drop(true);
        if let Some(parent) = config_path.parent() {
            cmd.current_dir(parent);
        }

        #[cfg(windows)]
        {
            // `creation_flags` is provided directly on `tokio::process::Command`
            // when targeting Windows — no need to import the std CommandExt
            // trait, which would just trigger an unused-import warning.
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| AppError::Spawn(friendly_spawn_error(&e)))?;
        let pid = child
            .id()
            .ok_or_else(|| AppError::Spawn("child has no pid".into()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| AppError::Spawn("missing stdout".into()))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| AppError::Spawn("missing stderr".into()))?;

        let log_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(LOG_BUFFER_CAP)));
        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
        let started_at = chrono::Utc::now();

        {
            let mut lock = self.instances.lock();
            lock.insert(
                key.clone(),
                Instance {
                    mode,
                    name: name.clone(),
                    pid,
                    started_at,
                    state: InstanceState::Running,
                    last_exit_code: None,
                    stop_tx,
                    log_buffer: log_buffer.clone(),
                },
            );
        }

        spawn_reader(self.app.clone(), mode, name.clone(), "stdout", stdout, log_buffer.clone());
        spawn_reader(self.app.clone(), mode, name.clone(), "stderr", stderr, log_buffer.clone());

        let manager = self.clone();
        let supervisor_name = name.clone();
        let supervisor_key = key.clone();
        let supervisor_buf = log_buffer.clone();
        tokio::spawn(async move {
            let exit_code: Option<i32>;
            tokio::select! {
                biased;
                _ = stop_rx.recv() => {
                    manager.set_state(&supervisor_key, InstanceState::Stopping);
                    emit_status(&manager.app, mode, &supervisor_name, InstanceState::Stopping, None);
                    push_log(&manager.app, mode.as_str(), &supervisor_name, "system", "stop signal received".into(), &supervisor_buf);
                    graceful_kill(&mut child, pid).await;
                    exit_code = child.wait().await.ok().and_then(|s| s.code());
                }
                res = child.wait() => {
                    exit_code = res.ok().and_then(|s| s.code());
                }
            }

            {
                let mut lock = manager.instances.lock();
                if let Some(inst) = lock.get_mut(&supervisor_key) {
                    inst.state = InstanceState::Exited;
                    inst.last_exit_code = exit_code;
                }
            }
            push_log(
                &manager.app,
                mode.as_str(),
                &supervisor_name,
                "system",
                format!("rathole exited (code: {:?})", exit_code),
                &supervisor_buf,
            );
            emit_status(&manager.app, mode, &supervisor_name, InstanceState::Exited, exit_code);
            crate::tray::rebuild_menu(&manager.app);
            crate::store::runtime_state::remove(mode.as_str(), &supervisor_name);
        });

        crate::store::runtime_state::add(mode.as_str(), &name, pid, started_at);

        push_log(
            &self.app,
            mode.as_str(),
            &name,
            "system",
            format!("started rathole pid={pid}"),
            &log_buffer,
        );
        emit_status(&self.app, mode, &name, InstanceState::Running, None);
        crate::tray::rebuild_menu(&self.app);

        Ok(self.status(mode, &name).unwrap_or(RunStatus {
            mode: mode.as_str().to_string(),
            name,
            pid: Some(pid),
            state: InstanceState::Running,
            started_at: Some(started_at.to_rfc3339()),
            last_exit_code: None,
        }))
    }

    /// Re-attach to a rathole process that was started by a previous launcher
    /// run and is still alive. We don't have its stdout/stderr pipes (those
    /// died with the previous launcher), so the log stream stays empty. We
    /// can still report status, send a stop signal, and detect when the
    /// process eventually exits via PID polling.
    pub async fn adopt(
        self: Arc<Self>,
        mode: Mode,
        name: String,
        pid: u32,
        started_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<RunStatus> {
        let key = Self::key(mode, &name);
        {
            let lock = self.instances.lock();
            if lock.contains_key(&key) {
                return Err(AppError::AlreadyRunning);
            }
        }

        if !is_rathole_process(pid) {
            return Err(AppError::NotRunning);
        }

        let log_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(LOG_BUFFER_CAP)));
        let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);

        {
            let mut lock = self.instances.lock();
            lock.insert(
                key.clone(),
                Instance {
                    mode,
                    name: name.clone(),
                    pid,
                    started_at,
                    state: InstanceState::Running,
                    last_exit_code: None,
                    stop_tx,
                    log_buffer: log_buffer.clone(),
                },
            );
        }

        push_log(
            &self.app,
            mode.as_str(),
            &name,
            "system",
            format!(
                "adopted rathole pid={pid} from a previous launcher session"
            ),
            &log_buffer,
        );
        push_log(
            &self.app,
            mode.as_str(),
            &name,
            "system",
            "live log stream is unavailable for adopted instances; stop and start again to reattach"
                .to_string(),
            &log_buffer,
        );

        let manager = self.clone();
        let supervisor_mode = mode;
        let supervisor_name = name.clone();
        let supervisor_key = key.clone();
        let supervisor_buf = log_buffer.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    _ = stop_rx.recv() => {
                        manager.set_state(&supervisor_key, InstanceState::Stopping);
                        emit_status(
                            &manager.app,
                            supervisor_mode,
                            &supervisor_name,
                            InstanceState::Stopping,
                            None,
                        );
                        push_log(
                            &manager.app,
                            supervisor_mode.as_str(),
                            &supervisor_name,
                            "system",
                            "stop signal received".to_string(),
                            &supervisor_buf,
                        );
                        kill_pid_graceful(pid).await;
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(2)) => {
                        if !is_pid_alive(pid) {
                            break;
                        }
                    }
                }
            }

            {
                let mut lock = manager.instances.lock();
                if let Some(inst) = lock.get_mut(&supervisor_key) {
                    inst.state = InstanceState::Exited;
                }
            }
            push_log(
                &manager.app,
                supervisor_mode.as_str(),
                &supervisor_name,
                "system",
                "rathole process is no longer running".to_string(),
                &supervisor_buf,
            );
            emit_status(
                &manager.app,
                supervisor_mode,
                &supervisor_name,
                InstanceState::Exited,
                None,
            );
            crate::tray::rebuild_menu(&manager.app);
            crate::store::runtime_state::remove(supervisor_mode.as_str(), &supervisor_name);
        });

        emit_status(&self.app, mode, &name, InstanceState::Running, None);
        crate::tray::rebuild_menu(&self.app);

        Ok(self.status(mode, &name).unwrap_or(RunStatus {
            mode: mode.as_str().to_string(),
            name,
            pid: Some(pid),
            state: InstanceState::Running,
            started_at: Some(started_at.to_rfc3339()),
            last_exit_code: None,
        }))
    }

    pub async fn stop(&self, mode: Mode, name: &str) -> AppResult<()> {
        let key = Self::key(mode, name);
        let tx = {
            let lock = self.instances.lock();
            let Some(inst) = lock.get(&key) else {
                return Err(AppError::NotRunning);
            };
            if matches!(inst.state, InstanceState::Exited) {
                return Err(AppError::NotRunning);
            }
            inst.stop_tx.clone()
        };
        let _ = tx.send(()).await;
        Ok(())
    }

    pub async fn stop_all(&self) {
        let txs: Vec<mpsc::Sender<()>> = {
            let lock = self.instances.lock();
            lock.values()
                .filter(|i| !matches!(i.state, InstanceState::Exited))
                .map(|i| i.stop_tx.clone())
                .collect()
        };
        for tx in txs {
            let _ = tx.send(()).await;
        }
    }

    fn set_state(&self, key: &str, state: InstanceState) {
        let mut lock = self.instances.lock();
        if let Some(inst) = lock.get_mut(key) {
            inst.state = state;
        }
    }
}

fn spawn_reader<R>(
    app: AppHandle,
    mode: Mode,
    name: String,
    stream: &'static str,
    reader: R,
    buf: Arc<Mutex<VecDeque<LogLine>>>,
) where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        loop {
            match lines.next_line().await {
                Ok(Some(line)) => push_log(&app, mode.as_str(), &name, stream, line, &buf),
                Ok(None) => break,
                Err(e) => {
                    push_log(
                        &app,
                        mode.as_str(),
                        &name,
                        "system",
                        format!("log reader error on {stream}: {e}"),
                        &buf,
                    );
                    break;
                }
            }
        }
    });
}

#[cfg(unix)]
async fn graceful_kill(child: &mut tokio::process::Child, pid: u32) {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    let _ = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
    if tokio::time::timeout(Duration::from_secs(STOP_GRACE_SECS), child.wait())
        .await
        .is_err()
    {
        let _ = child.kill().await;
    }
}

#[cfg(windows)]
async fn graceful_kill(child: &mut tokio::process::Child, _pid: u32) {
    let _ = child.kill().await;
    let _ = tokio::time::timeout(Duration::from_secs(STOP_GRACE_SECS), child.wait()).await;
}

/// Translate platform-specific spawn errors into actionable messages. The
/// most common case we want to catch is Windows Smart App Control /
/// Defender blocking an unsigned third-party binary — the raw error
/// ("os error 4556") gives a confused user nothing to act on.
fn friendly_spawn_error(e: &std::io::Error) -> String {
    let raw = e.to_string();
    #[cfg(windows)]
    {
        let code = e.raw_os_error().unwrap_or(0);
        // 4556 = Windows App Control / Smart App Control reputation block.
        // 225  = ERROR_VIRUS_INFECTED, used by Defender real-time scan.
        if matches!(code, 4556 | 225) || raw.contains("应用程序控制") {
            return format!(
                "Windows blocked rathole.exe via Smart App Control / Defender / AppLocker. \
                 The binary is unsigned third-party software, so the OS treats it as \
                 untrusted. Workaround: open Windows Security → Virus & threat protection \
                 → Manage settings → Add or remove exclusions, and add the rathole.exe \
                 path; or turn Smart App Control off under App & browser control. \
                 Underlying error: {raw}"
            );
        }
    }
    raw
}

/// Send SIGTERM (or platform equivalent) by PID, wait up to STOP_GRACE_SECS
/// for the process to exit, then SIGKILL if still alive. Used for adopted
/// processes where we no longer hold a `tokio::process::Child` handle.
async fn kill_pid_graceful(pid: u32) {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let _ = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
    }
    #[cfg(windows)]
    {
        kill_pid_force(pid);
    }

    let started = std::time::Instant::now();
    while started.elapsed() < Duration::from_secs(STOP_GRACE_SECS) {
        if !is_pid_alive(pid) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let _ = kill(Pid::from_raw(pid as i32), Signal::SIGKILL);
    }
    #[cfg(windows)]
    {
        kill_pid_force(pid);
    }
}

#[cfg(windows)]
fn kill_pid_force(pid: u32) {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
        true,
        ProcessRefreshKind::new(),
    );
    if let Some(proc) = sys.process(Pid::from_u32(pid)) {
        let _ = proc.kill();
    }
}

pub fn is_pid_alive(pid: u32) -> bool {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
        true,
        ProcessRefreshKind::new(),
    );
    sys.process(Pid::from_u32(pid)).is_some()
}

/// Returns true if the given PID is alive AND its process name contains
/// "rathole". This guards against adopting an unrelated process when the
/// kernel happens to reuse the PID after a previous rathole exit.
pub fn is_rathole_process(pid: u32) -> bool {
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
        true,
        ProcessRefreshKind::new().with_cmd(UpdateKind::OnlyIfNotSet),
    );
    if let Some(proc) = sys.process(Pid::from_u32(pid)) {
        proc.name().to_string_lossy().to_lowercase().contains("rathole")
    } else {
        false
    }
}

fn push_log(
    app: &AppHandle,
    mode: &str,
    name: &str,
    stream: &str,
    line: String,
    buf: &Arc<Mutex<VecDeque<LogLine>>>,
) {
    let entry = LogLine {
        mode: mode.to_string(),
        name: name.to_string(),
        stream: stream.to_string(),
        line,
        ts: chrono::Utc::now().to_rfc3339(),
    };
    {
        let mut g = buf.lock();
        if g.len() >= LOG_BUFFER_CAP {
            g.pop_front();
        }
        g.push_back(entry.clone());
    }
    let _ = app.emit("rathole-log", entry);
}

#[derive(Serialize, Clone)]
struct StatusEvent<'a> {
    mode: &'a str,
    name: &'a str,
    state: InstanceState,
    exit_code: Option<i32>,
}

fn emit_status(app: &AppHandle, mode: Mode, name: &str, state: InstanceState, exit: Option<i32>) {
    let _ = app.emit(
        "rathole-status",
        StatusEvent {
            mode: mode.as_str(),
            name,
            state,
            exit_code: exit,
        },
    );
}
