<script setup lang="ts">
  import { computed } from 'vue'
  import type { PropType } from 'vue'
  interface Field {
    name: string,
    thetype: string
  }

  const props = defineProps({
    input_schema: {
      type: Object as PropType<Array<Field>>,
      required: true
    }
  })

  const columns = computed(() => {
    return props.input_schema.length;
  })
</script>

<template>
  <div class="player-stats-container">
    <label v-for="field in input_schema" :for="field.name">{{ field.name }}</label>
    <input v-for="field in input_schema" :id="field.name" :name="field.name" :type="field.thetype == 'i32' ? 'number': 'text'" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(v-bind(columns), auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
