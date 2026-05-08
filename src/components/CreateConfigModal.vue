<template>
  <a-modal
    :open="open"
    :title="title"
    :confirm-loading="submitting"
    :ok-text="t('common.create')"
    :cancel-text="t('common.cancel')"
    :ok-button-props="{ disabled: !canSubmit }"
    :mask-closable="false"
    :width="entryMode === 'toml' ? 640 : 520"
    @ok="handleOk"
    @cancel="handleCancel"
    @update:open="(v) => $emit('update:open', v)"
  >
    <div v-if="mode === 'client'" class="entry-tabs">
      <a-segmented v-model:value="entryMode" :options="entryOptions" block />
    </div>

    <a-form
      v-if="entryMode === 'manual'"
      layout="vertical"
      :model="manualState"
      ref="manualFormRef"
      :rules="manualRules"
    >
      <a-form-item :label="t('config.nameLabel')" name="name" :extra="t('config.nameExtra')">
        <a-input
          v-model:value="manualState.name"
          :placeholder="t('config.namePlaceholder')"
          autocomplete="off"
          allow-clear
        />
      </a-form-item>
      <template v-if="mode === 'server'">
        <a-form-item :label="t('config.bindAddrLabel')" name="bindAddr">
          <a-input v-model:value="manualState.bindAddr" placeholder="0.0.0.0:2333" />
        </a-form-item>
      </template>
      <template v-else>
        <a-form-item :label="t('config.remoteAddrLabel')" name="remoteAddr">
          <a-input v-model:value="manualState.remoteAddr" placeholder="example.com:2333" />
        </a-form-item>
      </template>
      <a-form-item
        :label="t('config.defaultTokenLabel')"
        name="defaultToken"
        :extra="t('config.defaultTokenExtra')"
      >
        <a-input
          v-model:value="manualState.defaultToken"
          :placeholder="t('config.defaultTokenPlaceholder')"
          allow-clear
        />
      </a-form-item>
    </a-form>

    <div v-else class="toml-mode">
      <a-form layout="vertical">
        <a-form-item :label="t('config.tomlLabel')">
          <a-textarea
            v-model:value="tomlState.content"
            :rows="10"
            :placeholder="t('config.tomlPlaceholder')"
            class="toml-textarea"
          />
        </a-form-item>
        <div class="parse-row">
          <a-button type="primary" :loading="tomlState.parsing" @click="parseToml">
            {{ tomlState.parsed ? t("config.tomlReparse") : t("config.tomlParse") }}
          </a-button>
          <span v-if="tomlState.parseError" class="parse-error">
            {{ tomlState.parseError }}
          </span>
        </div>
      </a-form>

      <div v-if="tomlState.parsed" class="parsed">
        <a-divider class="divider">{{ t("config.serviceMappingTitle") }}</a-divider>

        <a-form layout="vertical">
          <a-form-item :label="t('config.nameLabel')" required :extra="t('config.nameExtra')">
            <a-input
              v-model:value="tomlState.name"
              :placeholder="t('config.namePlaceholder')"
              autocomplete="off"
              allow-clear
            />
          </a-form-item>

          <a-form-item :label="t('config.remoteAddrLabel')" required :extra="t('config.remoteAddrHelp')">
            <a-input v-model:value="tomlState.remoteAddr" placeholder="example.com:2333" />
          </a-form-item>

          <a-form-item :label="t('config.tokenLabel')">
            <a-input-password
              v-model:value="tomlState.defaultToken"
              :placeholder="t('config.tokenPlaceholder')"
              autocomplete="off"
              allow-clear
            />
          </a-form-item>
        </a-form>

        <div v-if="tomlState.transport" class="transport-info">
          <div class="transport-row">
            <span class="rl-muted">{{ t("config.transportDetected") }}</span>
            <a-tag color="blue" class="transport-tag">
              {{ (tomlState.transport.type ?? "tcp").toUpperCase() }}
            </a-tag>
          </div>
          <p v-if="tomlState.transportNotice" class="transport-notice">
            <ExclamationCircleFilled class="notice-icon" />
            <span>{{ tomlState.transportNotice }}</span>
          </p>
        </div>

        <div class="services-block">
          <p class="rl-muted hint">{{ t("config.serviceMappingHint") }}</p>
          <div v-if="!tomlState.serviceMappings.length" class="rl-muted empty">
            {{ t("config.serviceMappingEmpty") }}
          </div>
          <div
            v-for="svc in tomlState.serviceMappings"
            :key="svc.name"
            class="service-row"
          >
            <a-tag class="svc-name" color="blue">{{ svc.name }}</a-tag>
            <a-tooltip v-if="svc.token" :title="t('config.serviceHasOwnToken')">
              <LockOutlined class="svc-lock" />
            </a-tooltip>
            <span class="svc-bind rl-mono rl-muted">
              {{ svc.bind_addr || "—" }}
            </span>
            <span class="arrow">→</span>
            <a-input
              v-model:value="svc.local_addr"
              :placeholder="t('config.localAddrPlaceholder')"
              class="local-input"
            />
          </div>
        </div>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { message } from "antdv-next";
