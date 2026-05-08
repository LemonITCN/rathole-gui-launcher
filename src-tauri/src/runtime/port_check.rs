use std::net::ToSocketAddrs;

use serde::Serialize;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::net::TcpListener;

#[derive(Debug, Clone, Serialize)]
pub struct PortStatus {
    pub addr: String,
    pub available: bool,
    pub message: Option<String>,
}

pub async fn check_port(addr: &str) -> PortStatus {
    let resolved = match addr.to_socket_addrs() {
        Ok(mut iter) => iter.next(),
        Err(e) => {
            return PortStatus {
                addr: addr.to_string(),
                available: false,
                message: Some(format!("invalid address: {e}")),
            };
        }
    };
    let Some(socket_addr) = resolved else {
        return PortStatus {
            addr: addr.to_string(),
            available: false,
            message: Some("address resolved to nothing".into()),
        };
    };
    match TcpListener::bind(socket_addr).await {
        Ok(_) => PortStatus {
            addr: addr.to_string(),
            available: true,
            message: None,
        },
        Err(e) => PortStatus {
            addr: addr.to_string(),
            available: false,
            message: Some(e.to_string()),
        },
    }
}

pub async fn scan_ports(addrs: Vec<String>) -> Vec<PortStatus> {
    let mut out = Vec::with_capacity(addrs.len());
    for a in addrs {
        out.push(check_port(&a).await);
    }
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalRathole {
    pub pid: u32,
    pub name: String,
    pub cmd: Vec<String>,
    pub managed: bool,
}

pub fn find_external_rathole(managed_pids: &[u32]) -> Vec<ExternalRathole> {
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cmd(sysinfo::UpdateKind::OnlyIfNotSet),
    );
    let mut out = Vec::new();
    for (pid, proc) in sys.processes() {
        let name_os = proc.name();
        let name = name_os.to_string_lossy().to_lowercase();
        if !name.contains("rathole") {
            continue;
        }
        out.push(ExternalRathole {
            pid: pid.as_u32(),
            name: name_os.to_string_lossy().to_string(),
            cmd: proc
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect(),
            managed: managed_pids.contains(&pid.as_u32()),
        });
    }
    out
}
