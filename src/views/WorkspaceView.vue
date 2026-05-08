<template>
  <div class="workspace">
    <RatholeStatusBanner class="banner-spacing" />
    <template v-if="!props.name">
      <WorkspaceWelcome :mode="props.mode" />
    </template>
    <template v-else-if="props.mode === 'server'">
      <ServerEditor :name="props.name" />
    </template>
    <template v-else>
      <ClientEditor :name="props.name" />
    </template>
  </div>
</template>

<script setup lang="ts">
import WorkspaceWelcome from "@/components/WorkspaceWelcome.vue";
import ServerEditor from "@/components/server/ServerEditor.vue";
import ClientEditor from "@/components/client/ClientEditor.vue";
import RatholeStatusBanner from "@/components/RatholeStatusBanner.vue";
import type { Mode } from "@/types/rathole";

const props = defineProps<{ mode: Mode; name?: string }>();
</script>

<style lang="less" scoped>
.workspace {
  height: 100%;
  overflow: auto;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.banner-spacing {
  align-self: stretch;
  max-width: 1080px;
  margin: 24px auto 0;
  padding: 0 28px;
  width: 100%;
}
</style>
