<template>
  <a-form layout="vertical" class="transport-form">
    <a-form-item :label="t('transport.protocolLabel')">
      <a-segmented :value="kind" :options="kindOptions" @change="setKind" />
      <p class="rl-muted hint">{{ t("transport.protocolHint") }}</p>
    </a-form-item>

    <a-divider class="divider">{{ t("transport.tcpOptions") }}</a-divider>
    <a-row :gutter="16">
      <a-col v-if="role === 'client'" :span="12">
        <a-form-item :label="t('transport.proxyLabel')" :extra="t('transport.proxyExtra')">
          <a-input
            :value="tcp.proxy"
            :placeholder="t('transport.proxyPlaceholder')"
            allow-clear
            @update:value="(v) => setTcp('proxy', v || undefined)"
          />
        </a-form-item>
      </a-col>
      <a-col :span="6">
        <a-form-item label="TCP_NODELAY">
          <a-select
            :value="tcp.nodelay"
            :options="boolOptions"
            allow-clear
            :placeholder="t('services.nodelayPlaceholder')"
            @update:value="(v) => setTcp('nodelay', v ?? undefined)"
          />
        </a-form-item>
      </a-col>
      <a-col :span="6">
        <a-form-item :label="t('transport.keepaliveSecsLabel')">
          <a-input-number
            :value="tcp.keepalive_secs"
            :min="1"
            placeholder="20"
            style="width: 100%"
            @update:value="(v) => setTcp('keepalive_secs', v ?? undefined)"
          />
        </a-form-item>
      </a-col>
    </a-row>
    <a-form-item
      :label="t('transport.keepaliveIntervalLabel')"
      :extra="t('transport.keepaliveIntervalExtra')"
    >
      <a-input-number
        :value="tcp.keepalive_interval"
        :min="1"
        placeholder="8"
        style="width: 100%"
        @update:value="(v) => setTcp('keepalive_interval', v ?? undefined)"
      />
    </a-form-item>

    <template v-if="kind === 'tls' || (kind === 'websocket' && wsTls)">
      <a-divider class="divider">{{ t("transport.tlsCredentials") }}</a-divider>
      <template v-if="role === 'server'">
        <a-form-item :label="t('transport.pkcs12Label')" required>
          <a-input
            :value="serverTls.pkcs12"
            :placeholder="t('transport.pkcs12PathPlaceholder')"
            @update:value="(v) => setServerTls('pkcs12', v)"
          />
        </a-form-item>
        <a-form-item :label="t('transport.pkcs12PasswordLabel')" required>
          <a-input-password
            :value="serverTls.pkcs12_password"
            autocomplete="off"
            @update:value="(v) => setServerTls('pkcs12_password', v)"
          />
        </a-form-item>
      </template>
      <template v-else>
        <a-form-item :label="t('transport.trustedRootLabel')" required>
          <a-input
            :value="clientTls.trusted_root"
            :placeholder="t('transport.trustedRootPlaceholder')"
            @update:value="(v) => setClientTls('trusted_root', v)"
          />
        </a-form-item>
        <a-form-item :label="t('transport.hostnameLabel')" :extra="t('transport.hostnameExtra')">
          <a-input
            :value="clientTls.hostname"
            :placeholder="t('transport.hostnamePlaceholder')"
            allow-clear
            @update:value="(v) => setClientTls('hostname', v || undefined)"
          />
        </a-form-item>
      </template>
    </template>

    <template v-if="kind === 'noise'">
      <a-divider class="divider">{{ t("transport.noiseOptions") }}</a-divider>
      <a-form-item :label="t('transport.patternLabel')" :extra="t('transport.patternExtra')">
        <a-input
          :value="noise.pattern"
          placeholder="Noise_NK_25519_ChaChaPoly_BLAKE2s"
          allow-clear
          @update:value="(v) => setNoise('pattern', v || undefined)"
        />
      </a-form-item>
      <a-form-item :label="t('transport.privateKeyLabel')">
        <a-input-password
          :value="noise.local_private_key"
          allow-clear
          autocomplete="off"
          @update:value="(v) => setNoise('local_private_key', v || undefined)"
        />
      </a-form-item>
      <a-form-item
        :label="t('transport.publicKeyLabel')"
        :extra="
          role === 'server'
            ? t('transport.publicKeyExtraServer')
            : t('transport.publicKeyExtraClient')
        "
      >
        <a-input-password
          :value="noise.remote_public_key"
          allow-clear
          autocomplete="off"
          @update:value="(v) => setNoise('remote_public_key', v || undefined)"
        />
      </a-form-item>
    </template>

    <template v-if="kind === 'websocket'">
      <a-divider class="divider">{{ t("transport.websocketOptions") }}</a-divider>
      <a-form-item>
        <a-checkbox :checked="wsTls" @change="(e) => setWsTls(!!e.target.checked)">
          {{ t("transport.websocketTls") }}
        </a-checkbox>
        <p class="rl-muted hint">{{ t("transport.websocketTlsHint") }}</p>
      </a-form-item>
    </template>
  </a-form>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type {
  ClientTransport,
  ClientTlsSettings,
  NoiseSettings,
  ServerTransport,
  ServerTlsSettings,
  TcpTransportSettings,
  TransportKind,
  WebsocketSettings,
} from "@/types/rathole";

