<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="stars">Stars</label>
    <label for="coins">Coins</label>
    <label for="minigame-coins">Minigame Coins</label>
    <label for="orbs-used">Orbs Used</label>
    <label for="blue-spaces">Blue Spaces</label>
    <label for="red-spaces">Red Spaces</label>
    <label for="character-spaces">Character Spaces</label>
    <label for="question-spaces">Question Spaces</label>
    <label for="duel-spaces">Duel Spaces</label>
    <label for="miracle-spaces">Miracle Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>
    <label for="dk-spaces">DK Spaces</label>

    <input id="player-name" />
    <input id="character" />
    <input id="stars" type="number" />
    <input id="coins" type="number" />
    <input id="minigame-coins" type="number" />
    <input id="orbs-used" type="number" />
    <input id="blue-spaces" type="number" />
    <input id="red-spaces" type="number" />
    <input id="character-spaces" type="number" />
    <input id="question-spaces" type="number" />
    <input id="duel-spaces" type="number" />
    <input id="miracle-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
    <input id="dk-spaces" type="number" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(14, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
