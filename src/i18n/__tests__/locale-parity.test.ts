import { describe, expect, it } from 'vitest';
import zhCN from '../../locales/zh-CN.json';
import en from '../../locales/en.json';
import ja from '../../locales/ja.json';

/** 递归收集对象的全部键路径 (如 `settings.error.loadPaths`). */
function collectKeys(value: unknown, prefix = ''): string[] {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return prefix ? [prefix] : [];
  }
  return Object.entries(value as Record<string, unknown>)
    .flatMap(([key, child]) => collectKeys(child, prefix ? `${prefix}.${key}` : key));
}

describe('locale files', () => {
  it('en 与 zh-CN 的键集合完全一致', () => {
    expect(collectKeys(en).sort()).toEqual(collectKeys(zhCN).sort());
  });

  it('ja 与 zh-CN 的键集合完全一致', () => {
    expect(collectKeys(ja).sort()).toEqual(collectKeys(zhCN).sort());
  });
});
