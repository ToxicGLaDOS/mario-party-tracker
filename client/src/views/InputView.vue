<script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import type { Ref } from 'vue'
  import MarioPartyInput from "@/components/MarioPartyInput.vue";

  const selected_game = defineModel<string>();

  const possible_boards: Ref<string[]> = ref([])

  const all_boards: {[key: string]: string[]} = {
    "Mario Party": [
      "DK's Jungle Adventure",
      "Peach's Birthday Cake",
      "Yoshi's Tropical Island",
      "Wario's Battle Canyon",
      "Luigi's Engine Room",
      "Mario's Rainbow Castle",
      "Bowser's Magma Mountain",
      "Eternal Star",
      "Mini-Game Stadium"
    ],
    "Mario Party 2": [
      "Pirate Land",
      "Western Land",
      "Space Land",
      "Mystery Land",
      "Horror Land",
      "Bowser Land",
      "Mini-Game Trial"
    ],
    "Mario Party 3": [
      "Chilly Waters",
      "Deep Bloober Sea",
      "Spiny Desert",
      "Woody Woods",
      "Creepy Cavern",
      "Waluigi's Island"
      // Not inluding duel boards for now
    ],
    "Mario Party 4": [
      "Toad's Midway Madness",
      "Goomba's Greedy Gala",
      "Boo's Haunted Bash",
      "Koopa's Seaside Soiree",
      "Shy Guy's Jungle Jam",
      "Bowser's Gnarly Party",
      "Mega Board Mayhem",
      "Mini Board Mad-Dash"
    ],
    "Mario Party 5": [
      "Toy Dream",
      "Rainbow Dream",
      "Pirate Dream",
      "Undersea Dream",
      "Future Dream",
      "Sweet Dream",
      "Bowser Nightmare"
    ],
    "Mario Party 6": [
      "Towering Treetop",
      "E. Gadd's Garage",
      "Faire Square",
      "Snowflake Lake",
      "Castaway Bay",
      "Clockwork Castle",
      "Thirsty Gulch",
      "Astro Avenue",
      "Infernal Tower"
    ],
    "Mario Party 7": [
      "Grand Canal",
      "Pagoda Peak",
      "Pyramid Park",
      "Neon Heights",
      "Windmillville",
      "Bowser's Enchanted Inferno!"
    ],
    "Mario Party 8": [
      "DK's Treetop Temple",
      "Goomba's Booty Boardwalk",
      "King Boo's Haunted Hideaway",
      "Shy Guy's Perplex Express",
      "Koopa's Tycoon Town",
      "Bowser's Warped Orbit"
    ],
    "Mario Party DS": [
      "Wiggler's Garden",
      "Toadette's Music Room",
      "DK's Stone Statue",
      "Kamek's Library",
      "Bowser's Pinball Machine"
    ],
    "Mario Party 9": [
      "Toad Road",
      "Bob-omb Factory",
      "Boo's Horror Castle",
      "Blooper Beach",
      "Magma Mine",
      "Bowser Station",
      "DK's Jungle Ruins",
    ],
    "Mario Party: Island Tour": [
      "Perilous Palace Path",
      "Rocket Road",
      "Shy Guy's Shuffle City",
      "Banzai Bill's Mad Mountain",
      "Star-Crossed Skyway",
      "Kamek's Carpet Ride",
      "Bowser's Peculiar Peak"
    ],
    "Mario Party 10": [
      "Mushroom Park",
      "Haunted Trail",
      "Whimsical Waters",
      "Airship Central",
      "Chaos Castle"
    ],
    "Mario Party: Star Rush": [
      "World 0-1",
      "World 0-2",
      "World 0-3",
      "World 1-1",
      "World 1-2",
      "World 1-3",
      "World 2-1",
      "World 2-2",
      "World 2-3",
      "World 3-1",
      "World 3-2",
      "World 3-3",
      "World 4-1",
      "World 4-2",
      "World 4-3",
      "Coinathlon",
      "Map 1 (Balloon Bash)",
      "Map 2 (Balloon Bash)",
      "Map 3 (Balloon Bash)"
    ],
    "Mario Party: The Top 100": [
      "Minigame Match"
    ],
    "Super Mario Party": [
      "Whomp's Domino Ruins",
      "King Bob-omb's Powderkeg Mine",
      "Megafruit Paradise",
      "Kamek's Tantalizing Tower"
    ],
    "Mario Party Superstars": [
      "Peach's Birthday Cake",
      "Yoshi's Tropical Island",
      "Space Land",
      "Horror Land",
      "Woody Woods",
    ],
    "Mario Party Jamboree": [
      "Mega Wiggler's Tree Party",
      "Roll 'em Raceway",
      "Rainbow Galleria",
      "Goomba Lagoon",
      "King Bowser's Keep",
      "Western Land",
      "Mario's Rainbow Castle",
    ]
  }

  const snackbar_message = ref("Default message");
  const snackbar_show = ref(false);

  function on_game_change(event: Event) {
    if (event.target instanceof HTMLSelectElement) {
      possible_boards.value = all_boards[event.target.value];

      const board_select = document.getElementById("board");
      if (board_select instanceof HTMLSelectElement) {
        board_select.value = "disabled";
      }
    }
  }

  var form: HTMLFormElement;

  onMounted(() => {
    form = document.querySelector("#input_form") as HTMLFormElement;

    // Take over form submission
    form.addEventListener("submit", (event) => {
      event.preventDefault();
      sendData();
    });
  })

  async function sendData() {
    // Associate the FormData object with the form element
    const formData = new FormData(form);

    var data: any = {
      player_data: [
        {},
        {},
        {},
        {}
      ]
    }

    for (var key of formData.keys()) {
      var values = formData.getAll(key);

      // values.length is how many times we see this entry
      // so basically == 4 means stuff that happens 4 times
      // == 1 is stuff that happens once (board, turns, time, etc.)
      if (values.length == 4) {
        for (var i = 0; i < values.length; i++) {
          if ((document.getElementsByName(key)[0] as (HTMLSelectElement|HTMLInputElement)).type == "number") {

            // TODO: Remove this, we don't want defaults
            var value = parseInt(values[i].toString());
            if (isNaN(value)) {
              data.player_data[i][key] = 0;
            }
            else {
              data.player_data[i][key] = value;
            }
          }
          else {
            data.player_data[i][key] = values[i];
          }
        }
      }
      else if (values.length == 1) {
        if (key == "turns") { 
          // TODO: Remove this, we don't want defaults
          var value = parseInt(values[0].toString());
          if (isNaN(value)) {
            data[key] = 0;
          }
          else {
            data[key] = value;
          }

        }
        else if (key == "date") {
          var date = new Date(values[0].toString())
          // toISONString always returns UTC time
          data[key] = date.toISOString()
        }
        else {
          data[key] = values[0];
        }
      }
      else {
        throw Error("Unexpected number of values in form");
      }
    }

    var response = await fetch("/api/games", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data)
    })

    var responseMessage = "";
    response.json()
      .then((data) => {
        responseMessage = data.message;
        showSnackbar(responseMessage);
      })
  }

  var characters_json = await fetch("/api/characters")
    .then( response => response.json());

  var input_schemas = await fetch("/api/input/schema")
    .then( response => response.json());

  const input_schema = computed(() => {
    if (selected_game.value !== undefined && selected_game.value in input_schemas) {
      return input_schemas[selected_game.value];
    }
    else {
      return [];
    }
  })

  const characters = computed(() => {
    if (selected_game.value !== undefined && selected_game.value in characters_json) {
      return characters_json[selected_game.value];
    }
    else {
      return [];
    }
  })

  function showSnackbar(message: string) {
    snackbar_message.value = message
    snackbar_show.value = true;
    setTimeout(function(){
      snackbar_show.value = false;
    }, 3000);
  }
