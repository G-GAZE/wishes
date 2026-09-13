import { invoke } from "@tauri-apps/api/core";
import { EventTag, Tag } from "../types";

export type MembershipCondition =
  | { type: "tag_all", tags: Tag[] }
  | { type: "tag_any", tags: Tag[] }
  | { type: "filter_tag_all", tags: Tag[] }
  | { type: "filter_tag_any", tags: Tag[] }
  | { type: "include_ids", ids: number[] }
  | { type: "exclude_ids", ids: number[] };

export interface Membership {
  conditions: MembershipCondition[],
}

export type EventGroupCondition =
  | { type: "all" }
  | { type: "tag_all", tags: Tag[] }
  | { type: "tag_any", tags: Tag[] }
  | { type: "filter_tag_all", tags: Tag[] }
  | { type: "filter_tag_any", tags: Tag[] }
  | { type: "include_ids", ids: number[] }
  | { type: "exclude_ids", ids: number[] }
  // 以下规则未正式实现, 但保留声明
  | { type: "include_groups", groups: EventTag[] }
  | { type: "exclude_groups", groups: EventTag[] };

export interface EventGroup {
  conditions: EventGroupCondition[],
}

export interface DeckSummary {
  id: number,
  name: string,
  members: Membership,
  event_groups: Record<EventTag, EventGroup>
  tags: Tag[],
}

export interface CreateDeckRequest {
  name: string,
  members: Membership,
  event_groups: Record<EventTag, EventGroup>,
  tags: Tag[],
}

export interface UpdateDeckRequest {
  id: number,
  name?: string,
  members?: Membership,
  event_groups?: Record<EventTag, EventGroup>,
  tags?: Tag[],
}

export type EditableCondition = MembershipCondition | EventGroupCondition;

export async function listDecks(): Promise<DeckSummary[]> {
  return await invoke("list_decks");
}

export async function createDeck(req: CreateDeckRequest): Promise<DeckSummary> {
  return await invoke("create_deck", { req })
}

export async function updateDeck(req: UpdateDeckRequest): Promise<DeckSummary> {
  return await invoke('update_deck', { req });
}

export async function deleteDeck(id: number): Promise<void> {
  return await invoke('delete_deck', { id });
}