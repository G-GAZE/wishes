import { EventTag } from "../types";


export interface EventTagMeta {
  value: EventTag,
  /** 中文描述 */
  label: string,
}


export const EVENT_TAG_META: Record<string, EventTagMeta> = {
  standard: { value: 'standard', label: '常驻' },
  up:       { value: 'up',       label: '概率提升' },
  fes:      { value: 'fes',      label: '节日限定' },
  appoint:  { value: 'appoint',  label: '定轨' },
};

export const EVENT_TAG_OPTIONS: EventTagMeta[] = Object.values(EVENT_TAG_META);

export function getEventTagLabel(tag: EventTag): string | null {
  return EVENT_TAG_META[tag]?.label ?? null;
}
