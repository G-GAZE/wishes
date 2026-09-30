<script setup lang="ts">
import { useI18n } from 'vue-i18n';

/**
 * 底部导航组件, 固定于页面底部, 用于切换页面
 */

/**
 * 单个导航项信息
 */
export interface NavItem {
  key: string,
  labelKey: string,
  icon: string,
}

defineProps<{
  modelValue: string
}>()

defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const { t } = useI18n();

const navItems: NavItem[] = [
  { key: "home", labelKey: "nav.home", icon: "" },
  { key: "gacha", labelKey: "nav.gacha", icon: "" },
  { key: "catalog", labelKey: "nav.catalog", icon: "" },
  { key: "settings", labelKey: "nav.settings", icon: "" },
]

const version = __APP_VERSION__;
</script>


<template>
  <nav class="bottom-nav">
    <div
      v-for="item in navItems"
      :key="item.key"
      class="nav-item"
      :class="{ active: modelValue === item.key }"
      @click="$emit('update:modelValue', item.key)"
    >
      <div class="nav-label">{{ t(item.labelKey) }}</div>
    </div>
    <div class="version-tip">{{ t('nav.versionTip', { version }) }}</div>
  </nav>
</template>


<style scoped>
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  align-items: center;
  height: 5rem;
  background: #1c1e22;
  padding: 0 8px;
  z-index: 100;
  user-select: none;
  gap: 10px;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  padding: 6px 16px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.25s ease;
  color: #ededed;
  min-width: 64px;
}

.nav-item:hover {
  color: #ededed;
  background: #3c3f46;
}

.nav-item.active {
  color: #121316;
  background: #007bff;
}

.nav-label {
  font-size: 1.4rem;
  font-weight: bold;
  letter-spacing: 0.5px;
}

.version-tip {
  position: absolute;
  right: 8px;
  bottom: 8px;
  font-size: 1rem;
  color: #888;
  font-style: italic;
}
</style>
