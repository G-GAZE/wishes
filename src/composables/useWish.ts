import { ref } from "vue";
import { BannerInfo, BannerSummary, WishResponse } from "../types";
import { invoke } from "@tauri-apps/api/core";


export function useWish() {
  const banners = ref<BannerSummary[]>([]);
  const currentBannerInfo = ref<BannerInfo | null>(null);
  const currentWish = ref<WishResponse | null>(null);
  const history = ref<WishResponse[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadBanners() {
    try {
      banners.value = await invoke<BannerSummary[]>("get_banners");
      if (banners.value.length > 0) {
        await loadBannerInfo(banners.value[0].id);
      }
    } catch (e) {
      error.value = `加载卡池列表失败: ${e}`;
    }
  }

  async function loadBannerInfo(bannerId: number) {
    try {
      currentBannerInfo.value = await invoke<BannerInfo>("get_banner_info", { bannerId });
    } catch (e) {
      error.value = `加载卡池信息失败: ${e}`;
    }
  }

  async function doWish(bannerId: number) {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<WishResponse>("wish", { bannerId });
      currentWish.value = result;
      history.value.unshift(currentWish.value);
      await loadBannerInfo(bannerId);
      return result;
    } catch (e) {
      error.value = `抽卡失败: ${e}`
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return {
    banners,
    currentBannerInfo,
    currentWish,
    history,
    loading,
    error,
    loadBanners,
    loadBannerInfo,
    doWish,
  }
}