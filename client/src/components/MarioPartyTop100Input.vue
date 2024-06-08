<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="stars">Stars</label>
    <label for="coins">Coins</label>

    <input id="player-name" />
    <input id="character" />
    <input id="stars" type="number"/>
    <input id="coins" type="number"/>
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(4, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
