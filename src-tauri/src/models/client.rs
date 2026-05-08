use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::common::{ClientTlsSettings, NoiseSettings, TcpTransportSettings, WebsocketSettings};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientConfig {
    #[serde(default)]
    pub client: ClientSection,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientSection {
    #[serde(default)]
    pub remote_addr: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_token: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heartbeat_timeout: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<ClientTransport>,

    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub services: IndexMap<String, ClientService>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientTransport {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<TcpTransportSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ClientTlsSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise: Option<NoiseSettings>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub websocket: Option<WebsocketSettings>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientService {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    #[serde(default)]
    pub local_addr: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodelay: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<u32>,
}
