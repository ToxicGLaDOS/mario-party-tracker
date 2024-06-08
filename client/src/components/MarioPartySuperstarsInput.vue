<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="stars">Stars</label>
    <label for="coins">Coins</label>
    <label for="minigame-coins">Minigame Coins</label>
    <label for="hidden-blocks">Hidden Blocks</label>
    <label for="items-bought">Items Bought</label>
    <label for="items-used">Items Used</label>
    <label for="spaces-moved">Spaces Moved</label>
    <label for="blue-spaces">Blue Spaces</label>
    <label for="red-spaces">Red Spaces</label>
    <label for="item-spaces">Item Spaces</label>
    <label for="event-spaces">Event Spaces</label>
    <label for="chance-time-spaces">Chance Time Spaces</label>
    <label for="lucky-spaces">Lucky Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>
    <label for="vs-spaces">VS Spaces</label>
    <label for="koopa-bank-spaces">Koopa Bank Spaces</label>
    <label for="stickers-used">Stickers Used</label>

    <input id="player-name" />
    <input id="character" />
    <input id="stars" type="number" />
    <input id="coins" type="number" />
    <input id="minigame-coins" type="number" />
    <input id="hidden-blocks" type="number" />
    <input id="items-bought" type="number" />
    <input id="items-used" type="number" />
    <input id="spaces-moved" type="number" />
    <input id="blue-spaces" type="number" />
    <input id="red-spaces" type="number" />
    <input id="item-spaces" type="number" />
    <input id="event-spaces" type="number" />
    <input id="chance-time-spaces" type="number" />
    <input id="lucky-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
    <input id="vs-spaces" type="number" />
    <input id="koopa-bank-spaces" type="number" />
    <input id="stickers-used" type="number" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(19, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
