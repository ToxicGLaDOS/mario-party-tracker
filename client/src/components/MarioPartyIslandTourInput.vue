<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="green-spaces">Green Spaces</label>
    <label for="item-spaces">Item Spaces</label>
    <label for="dash-spaces">Dash Spaces</label>
    <label for="blue-event-spaces">Blue Event Spaces</label>
    <label for="piranha-plant-spaces">Piranha Plant Spaces</label>
    <label for="red-event-spaces">Red Event Spaces</label>
    <label for="free-for-all-spaces">Free-For-All Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>
    <label for="spaces-moved">Spaces Moved</label>

    <input id="player-name" />
    <input id="character" />
    <input id="green-spaces" type="number" />
    <input id="item-spaces" type="number" />
    <input id="dash-spaces" type="number" />
    <input id="blue-event-spaces" type="number" />
    <input id="piranha-plant-spaces" type="number" />
    <input id="red-event-spaces" type="number" />
    <input id="free-for-all-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
    <input id="spaces-moved" type="number" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(11, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
