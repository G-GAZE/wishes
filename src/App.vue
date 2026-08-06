<script setup lang="ts">
import { computed, defineAsyncComponent, nextTick, ref, watch } from "vue";
import BottomNav from "./components/BottomNav.vue";
import { useWish } from "./composables/useWish";
import type { WishResponse } from "./types.ts";
import WishResultOverlay from "./components/WishResultOverlay.vue";


const {
  banners,
  currentBannerInfo,
  currentWish,
  loadBanners,
  loadBannerInfo,
  doWish,
} = useWish();

const selectedBannerId = ref<number | null>(null);

loadBanners();

// 默认选中第一个卡池
watch(banners, (new_banners) => {
  if (new_banners.length > 0 && selectedBannerId.value === null) {
    selectedBannerId.value = new_banners[0].id;
    loadBannerInfo(selectedBannerId.value);
  }
  scrollToActiveBannerTab();
});

function onSelectBanner(bannerId: number) {
  selectedBannerId.value = bannerId;
  loadBannerInfo(bannerId);
  navigateTo("gacha");
}

async function onBannerChanged(id: number) {
  selectedBannerId.value = id;
  await loadBannerInfo(selectedBannerId.value)
  scrollToActiveBannerTab();
}

function scrollToActiveBannerTab() {
  // 未按预期工作
  nextTick(() => {
    const container = document.querySelector('.banner-tabs') as HTMLElement;
    const activeTab = container?.querySelector('.banner-tab.active') as HTMLElement;
    if (container && activeTab) {
      const containerWidth = container.offsetWidth;
      const tabWidth = activeTab.offsetWidth;
      const tabLeft = activeTab.offsetLeft;
      const scrollTarget = tabLeft - containerWidth / 2 + tabWidth / 2;
      container.scrollTo({ left: scrollTarget, behavior: 'smooth' });
    }
  });
}


const showWishResultOverlay = ref(false);
const tenWishResults = ref<WishResponse[]>([]);
const isTenWish = ref(false);


async function handleWishSingle() {
  if (!selectedBannerId.value) return;
  try {
    const response = await doWish(selectedBannerId.value);
    tenWishResults.value = [response];
    isTenWish.value = false;
    showWishResultOverlay.value = true;
  } catch (e) {
    // 无额外处理
    // TODO: 提示弹窗
  }
}

async function handleWishTen() {
  if (!selectedBannerId.value) return;
  try {
    // 循环十次
    // TODO: 并行接口
    tenWishResults.value = [];
    for (let i = 0; i < 10; i++) {
      const result = await doWish(selectedBannerId.value);
      tenWishResults.value.push(result);
    }
    isTenWish.value = true;
    showWishResultOverlay.value = true;
  } catch (e) {
    // TODO: 提示弹窗
  }
}

function closeResultOverlay() {
  showWishResultOverlay.value = false;
  currentWish.value = null;
  isTenWish.value = false;
  tenWishResults.value = [];
}

const HomePage = defineAsyncComponent(() => import("./views/HomePage.vue"));
const GachaPage = defineAsyncComponent(() => import("./views/GachaPage.vue"));
const CatalogPage = defineAsyncComponent(() => import("./views/CatalogPage.vue"));
const SettingsPage = defineAsyncComponent(() => import("./views/SettingsPage.vue"));

const currentPage = ref("home");
const isGachaPage = computed(() => currentPage.value === "gacha");

const pageMap: Record<string, any> = {
  home: HomePage,
  gacha: GachaPage,
  catalog: CatalogPage,
  settings: SettingsPage,
};

const currentPageComponent = computed(() => pageMap[currentPage.value]);

function navigateTo(page: string) {
  currentPage.value = page;
}
</script>


<template>
  <div id="app">
    <div class="app-title">Wishes 众愿</div>
    <header :class="{ expanded:  isGachaPage }">
      <div class="header-content">
        <div v-if="isGachaPage" class="banner-tabs">
          <div
            v-for="b in banners"
            :key="b.id"
            class="banner-tab"
            :class="{ active: selectedBannerId === b.id }"
            @click="onBannerChanged(b.id)"
          >
            {{ b.name }}
          </div>
        </div>
      </div>
    </header>

    <div class="page-container">
      <Transition name="slide" mode="out-in">
        <component
          :is="currentPageComponent"
          :key="currentPage"
          @select-banner="onSelectBanner"
          :banner-id="selectedBannerId"
          :banner-info="currentBannerInfo"
          :banners="banners"
          @wish-single="handleWishSingle"
          @wish-ten="handleWishTen"
        />
      </Transition>
    </div>

    <BottomNav v-model="currentPage"/>

    <WishResultOverlay
      v-if="showWishResultOverlay"
      :result="currentWish"
      :is-ten-wish="isTenWish"
      :ten-results="tenWishResults"
      @close="closeResultOverlay"
    />
  </div>
</template>


<style scoped>
.app-title {
  position: fixed;
  top: 0;
  left: 0;
  padding: 1rem 1.2rem;
  font-size: 2rem;
  letter-spacing: 0.5px;
  color: #e4e8ef;
  pointer-events: none;
  z-index: 20;
}

header {
  background: transparent;
  height: 0;
  transition: height 0.4s cubic-bezier(0.34, 1.56, 0.64, 1),
              background-color 0.4s ease;
  overflow: hidden;
  display: flex;
  align-items: flex-end;
  padding-top: 1rem;
  padding-left: 1rem;
  flex-shrink: 0;
  z-index: 10;
  user-select: none;
}

header.expanded {
  background-color: #1c1e22;
  height: 8rem;
  align-items: flex-end;
  padding-top: 1rem;
}

.header-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 0.2rem;
}

.banner-tabs {
  display: flex;
  gap: 0.8rem;
  overflow-x: auto;
  margin: 1rem;
  padding: 0.2rem;
  scrollbar-width: none;
  justify-content: center;
  scroll-behavior: smooth;
}

.banner-tabs::-webkit-scrollbar {
  display: none;
}

.banner-tab {
  gap: 2px;
  padding: 5px 40px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.06);
  color: #f4f4f4;
  font-size: 1.5rem;
  white-space: nowrap;
  cursor: pointer;
  transition: all 0.25s;
  flex-shrink: 0;
  font-weight: 500;
}

.banner-tab:hover {
  background: rgba(255, 255, 255, 0.12);
}

.banner-tab.active {
  background: #007bff;
  color: #fff;
  box-shadow: 0 4px 4px rgba(79, 110, 247, 0.3);
}

.page-container {
  flex: 1;
  overflow-y: auto;
  padding-bottom: 68px;
  scroll-behavior: smooth;
  overflow: hidden;
}

.slide-enter-active {
  transition: all 0.6s cubic-bezier(0.25, 0.10, 0.25, 1.00);
}

.slide-leave-active {
  transition: all 0s;
}

.slide-enter-from {
  transform: translateY(100%);
  opacity: 0;
}
</style>

<style>
html {
  font-size: 62.5%;
  overflow: hidden;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

#app {
  background: #121316;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

body {
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  background: #0b0e14;
  color: #e4e8ef;
  margin: 0;
  padding: 0;
  min-height: 100vh;
  overflow: hidden;
}
</style>