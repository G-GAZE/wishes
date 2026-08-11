<script setup lang="ts">
/**
 * 图鉴页面, 展示卡片、卡组、逻辑的统计数量
 * 调用后端命令 `get_catalog_stats` 获取数据
 */

import { onMounted, ref } from 'vue';
import { CatalogStats } from '../types';
import { invoke } from '@tauri-apps/api/core';


const stats = ref<CatalogStats | null>(null);
const loading = ref(false);


onMounted(() => {
  const loadCatalogStats = async () => {
    loading.value = true;
    try {
      stats.value = await invoke<CatalogStats>("get_catalog_stats", {});
    } catch (e) {
      console.error("获取图鉴统计数据失败: ", e)
      stats.value = null;
    } finally {
      loading.value = false;
    }
  }

  loadCatalogStats();
})
</script>


<template>
  <div class="page catalog-page">
    <span class="title">图鉴</span>
    <span class="subtitle">管理所有卡片、卡组与逻辑</span>
    <div class="catalog-grid">
      <div class="catalog-card">
        <div class="catalog-icon">🃏</div>
        <div class="catalog-title">卡片</div>
        <div class="catalog-count">{{ stats ? `${stats.cards} 张` : "加载失败" }}</div>
      </div>
      <div class="catalog-card">
        <div class="catalog-icon">📋</div>
        <div class="catalog-title">卡组</div>
        <div class="catalog-count">{{ stats ? `${stats.decks} 个` : "无法加载" }}</div>
      </div>
      <div class="catalog-card">
        <div class="catalog-icon">⚙️</div>
        <div class="catalog-title">抽卡逻辑</div>
        <div class="catalog-count">{{ stats ? `${stats.logics} 个` : "无法加载" }}</div>
      </div>
    </div>
  </div>
</template>


<style scoped>
.catalog-page {
  padding: 40px 16px;
  height: 100%;
  background-image: radial-gradient(
    ellipse at 10% 90%,
    rgba(77, 109, 255, 0.1) 0%,
    rgba(120, 80, 240, 0.06) 40%,
    transparent 70%
  );
}

.title {
  font-size: 2.5rem;
  font-weight: 600;
  user-select: none;
}

.subtitle {
  color: #8a99b4;
  font-size: 1.2rem;
  padding: 10px;
  user-select: none;
}

.catalog-grid {
  display: grid;
  margin-top: 10px;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.catalog-card {
  background: rgba(26, 33, 43, 0.25);
  backdrop-filter: blur(10px) saturate(180%);
  -webkit-backdrop-filter: blur(10px) saturate(180%);
  border-radius: 16px;
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.15);
  padding: 20px 16px;
  border: 1px solid #2a3340;
  transition: border-color 0.2s, transform 0.2s;
  cursor: pointer;
  padding-bottom: 30px;
  text-align: center;
}
.catalog-card:hover {
  border-color: #4f6ef7;
  transform: translateY(-5px);
}

.catalog-icon {
  font-size: 2.4rem;
  margin-bottom: 8px;
}

.catalog-title {
  font-weight: bold;
  font-size: 1.4rem;
}

.catalog-count {
  margin-top: 4px;
  color: #8a99b4;
  font-size: 1rem;
}
</style>