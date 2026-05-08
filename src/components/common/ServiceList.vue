<template>
  <SectionCard :title="title" :hint="t('services.hint')">
    <template #actions>
      <a-button type="primary" size="small" @click="addService">
        <template #icon><PlusOutlined /></template>
        {{ t("services.addService") }}
      </a-button>
    </template>

    <div v-if="!entries.length" class="empty rl-muted">
      {{ t("services.empty") }}
    </div>

    <div v-else class="services">
      <div v-for="entry in entries" :key="entry.id" class="service rl-card">
        <header class="service-head">
          <div class="left">
            <a-input
              :value="entry.name"
              size="small"
              class="name-input"
              :status="duplicateNames.has(entry.name) ? 'error' : ''"
              @update:value="(v) => renameEntry(entry.id, v)"
              :placeholder="t('services.namePlaceholder')"
            />
            <a-tag v-if="duplicateNames.has(entry.name)" color="red">
              {{ t("services.duplicateName") }}
            </a-tag>
            <a-tag v-else-if="!entry.name" color="orange">
              {{ t("services.unnamed") }}
            </a-tag>
          </div>
          <div class="right">
            <a-button type="text" size="small" danger @click="removeEntry(entry.id)">
              <template #icon><DeleteOutlined /></template>
            </a-button>
          </div>
        </header>

        <div class="service-body">
          <a-form layout="vertical">
            <a-row :gutter="12">
              <a-col :span="6">
                <a-form-item :label="t('services.protocolLabel')">
                  <a-select
                    :value="entry.value.type ?? 'tcp'"
                    :options="typeOptions"
                    @update:value="(v) => updateField(entry.id, 'type', v === 'tcp' ? undefined : v)"
                  />
                </a-form-item>
              </a-col>
              <a-col :span="18">
                <a-form-item :label="addrLabel" required>
                  <a-input
                    :value="role === 'server' ? entry.value.bind_addr : entry.value.local_addr"
                    :placeholder="addrPlaceholder"
                    @update:value="(v) => updateAddr(entry.id, v)"
                  />
                </a-form-item>
              </a-col>
            </a-row>
            <a-collapse ghost class="advanced-collapse">
              <a-collapse-panel key="adv" :header="t('services.advanced')">
                <a-row :gutter="12">
                  <a-col :span="12">
                    <a-form-item :label="t('services.tokenLabel')" :extra="t('services.tokenExtra')">
                      <a-input-password
                        :value="entry.value.token"
                        autocomplete="off"
                        allow-clear
                        @update:value="(v) => updateField(entry.id, 'token', v || undefined)"
                      />
                    </a-form-item>
                  </a-col>
                  <a-col :span="6">
                    <a-form-item :label="t('services.nodelayLabel')">
                      <a-select
                        :value="entry.value.nodelay"
                        :options="nodelayOptions"
                        allow-clear
                        :placeholder="t('services.nodelayPlaceholder')"
                        @update:value="(v) => updateField(entry.id, 'nodelay', v)"
                      />
                    </a-form-item>
                  </a-col>
                  <a-col v-if="role === 'client'" :span="6">
                    <a-form-item :label="t('services.retryLabel')">
                      <a-input-number
                        :value="(entry.value as any).retry_interval"
                        :min="1"
                        style="width: 100%"
                        :placeholder="t('services.retryPlaceholder')"
                        @update:value="(v) => updateField(entry.id, 'retry_interval', v ?? undefined)"
                      />
                    </a-form-item>
                  </a-col>
                </a-row>
              </a-collapse-panel>
            </a-collapse>
          </a-form>
        </div>
      </div>
    </div>
  </SectionCard>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { DeleteOutlined, PlusOutlined } from "@ant-design/icons-vue";
import SectionCard from "@/components/common/SectionCard.vue";
import type { ClientService, ServerService } from "@/types/rathole";

type Role = "server" | "client";
type Service = ServerService | ClientService;
type Entry = { id: string; name: string; value: Service };

