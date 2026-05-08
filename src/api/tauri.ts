import { invoke } from "@tauri-apps/api/core";
import type {
  AppInfo,
  ClientConfig,
  ConfigSummary,
  ExternalRathole,
  LogLine,
  Mode,
  PortStatus,
  RunStatus,
  ServerConfig,
  Settings,
} from "@/types/rathole";

export const api = {
  getAppInfo: () => invoke<AppInfo>("get_app_info"),

  updateSettings: (payload: {
    rathole_path?: string;
    auto_resume?: boolean;
    language?: string;
  }) => invoke<Settings>("update_settings", { payload }),

  listConfigs: (mode: Mode) =>
    invoke<ConfigSummary[]>("list_configs", { mode }),

  getServerConfig: (name: string) =>
    invoke<ServerConfig>("get_server_config", { name }),

  getClientConfig: (name: string) =>
    invoke<ClientConfig>("get_client_config", { name }),

  saveServerConfig: (name: string, config: ServerConfig) =>
    invoke<void>("save_server_config", { name, config }),

  saveClientConfig: (name: string, config: ClientConfig) =>
    invoke<void>("save_client_config", { name, config }),

  deleteConfig: (mode: Mode, name: string) =>
    invoke<void>("delete_config", { mode, name }),

  renameConfig: (mode: Mode, oldName: string, newName: string) =>
    invoke<void>("rename_config", { mode, oldName, newName }),

  duplicateConfig: (mode: Mode, source: string, target: string) =>
    invoke<void>("duplicate_config", { mode, source, target }),

  startConfig: (mode: Mode, name: string) =>
    invoke<RunStatus>("start_config", { mode, name }),

  stopConfig: (mode: Mode, name: string) =>
    invoke<void>("stop_config", { mode, name }),

  listRunning: () => invoke<RunStatus[]>("list_running"),

  getRunStatus: (mode: Mode, name: string) =>
    invoke<RunStatus | null>("get_run_status", { mode, name }),

  getRecentLogs: (mode: Mode, name: string, limit: number) =>
    invoke<LogLine[]>("get_recent_logs", { mode, name, limit }),

  checkPorts: (addrs: string[]) =>
    invoke<PortStatus[]>("check_ports", { addrs }),

  findExternalRathole: () =>
    invoke<ExternalRathole[]>("find_external_rathole"),

  openConfDir: (mode: Mode) =>
    invoke<string>("open_conf_dir", { mode }),
};
