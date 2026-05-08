use std::path::{Path, PathBuf};

use crate::error::{AppError, AppResult};

pub const SERVER_DIR: &str = "server_conf";
pub const CLIENT_DIR: &str = "client_conf";
pub const SETTINGS_FILE: &str = "launcher.json";
pub const RATHOLE_BIN: &str = if cfg!(windows) { "rathole.exe" } else { "rathole" };

/// Returns the directory that the launcher treats as its working root.
///
/// On Windows and Linux this is the directory containing the executable.
/// On macOS, when the binary is running from inside an `.app` bundle, the
/// directory containing the bundle is returned instead so that user data
/// and the rathole binary can live next to the bundle rather than inside it.
pub fn app_data_dir() -> PathBuf {
    let exe = std::env::current_exe().expect("cannot resolve current executable path");
    let exe_parent = exe
        .parent()
        .expect("executable path has no parent")
        .to_path_buf();

    #[cfg(target_os = "macos")]
    {
        if let Some(dir) = macos_bundle_sibling(&exe_parent) {
            return dir;
        }
    }

    exe_parent
}

#[cfg(target_os = "macos")]
fn macos_bundle_sibling(exe_parent: &Path) -> Option<PathBuf> {
    if !exe_parent.ends_with("MacOS") {
        return None;
    }
    let contents = exe_parent.parent()?;
    let bundle = contents.parent()?;
    if bundle.extension().and_then(|s| s.to_str()) == Some("app") {
        bundle.parent().map(Path::to_path_buf)
    } else {
        None
    }
}

pub fn server_conf_dir() -> PathBuf {
    app_data_dir().join(SERVER_DIR)
}

pub fn client_conf_dir() -> PathBuf {
    app_data_dir().join(CLIENT_DIR)
}

pub fn settings_path() -> PathBuf {
    app_data_dir().join(SETTINGS_FILE)
}

pub fn ensure_dirs() -> AppResult<()> {
    std::fs::create_dir_all(server_conf_dir())?;
    std::fs::create_dir_all(client_conf_dir())?;
    Ok(())
}

pub fn conf_dir_for(mode: Mode) -> PathBuf {
    match mode {
        Mode::Server => server_conf_dir(),
        Mode::Client => client_conf_dir(),
    }
}

pub fn config_file(mode: Mode, name: &str) -> AppResult<PathBuf> {
    if !is_valid_name(name) {
        return Err(AppError::InvalidName(name.to_string()));
    }
    Ok(conf_dir_for(mode).join(format!("{name}.toml")))
}

pub fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
        && !name.starts_with('.')
        && name != "."
        && name != ".."
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Server,
    Client,
}

impl Mode {
    pub fn from_str(value: &str) -> AppResult<Self> {
        match value {
            "server" => Ok(Mode::Server),
            "client" => Ok(Mode::Client),
            other => Err(AppError::Other(format!("unknown mode: {other}"))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Server => "server",
            Mode::Client => "client",
        }
    }

    pub fn cli_flag(self) -> &'static str {
        match self {
            Mode::Server => "--server",
            Mode::Client => "--client",
        }
    }
}
