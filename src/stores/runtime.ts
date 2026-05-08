import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { api } from "@/api/tauri";
import { onRatholeLog, onRatholeStatus } from "@/api/events";
import type { LogLine, Mode, RunStatus } from "@/types/rathole";
import { instanceKey } from "@/utils/keys";

const LOG_BUFFER_LIMIT = 1000;

export const useRuntimeStore = defineStore("runtime", () => {
  const statuses = ref<Record<string, RunStatus>>({});
  const logs = ref<Record<string, LogLine[]>>({});
  let unsubscribers: Array<() => void> = [];

  const running = computed(() =>
    Object.values(statuses.value).filter((s) => s.state === "running" || s.state === "starting"),
  );

  function statusOf(mode: Mode, name: string): RunStatus | undefined {
    return statuses.value[instanceKey(mode, name)];
  }

  function logsOf(mode: Mode, name: string): LogLine[] {
    return logs.value[instanceKey(mode, name)] ?? [];
  }

  function appendLog(line: LogLine) {
    const key = instanceKey(line.mode, line.name);
    const current = logs.value[key] ?? [];
    const next = current.concat(line);
    if (next.length > LOG_BUFFER_LIMIT) next.splice(0, next.length - LOG_BUFFER_LIMIT);
    logs.value = { ...logs.value, [key]: next };
  }

  function clearLogs(mode: Mode, name: string) {
    const key = instanceKey(mode, name);
    logs.value = { ...logs.value, [key]: [] };
  }

  async function refreshAll() {
    const list = await api.listRunning();
    const next: Record<string, RunStatus> = {};
    for (const s of list) next[instanceKey(s.mode, s.name)] = s;
    statuses.value = next;
  }

  async function loadInitialLogs(mode: Mode, name: string) {
    const lines = await api.getRecentLogs(mode, name, 200);
    logs.value = { ...logs.value, [instanceKey(mode, name)]: lines };
  }

  async function start(mode: Mode, name: string) {
    const status = await api.startConfig(mode, name);
    statuses.value = { ...statuses.value, [instanceKey(mode, name)]: status };
    return status;
  }

  async function stop(mode: Mode, name: string) {
    await api.stopConfig(mode, name);
  }

  async function bind() {
    const offLog = await onRatholeLog((line) => appendLog(line));
    const offStatus = await onRatholeStatus(async () => {
      await refreshAll();
    });
    unsubscribers = [offLog, offStatus];
    await refreshAll();
  }

  function unbind() {
    for (const off of unsubscribers) {
      try {
        off();
      } catch {
        // ignore
      }
    }
    unsubscribers = [];
  }

  return {
    statuses,
    logs,
    running,
    statusOf,
    logsOf,
    appendLog,
    clearLogs,
    refreshAll,
    loadInitialLogs,
    start,
    stop,
    bind,
    unbind,
  };
});
