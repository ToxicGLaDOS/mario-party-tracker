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
pub struct MarioParty {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub peak_coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub question_spaces: i32,
    pub minigame_spaces: i32,
    pub exclaimation_spaces: i32,
    pub mushroom_spaces: i32,
    pub bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty2 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub peak_coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub question_spaces: i32,
    pub exclaimation_spaces: i32,
    pub bowser_spaces: i32,
    pub battle_spaces: i32,
    pub item_spaces: i32,
    pub bank_spaces: i32,
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty3 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub peak_coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub question_spaces: i32,
    pub exclaimation_spaces: i32,
    pub bowser_spaces: i32,
    pub battle_spaces: i32,
    pub item_spaces: i32,
    pub bank_spaces: i32,
    pub game_guy_spaces: i32,
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty4 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub peak_coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub happening_spaces: i32,
    pub fortune_spaces: i32,
    pub bowser_spaces: i32,
    pub battle_spaces: i32,
    pub mushroom_spaces: i32,
    pub warp_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty5 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub peak_coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub capsule_spaces: i32,
    pub question_spaces: i32,
    pub bowser_spaces: i32,
    pub dk_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty6 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub orbs_used: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub character_spaces: i32,
    pub question_spaces: i32,
    pub duel_spaces: i32,
    pub miracle_spaces: i32,
    pub bowser_spaces: i32,
    pub dk_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty7 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub coins_spent_on_orbs: i32,
    pub orbs_used: i32,
    pub spaces_moved: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub character_spaces: i32,
    pub green_spaces: i32,
    pub duel_spaces: i32,
    pub mic_spaces: i32,
    pub dk_spaces: i32,
    pub bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty8 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub coins_spent_in_shop: i32,
    pub candy_eaten: i32,
    pub spaces_moved: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub green_spaces: i32,
    pub lucky_spaces: i32,
    pub dk_spaces: i32,
    pub bowser_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty9 {
    pub player_name: String,
    pub character: String,
    pub mini_stars: i32,
    pub mini_stars_lost: i32,
    pub minigame_stars: i32,
    pub minigame_stars_lost: i32,
    pub event_mini_stars: i32,
    pub event_mini_stars_lost: i32,
    pub bonus_star_mini_stars: i32,
    pub dice_block_spaces: i32,
    pub lucky_spaces: i32,
    pub unlucky_spaces: i32,
    pub spin_spaces: i32,
    pub event_spaces: i32,
    pub shuffle_spaces: i32,
    pub bowser_spaces: i32,
    pub captain_event_spaces: i32,
    pub boss_battle_spaces: i32,
    pub dash_spaces: i32,
    pub back_spaces: i32,
    pub free_for_all_spaces: i32,
    pub one_v_three_spaces: i32,
    pub battle_spaces: i32,
    pub bowser_jr_spaces: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty10 {
    pub player_name: String,
    pub character: String,
    pub mini_stars: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyDS {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyIslandTour {
    pub player_name: String,
    pub character: String,
    pub green_spaces: i32,
    pub item_spaces: i32,
    pub dash_spaces: i32,
    pub blue_event_spaces: i32,
    pub piranha_plant_spaces: i32,
    pub red_event_spaces: i32,
    pub free_for_all_spaces: i32,
    pub bowser_spaces: i32,
    pub spaces_moved: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyStarRush {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyTop100 {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct SuperMarioParty {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub hidden_blocks: i32,
    pub items_used: i32,
    pub allies: i32,
    pub spaces_moved: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub lucky_spaces: i32,
    pub vs_spaces: i32,
    pub event_spaces: i32,
    pub item_spaces: i32,
    pub ally_spaces: i32,
    pub bad_luck_spaces: i32,
    pub extra_bad_luck_spaces: i32
}


#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartySuperstars {
    pub player_name: String,
    pub character: String,
    pub stars: i32,
    pub coins: i32,
    pub minigame_coins: i32,
    pub hidden_blocks: i32,
    pub items_bought: i32,
    pub items_used: i32,
    pub spaces_moved: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub item_spaces: i32,
    pub event_spaces: i32,
    pub chance_time_spaces: i32,
    pub lucky_spaces: i32,
    pub bowser_spaces: i32,
    pub vs_spaces: i32,
    pub koopa_bank_spaces: i32,
    pub stickers_used: i32
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
