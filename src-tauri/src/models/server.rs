use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::{NoiseSettings, ServerTlsSettings, TcpTransportSettings, WebsocketSettings};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerConfig {
    #[serde(default)]
    pub server: ServerSection,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerSection {
    #[serde(default)]
    pub bind_addr: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_token: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heartbeat_interval: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<ServerTransport>,

    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub services: IndexMap<String, ServerService>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerTransport {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpTransportSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ServerTlsSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise: Option<NoiseSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket: Option<WebsocketSettings>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ServerService {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(default)]
    pub bind_addr: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodelay: Option<bool>,
}
