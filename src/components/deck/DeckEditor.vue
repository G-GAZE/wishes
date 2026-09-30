<script setup lang="ts">
import { computed, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { DeckSummary, EventGroup, Membership } from '../../api/decks';
import type { EventTag, Tag } from '../../types';
import TagSelector from '../common/TagSelector.vue';
import ConditionEditor from '../deck/ConditionEditor.vue';
import { eventTagLabel } from '../../constants/eventTags.ts';
import EventGroupModal from './EventGroupModal.vue';
import { useDialog } from 'naive-ui';
import { cloneDeep } from 'lodash-es';

interface DeckFormData {
  name: string, 
  tags: Tag[],
  members: Membership,
  event_groups: Record<EventTag, EventGroup>,
}

const props = defineProps<{
  /** 传入表示编辑, 不传表示新建 */
  deck?: DeckSummary;
}>();

const emit = defineEmits<{
  (e: 'save', data: DeckFormData): void;
  (e: 'cancel'): void;
}>();

const isEditMode = computed(() => !!props.deck);

function createEmptyForm(): DeckFormData {
  return {
    name: '',
    tags: [],
    members: { conditions: [] },
    event_groups: {},
  };
}

/** 深拷贝原数据, 以便取消可回滚 */
const formData = reactive<DeckFormData>(
  props.deck
    ? {
        name: props.deck.name,
        tags: cloneDeep(props.deck.tags),
        members: cloneDeep(props.deck.members),
        event_groups: cloneDeep(props.deck.event_groups),
    }
    : createEmptyForm()
);

const showEventGroupModal = ref(false);

const dialog = useDialog();
const { t } = useI18n();

function onEventGroupConfirm(tag: EventTag) {
  if (!formData.event_groups[tag]) {
    formData.event_groups[tag] = { conditions: [] };
  }
  showEventGroupModal.value = false;
}

const existingEventTags = computed(() => 
  Object.keys(formData.event_groups) as EventTag[]
)

// 添加条件到 members
function addMemberCondition() {
  formData.members.conditions.push({ type: 'tag_all', tags: [] });
}

// 删除 members 条件
function removeMemberCondition(index: number) {
  confirmDelete({
    content: t('deck.deleteMemberConfirm'),
    onConfirm: () => {
      formData.members.conditions.splice(index, 1);
    }
  })
}

// 添加 event group
function addEventGroup() {
  showEventGroupModal.value = true;
  // const tag = prompt('输入活动标签 (EventTag)');
  // if (tag && tag.trim()) {
  //   const key = tag.trim() as EventTag;
  //   if (!formData.event_groups[key]) {
  //     formData.event_groups[key] = { conditions: [] };
  //   }
  // }
}

// 删除 event group
function removeEventGroup(key: EventTag) {
  const condCount = formData.event_groups[key]?.conditions.length ?? 0;
  confirmDelete({
    title: t('deck.deleteEventGroupTitle'),
    content: condCount > 0
      ? t('deck.deleteEventGroupWithConds', { tag: key, count: condCount })
      : t('deck.deleteEventGroup', { tag: key }),
      onConfirm: () => {
        delete formData.event_groups[key];
      }
  })
}

// 添加条件到某个 event group
function addEventGroupCondition(key: EventTag) {
  formData.event_groups[key].conditions.push({ type: 'tag_all', tags: [] });
}

// 删除 event group 中的条件
function removeEventGroupCondition(key: EventTag, index: number) {
  confirmDelete({
    content: t('deck.deleteConditionConfirm'),
    onConfirm: () => {
      formData.event_groups[key].conditions.splice(index, 1);
    }
  })
}

function confirmDelete(options: { title?: string, content: string, onConfirm: () => void }) {
  dialog.warning({
    title: options.title ?? t('deck.deleteConfirmTitle'),
    content: options.content,
    positiveText: t('deck.deleteConfirmTitle'),
    negativeText: t('common.cancel'),
    onPositiveClick: () => {
      options.onConfirm();
    },
    positiveButtonProps: {
      color: '#e74c3c',
      textColor: '#fff',
    }
  })
}

// 保存
function save() {
  if (formData.name === '') return;
  emit('save', {
    name: formData.name,
    tags: formData.tags,
    members: formData.members,
    event_groups: formData.event_groups,
  });
}
</script>

<template>
  <div class="deck-editor">
    <div class="editor-header">
      <!-- <button class="btn-close" @click="emit('cancel')"></button> -->
      <h3>{{ isEditMode ? t('deck.editorTitleEdit') : t('deck.editorTitleCreate') }}</h3>
    </div>

    <div class="id-name-field">
      <div v-if="isEditMode && deck" class="deck-id">ID {{ deck.id }}</div>
      <label>{{ t('deck.nameLabel') }}</label>
      <input v-model="formData.name" class="name-input" :placeholder="t('deck.namePlaceholder')" />
    </div>

    <div class="editor-body">
      <!-- 名称 -->
      <!-- <div class="id-name-field">
        <div class="deck-id">ID {{ deck.id }}</div>
        <label>名称</label>
        <input v-model="formData.name" class="name-input" />
      </div> -->

      <!-- 标签 -->
      <div class="field">
        <label>{{ t('deck.tagsLabel') }}</label>
        <TagSelector v-model="formData.tags" />
      </div>

      <!-- Membership -->
      <div class="field">
        <label>{{ t('deck.membersLabel') }}</label>
        <span>{{ t('deck.membersHint') }}</span>

        <div v-for="(_, idx) in formData.members.conditions" :key="idx" class="condition-wrapper">
          <ConditionEditor v-model="formData.members.conditions[idx]" />
          <button class="btn-remove-cond" @click="removeMemberCondition(idx)">{{ t('deck.deleteCondition') }}</button>
        </div>

        <button class="btn-add-cond" @click="addMemberCondition">{{ t('deck.addCondition') }}</button>
      </div>

      <!-- Event Groups -->
      <div class="field">
        <label>{{ t('deck.eventGroupsLabel') }}</label>
        <span>{{ t('deck.eventGroupsHint') }}</span>

        <div v-for="(group, eventTag) in formData.event_groups" :key="eventTag" class="event-group">
          <div class="event-group-header">
            <div class="event-tag">
              {{ eventTag }}
              <span v-if="eventTagLabel(t, eventTag)" class="event-tag-hint">
                 {{ eventTagLabel(t, eventTag) }}
              </span>
            </div>
            <button class="btn-remove-group" @click="removeEventGroup(eventTag)">{{ t('deck.removeGroup') }}</button>
          </div>

          <div v-for="(_, idx) in group.conditions" :key="idx" class="condition-wrapper">
            <ConditionEditor v-model="group.conditions[idx]" />
            <button class="btn-remove-cond" @click="removeEventGroupCondition(eventTag, idx)">{{ t('deck.deleteCondition') }}</button>
          </div>

          <button class="btn-add-cond" @click="addEventGroupCondition(eventTag)">{{ t('deck.addCondition') }}</button>
        </div>

        <button class="btn-add-group" @click="addEventGroup">{{ t('deck.addGroup') }}</button>

        <EventGroupModal
          v-if="showEventGroupModal"
          :existing-tags="existingEventTags"
          @confirm="onEventGroupConfirm"
          @close="showEventGroupModal = false"
        />
      </div>
    </div>

    <div class="editor-footer">
      <button class="btn-cancel" @click="emit('cancel')">{{ t('common.cancel') }}</button>
      <button class="btn-save" @click="save" :disabled="formData.name === ''">{{ t('deck.save') }}</button>
    </div>
  </div>
</template>

<style scoped>
.deck-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  /* padding: 16px; */
  user-select: none;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-shrink: 0;
}
.editor-header h3 {
  font-size: 2rem;
  margin: 0;
}
.btn-close {
  background: none;
  border: none;
  color: #6a7a90;
  font-size: 1.5rem;
  cursor: pointer;
}
.btn-close:hover {
  color: #fff;
}

