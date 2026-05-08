use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("toml decode error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("toml encode error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("config name is invalid: {0}")]
    InvalidName(String),

    #[error("config not found: {0}")]
    NotFound(String),

    #[error("config already exists: {0}")]
    AlreadyExists(String),

    #[error("rathole binary not found at {0}")]
    BinaryMissing(String),

    #[error("instance is already running")]
    AlreadyRunning,

    #[error("instance is not running")]
    NotRunning,

    #[error("port {addr} is already in use")]
    PortInUse { addr: String },

    #[error("failed to spawn rathole: {0}")]
    Spawn(String),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.to_string().as_ref())
    }
}

pub type AppResult<T> = Result<T, AppError>;
