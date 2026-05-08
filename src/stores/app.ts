import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { api } from "@/api/tauri";
import type { AppInfo } from "@/types/rathole";

export const useAppStore = defineStore("app", () => {
  const info = ref<AppInfo | null>(null);
  const loading = ref(false);

  const ready = computed(() => info.value?.rathole_exists ?? false);

  async function refresh() {
    loading.value = true;
    try {
      info.value = await api.getAppInfo();
    } finally {
      loading.value = false;
    }
  }

  async function updateSettings(payload: { rathole_path?: string; auto_resume?: boolean }) {
    await api.updateSettings(payload);
    await refresh();
  }

  return { info, loading, ready, refresh, updateSettings };
});
