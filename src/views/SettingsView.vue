<template>
  <div class="settings">
    <a-typography-title :level="3" class="title">{{ t("settings.title") }}</a-typography-title>
    <p class="rl-muted lead">
      <i18n-t keypath="settings.lead" tag="span">
        <template #dir>
          <code class="rl-mono">{{ info?.app_data_dir }}</code>
        </template>
      </i18n-t>
    </p>

    <RatholeStatusBanner />

    <SectionCard :title="t('settings.languageSection')" :hint="t('settings.languageHint')">
      <a-radio-group :value="current" button-style="solid" @change="onLanguageChange">
        <a-radio-button v-for="code in supported" :key="code" :value="code">
          {{ t(`language.${code}`) }}
        </a-radio-button>
      </a-radio-group>
    </SectionCard>

    <SectionCard :title="t('settings.binarySection')" :hint="t('settings.binaryHint')">
      <a-form layout="vertical">
        <a-form-item :label="t('settings.pathLabel')">
          <div class="path-row">
            <a-input
              v-model:value="customPath"
              :placeholder="info?.rathole_path"
              allow-clear
              class="path-input"
            />
            <a-button @click="pickFile">{{ t("common.browse") }}</a-button>
          </div>
        </a-form-item>
        <div class="status-row">
          <template v-if="info?.rathole_exists">
            <CheckCircleFilled style="color: var(--rl-success)" />
            <span>
              {{ t("settings.binaryFound") }}
              <code class="rl-mono">{{ info.rathole_path }}</code>
            </span>
            <a-tag v-if="info.rathole_version">{{ info.rathole_version }}</a-tag>
          </template>
          <template v-else>
            <ExclamationCircleFilled style="color: var(--rl-warning)" />
            <span>
              {{ t("settings.binaryMissing") }}
              <code class="rl-mono">{{ info?.rathole_path }}</code>
            </span>
          </template>
        </div>
        <div class="actions">
          <a-button type="primary" :loading="saving" @click="save">{{ t("common.save") }}</a-button>
          <a-button @click="reset">{{ t("settings.useDefault") }}</a-button>
          <a-button @click="appStore.refresh()">{{ t("settings.rescan") }}</a-button>
        </div>
      </a-form>
    </SectionCard>

    <SectionCard :title="t('settings.dirsSection')" :hint="t('settings.dirsHint')">
      <a-descriptions :column="1" size="small" bordered>
        <a-descriptions-item :label="t('settings.workDir')">
          <span class="rl-mono">{{ info?.app_data_dir }}</span>
        </a-descriptions-item>
        <a-descriptions-item :label="t('settings.serverConfDir')">
          <span class="rl-mono">{{ info?.server_conf_dir }}</span>
        </a-descriptions-item>
        <a-descriptions-item :label="t('settings.clientConfDir')">
          <span class="rl-mono">{{ info?.client_conf_dir }}</span>
        </a-descriptions-item>
        <a-descriptions-item :label="t('settings.platform')">
          <span class="rl-mono">{{ info?.platform }}</span>
        </a-descriptions-item>
      </a-descriptions>
    </SectionCard>

    <SectionCard :title="t('settings.externalsSection')" :hint="t('settings.externalsHint')">
      <template #actions>
        <a-button size="small" @click="scan">{{ t("common.refresh") }}</a-button>
      </template>
      <div v-if="!externals.length" class="empty-list rl-muted">
        {{ t("settings.externalsEmpty") }}
      </div>
      <a-list v-else :data-source="externals" item-layout="horizontal">
        <template #renderItem="{ item }">
          <a-list-item>
            <a-list-item-meta>
              <template #title>
                <span class="rl-mono">{{ item.name }}</span>
                <a-tag v-if="item.managed" color="blue" style="margin-left: 8px">
                  {{ t("settings.externalManaged") }}
                </a-tag>
                <a-tag v-else color="default" style="margin-left: 8px">
                  {{ t("settings.externalExternal") }}
                </a-tag>
              </template>
              <template #description>
                <span class="rl-mono">PID {{ item.pid }} · {{ item.cmd.join(" ") }}</span>
              </template>
            </a-list-item-meta>
          </a-list-item>
        </template>
      </a-list>
    </SectionCard>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { CheckCircleFilled, ExclamationCircleFilled } from "@antdv-next/icons";
import { message } from "antdv-next";
import SectionCard from "@/components/common/SectionCard.vue";
import RatholeStatusBanner from "@/components/RatholeStatusBanner.vue";
import { api } from "@/api/tauri";
import { useAppStore } from "@/stores/app";
import { useLanguage } from "@/composables/useLanguage";
import type { LocaleCode } from "@/i18n";
import type { ExternalRathole } from "@/types/rathole";

const appStore = useAppStore();
const { t } = useI18n();
const { current, supported, setLanguage } = useLanguage();
const customPath = ref<string>("");
const saving = ref(false);
const externals = ref<ExternalRathole[]>([]);

const info = computed(() => appStore.info);

onMounted(async () => {
  await appStore.refresh();
  customPath.value = appStore.info?.rathole_path ?? "";
  await scan();
});

async function pickFile() {
  const selected = await openDialog({
    title: t("settings.pickFileTitle"),
    multiple: false,
    directory: false,
  });
  if (typeof selected === "string") {
    customPath.value = selected;
  }
}

async function save() {
  saving.value = true;
  try {
    await appStore.updateSettings({ rathole_path: customPath.value });
    message.success(t("settings.saved"));
  } catch (err: any) {
    message.error(
      typeof err === "string" ? err : err?.message ?? t("settings.saveFailedSettings"),
    );
  } finally {
    saving.value = false;
  }
}

async function reset() {
  customPath.value = "";
  await appStore.updateSettings({ rathole_path: "" });
  customPath.value = appStore.info?.rathole_path ?? "";
  message.success(t("settings.defaultsRestored"));
}

async function scan() {
  try {
    externals.value = await api.findExternalRathole();
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("settings.scanFailed"));
  }
}

async function onLanguageChange(event: any) {
  const next = event?.target?.value as LocaleCode;
  if (!next) return;
  await setLanguage(next);
}
</script>

<style lang="less" scoped>
.settings {
  height: 100%;
  overflow: auto;
  padding: 24px 28px 32px;
  max-width: 880px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.title {
  margin: 0 !important;
}

.lead {
  margin: 0 0 6px;
  line-height: 1.6;
}

.path-row {
  display: flex;
  gap: 8px;
  align-items: stretch;
}

.path-input {
  flex: 1;
  min-width: 0;
}

.path-row :deep(.ant-input-affix-wrapper),
.path-row :deep(.ant-input) {
  width: 100%;
}

.status-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  margin-bottom: 12px;
  flex-wrap: wrap;
  min-width: 0;
}

.status-row > * {
  min-width: 0;
  overflow-wrap: anywhere;
}

.status-row :deep(.ant-tag) {
  white-space: normal;
  max-width: 100%;
  overflow-wrap: anywhere;
}

.status-row code.rl-mono {
  overflow-wrap: anywhere;
}

.actions {
  display: flex;
  gap: 8px;
}

.empty-list {
  text-align: center;
  padding: 28px 0;
  font-size: 13px;
}
</style>
