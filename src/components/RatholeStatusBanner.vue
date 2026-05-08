<template>
  <div v-if="banner" class="rl-card banner" :class="`tone-${banner.tone}`">
    <div class="content">
      <component :is="banner.icon" class="icon" />
      <div class="text">
        <div class="title">{{ banner.title }}</div>
        <div class="detail">{{ banner.detail }}</div>
        <div v-if="updaterStore.downloading" class="progress">
          <a-progress
            class="progress-bar"
            :percent="updaterStore.downloadPercent ?? 0"
            :show-info="updaterStore.downloadPercent !== null"
            size="small"
            :status="updaterStore.downloadPercent === 100 ? 'success' : 'active'"
          />
          <span v-if="bytesText" class="rl-muted bytes">{{ bytesText }}</span>
        </div>
      </div>
    </div>
    <div v-if="banner.actions.length" class="actions">
      <a-button
        v-for="action in banner.actions"
        :key="action.key"
        :type="action.primary ? 'primary' : 'default'"
        :loading="action.loading"
        :disabled="action.disabled"
        @click="action.handler"
      >
        {{ action.label }}
      </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useRoute, useRouter } from "vue-router";
import { message } from "antdv-next";
import {
  CheckCircleFilled,
  CloudDownloadOutlined,
  DisconnectOutlined,
  ExclamationCircleFilled,
  InfoCircleFilled,
} from "@antdv-next/icons";
import { useUpdaterStore } from "@/stores/updater";
import { useAppStore } from "@/stores/app";

type Tone = "warning" | "info" | "success" | "danger";
type ActionDef = {
  key: string;
  label: string;
  primary?: boolean;
  loading?: boolean;
  disabled?: boolean;
  handler: () => void;
};

const updaterStore = useUpdaterStore();
const appStore = useAppStore();
const router = useRouter();
const route = useRoute();
const { t } = useI18n();

const onSettingsPage = computed(() => route.name === "settings");

function settingsAction(): ActionDef {
  if (onSettingsPage.value) {
    return {
      key: "retry",
      label: t("updater.retryCheck"),
      handler: () => updaterStore.check(),
      loading: updaterStore.checking,
    };
  }
  return {
    key: "settings",
    label: t("sidebar.settings"),
    handler: () => router.push({ name: "settings" }),
  };
}

onMounted(() => {
  updaterStore.bindProgress();
});

onBeforeUnmount(() => {
  updaterStore.unbindProgress();
});

const bytesText = computed(() => {
  const p = updaterStore.progress;
  if (!p) return "";
  const fmt = (n: number) => `${(n / 1024 / 1024).toFixed(1)} MB`;
  if (p.total) return `${fmt(p.downloaded)} / ${fmt(p.total)}`;
  return fmt(p.downloaded);
});

async function downloadNow() {
  try {
    await updaterStore.download();
    await appStore.refresh();
    message.success(t("updater.downloadSuccess"));
  } catch (err: any) {
    const text = typeof err === "string" ? err : err?.message ?? t("updater.downloadFailed");
    message.error(text);
  }
}

