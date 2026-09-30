<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>


<template>
  <div class="manager-banner">
    <h3>{{ t('banner.managerTitle') }}</h3>
  </div>
</template>


<style scoped>
.manager-banner {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  justify-content: center;
  align-items: center;
}

h3 {
  font-size: 2rem;
}
</style>
