use serde::{Serialize, Deserialize, Deserializer, de, de::{Visitor, Unexpected, Error}};
use std::collections::HashMap;
use std::fmt;
use listfields_derive::ListFields;
use crate::listfields::{ObjectData, EnumData, Variant, ListFields, Field};

#[derive(Deserialize, Debug)]
pub struct GameData {
    #[serde(flatten)]
    pub player_data: MarioPartyData,
    pub board: String,
    pub turns: i32 
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    peak_coins: i32,
    blue_spaces: i32,
    red_spaces: i32,
    question_spaces: i32,
    minigame_spaces: i32,
    exclaimation_spaces: i32,
    mushroom_spaces: i32,
    bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty2 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    peak_coins: i32,
    blue_spaces: i32,
    red_spaces: i32,
    question_spaces: i32,
    exclaimation_spaces: i32,
    bowser_spaces: i32,
    battle_spaces: i32,
    item_spaces: i32,
    bank_spaces: i32,
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty3 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    peak_coins: i32,
    blue_spaces: i32,
    red_spaces: i32,
    question_spaces: i32,
    exclaimation_spaces: i32,
    bowser_spaces: i32,
    battle_spaces: i32,
    item_spaces: i32,
    bank_spaces: i32,
    game_guy_spaces: i32,
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty4 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    peak_coins: i32,
    blue_spaces: i32,
    red_spaces: i32,
    happening_spaces: i32,
    fortune_spaces: i32,
    bowser_spaces: i32,
    battle_spaces: i32,
    mushroom_spaces: i32,
    warp_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty5 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    peak_coins: i32,
    blue_spaces: i32,
    red_spaces: i32,
    capsule_spaces: i32,
    question_spaces: i32,
    bowser_spaces: i32,
    dk_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty6 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    orbs_used: i32,
    blue_spaces: i32,
    red_spaces: i32,
    character_spaces: i32,
    question_spaces: i32,
    duel_spaces: i32,
    miracle_spaces: i32,
    bowser_spaces: i32,
    dk_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty7 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    coins_spent_on_orbs: i32,
    orbs_used: i32,
    spaces_moved: i32,
    blue_spaces: i32,
    red_spaces: i32,
    character_spaces: i32,
    green_spaces: i32,
    duel_spaces: i32,
    mic_spaces: i32,
    dk_spaces: i32,
    bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty8 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    coins_spent_in_shop: i32,
    candy_eaten: i32,
    spaces_moved: i32,
    blue_spaces: i32,
    red_spaces: i32,
    green_spaces: i32,
    lucky_spaces: i32,
    dk_spaces: i32,
    bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty9 {
    player_name: String,
    character: String,
    mini_stars: i32,
    mini_stars_lost: i32,
    minigame_stars: i32,
    minigame_stars_lost: i32,
    event_mini_stars: i32,
    event_mini_stars_lost: i32,
    bonus_star_mini_stars: i32,
    dice_block_spaces: i32,
    lucky_spaces: i32,
    unlucky_spaces: i32,
    spin_spaces: i32,
    event_spaces: i32,
    shuffle_spaces: i32,
    bowser_spaces: i32,
    captain_event_spaces: i32,
    boss_battle_spaces: i32,
    dash_spaces: i32,
    back_spaces: i32,
    free_for_all_spaces: i32,
    one_v_three_spaces: i32,
    battle_spaces: i32,
    bowser_jr_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioParty10 {
    player_name: String,
    character: String,
    mini_stars: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioPartyDS {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioPartyIslandTour {
    player_name: String,
    character: String,
    green_spaces: i32,
    item_spaces: i32,
    dash_spaces: i32,
    blue_event_spaces: i32,
    piranha_plant_spaces: i32,
    red_event_spaces: i32,
    free_for_all_spaces: i32,
    bowser_spaces: i32,
    spaces_moved: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioPartyStarRush {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioPartyTop100 {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
struct SuperMarioParty {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    hidden_blocks: i32,
    items_used: i32,
    allies: i32,
    spaces_moved: i32,
    blue_spaces: i32,
    red_spaces: i32,
    lucky_spaces: i32,
    vs_spaces: i32,
    event_spaces: i32,
    item_spaces: i32,
    ally_spaces: i32,
    bad_luck_spaces: i32,
    extra_bad_luck_spaces: i32
}


#[derive(Deserialize, ListFields, Clone, Debug)]
struct MarioPartySuperstars {
    player_name: String,
    character: String,
    stars: i32,
    coins: i32,
    minigame_coins: i32,
    hidden_blocks: i32,
    items_bought: i32,
    items_used: i32,
    spaces_moved: i32,
    blue_spaces: i32,
    red_spaces: i32,
    item_spaces: i32,
    event_spaces: i32,
    chance_time_spaces: i32,
    lucky_spaces: i32,
    bowser_spaces: i32,
    vs_spaces: i32,
    koopa_bank_spaces: i32,
    stickers_used: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
#[serde(tag = "game", content = "player_data")]
pub enum MarioPartyData {
    #[serde(rename = "Mario Party")]
    MarioParty(Vec<MarioParty>),
    #[serde(rename = "Mario Party 2")]
    MarioParty2(Vec<MarioParty2>),
    #[serde(rename = "Mario Party 3")]
    MarioParty3(Vec<MarioParty3>),
    #[serde(rename = "Mario Party 4")]
    MarioParty4(Vec<MarioParty4>),
    #[serde(rename = "Mario Party 5")]
    MarioParty5(Vec<MarioParty5>),
    #[serde(rename = "Mario Party 6")]
    MarioParty6(Vec<MarioParty6>),
    #[serde(rename = "Mario Party 7")]
    MarioParty7(Vec<MarioParty7>),
    #[serde(rename = "Mario Party 8")]
    MarioParty8(Vec<MarioParty8>),
    #[serde(rename = "Mario Party 9")]
    MarioParty9(Vec<MarioParty9>),
    #[serde(rename = "Mario Party 10")]
    MarioParty10(Vec<MarioParty10>),
    #[serde(rename = "Mario Party DS")]
    MarioPartyDS(Vec<MarioPartyDS>),
    #[serde(rename = "Mario Party: Island Tour")]
    MarioPartyIslandTour(Vec<MarioPartyIslandTour>),
    #[serde(rename = "Mario Party: Star Rush")]
    MarioPartyStarRush(Vec<MarioPartyStarRush>),
    #[serde(rename = "Mario Party: The Top 100")]
    MarioPartyTop100(Vec<MarioPartyTop100>),
    #[serde(rename = "Super Mario Party")]
    SuperMarioParty(Vec<SuperMarioParty>),
    #[serde(rename = "Mario Party Superstars")]
    MarioPartySuperstars(Vec<MarioPartySuperstars>)
}
