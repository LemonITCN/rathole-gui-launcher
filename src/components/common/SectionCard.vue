<template>
  <section class="section-card rl-card">
    <header v-if="title || $slots.title" class="section-head">
      <div>
        <div class="section-title">
          <slot name="title">{{ title }}</slot>
        </div>
        <p v-if="hint" class="section-hint rl-muted">{{ hint }}</p>
      </div>
      <div class="section-actions">
        <slot name="actions" />
      </div>
    </header>
    <div class="section-body" :class="{ padded }">
      <slot />
    </div>
  </section>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{ title?: string; hint?: string; padded?: boolean }>(),
  { padded: true },
);
</script>

<style lang="less" scoped>
.section-card {
  display: flex;
  flex-direction: column;
}

.section-head {
  padding: 14px 18px;
  border-bottom: 1px solid var(--rl-border);
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.section-title {
  font-weight: 600;
  font-size: 14px;
}

.section-hint {
  margin: 4px 0 0;
  font-size: 12px;
  line-height: 1.6;
}

.section-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.section-body {
  &.padded {
    padding: 16px 18px;
  }
}
</style>