type Role = "server" | "client";
type Transport = ServerTransport | ClientTransport;

const props = defineProps<{
  modelValue: Transport | undefined;
  role: Role;
}>();
const emit = defineEmits<{
  (e: "update:modelValue", value: Transport | undefined): void;
}>();

const { t } = useI18n();

const kindOptions = [
  { label: "TCP", value: "tcp" },
  { label: "TLS", value: "tls" },
  { label: "Noise", value: "noise" },
  { label: "WebSocket", value: "websocket" },
];

const boolOptions = computed(() => [
  { label: t("common.enable"), value: true },
  { label: t("common.disable"), value: false },
]);

const kind = computed<TransportKind>(() => (props.modelValue?.type ?? "tcp") as TransportKind);
const tcp = computed<TcpTransportSettings>(() => props.modelValue?.tcp ?? {});
const noise = computed<NoiseSettings>(() => props.modelValue?.noise ?? {});
const ws = computed<WebsocketSettings>(() => props.modelValue?.websocket ?? {});
const wsTls = computed<boolean>(() => !!props.modelValue?.websocket?.tls);
const serverTls = computed<ServerTlsSettings>(
  () =>
    (props.modelValue as ServerTransport | undefined)?.tls ?? {
      pkcs12: "",
      pkcs12_password: "",
    },
);
const clientTls = computed<ClientTlsSettings>(
  () =>
    (props.modelValue as ClientTransport | undefined)?.tls ?? { trusted_root: "" },
);

function commit(next: Transport | undefined) {
  if (!next) {
    emit("update:modelValue", undefined);
    return;
  }
  const cleaned: any = {};
  for (const [k, v] of Object.entries(next)) {
    if (v === undefined || v === null) continue;
    if (typeof v === "object" && !Array.isArray(v) && Object.keys(v).length === 0) continue;
    cleaned[k] = v;
  }
  emit("update:modelValue", Object.keys(cleaned).length ? cleaned : undefined);
}

function patch(patchObj: Partial<Transport>) {
  commit({ ...(props.modelValue ?? {}), ...patchObj } as Transport);
}

function setKind(value: string | number) {
  const next = String(value) as TransportKind;
  patch({ type: next === "tcp" ? undefined : next });
}

function setTcp<K extends keyof TcpTransportSettings>(key: K, value: TcpTransportSettings[K]) {
  patch({ tcp: pruned({ ...tcp.value, [key]: value }) });
}

function setNoise<K extends keyof NoiseSettings>(key: K, value: NoiseSettings[K]) {
  patch({ noise: pruned({ ...noise.value, [key]: value }) });
}

function setWsTls(checked: boolean) {
  patch({ websocket: pruned({ ...ws.value, tls: checked || undefined }) });
}

function setServerTls<K extends keyof ServerTlsSettings>(key: K, value: ServerTlsSettings[K]) {
  patch({ tls: { ...serverTls.value, [key]: value } } as Partial<ServerTransport>);
}

function setClientTls<K extends keyof ClientTlsSettings>(key: K, value: ClientTlsSettings[K]) {
  patch({ tls: pruned({ ...clientTls.value, [key]: value }) } as Partial<ClientTransport>);
}

function pruned<T extends Record<string, unknown>>(obj: T): T | undefined {
  const out: any = {};
  for (const [k, v] of Object.entries(obj)) {
    if (v === undefined || v === null || v === "") continue;
    out[k] = v;
  }
  return Object.keys(out).length ? (out as T) : undefined;
}
</script>

<style lang="less" scoped>
.transport-form :deep(.ant-form-item) {
  margin-bottom: 14px;
}

.hint {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.6;
}

.divider {
  margin: 6px 0 14px;
  font-size: 12px;
  color: var(--rl-text-muted);
}
</style>
