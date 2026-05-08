import { createI18n } from "vue-i18n";
import zhCN from "./locales/zh-CN.json";
import en from "./locales/en.json";
import ja from "./locales/ja.json";
import ko from "./locales/ko.json";

export const SUPPORTED_LOCALES = ["zh-CN", "en", "ja", "ko"] as const;
export type LocaleCode = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: LocaleCode = "zh-CN";
const STORAGE_KEY = "rathole-launcher.locale";

const messages = {
  "zh-CN": zhCN,
  en,
  ja,
  ko,
};

export function detectInitialLocale(): LocaleCode {
  try {
    const stored = window.localStorage.getItem(STORAGE_KEY);
    if (stored && (SUPPORTED_LOCALES as readonly string[]).includes(stored)) {
      return stored as LocaleCode;
    }
  } catch {
    // ignore — storage may be unavailable
  }
  const browser = navigator.language || (navigator as any).userLanguage || "";
  if (browser.toLowerCase().startsWith("zh")) return "zh-CN";
  if (browser.toLowerCase().startsWith("ja")) return "ja";
  if (browser.toLowerCase().startsWith("ko")) return "ko";
  if (browser.toLowerCase().startsWith("en")) return "en";
  return DEFAULT_LOCALE;
}

export function persistLocale(locale: LocaleCode) {
  try {
    window.localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // ignore
  }
}

export const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: detectInitialLocale(),
  fallbackLocale: "en",
  messages,
});

export type I18n = typeof i18n;
export type Messages = typeof zhCN;
