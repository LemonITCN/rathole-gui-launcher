import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { api } from "@/api/tauri";
import { onDownloadProgress } from "@/api/events";
import type { DownloadProgress, UpdateCheckResult } from "@/types/rathole";

export const useUpdaterStore = defineStore("updater", () => {
  const result = ref<UpdateCheckResult | null>(null);
  const checking = ref(false);
  const downloading = ref(false);
  const dismissed = ref(false);
  const progress = ref<DownloadProgress | null>(null);
  let unsubscribe: (() => void) | null = null;

  const githubReachable = computed(() => result.value?.github_reachable ?? null);
  const updateAvailable = computed(
    () => !!result.value && result.value.update_available && !!result.value.asset,
  );
  const binaryPresent = computed(() => result.value?.binary_present ?? false);
  const latestVersion = computed(() => result.value?.latest_version ?? null);
  const installedVersion = computed(() => result.value?.installed_version ?? null);
  const checkError = computed(() => result.value?.error ?? null);
  const downloadPercent = computed(() => {
    if (!progress.value) return null;
    if (!progress.value.total) return null;
    return Math.min(100, Math.round((progress.value.downloaded / progress.value.total) * 100));
  });

  async function check() {
    checking.value = true;
    try {
      result.value = await api.checkRatholeUpdate();
    } catch (err) {
      result.value = {
        update_available: false,
        binary_present: false,
        github_reachable: false,
        error: err instanceof Error ? err.message : String(err),
      };
    } finally {
      checking.value = false;
    }
  }

  async function bindProgress() {
    if (unsubscribe) return;
    unsubscribe = await onDownloadProgress((p) => {
      progress.value = p;
    });
  }

  function unbindProgress() {
    unsubscribe?.();
    unsubscribe = null;
  }

  async function download() {
    if (!result.value?.asset) {
      throw new Error("no asset to download");
    }
    downloading.value = true;
    progress.value = { downloaded: 0, total: result.value.asset.size };
    try {
      await api.downloadRatholeRelease(result.value.asset.url);
      // re-check after install so installed_version refreshes
      await check();
      dismissed.value = false;
    } finally {
      downloading.value = false;
      progress.value = null;
    }
  }

  function dismiss() {
    dismissed.value = true;
  }

  return {
    result,
    checking,
    downloading,
    dismissed,
    progress,
    githubReachable,
    updateAvailable,
    binaryPresent,
    latestVersion,
    installedVersion,
    checkError,
    downloadPercent,
    check,
    bindProgress,
    unbindProgress,
    download,
    dismiss,
  };
});
