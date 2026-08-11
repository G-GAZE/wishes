<script setup lang="ts">
/**
 * 底部导航组件, 固定于页面底部, 用于切换页面
 */

/**
 * 单个导航项信息
 */
export interface NavItem {
  key: string,
  label: string,
  icon: string,
}

defineProps<{
  modelValue: string
}>()

defineEmits<{
  (e: "update:modelValue", value: string): void
}>()

const navItems: NavItem[] = [
  { key: "home", label: "主页", icon: "" },
  { key: "gacha", label: "抽卡", icon: "" },
  { key: "catalog", label: "图鉴", icon: "" },
  { key: "settings", label: "设置", icon: "" },
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
      <div class="nav-label">{{ item.label }}</div>
    </div>
    <div class="version-tip">v{{ version }} - 测试版本, 不代表正式版品质</div>
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