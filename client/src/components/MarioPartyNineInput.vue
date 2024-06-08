<template>
  <div class="player-stats-container">
    <label for="player-name">Player Name</label>
    <label for="character">Character</label>
    <label for="mini-stars">Mini Stars</label>
    <label for="mini-stars-lost">Mini Stars Lost</label>
    <label for="minigame-stars">Mini Stars From Minigames</label>
    <label for="minigame-stars-lost">Mini Stars lost from Minigames</label>
    <label for="event-mini-stars">Mini Stars from Events</label>
    <label for="event-mini-stars-lost">Mini Stars from Events Lost</label>
    <label for="bonus-star-mini-stars">Mini Stars from Bonus Stars</label>
    <label for="dice-block-spaces">Dice Block Spaces</label>
    <label for="lucky-spaces">Lucky Spaces</label>
    <label for="unlucky-spaces">Unlucky Spaces</label>
    <label for="spin-spaces">Spin Spaces</label>
    <label for="event-spaces">Event Spaces</label>
    <label for="shuffle-spaces">Shuffle Spaces</label>
    <label for="bowser-spaces">Bowser Spaces</label>
    <label for="captain-event-spaces">Captain Event Spaces</label>
    <label for="boss-battle-spaces">Boss Battle Spaces</label>
    <label for="dash-spaces">Dash Spaces</label>
    <label for="back-spaces">Back Spaces</label>
    <label for="free-for-all-spaces">Free-for-All Spaces</label>
    <label for="1-v-3-spaces">1-vs.-3 Spaces</label>
    <label for="battle-spaces">Battle Spaces</label>
    <label for="bowser-jr-spaces">Bowser Jr. Spaces</label>

    <input id="player-name" />
    <input id="character" />
    <input id="mini-stars" type="number"/>
    <input id="mini-stars-lost" type="number" />
    <input id="minigame-stars" type="number" />
    <input id="minigame-stars-lost" type="number" />
    <input id="event-mini-stars" type="number" />
    <input id="event-mini-stars-lost" type="number" />
    <input id="bonus-star-mini-stars" type="number" />
    <input id="dice-block-spaces" type="number" />
    <input id="lucky-spaces" type="number" />
    <input id="unlucky-spaces" type="number" />
    <input id="spin-spaces" type="number" />
    <input id="event-spaces" type="number" />
    <input id="shuffle-spaces" type="number" />
    <input id="bowser-spaces" type="number" />
    <input id="captain-event-spaces" type="number" />
    <input id="boss-battle-spaces" type="number" />
    <input id="dash-spaces" type="number" />
    <input id="back-spaces" type="number" />
    <input id="free-for-all-spaces" type="number" />
    <input id="1-v-3-spaces" type="number" />
    <input id="battle-spaces" type="number" />
    <input id="bowser-jr-spaces" type="number" />
  </div>
</template>

<style scoped>
  .player-stats-container {
    display: grid;
    grid-template-columns: repeat(24, auto);
    gap: 10px;
  }
 
  input {
    display: flex;
    min-width: 0;
  }
</style>