import type { FormInstance } from "antdv-next";
import type { Rule } from "antdv-next/dist/form/types";
import { ExclamationCircleFilled, LockOutlined } from "@antdv-next/icons";
import { api } from "@/api/tauri";
import { useConfigStore } from "@/stores/configs";
import type {
  ClientConfig,
  ClientService,
  ClientTransport,
  Mode,
  ServerConfig,
} from "@/types/rathole";

type EntryMode = "manual" | "toml";

interface ServiceMapping {
  name: string;
  bind_addr: string;
  local_addr: string;
  token?: string;
}

const props = defineProps<{ open: boolean; mode: Mode }>();
const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
  (e: "created", name: string): void;
}>();

const { t } = useI18n();
const manualFormRef = ref<FormInstance>();
const submitting = ref(false);
const configStore = useConfigStore();

const entryMode = ref<EntryMode>("manual");
const entryOptions = computed(() => [
  { label: t("config.entryManual"), value: "manual" },
  { label: t("config.entryFromToml"), value: "toml" },
]);

const manualState = reactive({
  name: "",
  bindAddr: "0.0.0.0:2333",
  remoteAddr: "",
  defaultToken: "",
});

const tomlState = reactive({
  content: "",
  parsing: false,
  parseError: "",
  parsed: null as ServerConfig | null,
  name: "",
  remoteAddr: "",
  defaultToken: "",
  serviceMappings: [] as ServiceMapping[],
  transport: undefined as ClientTransport | undefined,
  transportNotice: "",
});

const title = computed(() =>
  props.mode === "server"
    ? t("config.createServerTitle")
    : t("config.createClientTitle"),
);

const manualRules = computed<Record<string, Rule[]>>(() => ({
  name: [
    { required: true, message: t("config.nameRequired") },
    {
      pattern: /^[A-Za-z0-9._-]{1,64}$/,
      message: t("config.nameInvalid"),
    },
  ],
  bindAddr: [{ required: true, message: t("config.bindAddrRequired") }],
  remoteAddr: [{ required: true, message: t("config.remoteAddrRequired") }],
}));

const canSubmit = computed(() => {
  if (entryMode.value === "manual") return true;
  if (!tomlState.parsed) return false;
  if (!tomlState.name.trim() || !tomlState.remoteAddr.trim()) return false;
  return true;
});

watch(
  () => props.open,
  (open) => {
    if (open) {
      entryMode.value = "manual";
      Object.assign(manualState, {
        name: "",
        bindAddr: "0.0.0.0:2333",
        remoteAddr: "",
        defaultToken: "",
      });
      Object.assign(tomlState, {
        content: "",
        parsing: false,
        parseError: "",
        parsed: null,
        name: "",
        remoteAddr: "",
        defaultToken: "",
        serviceMappings: [],
        transport: undefined,
        transportNotice: "",
      });
    }
  },
);

watch(
  () => props.mode,
  (m) => {
    if (m === "server") entryMode.value = "manual";
  },
);

async function parseToml() {
  if (!tomlState.content.trim()) {
    tomlState.parseError = t("config.tomlEmpty");
    return;
  }
  tomlState.parsing = true;
  tomlState.parseError = "";
  try {
    const cfg = await api.parseServerToml(tomlState.content);
    tomlState.parsed = cfg;
    tomlState.remoteAddr = cfg.server.bind_addr;
    tomlState.defaultToken = cfg.server.default_token ?? "";
    tomlState.serviceMappings = Object.entries(cfg.server.services ?? {}).map(
      ([name, svc]) => ({
        name,
        bind_addr: svc.bind_addr ?? "",
        local_addr: "127.0.0.1:0",
        token: svc.token,
      }),
    );
    const mapped = mapServerTransport(cfg);
    tomlState.transport = mapped.transport;
    tomlState.transportNotice = mapped.notice;
  } catch (err: any) {
    tomlState.parsed = null;
    tomlState.serviceMappings = [];
    tomlState.transport = undefined;
    tomlState.transportNotice = "";
    const msg = typeof err === "string" ? err : err?.message ?? String(err);
    tomlState.parseError = t("config.tomlParseFailed", { message: msg });
  } finally {
    tomlState.parsing = false;
  }
}

