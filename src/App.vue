<template>
  <a-config-provider :theme="themeConfig" :locale="antdLocale">
    <AppShell />
  </a-config-provider>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, computed } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { theme, message } from "antdv-next";
import zhCN from "antdv-next/locale/zh_CN";
import enUS from "antdv-next/locale/en_US";
import jaJP from "antdv-next/locale/ja_JP";
import koKR from "antdv-next/locale/ko_KR";
import AppShell from "@/components/AppShell.vue";
import { useAppStore } from "@/stores/app";
import { useRuntimeStore } from "@/stores/runtime";
import { useConfigStore } from "@/stores/configs";
import { useUpdaterStore } from "@/stores/updater";
import { useLanguage } from "@/composables/useLanguage";
import type { LocaleCode } from "@/i18n";

const appStore = useAppStore();
const runtimeStore = useRuntimeStore();
const configStore = useConfigStore();
const updaterStore = useUpdaterStore();
const router = useRouter();
const { t } = useI18n();
const { current, setLanguage } = useLanguage();

let unlistenTrayNav: UnlistenFn | null = null;
let unlistenTrayAction: UnlistenFn | null = null;

const antdLocaleMap = {
  "zh-CN": zhCN,
  en: enUS,
  ja: jaJP,
  ko: koKR,
} as const;

const themeConfig = computed(() => ({
  algorithm: theme.defaultAlgorithm,
  token: {
    colorPrimary: "#2563eb",
    colorInfo: "#2563eb",
    borderRadius: 8,
    fontSize: 14,
  },
}));

const antdLocale = computed(() => antdLocaleMap[current.value] ?? zhCN);

onMounted(async () => {
  await Promise.all([
    appStore.refresh(),
    configStore.refresh(),
    runtimeStore.bind(),
  ]);
  const persisted = appStore.info?.language as LocaleCode | undefined;
  if (persisted && persisted !== current.value) {
    await setLanguage(persisted, false);
  }
  // Fire and forget — banner reacts to whatever the check resolves to.
  updaterStore.check();
  // Tray menu items emit `tray-navigate` so the user lands on the matching
  // editor page when picking a running config from the menu bar.
  unlistenTrayNav = await listen<{ mode: string; name: string }>(
    "tray-navigate",
    (event) => {
      const { mode, name } = event.payload;
      if ((mode === "server" || mode === "client") && name) {
        router.push({ name: `${mode}-detail`, params: { name } });
      }
    },
  );
  unlistenTrayAction = await listen<{
    action: string;
    mode: string;
    name: string;
    ok: boolean;
    error?: string;
  }>("tray-action-result", (event) => {
    const { action, name, ok, error } = event.payload;
    if (!ok) {
      const prefix =
        action === "start" ? t("editor.startFailed") : t("editor.stopFailed");
      message.error(`${name}: ${prefix}${error ? ` - ${error}` : ""}`);
    }
  });
});

onBeforeUnmount(() => {
  runtimeStore.unbind();
  unlistenTrayNav?.();
  unlistenTrayAction?.();
});
</script>

<style lang="less">
:root,
body,
#app {
  background: var(--rl-bg);
}
</style>
