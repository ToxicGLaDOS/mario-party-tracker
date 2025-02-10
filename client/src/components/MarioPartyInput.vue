<script setup lang="ts">
  import type { PropType } from 'vue'
  interface Field {
    name: string,
    ty: string
  }

  const props = defineProps({
    input_schema: {
      type: Object as PropType<Array<Field>>,
      required: true
    }
  })
</script>

<template>
  <div class="player-stats-container">
    <div v-for="field in input_schema" class="item">
      <label :for="field.name">{{ field.name }}</label>
      <input :id="field.name" :name="field.name" :type="field.ty== 'i32' ? 'number': 'text'" />
    </div>
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .item {
    display: flex;
    flex-direction: column;
    padding-left: 10px;
    padding-right: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }

  label {
    display: flex;
  }
</style>
