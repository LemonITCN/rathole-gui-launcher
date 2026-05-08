export type Mode = "server" | "client";
export type ServiceType = "tcp" | "udp";
export type TransportKind = "tcp" | "tls" | "noise" | "websocket";

export interface TcpTransportSettings {
  proxy?: string;
  nodelay?: boolean;
  keepalive_secs?: number;
  keepalive_interval?: number;
}

export interface ServerTlsSettings {
  pkcs12: string;
  pkcs12_password: string;
}

export interface ClientTlsSettings {
  trusted_root: string;
  hostname?: string;
}

export interface NoiseSettings {
  pattern?: string;
  local_private_key?: string;
  remote_public_key?: string;
}

export interface WebsocketSettings {
  tls?: boolean;
}

export interface ServerService {
  type?: ServiceType;
  token?: string;
  bind_addr: string;
  nodelay?: boolean;
}

export interface ServerTransport {
  type?: TransportKind;
  tcp?: TcpTransportSettings;
  tls?: ServerTlsSettings;
  noise?: NoiseSettings;
  websocket?: WebsocketSettings;
}

export interface ServerSection {
  bind_addr: string;
  default_token?: string;
  heartbeat_interval?: number;
  transport?: ServerTransport;
  services: Record<string, ServerService>;
}

export interface ServerConfig {
  server: ServerSection;
}

export interface ClientService {
  type?: ServiceType;
  token?: string;
  local_addr: string;
  nodelay?: boolean;
  retry_interval?: number;
}

export interface ClientTransport {
  type?: TransportKind;
  tcp?: TcpTransportSettings;
  tls?: ClientTlsSettings;
  noise?: NoiseSettings;
  websocket?: WebsocketSettings;
}

export interface ClientSection {
  remote_addr: string;
  default_token?: string;
  heartbeat_timeout?: number;
  retry_interval?: number;
  transport?: ClientTransport;
  services: Record<string, ClientService>;
}

export interface ClientConfig {
  client: ClientSection;
}

export interface ConfigSummary {
  name: string;
  modified?: string;
  size: number;
}

export type RunState = "starting" | "running" | "stopping" | "exited";

export interface RunStatus {
  mode: Mode;
  name: string;
  pid?: number;
  state: RunState;
  started_at?: string;
  last_exit_code?: number;
}

export interface RunStatusEvent {
  mode: Mode;
  name: string;
  state: RunState;
  exit_code?: number;
}

export interface LogLine {
  mode: Mode;
  name: string;
  stream: "stdout" | "stderr" | "system";
  line: string;
  ts: string;
}

export interface PortStatus {
  addr: string;
  available: boolean;
  message?: string;
}

export interface ExternalRathole {
  pid: number;
  name: string;
  cmd: string[];
  managed: boolean;
}

export interface AppInfo {
  platform: string;
  app_data_dir: string;
  server_conf_dir: string;
  client_conf_dir: string;
  rathole_path: string;
  rathole_exists: boolean;
  rathole_version?: string;
  auto_resume: boolean;
  language?: string;
}

export interface Settings {
  rathole_path?: string;
  auto_resume: boolean;
  language?: string;
}

export interface AssetInfo {
  name: string;
  url: string;
  size: number;
  target: string;
}

export interface UpdateCheckResult {
  installed_version?: string;
  latest_version?: string;
  release_name?: string;
  published_at?: string;
  asset?: AssetInfo;
  update_available: boolean;
  binary_present: boolean;
  github_reachable: boolean;
  error?: string;
}

export interface DownloadProgress {
  downloaded: number;
  total?: number;
}
