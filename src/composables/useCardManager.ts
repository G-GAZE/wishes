import { computed, ref } from "vue";
import { CardCreateRequest, CardSummary, CardUpdateRequest, createCard, deleteCard, listCards, updateCard } from "../api/card";
import { Tag } from "../types";


export function useCardManager() {
  const cards = ref<CardSummary[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const searchQuery = ref("");
  const selectedTagFilter = ref<Tag | null>(null);
  const filteredCards = computed(() => {
    let result = cards.value;

    // 文本搜索
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLocaleLowerCase();
      result = cards.value.filter(card => 
        card.content.toLocaleLowerCase().includes(q)
      )
    }

    return result
  })

  async function loadCards() {
    loading.value = true;
    error.value = null;
    try {
      const loaded = await listCards();
      cards.value = loaded.sort((a, b) => b.id - a.id);   // Id 降序排序

    } catch (e) {
      error.value = `Card 加载失败: ${e}`;
    } finally {
      loading.value = false;
    }
  }

  async function create(req: CardCreateRequest): Promise<CardSummary | null> {
    loading.value = true;
    error.value = null;
    try {
      const newCard = await createCard(req);
      cards.value.unshift(newCard);
      return newCard;
    } catch (e) {
      error.value = `创建 Card 失败: ${e}`;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function update(req: CardUpdateRequest): Promise<CardSummary | null> {
    loading.value = true;
    error.value = null;
    try {
      const updated = await updateCard(req);
      const index = cards.value.findIndex(c => c.id === req.id);
      if (index !== -1) {
        cards.value[index] = updated;
      }
      return updated;
    } catch (e) {
      error.value = `更新 Card 失败: ${e}`;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function remove(id: number): Promise<boolean> {
    loading.value = true;
    error.value = null;
    try {
      await deleteCard(id);
      cards.value = cards.value.filter(c => c.id !== id);
      return true;
    } catch (e) {
      error.value = `删除 Card 失败: ${e}`;
      return false;
    } finally {
      loading.value = false;
    }
  }

  const allUsedTags = computed(() => {
    const tagMap = new Map<string, Tag>();
    for (const card of cards.value) {
      for (const tag of card.tags) {
        const key = `${tag.namespace}:${tag.value}`;
        if (!tagMap.has(key)) {
          tagMap.set(key, tag);
        }
      }
    }
    return Array.from(tagMap.values());
  })

  return {
    cards,
    loading,
    error,
    searchQuery,
    filteredCards,
    allUsedTags,
    selectedTagFilter,
    loadCards,
    create,
    update,
    remove,
  }
}