.editor-body {
  flex: 1;
  overflow-y: auto;
  padding-right: 8px;
  scrollbar-width: thin;
  scrollbar-color: #2a3340 transparent;
}
.editor-body::-webkit-scrollbar {
  width: 4px;
}
.editor-body::-webkit-scrollbar-thumb {
  background: #2a3340;
  border-radius: 4px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  border-top: 1px solid #2a3340;
  padding-top: 10px;
  margin-bottom: 20px;
}
.field label {
  font-size: 1.7rem;
  font-weight: bold;
  color: #b0c0d0;
  font-weight: bold;
}
.field span {
  font-size: 1.3rem;
  color: #8a99b4;
}

.id-name-field {
  display: flex;
  flex-direction: row;
  gap: 6px;
  align-items: center;
  margin-bottom: 12px;
}

.deck-id {
  background: #4b7ac1;
  text-align: center;
  color: #fff;
  font-size: 1.2rem;
  letter-spacing: 0.3px;
  padding: 3px 8px 3px 10px;
  border-radius: 100px 100px 100px 100px;
  margin-right: 16px;
}

.id-name-field label {
  font-size: 1.3rem;
  font-weight: 500;
  color: #b0c0d0;
}

.name-input {
  flex: 1;
  border: none;
  border-radius: 0 5px 0 0;
  border-bottom: 2px solid #2a3340;
  background: #141a22;
  padding: 6px 8px;
  font-size: 1.2rem;
  min-width: 180px;
  color: #e0e3e7;
}
.name-input:focus {
  outline: none;
  border-color: #007bff;
}

