<template>
  <a-modal
    :open="open"
    :title="title"
    :confirm-loading="submitting"
    :ok-text="t('common.create')"
    :cancel-text="t('common.cancel')"
    :mask-closable="false"
    @ok="handleOk"
    @cancel="handleCancel"
    @update:open="(v) => $emit('update:open', v)"
  >
    <a-form layout="vertical" :model="formState" ref="formRef" :rules="rules">
      <a-form-item :label="t('config.nameLabel')" name="name" :extra="t('config.nameExtra')">
        <a-input
          v-model:value="formState.name"
          :placeholder="t('config.namePlaceholder')"
          autocomplete="off"
          allow-clear
        />
      </a-form-item>
      <template v-if="mode === 'server'">
        <a-form-item :label="t('config.bindAddrLabel')" name="bindAddr">
          <a-input v-model:value="formState.bindAddr" placeholder="0.0.0.0:2333" />
        </a-form-item>
      </template>
      <template v-else>
        <a-form-item :label="t('config.remoteAddrLabel')" name="remoteAddr">
          <a-input v-model:value="formState.remoteAddr" placeholder="example.com:2333" />
        </a-form-item>
      </template>
      <a-form-item
        :label="t('config.defaultTokenLabel')"
        name="defaultToken"
        :extra="t('config.defaultTokenExtra')"
      >
        <a-input
          v-model:value="formState.defaultToken"
          :placeholder="t('config.defaultTokenPlaceholder')"
          allow-clear
        />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { message } from "ant-design-vue";
import type { FormInstance, Rule } from "ant-design-vue/es/form";
import { api } from "@/api/tauri";
import { useConfigStore } from "@/stores/configs";
import type { Mode } from "@/types/rathole";

const props = defineProps<{ open: boolean; mode: Mode }>();
const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
  (e: "created", name: string): void;
}>();

const { t } = useI18n();
const formRef = ref<FormInstance>();
const submitting = ref(false);
const configStore = useConfigStore();

const formState = reactive({
  name: "",
  bindAddr: "0.0.0.0:2333",
  remoteAddr: "",
  defaultToken: "",
});

const title = computed(() =>
  props.mode === "server"
    ? t("config.createServerTitle")
    : t("config.createClientTitle"),
);

const rules = computed<Record<string, Rule[]>>(() => ({
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

watch(
  () => props.open,
  (open) => {
    if (open) {
      formState.name = "";
      formState.bindAddr = "0.0.0.0:2333";
      formState.remoteAddr = "";
      formState.defaultToken = "";
    }
  },
);

async function handleOk() {
  try {
    await formRef.value?.validate();
  } catch {
    return;
  }
  submitting.value = true;
  try {
    if (props.mode === "server") {
      await api.saveServerConfig(formState.name, {
        server: {
          bind_addr: formState.bindAddr,
          default_token: formState.defaultToken || undefined,
          services: {},
        },
      });
    } else {
      await api.saveClientConfig(formState.name, {
        client: {
          remote_addr: formState.remoteAddr,
          default_token: formState.defaultToken || undefined,
          services: {},
        },
      });
    }
    await configStore.refresh(props.mode);
    message.success(t("config.created"));
    emit("created", formState.name);
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
