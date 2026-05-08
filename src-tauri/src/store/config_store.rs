use std::fs;

use serde::Serialize;

use crate::error::{AppError, AppResult};
use crate::models::{ClientConfig, ServerConfig};
use crate::paths::{config_file, conf_dir_for, is_valid_name, Mode};

#[derive(Debug, Serialize, Clone)]
pub struct ConfigSummary {
    pub name: String,
    pub modified: Option<String>,
    pub size: u64,
}

pub fn list_configs(mode: Mode) -> AppResult<Vec<ConfigSummary>> {
    let dir = conf_dir_for(mode);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }
        let metadata = entry.metadata()?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339().into());
        out.push(ConfigSummary {
            name: stem.to_string(),
            modified,
            size: metadata.len(),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn load_server(name: &str) -> AppResult<ServerConfig> {
    let path = config_file(Mode::Server, name)?;
    if !path.exists() {
        return Err(AppError::NotFound(name.to_string()));
    }
    let raw = fs::read_to_string(&path)?;
    let cfg: ServerConfig = toml::from_str(&raw)?;
    Ok(cfg)
}

pub fn load_client(name: &str) -> AppResult<ClientConfig> {
    let path = config_file(Mode::Client, name)?;
    if !path.exists() {
        return Err(AppError::NotFound(name.to_string()));
    }
    let raw = fs::read_to_string(&path)?;
    let cfg: ClientConfig = toml::from_str(&raw)?;
    Ok(cfg)
}

pub fn save_server(name: &str, config: &ServerConfig) -> AppResult<()> {
    let path = config_file(Mode::Server, name)?;
    fs::create_dir_all(conf_dir_for(Mode::Server))?;
    let serialized = toml::to_string_pretty(config)?;
    write_atomic(&path, serialized.as_bytes())?;
    Ok(())
}

pub fn save_client(name: &str, config: &ClientConfig) -> AppResult<()> {
    let path = config_file(Mode::Client, name)?;
    fs::create_dir_all(conf_dir_for(Mode::Client))?;
    let serialized = toml::to_string_pretty(config)?;
    write_atomic(&path, serialized.as_bytes())?;
    Ok(())
}

pub fn delete_config(mode: Mode, name: &str) -> AppResult<()> {
    let path = config_file(mode, name)?;
    if !path.exists() {
        return Err(AppError::NotFound(name.to_string()));
    }
    fs::remove_file(path)?;
    Ok(())
}

pub fn rename_config(mode: Mode, old: &str, new: &str) -> AppResult<()> {
    if !is_valid_name(new) {
        return Err(AppError::InvalidName(new.to_string()));
    }
    let from = config_file(mode, old)?;
    let to = config_file(mode, new)?;
    if !from.exists() {
        return Err(AppError::NotFound(old.to_string()));
    }
    if to.exists() {
        return Err(AppError::AlreadyExists(new.to_string()));
    }
    fs::rename(from, to)?;
    Ok(())
}

fn write_atomic(target: &std::path::Path, data: &[u8]) -> AppResult<()> {
    let tmp = target.with_extension("toml.tmp");
    fs::write(&tmp, data)?;
    fs::rename(&tmp, target)?;
    Ok(())
}
