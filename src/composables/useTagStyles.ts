import { Tag } from "../types";


const VALUE_SLUG_MAP: Record<string, string> = {
  '火': 'pyro',
  '水': 'hydro',
  '风': 'anemo',
  '雷': 'electro',
  '草': 'dendro',
  '冰': 'cryo',
  '岩': 'geo',
  '单手剑': 'sword',
  '双手剑': 'claymore',
  '长柄武器': 'polearm',
  '法器': 'catalyst',
  '弓': 'bow',
  '毁灭': 'destruction',
  '巡猎': 'hunt',
  '智识': 'erudition',
  '同谐': 'harmony',
  '虚无': 'nihility',
  '存护': 'preservation',
  '丰饶': 'abundance',
  '记忆': 'remembrance',
  '欢愉': 'elation',
};


/**
 * 将标签值转换为安全的仅由字母、数字、连字符构成的 CSS 类名
 * 
 * `TODO[2026-08-17]`: 在前端建立映射表只是暂时的, 之后需要将后端的标签值统一为英文, 并从后端返回可配置的映射
 * 
 * @param value 标签值 (可能为非 ASCII)
 * @returns 映射表中对应的 ASCII 类名, 或使用 unknown 默认值
 */
function getSafeSlug(value: string): string {
  if (VALUE_SLUG_MAP[value]) return VALUE_SLUG_MAP[value];

  return value
    .toLocaleLowerCase()
    .replace(/\s+/g, '-')             // 空格转连字符
    .replace(/[^a-z0-9-]/g, '')       // 只保留字母、数字和连字符
    || 'unknown'                      // 默认值
}


/**
 * 根据标签生成 CSS 类名
 * @param tag 当前标签
 * @param _contextTags 当前对象拥有的所有标签, 由于提取关键上下文
 * @returns 生成的类名
 */
function getTagClass(tag: Tag, _contextTags: Tag[] = []): string {
  //
  // TODO[2026-08-16]: 未来加入游戏差异化逻辑
  //
  // 例如:
  // const gameTag = _contextTags.find(t => t.namespace === 'game');
  // if (gameTag?.value === 'bluearchive' && tag.namespace === 'rarity') {
  //   return `tag-ba-rarity-${tag.value}`;
  // }
  // 当前使用默认实现:
  const slug = getSafeSlug(tag.value);
  return `tag-${tag.namespace}-${slug}`;
}


/**
 * 为多个标签生成类名数组
 * @param tags 当前标签数组
 * @param contextTags 上下文标签数组, 默认使用 `tags`
 * @returns 与每个标签对应的类名数组
 */
export function useTagStyles(tags: Tag[], contextTags?: Tag[]) {
  const finalContext = contextTags ?? tags;
  return tags.map(t => getTagClass(t, finalContext));
}