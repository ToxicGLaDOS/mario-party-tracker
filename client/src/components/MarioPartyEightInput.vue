<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="stars">Stars</label>
    <label for="coins">Coins</label>
    <label for="minigame-coins">Minigame Coins</label>
    <label for="coins-spent-in-shop">Coins spent in shop</label>
    <label for="candy-eaten">Candy Eaten</label>
    <label for="spaces-moved">Spaces Moved</label>
    <label for="blue-spaces">Blue Spaces</label>
    <label for="red-spaces">Red Spaces</label>
    <label for="green-spaces">Green Spaces</label>
    <label for="lucky-spaces">Lucky Spaces</label>
    <label for="dk-spaces">DK Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>

    <input id="player-name" />
    <input id="character" />
    <input id="stars" type="number" />
    <input id="coins" type="number" />
    <input id="minigame-coins" type="number" />
    <input id="coins-spent-in-shop" type="number" />
    <input id="candy-eaten" type="number" />
    <input id="spaces-moved" type="number" />
    <input id="blue-spaces" type="number" />
    <input id="red-spaces" type="number" />
    <input id="green-spaces" type="number" />
    <input id="lucky-spaces" type="number" />
    <input id="dk-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
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
