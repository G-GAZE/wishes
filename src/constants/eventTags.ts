import { EventTag } from "../types";


export interface EventTagMeta {
  value: EventTag,
  /** i18n 键, 显示名见 `src/locales/*.json` 的 `eventTag.*` */
  labelKey: string,
}


export const EVENT_TAG_META: Record<string, EventTagMeta> = {
  standard: { value: 'standard', labelKey: 'eventTag.standard' },
  up:       { value: 'up',       labelKey: 'eventTag.up' },
  fes:      { value: 'fes',      labelKey: 'eventTag.fes' },
  appoint:  { value: 'appoint',  labelKey: 'eventTag.appoint' },
};

export const EVENT_TAG_OPTIONS: EventTagMeta[] = Object.values(EVENT_TAG_META);

/** 获取活动标签的本地化显示名, 未知标签返回空字符串. */
export function eventTagLabel(t: (key: string) => string, tag: EventTag): string {
  const key = EVENT_TAG_META[tag]?.labelKey;
  return key ? t(key) : '';
}
