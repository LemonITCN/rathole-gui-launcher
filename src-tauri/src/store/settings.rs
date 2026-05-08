use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;
use crate::paths::{app_data_dir, settings_path, RATHOLE_BIN};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Settings {
    /// Override path for the rathole binary. When `None` the launcher
    /// looks for `rathole` (or `rathole.exe`) next to its own executable.
    #[serde(default)]
    pub rathole_path: Option<String>,

    /// When true, configs that were running on the previous launch are
    /// automatically restarted at startup.
    #[serde(default)]
    pub auto_resume: bool,

    /// UI locale tag. One of `zh-CN`, `en`, `ja`, `ko`. `None` means
    /// the frontend should use its own auto-detection rules.
    #[serde(default)]
    pub language: Option<String>,
}

impl Settings {
    pub fn load() -> Self {
        let path = settings_path();
        if !path.exists() {
            return Self::default();
        }
        match fs::read_to_string(&path) {
            Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> AppResult<()> {
        fs::create_dir_all(app_data_dir())?;
        let raw = serde_json::to_string_pretty(self)?;
        let path = settings_path();
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, raw)?;
        fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn resolved_rathole_path(&self) -> PathBuf {
        if let Some(custom) = self.rathole_path.as_ref() {
            let p = PathBuf::from(custom);
            if !p.as_os_str().is_empty() {
                return p;
            }
        }
        app_data_dir().join(RATHOLE_BIN)
    }
}

pub type SettingsHandle = Arc<RwLock<Settings>>;
