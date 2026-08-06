<script setup lang="ts">
import { computed, onUnmounted, ref, watch } from 'vue';
import { WishResponse } from '../types';

const props = defineProps<{
  result: WishResponse | null,  // 单抽结果
  isTenWish: boolean,
  tenResults?: WishResponse[],   // 十连结果
}>();

const emit = defineEmits<{
  (e: 'close'): void
}>();

// 计时器间隔
const timerInterval = 2000;

// 阶段控制
const phase = ref<'playing' | 'summary' | 'closing'>('playing');
const isClosing = ref(false);


// 十连状态
const currentIndex = ref(0);
let continueHintTimer: ReturnType<typeof setTimeout> | null = null;
let closeHintTimer: ReturnType<typeof setTimeout> | null = null;

// 当前展示卡片
const currentCard = computed(() => {
  if (props.isTenWish && props.tenResults) {
    return props.tenResults[currentIndex.value] || null;
  }
  return null;
})

const isTenCompleted = computed(() => {
  if (!props.isTenWish || !props.tenResults) return false;
  const res = currentIndex.value >= props.tenResults.length - 1;
  return res;
})

function goToSummary() {
  if (phase.value === 'playing') {
    phase.value = 'summary';
    showContinueHint.value = false;
    if (continueHintTimer) clearTimeout(continueHintTimer);
    restartCloseHintTimer();
  }
}

function closeOverlay() {
  if (isClosing.value) return;
  isClosing.value = true;
  setTimeout(() => {
    emit('close');
  }, 300);
}

const showContinueHint = ref(false);
const showCloseHint = ref(false);

const summaryStats = computed(() => {
  if (!props.tenResults) return null;
  const stats: Record<string, number> = {};
  props.tenResults.forEach(r => {
    const rarityTag = r.tags.find(t => t.namespace === "rarity");
    const rarity = rarityTag ? rarityTag.value : "unknown";
    stats[rarity] = (stats[rarity] || 0) + 1;
  })
  return stats
});

function restartCloseHintTimer() {
  if (closeHintTimer) clearTimeout(closeHintTimer);
  closeHintTimer = setTimeout(() => {
    showCloseHint.value = true;
  }, timerInterval)
}

function restartContinueHintTimer() {
  if (continueHintTimer) clearTimeout(continueHintTimer);
  if (!props.isTenWish) return;
  continueHintTimer = setTimeout(() => {
    showContinueHint.value = true;
  }, timerInterval)
}

watch(() => props.isTenWish, (newVal) => {
  if (newVal) {
    restartContinueHintTimer();
  } else {
    restartCloseHintTimer();
  }
}, { immediate: true });

onUnmounted(() => {
  if (continueHintTimer) clearTimeout(continueHintTimer);
  if (closeHintTimer) clearTimeout(closeHintTimer);
})

// 处理点击 单抽关闭 十连继续/展示汇总结果/关闭
function handleOverlayClick() {
  if (isClosing.value) return;
  if (phase.value === 'summary') {
    closeOverlay();
    return;
  }
  if (props.isTenWish) {
    if (isTenCompleted.value) {
      phase.value = 'summary';
      showContinueHint.value = false;
      if (continueHintTimer) clearTimeout(continueHintTimer);
      restartCloseHintTimer();
    } else {
      if (currentIndex.value < (props.tenResults?.length || 0) - 1) {
        currentIndex.value++;
        showContinueHint.value = false;
        restartContinueHintTimer();
      }
    }
  } else {
    closeOverlay();
  }
}

function handleSkip() {
  if (isClosing.value) return;
  if (phase.value == 'playing') {
    goToSummary();
  }
}

function getRarity(item: WishResponse): string {
  const tag = item.tags.find(t => t.namespace === "rarity");
  return tag ? tag.value : "unknown";
}
</script>


<template>
  <div class="overlay" :class="{ closing: isClosing }" @click="handleOverlayClick">
    <div class="overlay-content" @click.stop>

      <!-- 单抽 -->
      <template v-if="!isTenWish && result">
        <div class="card-content animate-in">
          {{ result.content }}
        </div>
        <div v-if="result.tags.length" class="tags">
          <span
            v-for="tag in result.tags"
            :key="tag.namespace" class="tag"
            :class="tag.namespace === 'rarity' ? `rarity-${tag.value}` : ''"
          >
            {{ tag.value }}
          </span>
        </div>
        <div v-if="result.event_tags.length" class="event-tags">
          <span v-for="et in result.event_tags" :key="et" class="event-tag" :class="`event-${et}`">
            {{ et }}
          </span>
        </div>
      </template>

      <template v-if="isTenWish && tenResults?.length">
        <Transition name="ten-player-summary-switch" mode="out-in">
          <!-- 播放 -->
          <div v-if="phase === 'playing'" key="playing" class="ten-player">
            <div class="ten-counter">
              {{ currentIndex + 1 }} / {{ tenResults.length }}
            </div>
            <Transition name="card-switch" mode="out-in">
              <div :key="currentIndex" class="card-wrapper">
                <div class="card-content animate-in">
                  {{ currentCard?.content }}
                </div>
                <div v-if="currentCard?.tags.length" class="tags">
                  <span
                    v-for="tag in currentCard.tags"
                    :key="tag.namespace"
                    class="tag"
                    :class="tag.namespace === 'rarity' ? `rarity-${tag.value}` : ''"
                  >
                    {{ tag.value }}
                  </span>
                </div>
                <div v-if="currentCard?.event_tags.length" class="event-tags">
                  <span v-for="et in currentCard.event_tags" :key="et" class="event-tag" :class="`event-${et}`">
                    {{ et }}
                  </span>
                </div>
              </div>
            </Transition>
          </div>

          <!-- 汇总 -->
          <div v-else-if="phase === 'summary'" key="summary">
            <div class="summary-stats">
              <div v-for="(count, rarity) in summaryStats" :key="rarity" class="stat-item" :class="`rarity-${rarity}`">
                Rarity {{ rarity }} : {{ count }}
              </div>
            </div>
            <div class="summary-list">
              <div v-for="(item, idx) in tenResults" :key="idx" class="summary-item" :class="`rarity-${getRarity(item)}`">
                {{ item.content }}
              </div>
            </div>
          </div>
        </Transition>
      </template>
      <div v-if="isTenWish && tenResults?.length && phase == 'playing'" class="skip-btn" @click.stop="handleSkip">
        跳过 >
      </div>
      <div v-if="showContinueHint" class="click-hint">点击以继续</div>
      <div v-if="showCloseHint" class="click-hint">点击以关闭</div>
    </div>
  </div>