const banner = computed(() => {
  const result = updaterStore.result;
  if (!result) return null;

  if (updaterStore.dismissed && result.binary_present) return null;

  const target = result.asset?.target;
  const installed = result.installed_version;
  const latest = result.latest_version;

  // 1) Binary missing
  if (!result.binary_present) {
    if (!result.github_reachable) {
      const acts: ActionDef[] = [
        { key: "retry", label: t("updater.retryCheck"), handler: () => updaterStore.check(), loading: updaterStore.checking },
      ];
      if (!onSettingsPage.value) {
        acts.push({ key: "settings", label: t("sidebar.settings"), handler: () => router.push({ name: "settings" }) });
      }
      return {
        tone: "warning" as Tone,
        icon: DisconnectOutlined,
        title: t("updater.missingTitle"),
        detail: t("updater.missingUnreachable", {
          path: appStore.info?.rathole_path ?? "",
        }),
        actions: acts,
      };
    }
    if (!result.asset) {
      return {
        tone: "warning" as Tone,
        icon: ExclamationCircleFilled,
        title: t("updater.missingTitle"),
        detail: t("updater.noMatchingAsset", { os: navigatorPlatform() }),
        actions: [settingsAction()],
      };
    }
    const acts: ActionDef[] = [
      {
        key: "download",
        label: updaterStore.downloading ? t("updater.downloading") : t("updater.download"),
        primary: true,
        loading: updaterStore.downloading,
        disabled: updaterStore.downloading,
        handler: downloadNow,
      },
    ];
    if (!onSettingsPage.value) {
      acts.push({
        key: "settings",
        label: t("updater.specifyManually"),
        handler: () => router.push({ name: "settings" }),
        disabled: updaterStore.downloading,
      });
    }
    const isRosettaFallback = !!target && target.includes("Rosetta");
    const detail = isRosettaFallback
      ? `${t("updater.downloadOffer", { version: latest ?? "" })} ${t("updater.rosettaNote")}`
      : t("updater.downloadOffer", { version: latest ?? "" });
    return {
      tone: "warning" as Tone,
      icon: CloudDownloadOutlined,
      title: t("updater.missingTitle"),
      detail,
      actions: acts,
    };
  }

  // 2) Binary present but update available
  if (result.update_available && result.asset) {
    return {
      tone: "info" as Tone,
      icon: InfoCircleFilled,
      title: t("updater.updateTitle", { version: latest ?? "" }),
      detail: t("updater.updateDetail", {
        installed: installed ?? t("updater.unknown"),
        latest: latest ?? "",
      }),
      actions: [
        {
          key: "upgrade",
          label: updaterStore.downloading ? t("updater.downloading") : t("updater.upgrade"),
          primary: true,
          loading: updaterStore.downloading,
          disabled: updaterStore.downloading,
          handler: downloadNow,
        } as ActionDef,
        {
          key: "later",
          label: t("updater.later"),
          handler: () => updaterStore.dismiss(),
          disabled: updaterStore.downloading,
        } as ActionDef,
      ],
    };
  }

  // 3) Binary present, GitHub unreachable - tiny notice
  if (!result.github_reachable) {
    return {
      tone: "info" as Tone,
      icon: DisconnectOutlined,
      title: t("updater.unreachableTitle"),
      detail: t("updater.unreachableDetail"),
      actions: [
        { key: "retry", label: t("updater.retryCheck"), handler: () => updaterStore.check(), loading: updaterStore.checking } as ActionDef,
      ],
    };
  }

  return null;
});

function navigatorPlatform(): string {
  return typeof navigator !== "undefined" ? navigator.platform : "";
}
</script>

<style lang="less" scoped>
.banner {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 14px 16px;
  border-color: var(--rl-border);
}

.tone-warning {
  background: rgba(254, 246, 232, 0.7);
  border-color: rgba(217, 119, 6, 0.4);
}

.tone-info {
  background: rgba(239, 246, 255, 0.7);
  border-color: rgba(37, 99, 235, 0.3);
}

.tone-success {
  background: rgba(240, 253, 244, 0.6);
  border-color: rgba(21, 163, 74, 0.3);
}

.tone-danger {
  background: rgba(254, 242, 242, 0.7);
  border-color: rgba(220, 38, 38, 0.3);
}

.content {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  flex: 1;
}

.icon {
  font-size: 18px;
  margin-top: 3px;
}

.tone-warning .icon {
  color: var(--rl-warning);
}

.tone-info .icon {
  color: var(--rl-accent);
}

.tone-danger .icon {
  color: var(--rl-danger);
}

.tone-success .icon {
  color: var(--rl-success);
}

.text {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.title {
  font-weight: 600;
}

.detail {
  font-size: 13px;
  color: var(--rl-text-muted);
  line-height: 1.6;
  word-break: break-word;
}

.progress {
  margin-top: 8px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.progress-bar {
  flex: 1;
  min-width: 0;
}

.bytes {
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  white-space: nowrap;
  flex-shrink: 0;
}

.actions {
  display: flex;
  gap: 8px;
  align-items: flex-start;
  flex-shrink: 0;
}
</style>
