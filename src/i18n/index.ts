import { createI18n } from 'vue-i18n';
import zhCN from '../locales/zh-CN.json';
import en from '../locales/en.json';
import ja from '../locales/ja.json';

/** 当前支持的语言列表. */
export const SUPPORTED_LOCALES = ['zh-CN', 'en', 'ja'] as const;

/** 语言代码类型. */
export type Locale = (typeof SUPPORTED_LOCALES)[number];

/** 语言偏好的 `localStorage` 键. */
export const LOCALE_STORAGE_KEY = 'wishes.locale';

/** 读取已存储的语言偏好, 缺失或非法时回退到简体中文. */
export function getStoredLocale(): Locale {
  const stored = localStorage.getItem(LOCALE_STORAGE_KEY);
  if (stored && (SUPPORTED_LOCALES as readonly string[]).includes(stored)) {
    return stored as Locale;
  }
  return 'zh-CN';
}

export const i18n = createI18n({
  legacy: false,
  locale: getStoredLocale(),
  fallbackLocale: 'en',
  messages: {
    'zh-CN': zhCN,
    en,
    ja,
  },
});

/** 切换语言: 更新 i18n、持久化偏好并同步 `<html lang>`. */
export function setLocale(locale: Locale) {
  i18n.global.locale.value = locale;
  localStorage.setItem(LOCALE_STORAGE_KEY, locale);
  document.documentElement.lang = locale;
}

// 启动时同步 `<html lang>`
document.documentElement.lang = i18n.global.locale.value;