</script>

<template>
  <main> 
    <form id="input_form">
      <div class="vertical-align">
        <h1 class="title">
          Mario Party Data Input
        </h1>
        <div class="game-info-center">
          <div class="game-info-container">
            <div class="game-item">
              <label for="game">Game</label>
              <select name="game" id="game" v-on:input="on_game_change" v-model="selected_game">
                <option disabled selected value> -- Game -- </option>
                <option value="Mario Party">Mario Party</option>
                <option value="Mario Party 2">Mario Party 2</option>
                <option value="Mario Party 3">Mario Party 3</option>
                <option value="Mario Party 4">Mario Party 4</option>
                <option value="Mario Party 5">Mario Party 5</option>
                <option value="Mario Party 6">Mario Party 6</option>
                <option value="Mario Party 7">Mario Party 7</option>
                <option value="Mario Party 8">Mario Party 8</option>
                <option value="Mario Party DS">Mario Party DS</option>
                <option value="Mario Party 9">Mario Party 9</option>
                <option value="Mario Party: Island Tour">Mario Party: Island Tour</option>
                <option value="Mario Party 10">Mario Party 10</option>
                <option value="Mario Party: Star Rush">Mario Party: Star Rush</option>
                <option value="Mario Party: The Top 100">Mario Party: The Top 100</option>
                <option value="Super Mario Party">Super Mario Party</option>
                <option value="Mario Party Superstars">Mario Party Superstars</option>
                <option value="Mario Party Jamboree">Mario Party Jamboree</option>
              </select>
            </div>
            <div class="game-item" id="board-container">
              <label for="board">Board</label>
              <select name="board" id="board">
                <option disabled selected value="disabled"> -- Board -- </option>
                <option :value=board v-for="board in possible_boards">{{board}}</option>
              </select>
            </div>
            <div class="game-item">
              <label for="turns" type="number">Turns</label>
              <input name="turns" id="turns" type="number" />
            </div>
            <div class="game-item">
              <label for="date" type="date">Date</label>
              <input name="date" id="date" type="datetime-local" />
            </div>
          </div>
        </div>

        <div class="individual-inputs-container">
          <MarioPartyInput v-for="_ in 4" :input_schema="input_schema" :characters="characters" />
        </div>

        <input type="submit" class="submit-button" value="Submit" />
      </div>
    </form>
    <div id="snackbar" :class="snackbar_show ? 'show' : ''">{{snackbar_message}}</div>
  </main>
