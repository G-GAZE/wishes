import { invoke } from "@tauri-apps/api/core";
import { Tag } from "../types";

export interface CardSummary {
  id: number,
  content: string,
  tags: Tag[],
}

export interface CardCreateRequest {
  content: string,
  tags: Tag[],
}


export interface CardUpdateRequest {
  id: number,
  new_content: string,
  new_tags: Tag[],
}

export async function listCards(): Promise<CardSummary[]> {
  return await invoke<CardSummary[]>("list_cards");
}

export async function listCardsByTags(tags: Tag[]): Promise<CardSummary[]> {
  return await invoke<CardSummary[]>("list_cards_by_tags", { tags });
}

export async function createCard(req: CardCreateRequest): Promise<CardSummary> {
  return await invoke<CardSummary>("create_card", { req });
}

export async function updateCard(req: CardUpdateRequest): Promise<CardSummary> {
  return await invoke<CardSummary>("update_card", { req });
}

export async function deleteCard(id: number): Promise<void> {
  return await invoke("delete_card", { id });
}