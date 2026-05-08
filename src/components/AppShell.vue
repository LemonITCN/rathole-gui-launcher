<template>
  <a-layout class="app-shell">
    <a-layout-sider class="app-sider" width="260" :trigger="null">
      <div class="brand">
        <div class="brand-mark">RH</div>
        <div class="brand-meta">
          <div class="brand-title">Rathole</div>
          <div class="brand-subtitle">{{ t("sidebar.brandSubtitle") }}</div>
        </div>
      </div>

      <div class="mode-switch">
        <a-segmented
          v-model:value="activeMode"
          :options="modeOptions"
          block
          @change="onModeChange"
        />
      </div>

      <div class="config-list-header">
        <span class="rl-section-title">{{ t("sidebar.configsHeader") }}</span>
        <a-tooltip :title="t('sidebar.newConfigTooltip')">
          <a-button type="text" size="small" @click="showCreate = true">
            <template #icon><PlusOutlined /></template>
          </a-button>
        </a-tooltip>
      </div>

      <ConfigList
        :mode="activeMode"
        :selected="selectedName"
        @select="onSelect"
      />

      <div class="sider-footer">
        <a-button type="text" class="settings-btn" @click="goSettings">
          <template #icon><SettingOutlined /></template>
          {{ t("sidebar.settings") }}
        </a-button>
        <LanguagePicker />
      </div>
    </a-layout-sider>

    <a-layout class="app-content">
      <router-view />
    </a-layout>

    <CreateConfigModal
      v-model:open="showCreate"
      :mode="activeMode"
      @created="onCreated"
    />
  </a-layout>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { PlusOutlined, SettingOutlined } from "@ant-design/icons-vue";
import ConfigList from "@/components/ConfigList.vue";
import CreateConfigModal from "@/components/CreateConfigModal.vue";
import LanguagePicker from "@/components/LanguagePicker.vue";
import type { Mode } from "@/types/rathole";

const router = useRouter();
const route = useRoute();
const { t } = useI18n();
const showCreate = ref(false);

const modeOptions = computed(() => [
  { label: t("sidebar.modeServer"), value: "server" },
  { label: t("sidebar.modeClient"), value: "client" },
]);

const activeMode = ref<Mode>(deriveMode(route.name as string));
const selectedName = computed(() => (route.params.name as string) ?? "");

watch(
  () => route.name,
  (name) => {
    if (name === "server" || name === "server-detail") activeMode.value = "server";
    if (name === "client" || name === "client-detail") activeMode.value = "client";
  },
);

function deriveMode(name: string): Mode {
  if (name === "client" || name === "client-detail") return "client";
  return "server";
}

function onModeChange(value: Mode | string | number) {
  const next = String(value) as Mode;
  router.push({ name: next });
}

function onSelect(name: string) {
  router.push({ name: `${activeMode.value}-detail`, params: { name } });
}

function onCreated(name: string) {
  router.push({ name: `${activeMode.value}-detail`, params: { name } });
}

function goSettings() {
  router.push({ name: "settings" });
}
</script>

<style lang="less" scoped>
.app-shell {
  height: 100vh;
}

.app-sider {
  background: var(--rl-surface) !important;
  border-right: 1px solid var(--rl-border);
  display: flex;
  flex-direction: column;
  padding: 18px 12px 12px;
  gap: 14px;
}

.app-sider :deep(.ant-layout-sider-children) {
  display: flex;
  flex-direction: column;
  gap: 14px;
  height: 100%;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 0 4px;
}

.brand-mark {
  width: 36px;
  height: 36px;
  border-radius: 9px;
  background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  letter-spacing: 0.5px;
}

.brand-title {
  font-weight: 600;
  font-size: 15px;
  line-height: 1.2;
}

.brand-subtitle {
  color: var(--rl-text-muted);
  font-size: 12px;
}

.mode-switch {
  padding: 0 2px;
}

.config-list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 6px 0;
}

.sider-footer {
  margin-top: auto;
  border-top: 1px solid var(--rl-border);
  padding-top: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.settings-btn {
  flex: 1;
  justify-content: flex-start;
}

.app-content {
  background: var(--rl-bg);
  overflow: hidden;
}
</style>
