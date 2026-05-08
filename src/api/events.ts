import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DownloadProgress, LogLine, RunStatusEvent } from "@/types/rathole";

export const onRatholeLog = (handler: (line: LogLine) => void): Promise<UnlistenFn> =>
  listen<LogLine>("rathole-log", (event) => handler(event.payload));

export const onRatholeStatus = (
  handler: (status: RunStatusEvent) => void,
): Promise<UnlistenFn> =>
  listen<RunStatusEvent>("rathole-status", (event) => handler(event.payload));

export const onDownloadProgress = (
  handler: (progress: DownloadProgress) => void,
): Promise<UnlistenFn> =>
  listen<DownloadProgress>("rathole-download-progress", (event) =>
    handler(event.payload),
  );
