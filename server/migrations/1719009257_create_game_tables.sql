CREATE TABLE Games (
    id SERIAL PRIMARY KEY,
    user_id INTEGER,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    turns INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id)
);

CREATE TYPE MarioPartyCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'DonkeyKong');
CREATE TYPE MarioParty2Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'DonkeyKong');
CREATE TYPE MarioParty3Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'DonkeyKong', 'Daisy', 'Waluigi');
CREATE TYPE MarioParty4Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'DonkeyKong', 'Daisy', 'Waluigi');
CREATE TYPE MarioParty5Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Boo', 'KoopaKid');
CREATE TYPE MarioParty6Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Boo', 'KoopaKid', 'Toadette');
CREATE TYPE MarioParty7Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Boo', 'Toadette', 'Birdo', 'DryBones');
CREATE TYPE MarioParty8Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Boo', 'Toadette', 'Birdo', 'DryBones', 'Blooper', 'HammerBro');
CREATE TYPE MarioParty9Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Birdo', 'Koopa', 'ShyGuy', 'Magikoopa');
CREATE TYPE MarioParty10Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Toadette', 'DonkeyKong', 'Rosalina', 'Bowser', 'Spike');
CREATE TYPE MarioPartyDSCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad');
CREATE TYPE MarioPartyIslandTourCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Boo', 'BowserJr');
CREATE TYPE MarioPartyStarRushCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Toad', 'Toadette', 'Rosalina', 'DonkeyKong', 'DiddyKong');
CREATE TYPE MarioPartyTop100Characters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Rosalina');
CREATE TYPE SuperMarioPartyCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Rosalina', 'Bowser', 'Goomba', 'ShyGuy', 'Koopa', 'MontyMole', 'BowserJr', 'Boo', 'HammerBro', 'DonkeyKong', 'DiddyKong', 'DryBones', 'PomPom');
CREATE TYPE MarioPartySuperstarsCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Rosalina', 'DonkeyKong', 'Birdo');
CREATE TYPE MarioPartyJamboreeCharacters AS ENUM ('Mario', 'Luigi', 'Peach', 'Yoshi', 'Wario', 'Daisy', 'Waluigi', 'Rosalina', 'Bowser', 'Goomba', 'ShyGuy', 'Koopa', 'MontyMole', 'BowserJr', 'Boo', 'Toad', 'Toadette', 'DonkeyKong', 'Birdo', 'Spike', 'Pauline', 'Ninji');

CREATE TABLE MarioPartyEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character MarioPartyCharacters NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    peak_coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    question_spaces INTEGER NOT NULL,
    minigame_spaces INTEGER NOT NULL,
    exclaimation_spaces INTEGER NOT NULL,
    mushroom_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty2Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    peak_coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    question_spaces INTEGER NOT NULL,
    exclaimation_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    battle_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    bank_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty3Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    peak_coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    question_spaces INTEGER NOT NULL,
    exclaimation_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    battle_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    bank_spaces INTEGER NOT NULL,
    game_guy_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty4Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    peak_coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    happening_spaces INTEGER NOT NULL,
    fortune_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    battle_spaces INTEGER NOT NULL,
    mushroom_spaces INTEGER NOT NULL,
    warp_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty5Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    peak_coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    capsule_spaces INTEGER NOT NULL,
    question_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    dk_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty6Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    orbs_used INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    character_spaces INTEGER NOT NULL,
    question_spaces INTEGER NOT NULL,
    duel_spaces INTEGER NOT NULL,
    miracle_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    dk_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty7Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    coins_spent_on_orbs INTEGER NOT NULL,
    orbs_used INTEGER NOT NULL,
    spaces_moved INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    character_spaces INTEGER NOT NULL,
    green_spaces INTEGER NOT NULL,
    duel_spaces INTEGER NOT NULL,
    mic_spaces INTEGER NOT NULL,
    dk_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty8Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    coins_spent_in_shop INTEGER NOT NULL,
    candy_eaten INTEGER NOT NULL,
    spaces_moved INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    green_spaces INTEGER NOT NULL,
    lucky_spaces INTEGER NOT NULL,
    dk_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioParty9Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    mini_stars INTEGER NOT NULL,
    mini_stars_lost INTEGER NOT NULL,
    minigame_stars INTEGER NOT NULL,
    minigame_stars_lost INTEGER NOT NULL,
    event_mini_stars INTEGER NOT NULL,
    event_mini_stars_lost INTEGER NOT NULL,
    bonus_star_mini_stars INTEGER NOT NULL,
    dice_block_spaces INTEGER NOT NULL,
    lucky_spaces INTEGER NOT NULL,
    unlucky_spaces INTEGER NOT NULL,
    spin_spaces INTEGER NOT NULL,
    event_spaces INTEGER NOT NULL,
    shuffle_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    captain_event_spaces INTEGER NOT NULL,
    boss_battle_spaces INTEGER NOT NULL,
    dash_spaces INTEGER NOT NULL,
    back_spaces INTEGER NOT NULL,
    free_for_all_spaces INTEGER NOT NULL,
    one_v_three_spaces INTEGER NOT NULL,
    battle_spaces INTEGER NOT NULL,
    bowser_jr_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);


