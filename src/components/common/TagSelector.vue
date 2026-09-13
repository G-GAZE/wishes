<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import type { Tag } from '../../types';

const props = defineProps<{
  modelValue: Tag[];
  existingTags?: Tag[]; // 可选的全量标签列表，若未提供则通过内部维护
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: Tag[]): void;
}>();

const localTags = ref<Tag[]>([...props.modelValue]);
const newNamespace = ref('');
const newValue = ref('');

watch(() => props.modelValue, (val) => {
  localTags.value = [...val];
}, { deep: true });

// 计算可选标签 (从 existingTags 中排除已选)
const availableTagsGrouped = computed(() => {
  const selectedSet = new Set(localTags.value.map(t => `${t.namespace}:${t.value}`));
  const groupMap = new Map<string, Tag[]>();
  (props.existingTags || []).forEach(tag => {
    const key = `${tag.namespace}:${tag.value}`;
    if (selectedSet.has(key)) return;
    if (!groupMap.has(tag.namespace)) {
      groupMap.set(tag.namespace, []);
    }
    groupMap.get(tag.namespace)!.push(tag);
  });
  return Array.from(groupMap.entries()).sort((a, b) => a[0].localeCompare(b[0]))
    .map(([namespace, tags]) => ({
      namespace,
      tags: tags.sort((a, b) => a.value.localeCompare(b.value))
    }));
});

function addTag() {
  const ns = newNamespace.value.trim();
  const val = newValue.value.trim();
  if (!ns || !val) return;
  const exists = localTags.value.some(t => t.namespace === ns && t.value === val);
  if (exists) return;
  localTags.value.push({ namespace: ns, value: val });
  newNamespace.value = '';
  newValue.value = '';
  emitUpdate();
}

function removeTag(tag: Tag) {
  const idx = localTags.value.findIndex(t => t.namespace === tag.namespace && t.value === tag.value);
  if (idx !== -1) {
    localTags.value.splice(idx, 1);
    emitUpdate();
  }
}

function addAvailableTag(tag: Tag) {
  localTags.value.push({ ...tag });
  emitUpdate();
}

function emitUpdate() {
  emit('update:modelValue', localTags.value);
}
</script>

<template>
  <div class="tag-selector">
    <!-- 已选标签 -->
    <div class="selected-tags">
      <span v-for="tag in localTags" :key="`${tag.namespace}:${tag.value}`" class="tag-item">
        {{ tag.namespace }}: {{ tag.value }}
        <button class="tag-remove" @click="removeTag(tag)">✕</button>
      </span>
      <span v-if="localTags.length === 0" class="no-tags">暂无标签</span>
    </div>

    <!-- 可选标签 -->
    <div v-if="availableTagsGrouped.length > 0" class="available-tags-wrapper">
      <div v-for="group in availableTagsGrouped" :key="group.namespace" class="tag-group">
        <span class="group-label">{{ group.namespace }}</span>
        <div class="group-tags">
          <span
            v-for="tag in group.tags"
            :key="`${tag.namespace}:${tag.value}`"
            class="available-tag"
            @click="addAvailableTag(tag)"
          >
            {{ tag.value }}
          </span>
        </div>
      </div>
    </div>

    <!-- 新建标签 -->
    <div class="new-tag-row">
      <input
        v-model="newNamespace"
        placeholder="命名空间"
        class="new-tag-input ns"
        @keydown.enter="addTag"
      />
      <span class="sep">:</span>
      <input
        v-model="newValue"
        placeholder="值"
        class="new-tag-input value"
        @keydown.enter="addTag"
      />
      <button class="btn-add-tag" @click="addTag">+ 添加</button>
    </div>
  </div>
</template>

<style scoped>
.tag-selector {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding: 4px 0;
}
.tag-item {
  background: rgba(79, 110, 247, 0.15);
  padding: 2px 8px 2px 14px;
  border-radius: 16px;
  font-size: 1.1rem;
  color: #b0c0d0;
  display: flex;
  align-items: center;
  gap: 4px;
}
.tag-remove {
  background: none;
  border: none;
  color: #6a7a90;
  cursor: pointer;
  font-size: 0.9rem;
  padding: 0 2px;
}
.tag-remove:hover {
  color: #ff6b6b;
}
.no-tags {
  color: #4a5a70;
  font-size: 1.1rem;
}
.available-tags-wrapper {
  background: #0b0e14;
  border-radius: 8px;
  padding: 6px 8px;
  max-height: 120px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.available-tags-wrapper::-webkit-scrollbar {
  width: 4px;
}
.available-tags-wrapper::-webkit-scrollbar-thumb {
  background: #2a3340;
  border-radius: 4px;
}
.tag-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.group-label {
  font-size: 1.1rem;
  font-weight: bold;
  color: #4a5a70;
  text-transform: uppercase;
}
.group-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  padding-left: 4px;
}
.available-tag {
  background: rgba(255,255,255,0.06);
  padding: 1px 12px;
  border-radius: 12px;
  font-size: 1rem;
  color: #b0c0d0;
  cursor: pointer;
  transition: background 0.2s;
}
.available-tag:hover {
  background: rgba(79, 110, 247, 0.2);
  color: #fff;
}
.new-tag-row {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-wrap: wrap;
}
.new-tag-input {
  background: #0b0e14;
  border: none;
  border-bottom: 2px solid #2a3340;
  padding: 2px 10px;
  color: #e4e8ef;
  font-size: 1.3rem;
}
.new-tag-input:focus {
  outline: none;
  border-color: #007bff;
}
.new-tag-input.ns {
  flex: 1;
  min-width: 60px;
}
.new-tag-input.value {
  flex: 1;
  min-width: 60px;
}
.sep {
  color: #6a7a90;
}
.btn-add-tag {
  background: #3b4b5e;
  border: none;
  border-radius: 6px;
  padding: 2px 16px;
  color: #fff;
  font-size: 1.2rem;
  cursor: pointer;
}
.btn-add-tag:hover {
  background: #475a70;
}
</style>