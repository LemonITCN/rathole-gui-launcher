<template>
  <div class="editor">
    <a-skeleton v-if="loading" active />
    <template v-else-if="config">
      <EditorHeader
        mode="server"
        :name="props.name"
        :status="status"
        :dirty="dirty"
        :saving="saving"
        :starting="starting"
        :stopping="stopping"
        @save="save"
        @start="start"
        @stop="stop"
        @rename="renameOpen = true"
        @duplicate="duplicateOpen = true"
        @delete="onDelete"
        @open-dir="openDir"
      />

      <SectionCard :title="t('editor.basicTitle')" :hint="t('editor.basicHintServer')">
        <a-form layout="vertical">
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item :label="t('editor.bindAddrLabel')" required>
                <a-input
                  v-model:value="config.server.bind_addr"
                  :placeholder="t('editor.bindAddrPlaceholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item :label="t('editor.defaultTokenLabel')">
                <a-input-password
                  v-model:value="defaultToken"
                  autocomplete="off"
                  allow-clear
                  :placeholder="t('editor.defaultTokenPlaceholder')"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </SectionCard>

      <ServiceList v-model="config.server.services" role="server" />

      <a-collapse ghost class="advanced">
        <a-collapse-panel key="adv" :header="t('editor.advancedHeaderServer')">
          <SectionCard :title="t('editor.heartbeatTitle')" :hint="t('editor.heartbeatHint')">
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item
                  :label="t('editor.heartbeatIntervalLabel')"
                  :extra="t('editor.heartbeatIntervalExtra')"
                >
                  <a-input-number
                    v-model:value="config.server.heartbeat_interval"
                    :min="0"
                    style="width: 100%"
                    placeholder="30"
                  />
                </a-form-item>
              </a-col>
            </a-row>
          </SectionCard>

          <SectionCard :title="t('editor.transportTitle')" :hint="t('editor.transportHintServer')">
            <TransportEditor v-model="config.server.transport" role="server" />
          </SectionCard>
        </a-collapse-panel>
      </a-collapse>

      <LogPanel mode="server" :name="props.name" />

      <NameDialog
        v-model:open="renameOpen"
        :title="t('editor.renameTitle')"
        :label="t('editor.renameLabel')"
        :initial="props.name"
        :ok-text="t('common.save')"
        :submit="handleRename"
      />
      <NameDialog
        v-model:open="duplicateOpen"
        :title="t('editor.duplicateTitle')"
        :label="t('editor.renameLabel')"
        :initial="`${props.name}-copy`"
        :ok-text="t('common.create')"
        :submit="handleDuplicate"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { Modal, message } from "ant-design-vue";
import { useRouter } from "vue-router";
import EditorHeader from "@/components/common/EditorHeader.vue";
import SectionCard from "@/components/common/SectionCard.vue";
import ServiceList from "@/components/common/ServiceList.vue";
import TransportEditor from "@/components/common/TransportEditor.vue";
import LogPanel from "@/components/common/LogPanel.vue";
import NameDialog from "@/components/common/NameDialog.vue";
import { api } from "@/api/tauri";
import { useConfigStore } from "@/stores/configs";
import { useRuntimeStore } from "@/stores/runtime";
import type { ServerConfig } from "@/types/rathole";

const props = defineProps<{ name: string }>();

const router = useRouter();
const configStore = useConfigStore();
const runtimeStore = useRuntimeStore();
const { t } = useI18n();

const config = ref<ServerConfig | null>(null);
const original = ref<string>("");
const loading = ref(false);
const saving = ref(false);
const starting = ref(false);
const stopping = ref(false);

const renameOpen = ref(false);
const duplicateOpen = ref(false);

const status = computed(() => runtimeStore.statusOf("server", props.name));

const dirty = computed(() => {
  if (!config.value) return false;
  return JSON.stringify(config.value) !== original.value;
});

const defaultToken = computed<string>({
  get: () => config.value?.server.default_token ?? "",
  set: (v) => {
    if (!config.value) return;
    config.value.server.default_token = v ? v : undefined;
  },
});

watch(
  () => props.name,
  async () => {
    await load();
  },
  { immediate: true },
);

async function load() {
  loading.value = true;
  try {
    const cfg = await api.getServerConfig(props.name);
    if (!cfg.server.services) cfg.server.services = {};
    config.value = cfg;
    original.value = JSON.stringify(cfg);
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("editor.loadFailed"));
    config.value = null;
  } finally {
    loading.value = false;
  }
}

async function save() {
  if (!config.value) return;
  saving.value = true;
  try {
    await api.saveServerConfig(props.name, config.value);
    original.value = JSON.stringify(config.value);
    await configStore.refresh("server");
    message.success(t("common.saved"));
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("editor.saveFailed"));
  } finally {
    saving.value = false;
  }
}

async function start() {
  starting.value = true;
  try {
    await runtimeStore.start("server", props.name);
    message.success(t("editor.started"));
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("editor.startFailed"));
  } finally {
    starting.value = false;
  }
}

async function stop() {
  stopping.value = true;
  try {
    await runtimeStore.stop("server", props.name);
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("editor.stopFailed"));
  } finally {
    stopping.value = false;
  }
}

async function handleRename(next: string) {
  if (next === props.name) return;
  await configStore.rename("server", props.name, next);
  router.replace({ name: "server-detail", params: { name: next } });
}

async function handleDuplicate(next: string) {
  await configStore.duplicate("server", props.name, next);
  router.push({ name: "server-detail", params: { name: next } });
}

function onDelete() {
  Modal.confirm({
    title: t("editor.deleteConfirmTitle", { name: props.name }),
    content: t("editor.deleteConfirmContent"),
    okText: t("common.delete"),
    okType: "danger",
    cancelText: t("common.cancel"),
    async onOk() {
      try {
        await configStore.remove("server", props.name);
        router.replace({ name: "server" });
      } catch (err: any) {
        message.error(typeof err === "string" ? err : err?.message ?? t("editor.deleteFailed"));
      }
    },
  });
}

async function openDir() {
  try {
    await api.openConfDir("server");
    message.success(t("editor.openDirSuccess"));
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("editor.openDirFailed"));
  }
}
</script>

<style lang="less" scoped>
.editor {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.advanced :deep(.ant-collapse-content-box) {
  padding: 0 !important;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.advanced :deep(.ant-collapse-header) {
  padding-left: 4px !important;
  font-weight: 500;
}
</style>