</template>


<style scoped>
@keyframes popIn {
  0% { transform: scale(0.5); opacity: 0; }
  100% { transform: scale(1); opacity: 1; }
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

@keyframes fadeOut {
  0% { opacity: 1; }
  100% { opacity: 0; }
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  backdrop-filter: blur(12px);
  z-index: 999;                         /* 覆盖整个界面 */
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.4s ease;
}

.overlay.closing {
  animation: fadeOut 0.3s ease forwards;
}

.overlay-content {
  text-align: center;
  padding: 40px;
  max-width: 600px;
  width: 90%;
}

.skip-btn {
  position: fixed;
  top: 6px;
  right: 0;
  color: #b0c0d0;
  padding: 4px 16px;
  cursor: pointer;
  font-size: 1.5rem;
}

.ten-counter {
  font-size: 1.5rem;
  color: #8a99b4;
  margin-bottom: 16px;
  letter-spacing: 2px;
  user-select: none;
}

.card-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.card-content {
  font-size: 4rem;
  font-weight: bold;
  color: #fff;
  margin-bottom: 12px;
  text-shadow: 0 2px 15px rgba(255, 255, 255, 0.5);
}

.animate-in {
  animation: popIn 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.card-switch-enter-active,
.card-switch-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}
.card-switch-enter-from {
  opacity: 0;
  transform: scale(0.8);
}
.card-switch-leave-to {
  opacity: 0;
  transform: scale(1.2);
}

.ten-player-summary-switch-enter-active,
.ten-player-summary-switch-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}
.ten-player-summary-switch-enter-from {
  opacity: 0;
  transform: scale(0.8);
}
.ten-player-summary-switch-leave-to {
  opacity: 0;
  transform: scale(1.2);
}

.tags {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 8px;
  margin-top: 8px;
}

.tag {
  background: rgba(255, 255, 255, 0.1);
  padding: 3px 16px;
  border-radius: 100px;
  font-size: 1.4rem;
  color: #b0c0d0;
  transition: all 0.2s ease;
}
.tag:hover {
  transform: translateY(-3px);
}

.event-tags {
  margin-top: 12px;
}

.event-tag {
  display: inline-block;
  padding: 3px 20px;
  border-radius: 100px;
  font-weight: bold;
  font-size: 1.4rem;
  transition: all 0.2s ease;
}
.event-tag:hover {
  transform: translateY(-3px);
}

.click-hint {
  position: absolute;
  left: calc((50% - 100px));
  bottom: 40px;
  width: 200px;
  height: 20px;
  text-align: center;
  margin-top: 40px;
  color: #6a7a90;
  font-size: 1.3rem;
  letter-spacing: 1px;
  user-select: none;
  pointer-events: none;
  animation: fadeIn 0.6s ease;
  text-shadow: 0 2px 25px rgba(255, 255, 255, 0.9);
}

.summary-stats {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 20px;
}

.stat-item {
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(4px);
  padding: 6px 20px;
  border-radius: 100px;
  font-size: 1.5rem;
  font-weight: 600;
  color: #e8edf5;
  box-shadow: 0 0 20px rgba(255, 215, 0, 0.1);
  transition: all 0.2s ease;
}
.stat-item:hover {
  transform: translateY(-3px);
}

.summary-list {
  max-height: 300px;
  overflow-y: auto;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
  padding-top: 4px;
}

.summary-item {
  background: rgba(255, 255, 255, 0.06);
  padding: 4px 16px;
  border-radius: 100px;
  font-size: 1.3rem;
  color: #c8d4e8;
  transition: all 0.2s ease;
}
.summary-item:hover {
  transform: translateY(-3px);
}

.rarity-5,
.rarity-S {
  background: rgba(255, 228, 73, 0.15);
  color: #ffd700;
}
.rarity-4,
.rarity-A {
  background: rgba(180, 138, 255, 0.15);
  color: #b48aff;
}
.rarity-3,
.rarity-B {
  background: rgba(107, 184, 255, 0.12);
  color: #6bb8ff;
}

.event-up {
  background: #f7b731;
  color: #0b0e14;
}
.event-standard {
  background: rgba(255, 255, 255, 0.1);
}
</style>