<template>
  <span class="status-badge" :class="[`status-${tone}`, { compact }]">
    <span class="status-dot" />
    <span v-if="!compact" class="status-label">{{ label }}</span>
  </span>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { RunState } from "@/types/rathole";

const props = withDefaults(
  defineProps<{
    state?: RunState | "stopped";
    compact?: boolean;
  }>(),
  { state: "stopped", compact: false },
);

const { t } = useI18n();

const tone = computed(() => {
  switch (props.state) {
    case "running":
      return "success";
    case "starting":
    case "stopping":
      return "warning";
    case "exited":
      return "danger";
    default:
      return "idle";
  }
});

const label = computed(() => {
  switch (props.state) {
    case "running":
      return t("common.running");
    case "starting":
      return t("common.starting");
    case "stopping":
      return t("common.stopping");
    case "exited":
      return t("common.exited");
    default:
      return t("common.stopped");
  }
});
</script>

<style lang="less" scoped>
.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--rl-text-muted);
  line-height: 1;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #cbd0d8;
  box-shadow: 0 0 0 2px rgba(203, 208, 216, 0.25);
}

.status-success .status-dot {
  background: var(--rl-success);
  box-shadow: 0 0 0 2px rgba(21, 163, 74, 0.18);
  animation: pulse 1.6s ease-out infinite;
}

.status-warning .status-dot {
  background: var(--rl-warning);
  box-shadow: 0 0 0 2px rgba(217, 119, 6, 0.18);
}

.status-danger .status-dot {
  background: var(--rl-danger);
  box-shadow: 0 0 0 2px rgba(220, 38, 38, 0.18);
}

@keyframes pulse {
  0%, 100% {
    box-shadow: 0 0 0 2px rgba(21, 163, 74, 0.18);
  }
  50% {
    box-shadow: 0 0 0 6px rgba(21, 163, 74, 0.05);
  }
}
</style>
