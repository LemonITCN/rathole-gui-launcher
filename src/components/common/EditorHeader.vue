<template>
  <header class="editor-header rl-card">
    <div class="header-row">
      <div class="meta">
        <div class="title-row">
          <h2 class="title">{{ name }}</h2>
          <a-tag :color="modeColor" class="mode-tag">{{ modeLabel }}</a-tag>
          <StatusBadge :state="status?.state ?? 'stopped'" />
        </div>
        <div v-if="hasSubRow" class="sub-row">
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
        <a-popover v-model:open="menuOpen" trigger="click" placement="bottomRight">
          <template #content>
            <div class="action-menu">
              <button class="action-item" @click="pick('duplicate')">
                <CopyOutlined class="icon" />
                <span>{{ t("editor.duplicateMenu") }}</span>
              </button>
              <button class="action-item" @click="pick('rename')">
                <EditOutlined class="icon" />
                <span>{{ t("editor.renameMenu") }}</span>
              </button>
              <button class="action-item" @click="pick('open-dir')">
                <FolderOpenOutlined class="icon" />
                <span>{{ t("editor.openDirMenu") }}</span>
              </button>
              <div class="action-divider" />
              <button class="action-item danger" @click="pick('delete')">
                <DeleteOutlined class="icon" />
                <span>{{ t("editor.deleteMenu") }}</span>
              </button>
            </div>
          </template>
          <a-button>
            <template #icon><MoreOutlined /></template>
          </a-button>
        </a-popover>
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
    </div>

    <div v-if="needsRestart" class="restart-strip">
      <ExclamationCircleFilled class="restart-icon" />
      <div class="restart-text">
        <span class="restart-title">{{ t("editor.savedWhileRunningTitle") }}</span>
        <span class="restart-body rl-muted">{{ t("editor.savedWhileRunningBody") }}</span>
      </div>
      <div class="restart-actions">
        <a-button
          type="primary"
          size="small"
          :loading="restarting"
          @click="$emit('restart')"
        >
          {{ t("editor.restartNow") }}
        </a-button>
        <a-button
          size="small"
          :disabled="restarting"
          @click="$emit('dismiss-restart')"
        >
          {{ t("editor.dismiss") }}
        </a-button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import {
  CopyOutlined,
  DeleteOutlined,
  EditOutlined,
  ExclamationCircleFilled,
  FolderOpenOutlined,
  MoreOutlined,
  PauseCircleOutlined,
  PlayCircleOutlined,
} from "@antdv-next/icons";
import StatusBadge from "@/components/StatusBadge.vue";
import { formatRelativeTime } from "@/utils/keys";
import type { Mode, RunStatus } from "@/types/rathole";

type MenuAction = "duplicate" | "rename" | "open-dir" | "delete";

const props = defineProps<{
  mode: Mode;
  name: string;
  status?: RunStatus;
  dirty: boolean;
  saving: boolean;
  starting: boolean;
  stopping: boolean;
  needsRestart?: boolean;
  restarting?: boolean;
}>();

const emit = defineEmits<{
  (e: "save"): void;
  (e: "start"): void;
  (e: "stop"): void;
  (e: "rename"): void;
  (e: "duplicate"): void;
  (e: "delete"): void;
  (e: "open-dir"): void;
  (e: "restart"): void;
  (e: "dismiss-restart"): void;
}>();

const { t } = useI18n();
const menuOpen = ref(false);

const modeLabel = computed(() =>
  props.mode === "server" ? t("sidebar.modeServer") : t("sidebar.modeClient"),
);
const modeColor = computed(() => (props.mode === "server" ? "blue" : "purple"));

const hasSubRow = computed(
  () =>
    !!props.status?.pid ||
    !!props.status?.started_at ||
    (props.status?.last_exit_code !== undefined &&
      props.status?.state === "exited") ||
    props.dirty,
);

function pick(action: MenuAction) {
  menuOpen.value = false;
  emit(action);
}
</script>

<style lang="less" scoped>
.editor-header {
  display: flex;
  flex-direction: column;
  position: sticky;
  top: 0;
  z-index: 10;
  background: var(--rl-surface);
  margin: 0 -28px;
  border: none;
  border-bottom: 1px solid var(--rl-border);
  border-radius: 0;
  box-shadow: none;
}

.header-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 28px;
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

.action-menu {
  display: flex;
  flex-direction: column;
  min-width: 180px;
  margin: -4px -8px;
}

.action-item {
  appearance: none;
  background: transparent;
  border: none;
  text-align: left;
  padding: 8px 10px;
  font-size: 13px;
  color: var(--rl-text);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  border-radius: 6px;
}

.action-item:hover {
  background: rgba(37, 99, 235, 0.08);
}

.action-item.danger {
  color: var(--rl-danger);
}

.action-item.danger:hover {
  background: rgba(220, 38, 38, 0.08);
}

.action-item .icon {
  font-size: 13px;
}

.action-divider {
  height: 1px;
  background: var(--rl-border);
  margin: 4px 0;
}

.restart-strip {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 28px;
  background: rgba(254, 246, 232, 0.85);
  border-top: 1px solid rgba(217, 119, 6, 0.35);
}

.restart-icon {
  color: var(--rl-warning);
  font-size: 16px;
  flex-shrink: 0;
}

.restart-text {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0 10px;
  font-size: 12px;
  line-height: 1.55;
}

.restart-title {
  font-weight: 600;
  color: var(--rl-text);
}

.restart-body {
  font-size: 12px;
}

.restart-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}
</style>
