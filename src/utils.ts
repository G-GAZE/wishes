import { Tag } from "./types";


export function getTagValue(tags: Tag[], namespace: string): string | null {
  const tag = tags.find((t) => {t.namespace === namespace});
  return tag ? tag.value : null;
}
