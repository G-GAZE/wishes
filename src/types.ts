export interface Tag {
  namespace: string,
  value: string,
}

export interface WishResponse {
  id: number,
  content: string,
  tags: Tag[],
  event_tags: string[],
}

export interface BannerSummary {
  id: number,
  name: string,
  tags: Tag[],
}

export interface BannerInfo {
  id: number,
  name: string,
  tags: Tag[],
  deck_name: string,
  logic_name: string,
  total_counter: number,
}

export interface CatalogStats {
  cards: number,
  decks: number,
  logics: number,
}