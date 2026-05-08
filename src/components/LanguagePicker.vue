<template>
  <a-popover
    v-model:open="open"
    trigger="click"
    placement="topRight"
  >
    <template #content>
      <div class="lang-menu">
        <button
          v-for="code in supported"
          :key="code"
          class="lang-item"
          :class="{ active: code === current }"
          @click="onPick(code)"
        >
          <CheckOutlined v-if="code === current" class="check" />
          <span v-else class="check-placeholder" />
          <span>{{ t(`language.${code}`) }}</span>
        </button>
      </div>
    </template>

    <a-button type="text" :title="t('settings.languageSection')" class="lang-button">
      <template #icon><GlobalOutlined /></template>
      <span class="lang-current">{{ shortLabel }}</span>
    </a-button>
  </a-popover>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { CheckOutlined, GlobalOutlined } from "@antdv-next/icons";
import { useLanguage } from "@/composables/useLanguage";
import type { LocaleCode } from "@/i18n";

const { t } = useI18n();
const { current, supported, setLanguage } = useLanguage();
const open = ref(false);

const shortLabel = computed(() => {
  switch (current.value) {
    case "zh-CN":
      return "中";
    case "en":
      return "EN";
    case "ja":
      return "あ";
    case "ko":
      return "한";
    default:
      return current.value;
  }
});

async function onPick(code: LocaleCode) {
  open.value = false;
  await setLanguage(code);
}
</script>

<style lang="less" scoped>
.lang-button {
  height: 32px;
  padding: 0 8px;
}

.lang-current {
  margin-left: 4px;
  font-size: 12px;
  color: var(--rl-text-muted);
}

.lang-menu {
  display: flex;
  flex-direction: column;
  min-width: 140px;
  margin: -4px -8px;
}

.lang-item {
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
  gap: 6px;
  border-radius: 6px;
}

.lang-item:hover {
  background: rgba(37, 99, 235, 0.08);
}

.lang-item.active {
  color: var(--rl-accent);
  font-weight: 500;
}

.check {
  font-size: 12px;
}

.check-placeholder {
  display: inline-block;
  width: 12px;
}
</style>
