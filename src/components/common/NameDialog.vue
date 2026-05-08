<template>
  <a-modal
    :open="open"
    :title="title"
    :ok-text="okText ?? t('common.ok')"
    :cancel-text="t('common.cancel')"
    :confirm-loading="loading"
    @ok="handleOk"
    @cancel="emit('update:open', false)"
    @update:open="(v: boolean) => emit('update:open', v)"
  >
    <a-form layout="vertical">
      <a-form-item
        :label="label"
        :validate-status="error ? 'error' : ''"
        :help="error || hint || t('config.nameExtra')"
      >
        <a-input v-model:value="value" autocomplete="off" allow-clear @press-enter="handleOk" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

const props = withDefaults(
  defineProps<{
    open: boolean;
    title: string;
    label: string;
    initial?: string;
    okText?: string;
    hint?: string;
    submit: (value: string) => Promise<void> | void;
  }>(),
  { initial: "" },
);

const emit = defineEmits<{
  (e: "update:open", value: boolean): void;
}>();

const { t } = useI18n();
const value = ref(props.initial);
const loading = ref(false);
const error = ref("");

watch(
  () => props.open,
  (open) => {
    if (open) {
      value.value = props.initial;
      error.value = "";
    }
  },
);

async function handleOk() {
  const v = value.value.trim();
  if (!/^[A-Za-z0-9._-]{1,64}$/.test(v)) {
    error.value = t("config.nameInvalidLong");
    return;
  }
  error.value = "";
  loading.value = true;
  try {
    await props.submit(v);
    emit("update:open", false);
  } catch (err: any) {
    error.value = typeof err === "string" ? err : err?.message ?? t("config.createFailed");
  } finally {
    loading.value = false;
  }
}
</script>
