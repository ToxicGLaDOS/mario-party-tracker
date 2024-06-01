<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="stars">Stars</label>
    <label for="coins">Coins</label>
    <label for="minigame-coins">Minigame Coins</label>
    <label for="peak-coins">Peak Coins</label>
    <label for="blue-spaces">Blue Spaces</label>
    <label for="red-spaces">Red Spaces</label>
    <label for="question-spaces">Question Spaces</label>
    <label for="fortune-spaces">Fortune Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>
    <label for="battle-spaces">Battle Spaces</label>
    <label for="mushroom-spaces">Mushroom Spaces</label>

    <input id="player-name" />
    <input id="character" />
    <input id="stars" type="number" />
    <input id="coins" type="number" />
    <input id="minigame-coins" type="number" />
    <input id="peak-coins" type="number" />
    <input id="blue-spaces" type="number" />
    <input id="red-spaces" type="number" />
    <input id="question-spaces" type="number" />
    <input id="fortune-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
    <input id="battle-spaces" type="number" />
    <input id="mushroom-spaces" type="number" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(13, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
