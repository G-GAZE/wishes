/**
 * 标签类型, 与后端 `Tag` 结构对应
 */
export interface Tag {
  namespace: string,
  value: string,
}

/**
 * EventTag 为 string 的活动标签语义化别名
 */
export type EventTag = string;

/**
 * 单次抽卡响应, 包含卡片内容和标签信息
 */
export interface WishResponse {
  id: number,
  content: string,
  tags: Tag[],
  event_tags: string[],
}

/**
 * 卡池摘要信息, 用于列表展示
 */
export interface BannerSummary {
  id: number,
  name: string,
  tags: Tag[],
}

/**
 * 卡池详细信息, 包含关联资源和抽卡统计
 */
export interface BannerInfo {
  id: number,
  name: string,
  tags: Tag[],
  deck_name: string,
  logic_name: string,
  total_counter: number,
}

/**
 * 图鉴统计信息
 */
export interface CatalogStats {
  cards: number,
  decks: number,
  logics: number,
}