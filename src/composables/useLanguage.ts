import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { api } from "@/api/tauri";
import {
  SUPPORTED_LOCALES,
  persistLocale,
  type LocaleCode,
} from "@/i18n";

export function useLanguage() {
  const { locale } = useI18n();

  const current = computed<LocaleCode>(() => locale.value as LocaleCode);

  async function setLanguage(next: LocaleCode, persist = true) {
    if (locale.value === next) return;
    locale.value = next;
    persistLocale(next);
    if (persist) {
      try {
        await api.updateSettings({ language: next });
      } catch {
        // best-effort: backend persistence failure should not break the UI switch
      }
    }
  }

  return { current, supported: SUPPORTED_LOCALES, setLanguage };
}
