import { defineStore } from "pinia";
import { ref } from "vue";
import { api } from "@/api/tauri";
import type { ConfigSummary, Mode } from "@/types/rathole";

export const useConfigStore = defineStore("configs", () => {
  const server = ref<ConfigSummary[]>([]);
  const client = ref<ConfigSummary[]>([]);
  const loading = ref(false);

  async function refresh(mode?: Mode) {
    loading.value = true;
    try {
      if (!mode || mode === "server") server.value = await api.listConfigs("server");
      if (!mode || mode === "client") client.value = await api.listConfigs("client");
    } finally {
      loading.value = false;
    }
  }

  function listFor(mode: Mode): ConfigSummary[] {
    return mode === "server" ? server.value : client.value;
  }

  async function remove(mode: Mode, name: string) {
    await api.deleteConfig(mode, name);
    await refresh(mode);
  }

  async function rename(mode: Mode, oldName: string, newName: string) {
    await api.renameConfig(mode, oldName, newName);
    await refresh(mode);
  }

  async function duplicate(mode: Mode, source: string, target: string) {
    await api.duplicateConfig(mode, source, target);
    await refresh(mode);
  }

  return { server, client, loading, refresh, listFor, remove, rename, duplicate };
});
