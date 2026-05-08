import { i18n } from "@/i18n";
import type { Mode } from "@/types/rathole";

export const instanceKey = (mode: Mode, name: string): string => `${mode}:${name}`;

export const isLikelyAddress = (value: string): boolean =>
  /^[\w.\-:[\]]+:\d{1,5}$/.test(value);

export const formatRelativeTime = (iso?: string): string => {
  if (!iso) return "";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return iso;
  const t = i18n.global.t;
  const diff = Date.now() - date.getTime();
  if (diff < 60_000) return t("time.justNow");
  if (diff < 3_600_000) return t("time.minutesAgo", { n: Math.floor(diff / 60_000) });
  if (diff < 86_400_000) return t("time.hoursAgo", { n: Math.floor(diff / 3_600_000) });
  return date.toLocaleString();
};
