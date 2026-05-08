<template>
  <div class="welcome">
    <a-typography-title :level="3" class="title">
      {{ heading }}
    </a-typography-title>
    <p class="rl-muted lead">
      {{ lead }}
    </p>


    <div class="cards">
      <div class="rl-card guide-card">
        <div class="guide-title">{{ t("welcome.serverGuideTitle") }}</div>
        <p class="guide-desc">{{ t("welcome.serverGuideDesc") }}</p>
        <ul class="guide-list">
          <li>
            <span class="rl-mono">bind_addr</span>：{{ t("welcome.serverGuideAttr1") }}
          </li>
          <li>
            <span class="rl-mono">services.*.bind_addr</span>：{{ t("welcome.serverGuideAttr2") }}
          </li>
        </ul>
      </div>
      <div class="rl-card guide-card">
        <div class="guide-title">{{ t("welcome.clientGuideTitle") }}</div>
        <p class="guide-desc">{{ t("welcome.clientGuideDesc") }}</p>
        <ul class="guide-list">
          <li>
            <span class="rl-mono">remote_addr</span>：{{ t("welcome.clientGuideAttr1") }}
          </li>
          <li>
            <span class="rl-mono">services.*.local_addr</span>：{{ t("welcome.clientGuideAttr2") }}
          </li>
        </ul>
      </div>
    </div>

    <div v-if="!list.length" class="empty rl-muted">
      {{ emptyHint }}
    </div>
    <div v-else class="recent">
      <div class="rl-section-title">{{ t("welcome.recentConfigs") }}</div>
      <div class="recent-list">
        <button
          v-for="item in list.slice(0, 6)"
          :key="item.name"
          class="recent-item"
          @click="open(item.name)"
        >
          <span class="recent-name">{{ item.name }}</span>
          <StatusBadge :state="statusFor(item.name)" compact />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import StatusBadge from "@/components/StatusBadge.vue";
import { useConfigStore } from "@/stores/configs";
import { useRuntimeStore } from "@/stores/runtime";
import type { Mode, RunState } from "@/types/rathole";

const props = defineProps<{ mode: Mode }>();
const router = useRouter();
const configStore = useConfigStore();
const runtimeStore = useRuntimeStore();
const { t } = useI18n();

const list = computed(() => configStore.listFor(props.mode));
const heading = computed(() =>
  props.mode === "server" ? t("welcome.serverHeading") : t("welcome.clientHeading"),
);
const lead = computed(() =>
  props.mode === "server" ? t("welcome.serverLead") : t("welcome.clientLead"),
);
const emptyHint = computed(() =>
  props.mode === "server" ? t("welcome.emptyHintServer") : t("welcome.emptyHintClient"),
);

function statusFor(name: string): RunState | "stopped" {
  return runtimeStore.statusOf(props.mode, name)?.state ?? "stopped";
}

function open(name: string) {
  router.push({ name: `${props.mode}-detail`, params: { name } });
}
</script>

<style lang="less" scoped>
.welcome {
  max-width: 880px;
  margin: 0 auto;
  padding: 24px 28px 32px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.title {
  margin: 0 !important;
}

.lead {
  margin: 0;
  line-height: 1.6;
}

.warning {
  padding: 14px 16px;
  border-color: rgba(217, 119, 6, 0.4);
  background: rgba(254, 246, 232, 0.7);
}

.warning-row {
  display: flex;
  align-items: flex-start;
  gap: 10px;
}

.warning-icon {
  color: var(--rl-warning);
  font-size: 18px;
  margin-top: 2px;
}

.warning-title {
  font-weight: 600;
}

.warning-detail {
  margin-top: 4px;
  font-size: 13px;
  color: var(--rl-text-muted);
  line-height: 1.6;
}

.cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 16px;
}

.guide-card {
  padding: 18px 20px;
}

.guide-title {
  font-weight: 600;
  font-size: 15px;
  margin-bottom: 6px;
}

.guide-desc {
  margin: 0 0 10px;
  color: var(--rl-text-muted);
  line-height: 1.6;
}

.guide-list {
  margin: 0;
  padding-left: 18px;
  color: var(--rl-text-muted);
  font-size: 13px;
  line-height: 1.7;
}

.recent {
  margin-top: 6px;
}

.recent-list {
  margin-top: 8px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 8px;
}

.recent-item {
  appearance: none;
  background: var(--rl-surface);
  border: 1px solid var(--rl-border);
  border-radius: 8px;
  padding: 10px 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  text-align: left;
}

.recent-item:hover {
  border-color: rgba(37, 99, 235, 0.4);
}

.recent-name {
  font-weight: 500;
}

.empty {
  padding: 12px 0;
  font-size: 13px;
}
</style>
