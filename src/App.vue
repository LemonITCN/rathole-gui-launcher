<template>
  <a-config-provider :theme="themeConfig" :locale="antdLocale">
    <AppShell />
  </a-config-provider>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, computed } from "vue";
import { theme } from "ant-design-vue";
import zhCN from "ant-design-vue/es/locale/zh_CN";
import enUS from "ant-design-vue/es/locale/en_US";
import jaJP from "ant-design-vue/es/locale/ja_JP";
import koKR from "ant-design-vue/es/locale/ko_KR";
import AppShell from "@/components/AppShell.vue";
import { useAppStore } from "@/stores/app";
import { useRuntimeStore } from "@/stores/runtime";
import { useConfigStore } from "@/stores/configs";
import { useLanguage } from "@/composables/useLanguage";
import type { LocaleCode } from "@/i18n";

const appStore = useAppStore();
const runtimeStore = useRuntimeStore();
const configStore = useConfigStore();
const { current, setLanguage } = useLanguage();

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
});

onBeforeUnmount(() => {
  runtimeStore.unbind();
});
</script>

<style lang="less">
:root,
body,
#app {
  background: var(--rl-bg);
}
</style>
