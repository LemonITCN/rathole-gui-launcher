<template>
  <header class="editor-header rl-card">
    <div class="meta">
      <div class="title-row">
        <h2 class="title">{{ name }}</h2>
        <a-tag :color="modeColor" class="mode-tag">{{ modeLabel }}</a-tag>
        <StatusBadge :state="status?.state ?? 'stopped'" />
      </div>
      <div class="sub-row">
        <span v-if="status?.pid" class="rl-muted">{{ t("editor.pid", { pid: status.pid }) }}</span>
        <span v-if="status?.started_at" class="rl-muted">
          {{ t("editor.startedAt", { time: formatRelativeTime(status.started_at) }) }}
        </span>
        <span
          v-if="status?.last_exit_code !== undefined && status?.state === 'exited'"
          class="rl-muted"
        >
          {{ t("editor.exitCode", { code: status.last_exit_code }) }}
        </span>
        <span v-if="dirty" class="dirty-tag">{{ t("editor.unsaved") }}</span>
      </div>
    </div>
    <div class="actions">
      <a-dropdown :trigger="['click']">
        <a-button>
          <template #icon><MoreOutlined /></template>
        </a-button>
        <template #overlay>
          <a-menu>
            <a-menu-item key="duplicate" @click="$emit('duplicate')">
              <CopyOutlined /> {{ t("editor.duplicateMenu") }}
            </a-menu-item>
            <a-menu-item key="rename" @click="$emit('rename')">
              <EditOutlined /> {{ t("editor.renameMenu") }}
            </a-menu-item>
            <a-menu-item key="open-dir" @click="$emit('open-dir')">
              <FolderOpenOutlined /> {{ t("editor.openDirMenu") }}
            </a-menu-item>
            <a-menu-divider />
            <a-menu-item key="delete" danger @click="$emit('delete')">
              <DeleteOutlined /> {{ t("editor.deleteMenu") }}
            </a-menu-item>
          </a-menu>
        </template>
      </a-dropdown>
      <a-button :disabled="!dirty || saving" :loading="saving" @click="$emit('save')">
        {{ t("common.save") }}
      </a-button>
      <a-button
        v-if="status?.state !== 'running' && status?.state !== 'starting'"
        type="primary"
        :loading="starting"
        :disabled="dirty"
        @click="$emit('start')"
      >
        <template #icon><PlayCircleOutlined /></template>
        {{ t("common.start") }}
      </a-button>
      <a-button
        v-else
        danger
        :loading="stopping"
        @click="$emit('stop')"
      >
        <template #icon><PauseCircleOutlined /></template>
        {{ t("common.stop") }}
      </a-button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import {
  CopyOutlined,
  DeleteOutlined,
  EditOutlined,
  FolderOpenOutlined,
  MoreOutlined,
  PauseCircleOutlined,
  PlayCircleOutlined,
} from "@ant-design/icons-vue";
import StatusBadge from "@/components/StatusBadge.vue";
import { formatRelativeTime } from "@/utils/keys";
import type { Mode, RunStatus } from "@/types/rathole";

const props = defineProps<{
  mode: Mode;
  name: string;
  status?: RunStatus;
  dirty: boolean;
  saving: boolean;
  starting: boolean;
  stopping: boolean;
}>();

defineEmits<{
  (e: "save"): void;
  (e: "start"): void;
  (e: "stop"): void;
  (e: "rename"): void;
  (e: "duplicate"): void;
  (e: "delete"): void;
  (e: "open-dir"): void;
}>();

const { t } = useI18n();

const modeLabel = computed(() =>
  props.mode === "server" ? t("sidebar.modeServer") : t("sidebar.modeClient"),
);
const modeColor = computed(() => (props.mode === "server" ? "blue" : "purple"));
</script>

<style lang="less" scoped>
.editor-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 20px;
}

.title-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  letter-spacing: -0.01em;
}

.mode-tag {
  font-size: 11px;
}

.sub-row {
  display: flex;
  flex-wrap: wrap;
  gap: 14px;
  margin-top: 6px;
  font-size: 12px;
}

.dirty-tag {
  color: var(--rl-warning);
  font-weight: 500;
}

.actions {
  display: flex;
  gap: 8px;
  align-items: center;
}
</style>
