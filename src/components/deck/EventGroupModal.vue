<script setup lang="ts">
import { computed, ref } from 'vue';
import { NSelect } from 'naive-ui';
import type { SelectOption } from 'naive-ui';
import type { EventTag } from '../../types';
import { EVENT_TAG_OPTIONS } from '../../constants/eventTags';

const props = withDefaults(
  defineProps<{
    existingTags: EventTag[];
  }>(),
  {
    existingTags: () => [],
  }
);

const emit = defineEmits<{
  (e: 'confirm', tag: EventTag): void;
  (e: 'close'): void;
}>();

const selected = ref<EventTag | null>(null);

const options = computed<SelectOption[]>(() =>
  EVENT_TAG_OPTIONS.map(opt => ({
    value: opt.value,
    label: `${opt.value} (${opt.label})`,
    disabled: (props.existingTags ?? []).includes(opt.value),
  }))
);

const canConfirm = computed(() => selected.value !== null);

function confirm() {
  if (!selected.value) return;
  emit('confirm', selected.value);
}

function close() {
  emit('close');
}
</script>

<template>
  <div class="modal-overlay" @click="close">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>添加活动组</h3>
        <button class="close-btn" @click="close">✕</button>
      </div>

      <div class="modal-body">
        <label class="field-label">活动组类型</label>
        <n-select
          v-model:value="selected"
          :options="options"
          placeholder="请选择活动组类型"
          class="event-tag-select"
        />
        <p class="hint">已存在的活动组不可重复添加</p>
      </div>

      <div class="modal-footer">
        <button class="btn-cancel" @click="close">取消</button>
        <button class="btn-confirm" :disabled="!canConfirm" @click="confirm">
          确认添加
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
}
.modal-content {
  background: #1a212b80;
  border-radius: 16px;
  padding: 20px;
  width: 90%;
  max-width: 420px;
  border: 1px solid #2a3340;
  display: flex;
  flex-direction: column;
}
.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.modal-header h3 {
  font-size: 2rem;
  margin: 0;
}
.close-btn {
  background: none;
  border: none;
  color: #6a7a90;
  font-size: 1.8rem;
  cursor: pointer;
}
.close-btn:hover {
  color: #fff;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.field-label {
  font-size: 1.3rem;
  font-weight: 500;
  color: #b0c0d0;
}

.event-tag-select {
  width: 100%;
}

.hint {
  font-size: 1.1rem;
  color: #6a7a90;
  margin-top: 4px;
}

.modal-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding-top: 20px;
}
.btn-confirm {
  background: #007bff;
  border: none;
  border-radius: 8px;
  padding: 8px 24px;
  color: #fff;
  font-weight: 600;
  cursor: pointer;
}
.btn-confirm:hover:not(:disabled) {
  background: #3d9bff;
}
.btn-confirm:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.btn-cancel {
  background: transparent;
  border: 1px solid #2d3642;
  border-radius: 8px;
  padding: 8px 24px;
  color: #b0c0d0;
  cursor: pointer;
}
.btn-cancel:hover {
  background: #2d3642;
}
</style>