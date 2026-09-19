<script setup lang="ts">
/**
 * 设置页面, 展示应用设置项
 * "关于" 作为列表入口, 通过视图切换展示 AboutPage 子视图
 */

import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';
import AboutPage from './AboutPage.vue';

const loading = ref(false);
const data_dir = ref<string | null>(null);

/** 视图状态: settings (设置列表) | about (关于页) */
const currentView = ref<'settings' | 'about'>('settings');

onMounted(() => {
  const loadDataDir = async () => {
    loading.value = true;
    try {
      data_dir.value = await invoke("get_data_dir_path");
    } catch (e) {
      console.error("获取数据目录路径失败: ", e);
      data_dir.value = null;
    } finally {
      loading.value = false;
    }
  };

  loadDataDir();
});
</script>

<template>
  <div class="settings-page-wrapper">
    <!-- 设置视图 -->
    <Transition name="slide-settings">
      <div v-if="currentView === 'settings'" class="settings-page">
        <span class="title">设置</span>
        <span class="subtitle">应用偏好与数据管理</span>

        <div class="settings-list">
          <div class="setting-item">
            <span class="setting-label">数据目录</span>
            <span class="setting-value">{{ data_dir ? data_dir : "无法加载" }}</span>
          </div>
          <div class="setting-item">
            <span class="setting-label">深色模式</span>
            <span class="setting-value">已启用</span>
          </div>
          <div class="setting-item clickable" @click="currentView = 'about'">
            <span class="setting-label">关于</span>
            <span class="setting-value">
              项目简介与开源协议
              <span class="arrow">›</span>
            </span>
          </div>
        </div>
      </div>
    </Transition>

    <!-- 关于视图 -->
    <Transition name="slide-about">
      <div v-if="currentView === 'about'" class="about-view-wrapper">
        <AboutPage @back="currentView = 'settings'" />
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.settings-page-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-image: radial-gradient(
    ellipse at 10% 90%,
    rgba(77, 109, 255, 0.1) 0%,
    rgba(120, 80, 240, 0.06) 40%,
    transparent 70%
  );
}

.settings-page,
.about-view-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.settings-page {
  padding: 40px 16px;
  /* background-image: radial-gradient(
    ellipse at 10% 90%,
    rgba(77, 109, 255, 0.1) 0%,
    rgba(120, 80, 240, 0.06) 40%,
    transparent 70%
  ); */
  overflow-y: auto;
}

.title {
  display: block;
  font-size: 2.5rem;
  font-weight: 600;
  user-select: none;
}
.subtitle {
  display: block;
  color: #8a99b4;
  font-size: 1.2rem;
  padding: 10px;
  user-select: none;
}

.settings-list {
  background: rgba(26, 33, 43, 0.1);
  backdrop-filter: blur(10px) saturate(180%);
  -webkit-backdrop-filter: blur(10px) saturate(180%);
  border-radius: 16px;
  border: 1px solid #2a3340;
  overflow: hidden;
  margin-top: 10px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 18px;
  border-bottom: 1px solid #2a3340;
  transition: background 0.2s;
}
.setting-item:last-child {
  border-bottom: none;
}

.setting-item.clickable {
  cursor: pointer;
  user-select: none;
}
.setting-item.clickable:hover {
  background: rgba(79, 110, 247, 0.08);
}

.setting-label {
  font-size: 1.3rem;
  color: #e4e8ef;
}

.setting-value {
  color: #8a99b4;
  font-size: 1.1rem;
  display: flex;
  align-items: center;
  gap: 6px;
}
.setting-value .arrow {
  color: #4a5a70;
  font-size: 1.6rem;
  line-height: 1;
}

/* --- 滑动过渡 --- */
.slide-settings-enter-active,
.slide-settings-leave-active,
.slide-about-enter-active,
.slide-about-leave-active {
  transition: transform 0.4s ease;
}

/* 设置视图: 从左侧滑入 / 向左侧滑出 */
.slide-settings-enter-from,
.slide-settings-leave-to {
  transform: translateX(-100%);
}

/* 关于视图: 从右侧滑入 / 向右侧滑出 */
.slide-about-enter-from,
.slide-about-leave-to {
  transform: translateX(100%);
}
</style>