function mapServerTransport(cfg: ServerConfig): {
  transport: ClientTransport | undefined;
  notice: string;
} {
  const srv = cfg.server.transport;
  if (!srv || !srv.type || srv.type === "tcp") {
    return { transport: undefined, notice: "" };
  }
  const transport: ClientTransport = { type: srv.type };
  let notice = "";

  if (srv.type === "websocket") {
    if (srv.websocket?.tls !== undefined) {
      transport.websocket = { tls: srv.websocket.tls };
    }
    if (srv.websocket?.tls) {
      notice = t("config.transportNeedsTls");
    }
  } else if (srv.type === "tls") {
    transport.tls = { trusted_root: "" };
    notice = t("config.transportNeedsTls");
  } else if (srv.type === "noise") {
    if (srv.noise?.pattern) {
      transport.noise = { pattern: srv.noise.pattern };
    } else {
      transport.noise = {};
    }
    notice = t("config.transportNeedsNoise");
  }
  return { transport, notice };
}

async function handleOk() {
  if (entryMode.value === "manual") {
    await submitManual();
  } else {
    await submitFromToml();
  }
}

async function submitManual() {
  try {
    await manualFormRef.value?.validate();
  } catch {
    return;
  }
  submitting.value = true;
  try {
    if (props.mode === "server") {
      await api.saveServerConfig(manualState.name, {
        server: {
          bind_addr: manualState.bindAddr,
          default_token: manualState.defaultToken || undefined,
          services: {},
        },
      });
    } else {
      await api.saveClientConfig(manualState.name, {
        client: {
          remote_addr: manualState.remoteAddr,
          default_token: manualState.defaultToken || undefined,
          services: {},
        },
      });
    }
    await configStore.refresh(props.mode);
    message.success(t("config.created"));
    emit("created", manualState.name);
    emit("update:open", false);
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("config.createFailed"));
  } finally {
    submitting.value = false;
  }
}

async function submitFromToml() {
  if (!tomlState.parsed) {
    message.error(t("config.tomlNotParsed"));
    return;
  }
  if (!/^[A-Za-z0-9._-]{1,64}$/.test(tomlState.name.trim())) {
    message.error(t("config.nameInvalid"));
    return;
  }
  if (!tomlState.remoteAddr.trim()) {
    message.error(t("config.remoteAddrRequired"));
    return;
  }

  const services: Record<string, ClientService> = {};
  for (const m of tomlState.serviceMappings) {
    if (!m.local_addr.trim()) {
      message.error(t("config.fillAllServices"));
      return;
    }
    const svc: ClientService = { local_addr: m.local_addr.trim() };
    if (m.token) svc.token = m.token;
    services[m.name] = svc;
  }

  const config: ClientConfig = {
    client: {
      remote_addr: tomlState.remoteAddr.trim(),
      default_token: tomlState.defaultToken || undefined,
      transport: tomlState.transport,
      services,
    },
  };

  submitting.value = true;
  try {
    await api.saveClientConfig(tomlState.name.trim(), config);
    await configStore.refresh("client");
    message.success(t("config.created"));
    emit("created", tomlState.name.trim());
    emit("update:open", false);
  } catch (err: any) {
    message.error(typeof err === "string" ? err : err?.message ?? t("config.createFailed"));
  } finally {
    submitting.value = false;
  }
}

function handleCancel() {
  emit("update:open", false);
}
</script>

<style lang="less" scoped>
.entry-tabs {
  margin-bottom: 16px;
}

.toml-mode :deep(.ant-form-item) {
  margin-bottom: 14px;
}

.toml-textarea {
  font-family: ui-monospace, "SF Mono", Menlo, Monaco, "JetBrains Mono",
    Consolas, monospace;
  font-size: 12px;
  line-height: 1.55;
}

.parse-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 4px;
}

.parse-error {
  color: var(--rl-danger);
  font-size: 12px;
}

.divider {
  margin: 18px 0 14px !important;
  font-size: 13px;
  color: var(--rl-text-muted);
}

.services-block {
  margin-top: 8px;
}

.hint {
  font-size: 12px;
  margin: 0 0 10px;
}

.empty {
  text-align: center;
  padding: 12px 0;
  font-size: 13px;
}

.service-row {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 0;
}

.svc-name {
  flex-shrink: 0;
  margin: 0;
}

.svc-lock {
  color: var(--rl-text-muted);
  flex-shrink: 0;
  font-size: 12px;
}

.transport-info {
  margin: 4px 0 12px;
  padding: 10px 12px;
  border: 1px solid var(--rl-border);
  border-radius: var(--rl-radius-sm);
  background: rgba(37, 99, 235, 0.04);
}

.transport-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.transport-tag {
  margin: 0;
  font-size: 11px;
}

.transport-notice {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  margin: 8px 0 0;
  font-size: 12px;
  color: var(--rl-warning);
  line-height: 1.5;
}

.notice-icon {
  font-size: 13px;
  margin-top: 2px;
  flex-shrink: 0;
}

.svc-bind {
  flex-shrink: 0;
  font-size: 11px;
  min-width: 110px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.arrow {
  color: var(--rl-text-muted);
  flex-shrink: 0;
}

.local-input {
  flex: 1;
}
</style>
