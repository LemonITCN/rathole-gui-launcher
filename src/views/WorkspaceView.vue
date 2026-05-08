<template>
  <div class="workspace">
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
import type { Mode } from "@/types/rathole";

const props = defineProps<{ mode: Mode; name?: string }>();
</script>

<style lang="less" scoped>
.workspace {
  height: 100%;
  overflow: auto;
  padding: 24px 28px 32px;
}
</style>
