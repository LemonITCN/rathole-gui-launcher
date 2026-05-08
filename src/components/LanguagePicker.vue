<template>
  <a-dropdown :trigger="['click']" placement="topRight">
    <a-button type="text" :title="t('settings.languageSection')" class="lang-button">
      <template #icon><GlobalOutlined /></template>
      <span class="lang-current">{{ shortLabel }}</span>
    </a-button>
    <template #overlay>
      <a-menu @click="onClick" :selected-keys="[current]">
        <a-menu-item v-for="code in supported" :key="code">
          {{ t(`language.${code}`) }}
        </a-menu-item>
      </a-menu>
    </template>
  </a-dropdown>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { GlobalOutlined } from "@ant-design/icons-vue";
import { useLanguage } from "@/composables/useLanguage";
import type { LocaleCode } from "@/i18n";

const { t } = useI18n();
const { current, supported, setLanguage } = useLanguage();

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

async function onClick(event: { key: string }) {
  await setLanguage(event.key as LocaleCode);
}
</script>

<style lang="less" scoped>
.lang-button {
  height: 32px;
}

.lang-current {
  margin-left: 4px;
  font-size: 12px;
  color: var(--rl-text-muted);
}
</style>
