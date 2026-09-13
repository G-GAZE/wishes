import { computed, ref } from "vue";
import { createDeck, CreateDeckRequest, DeckSummary, deleteDeck, listDecks, updateDeck, UpdateDeckRequest } from "../api/decks";


export function useDeckManager() {
  const decks = ref<DeckSummary[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const searchQuery = ref("");
  const filteredDecks = computed(() => {
    let result = decks.value;

    if (searchQuery.value.trim()) {
      const q = searchQuery.value.toLocaleLowerCase();
      result = decks.value.filter(deck => 
        deck.name.toLocaleLowerCase().includes(q)
      )
    }

    return result
  });

  // 获取卡片使用的所有标签可以直接使用 useCardManager 的 allUsedTags

  async function loadDecks() {
    loading.value = true;
    error.value = null;
    try {
      const loaded = await listDecks();
      decks.value = loaded.sort((a, b) => b.id - a.id);
    } catch (e) {
      error.value = `Deck 加载失败: ${e}`;
    } finally {
      loading.value = false;
    }
  }

  function getDeckById(id: number): DeckSummary | undefined {
    return decks.value.find(d => d.id === id);
  }

  async function create(req: CreateDeckRequest): Promise<DeckSummary | null> {
    loading.value = true;
    error.value = null;
    try {
      const newDeck = await createDeck(req);
      decks.value.unshift(newDeck);
      return newDeck
    } catch (e) {
      error.value = `创建 Deck 失败: ${e}`;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function update(req: UpdateDeckRequest): Promise<DeckSummary | null> {
    loading.value = true;
    error.value = null;
    try {
      const updated = await updateDeck(req);
      const index = decks.value.findIndex(d => d.id === req.id);
      if (index !== -1) {
        decks.value[index] = updated;
      }
      return updated;
    } catch (e) {
      error.value = `更新 Deck 失败: ${e}`;
      return null;
    } finally {
      loading.value = false;
    }
  }

  async function remove(id: number): Promise<boolean> {
    loading.value = true;
    error.value = null;
    try {
      await deleteDeck(id);
      decks.value = decks.value.filter(d => d.id !== id);
      return true;
    } catch (e) {
      error.value = `删除 Deck 失败: ${e}`;
      return false;
    } finally {
      loading.value = false;
    }
  }

  return {
    decks,
    searchQuery,
    filteredDecks,
    loading,
    error,
    loadDecks,
    getDeckById,
    create,
    update,
    remove
  }
}