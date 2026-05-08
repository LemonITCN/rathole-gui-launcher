use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TcpTransportSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodelay: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keepalive_secs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keepalive_interval: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerTlsSettings {
    #[serde(default)]
    pub pkcs12: String,
    #[serde(default)]
    pub pkcs12_password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientTlsSettings {
    #[serde(default)]
    pub trusted_root: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NoiseSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WebsocketSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
}