.form-input {
  width: 100%;
  background: #0b0e14;
  border: none;
  border-bottom: 3px solid #2a3340;
  padding: 6px 8px;
  color: #e4e8ef;
  font-size: 1.3rem;
  font-family: inherit;
}
.form-input:focus {
  outline: none;
  border-color: #007bff;
}

.event-tag-hint {
  color: #6a7a90;
  font-weight: normal;
  font-size: 1.2rem;
  margin-left: 4px;
}

.condition-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  background: rgba(26, 33, 43, 0.3);
  padding: 6px 10px;
  border-radius: 6px;
}
.condition-wrapper > * {
  flex: 1;
}

.btn-remove-cond {
  flex: 0 0 auto;
  background: #212830;
  border: 1px solid #3d444d;
  border-radius: 4px;
  padding: 4px 14px;
  color: #fa5e55;
  cursor: pointer;
  font-size: 1.2rem;
  font-weight: bold;
}
.btn-remove-cond:hover {
  background: #b62324;
  color: #ffe9ca;
}

.btn-add-cond,
.btn-add-group {
  background: #3b4b5e;
  border: none;
  border-radius: 6px;
  padding: 4px 16px;
  color: #fff;
  font-size: 1.1rem;
  cursor: pointer;
  margin-top: 4px;
}
.btn-add-cond:hover,
.btn-add-group:hover {
  background: #475a70;
}

.event-group {
  border-left: 3px solid #2a3340;
  padding-left: 12px;
  margin-bottom: 12px;
}
.event-group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.event-tag {
  font-size: 1.4rem;
  font-weight: bold;
  color: #4f6ef7;
}
.btn-remove-group {
  background: #212830;
  border: 1px solid #3d444d;
  border-radius: 4px;
  padding: 4px 14px;
  color: #fa5e55;
  cursor: pointer;
  font-size: 1.2rem;
  font-weight: bold;
}
.btn-remove-group:hover {
  background: #b62324;
  color: #ffe9ca;
}

.editor-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding-top: 16px;
  /* border-top: 1px solid #2a3340; */
  flex-shrink: 0;
}
.btn-save {
  background: #007bff;
  border: none;
  border-radius: 8px;
  padding: 8px 24px;
  color: #fff;
  font-weight: 600;
  cursor: pointer;
}
.btn-save:hover:not(:disabled) {
  background: #3d9bff;
}
.btn-save:disabled {
  opacity: 0.4;
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