</template>

<style scoped>
  main {
    display: flex;
    min-width: 0;
  }

  select {
    display: flex;
    min-width: 0;
  }

  #board-container {
    flex-grow: 1;
  }

  .title {
    align-self: center;
    font-size: 75px;
  }
  
  .submit-button {
    align-self: center;
    margin: 50px;
  }

  .game-info-center {
    width: 100%;
    flex-direction: column;
    display: flex;
    align-items: center;
  }

  .game-item {
    display: flex;
    flex-direction: column;
  }

  .game-info-container {
    display: flex;
    padding-top: 20px;
    padding-bottom: 20px;
    gap: 10px;
    width: 100%;
  }

  .individual-inputs-container {
    display: flex;
    align-items: center;
  }

  .vertical-align {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
  }

  #snackbar {
    visibility: hidden;
    min-width: 250px;
    margin-left: -125px;
    background-color: #333;
    color: #fff;
    text-align: center;
    border-radius: 2px;
    padding: 16px;
    position: fixed;
    z-index: 1;
    right: 30px;
    top: 30px;
    font-size: 17px;
  }

  #snackbar.show {
    visibility: visible;
    -webkit-animation: fadein 0.5s, fadeout 0.5s 2.5s;
    animation: fadein 0.5s, fadeout 0.5s 2.5s;
  }

  @-webkit-keyframes fadein {
    from {top: 0; opacity: 0;}
    to {top: 30px; opacity: 1;}
  }

  @keyframes fadein {
    from {top: 0; opacity: 0;}
    to {top: 30px; opacity: 1;}
  }

  @-webkit-keyframes fadeout {
    from {top: 30px; opacity: 1;}
    to {top: 0; opacity: 0;}
  }

  @keyframes fadeout {
    from {top: 30px; opacity: 1;}
    to {top: 0; opacity: 0;}
  }

</style>
