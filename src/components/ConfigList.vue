<template>
  <div class="config-list">
    <a-empty
      v-if="!items.length"
      :image="undefined"
      :description="emptyText"
      class="empty"
    />
    <button
      v-for="item in items"
      :key="item.name"
      class="config-item"
      :class="{ active: item.name === selected }"
      @click="$emit('select', item.name)"
    >
      <div class="row">
        <span class="name">{{ item.name }}</span>
        <StatusBadge :state="statusFor(item.name)" compact />
      </div>
      <div class="row meta">
        <span>{{ formatRelativeTime(item.modified) }}</span>
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { useConfigStore } from "@/stores/configs";
import { useRuntimeStore } from "@/stores/runtime";
import StatusBadge from "@/components/StatusBadge.vue";
import { formatRelativeTime } from "@/utils/keys";
import type { Mode, RunState } from "@/types/rathole";

const props = defineProps<{ mode: Mode; selected: string }>();
defineEmits<{ (e: "select", name: string): void }>();

const configStore = useConfigStore();
const runtimeStore = useRuntimeStore();
const { t } = useI18n();

const items = computed(() => configStore.listFor(props.mode));
const emptyText = computed(() =>
  props.mode === "server" ? t("config.emptyServer") : t("config.emptyClient"),
);

function statusFor(name: string): RunState | "stopped" {
  return runtimeStore.statusOf(props.mode, name)?.state ?? "stopped";
}
</script>

<style lang="less" scoped>
.config-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 0 4px;
}

.empty {
  margin: 16px 0;
  color: var(--rl-text-muted);
}

.config-item {
  appearance: none;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 8px;
  padding: 10px 12px;
  text-align: left;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 4px;
  transition: background 0.12s ease, border-color 0.12s ease;
}

.config-item:hover {
  background: var(--rl-bg);
}

.config-item.active {
  background: rgba(37, 99, 235, 0.08);
  border-color: rgba(37, 99, 235, 0.18);
}

.row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.name {
  font-weight: 500;
  font-size: 13px;
  color: var(--rl-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta {
  font-size: 11px;
  color: var(--rl-text-muted);
}
</style>