const props = defineProps<{
  modelValue: Record<string, Service>;
  role: Role;
}>();
const emit = defineEmits<{
  (e: "update:modelValue", value: Record<string, Service>): void;
}>();

const { t } = useI18n();

let nextId = 1;
const entries = ref<Entry[]>(toEntries(props.modelValue));

const title = computed(() =>
  props.role === "server" ? t("services.serverTitle") : t("services.clientTitle"),
);
const addrLabel = computed(() =>
  props.role === "server" ? t("services.serverAddrLabel") : t("services.clientAddrLabel"),
);
const addrPlaceholder = computed(() =>
  props.role === "server"
    ? t("services.serverAddrPlaceholder")
    : t("services.clientAddrPlaceholder"),
);

const typeOptions = [
  { label: "TCP", value: "tcp" },
  { label: "UDP", value: "udp" },
];
const nodelayOptions = computed(() => [
  { label: t("common.enable"), value: true },
  { label: t("common.disable"), value: false },
]);

const duplicateNames = computed(() => {
  const seen = new Set<string>();
  const dup = new Set<string>();
  for (const e of entries.value) {
    if (!e.name) continue;
    if (seen.has(e.name)) dup.add(e.name);
    seen.add(e.name);
  }
  return dup;
});

watch(
  () => props.modelValue,
  (next) => {
    if (!equalsCurrent(next)) {
      entries.value = toEntries(next);
    }
  },
  { deep: true },
);

watch(
  entries,
  () => {
    emit("update:modelValue", toRecord(entries.value));
  },
  { deep: true },
);

function toEntries(map: Record<string, Service>): Entry[] {
  return Object.entries(map ?? {}).map(([name, value]) => ({
    id: `s${nextId++}`,
    name,
    value: { ...value },
  }));
}

function toRecord(arr: Entry[]): Record<string, Service> {
  const out: Record<string, Service> = {};
  for (const e of arr) {
    if (!e.name) continue;
    out[e.name] = e.value;
  }
  return out;
}

function equalsCurrent(next: Record<string, Service>): boolean {
  const current = toRecord(entries.value);
  return JSON.stringify(current) === JSON.stringify(next ?? {});
}

function addService() {
  const base = props.role === "server"
    ? ({ bind_addr: "0.0.0.0:0" } as ServerService)
    : ({ local_addr: "127.0.0.1:0" } as ClientService);
  entries.value.push({
    id: `s${nextId++}`,
    name: nextDefaultName(),
    value: base,
  });
}

function nextDefaultName(): string {
  let i = entries.value.length + 1;
  let name = `service_${i}`;
  const taken = new Set(entries.value.map((e) => e.name));
  while (taken.has(name)) {
    i += 1;
    name = `service_${i}`;
  }
  return name;
}

function renameEntry(id: string, value: string) {
  const e = entries.value.find((x) => x.id === id);
  if (e) e.name = value.trim();
}

function updateField(id: string, key: string, value: unknown) {
  const e = entries.value.find((x) => x.id === id);
  if (!e) return;
  const next: any = { ...e.value };
  if (value === undefined || value === null || value === "") {
    delete next[key];
  } else {
    next[key] = value;
  }
  e.value = next;
}

function updateAddr(id: string, value: string) {
  const key = props.role === "server" ? "bind_addr" : "local_addr";
  updateField(id, key, value);
}

function removeEntry(id: string) {
  entries.value = entries.value.filter((e) => e.id !== id);
}
</script>

<style lang="less" scoped>
.empty {
  padding: 28px 0;
  text-align: center;
}

.services {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.service {
  border-color: var(--rl-border);
  box-shadow: none;
}

.service-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid var(--rl-border);
  gap: 12px;
}

.service-head .left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.name-input {
  max-width: 240px;
  font-weight: 500;
}

.service-body {
  padding: 12px 14px 4px;
}

.advanced-collapse :deep(.ant-collapse-header) {
  padding-left: 0;
  font-size: 12px;
  color: var(--rl-text-muted);
}
</style>
