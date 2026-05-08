<template>
  <SectionCard :title="t('log.title')" :hint="hint" :padded="false">
    <template #actions>
      <a-checkbox v-model:checked="autoScroll">{{ t("log.autoScroll") }}</a-checkbox>
      <a-button size="small" @click="copyLogs" :disabled="!lines.length">
        {{ t("log.copy") }}
      </a-button>
      <a-button size="small" danger ghost @click="clear" :disabled="!lines.length">
        {{ t("log.clear") }}
      </a-button>
    </template>
    <div ref="scrollerRef" class="log-scroller" @scroll="onScroll">
      <div v-if="!lines.length" class="empty rl-muted">
        {{ t("log.empty") }}
      </div>
      <div
        v-for="(line, idx) in lines"
        :key="idx"
        class="log-line"
        :class="`stream-${line.stream}`"
      >
        <span class="ts rl-mono">{{ formatTime(line.ts) }}</span>
        <span class="stream-tag rl-mono">{{ line.stream }}</span>
        <span class="content rl-mono">{{ line.line }}</span>
      </div>
    </div>
  </SectionCard>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { message } from "ant-design-vue";
import SectionCard from "@/components/common/SectionCard.vue";
import { useRuntimeStore } from "@/stores/runtime";
import type { Mode } from "@/types/rathole";

const props = defineProps<{ mode: Mode; name: string }>();
const runtimeStore = useRuntimeStore();
const { t } = useI18n();
const scrollerRef = ref<HTMLDivElement>();
const autoScroll = ref(true);
let userScrolledUp = false;

const lines = computed(() => runtimeStore.logsOf(props.mode, props.name));
const hint = computed(() => t("log.countHint", { count: lines.value.length }));

watch(
  lines,
  async () => {
    if (!autoScroll.value || userScrolledUp) return;
    await nextTick();
    if (scrollerRef.value) {
      scrollerRef.value.scrollTop = scrollerRef.value.scrollHeight;
    }
  },
  { flush: "post" },
);

watch(
  [() => props.mode, () => props.name],
  async () => {
    await runtimeStore.loadInitialLogs(props.mode, props.name);
  },
  { immediate: false },
);

onMounted(async () => {
  await runtimeStore.loadInitialLogs(props.mode, props.name);
  await nextTick();
  if (scrollerRef.value) {
    scrollerRef.value.scrollTop = scrollerRef.value.scrollHeight;
  }
});

function onScroll() {
  if (!scrollerRef.value) return;
  const el = scrollerRef.value;
  userScrolledUp = el.scrollHeight - el.scrollTop - el.clientHeight > 24;
}

function clear() {
  runtimeStore.clearLogs(props.mode, props.name);
}

async function copyLogs() {
  const text = lines.value.map((l) => `${l.ts} [${l.stream}] ${l.line}`).join("\n");
  try {
    await navigator.clipboard.writeText(text);
    message.success(t("log.copySuccess"));
  } catch {
    message.error(t("log.copyFailed"));
  }
}

function formatTime(ts: string) {
  const d = new Date(ts);
  if (Number.isNaN(d.getTime())) return ts;
  const pad = (n: number) => n.toString().padStart(2, "0");
  return `${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}`;
}
</script>

<style lang="less" scoped>
.log-scroller {
  height: 280px;
  overflow-y: auto;
  background: #0f172a;
  color: #cbd5e1;
  padding: 12px 16px;
  font-size: 12px;
  line-height: 1.55;
  border-radius: 0 0 var(--rl-radius) var(--rl-radius);
}

.empty {
  color: #64748b;
  text-align: center;
  padding: 32px 0;
}

.log-line {
  display: flex;
  gap: 8px;
  align-items: baseline;
  white-space: pre-wrap;
  word-break: break-word;
}

.ts {
  color: #475569;
  flex: 0 0 auto;
}

.stream-tag {
  color: #64748b;
  flex: 0 0 56px;
  text-transform: uppercase;
  font-size: 10px;
  padding-top: 2px;
}

.stream-stderr .content {
  color: #fca5a5;
}

.stream-system .content {
  color: #fde68a;
  font-style: italic;
}

.content {
  flex: 1;
}
</style>
