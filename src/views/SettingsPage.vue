<script setup lang="ts">
/**
 * 设置页面, 展示应用版本、数据目录路径等信息
 * 调用后端命令 `get_data_dir_path` 获取数据目录路径
 */

import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';

const version = __APP_VERSION__;

const loading = ref(false);
const data_dir = ref<string | null>(null);


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
  }

  loadDataDir();
})
</script>


<template>
  <div class="page settings-page">
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
    </div>

    <span class="about-title">关于 Wishes</span>
    <span class="subtitle">关于此版本的 Wishes</span>
    <div class="about-section">
      <div class="about-content">
        <p><strong>Wishes (众愿)</strong> 是一款基于 Tauri 2 和 Vue 3 构建的通用的、可高度自定义的模拟抽卡工具。</p>
        <p>当前版本: <span class="version">{{ version }}</span></p>
        <p>
          GitHub 项目地址:
          <a href="https://github.com/G-GAZE/wishes" target="_blank" rel="noopener noreferrer">
            https://github.com/G-GAZE/wishes
          </a>
        </p>
        <p class="license">基于 <strong>MIT</strong> 开源协议</p>
      </div>
    </div>
  </div>
</template>


<style scoped>
.settings-page {
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

.settings-list {
  /* background: rgba(120, 80, 240, 0.06); */
  background: rgba(26, 33, 43, 0.1);
  backdrop-filter: blur(10px) saturate(180%);
  -webkit-backdrop-filter: blur(10px) saturate(180%);
  border-radius: 16px;
  border: 1px solid #2a3340;
  overflow: hidden;
  margin-top: 10px;
  margin-bottom: 20px;
}
.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 18px;
  border-bottom: 1px solid #2a3340;
}
.setting-item:last-child {
  border-bottom: none;
}
.setting-label {
  font-size: 1.3rem;
}
.setting-value {
  color: #8a99b4;
  font-size: 1.1rem;
}

.about-title {
  font-size: 2.5rem;
  font-weight: bold;
  padding-top: 25px;
  margin-bottom: 10px;
  user-select: none;
}
.about-section {
  background: rgba(26, 33, 43, 0.1);
  backdrop-filter: blur(10px) saturate(180%);
  -webkit-backdrop-filter: blur(10px) saturate(180%);
  border-radius: 16px;
  border: 1px solid #2a3340;
  overflow: hidden;
  padding: 14px 18px;
  margin-top: 10px;
}
.about-section p {
  margin: 6px 0;
  font-size: 1.3rem;
  color: #e4e8ef;
  line-height: 1.4;
}
.about-section .version {
  color: #007bff;
  font-weight: bold;
}
.about-section a {
  color: #007bff;
  text-decoration: none;
}
.about-section a:hover {
  text-decoration: underline;
}
.license {
  color: #6a7a90;
  font-size: 1.3rem;
}
</style>