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
    },
    characters: {
      type: Object as PropType<Array<Field>>,
      required: true
    }
  })

  // Spacecase is a made up name
  // Converts foo_bar -> Foo bar
  function snakecaseToSpacecase(str: String) {
    var s = str.replaceAll("_", " ");
    s = String(s[0]).toUpperCase() + String(s).slice(1);
    return s;
  }
</script>

<template>
  <div class="player-stats-container">
    <div v-for="field in input_schema" class="item">
      <label :for="field.name">{{ snakecaseToSpacecase(field.name) }}</label>
      <input v-if="field.name != 'character'" :id="field.name" :name="field.name" :type="field.ty== 'i32' ? 'number': 'text'" />
      <!-- TODO: I hate this special casing, but the backend stuff to get enums in the input schema
        is hard and probably requires a rewrite of listfields-derive or something.
        But doing it would let me remove the /api/characters endpoint and just use input schema,
        so it's probably worth it. eventually...
      -->
      <select v-if="field.name == 'character'" :id="field.name" :name="field.name">
        <option disabled selected value> -- Character -- </option>
        <option v-for="character in characters" :value="character">{{ character }}</option>
      </select>
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