CREATE TABLE MarioParty10Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    mini_stars INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioPartyDSEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioPartyIslandTourEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    green_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    dash_spaces INTEGER NOT NULL,
    blue_event_spaces INTEGER NOT NULL,
    piranha_plant_spaces INTEGER NOT NULL,
    red_event_spaces INTEGER NOT NULL,
    free_for_all_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    spaces_moved INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioPartyStarRushEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioPartyTop100Entries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE SuperMarioPartyEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    hidden_blocks INTEGER NOT NULL,
    items_used INTEGER NOT NULL,
    allies INTEGER NOT NULL,
    spaces_moved INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    lucky_spaces INTEGER NOT NULL,
    vs_spaces INTEGER NOT NULL,
    event_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    ally_spaces INTEGER NOT NULL,
    bad_luck_spaces INTEGER NOT NULL,
    extra_bad_luck_spaces INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);


CREATE TABLE MarioPartySuperstarsEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    minigame_coins INTEGER NOT NULL,
    hidden_blocks INTEGER NOT NULL,
    items_bought INTEGER NOT NULL,
    items_used INTEGER NOT NULL,
    spaces_moved INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    event_spaces INTEGER NOT NULL,
    chance_time_spaces INTEGER NOT NULL,
    lucky_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    vs_spaces INTEGER NOT NULL,
    koopa_bank_spaces INTEGER NOT NULL,
    stickers_used INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);

CREATE TABLE MarioPartyJamboreeEntries (
    id SERIAL PRIMARY KEY,
    game_id INTEGER,
    player_name TEXT NOT NULL,
    character TEXT NOT NULL,
    stars INTEGER NOT NULL,
    coins INTEGER NOT NULL,
    blue_spaces INTEGER NOT NULL,
    red_spaces INTEGER NOT NULL,
    lucky_spaces INTEGER NOT NULL,
    unlucky_spaces INTEGER NOT NULL,
    item_spaces INTEGER NOT NULL,
    bowser_spaces INTEGER NOT NULL,
    event_spaces INTEGER NOT NULL,
    chance_time_spaces INTEGER NOT NULL,
    vs_spaces INTEGER NOT NULL,
    coins_received INTEGER NOT NULL,
    minigames_won INTEGER NOT NULL,
    hidden_blocks_found INTEGER NOT NULL,
    showdown_minigames_won INTEGER NOT NULL,
    items_bought INTEGER NOT NULL,
    items_used INTEGER NOT NULL,
    spaces_traveled INTEGER NOT NULL,
    buddy_spaces_moved INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Games(id)
);
