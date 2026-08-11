import { Tag } from "./types";

/**
 * 在标签数组中按命名空间查找并返回对应的值.
 * @param tags 标签数组
 * @param namespace 要查找的命名空间
 * @returns 若找到则返回标签值, 否则返回 `null`
 */
export function getTagValue(tags: Tag[], namespace: string): string | null {
  const tag = tags.find((t) => {t.namespace === namespace});
  return tag ? tag.value : null;
}
