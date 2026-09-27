<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>


<template>
  <div class="manager-logic">
    <h3>{{ t('logic.managerTitle') }}</h3>
  </div>
</template>


<style scoped>
.manager-logic {
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
