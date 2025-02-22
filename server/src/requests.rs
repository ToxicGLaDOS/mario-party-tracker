use serde::{Serialize, Deserialize, Deserializer, de, de::{Visitor, Unexpected, Error}};
use std::collections::HashMap;
use std::fmt;
use listfields_derive::ListFields;
use crate::listfields::{ObjectData, EnumData, Variant, ListFields, Field};
use chrono::{DateTime, Utc, Local};

#[derive(Deserialize, Debug)]
pub struct GameData {
    #[serde(flatten)]
    pub player_data: MarioPartyData,
    pub board: String,
    pub turns: i32,
    pub date: DateTime<Utc>
}

// sqlx::Type is what lets us .bind() the enum to the sql query directly
// instead of having to convert it to a string. This relies on a sql enum
// existing for it though
#[derive(Serialize, Deserialize, ListFields, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioPartyCharacters\"")] // Preserves casing
pub enum MarioPartyCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty2Characters\"")] // Preserves casing
pub enum MarioParty2Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty3Characters\"")] // Preserves casing
pub enum MarioParty3Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    Daisy,
    Waluigi
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty4Characters\"")] // Preserves casing
pub enum MarioParty4Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    Daisy,
    Waluigi
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty5Characters\"")] // Preserves casing
pub enum MarioParty5Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Boo,
    #[serde(rename = "Koopa Kid")]
    #[sqlx(rename = "Koopa Kid")]
    KoopaKid
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty6Characters\"")] // Preserves casing
pub enum MarioParty6Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Boo,
    #[serde(rename = "Koopa Kid")]
    #[sqlx(rename = "Koopa Kid")]
    KoopaKid,
    Toadette
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty7Characters\"")] // Preserves casing
pub enum MarioParty7Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Boo,
    Toadette,
    Birdo,
    #[serde(rename = "Dry Bones")]
    #[sqlx(rename = "Dry Bones")]
    DryBones
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty8Characters\"")] // Preserves casing
pub enum MarioParty8Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Boo,
    Toadette,
    Birdo,
    #[serde(rename = "Dry Bones")]
    #[sqlx(rename = "Dry Bones")]
    DryBones,
    Blooper,
    #[serde(rename = "Hammer Bro")]
    #[sqlx(rename = "Hammer Bro")]
    HammerBro
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty9Characters\"")] // Preserves casing
pub enum MarioParty9Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Birdo,
    Koopa,
    #[serde(rename = "Shy Guy")]
    #[sqlx(rename = "Shy Guy")]
    ShyGuy,
    Magikoopa
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty10Characters\"")] // Preserves casing
pub enum MarioParty10Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Toadette,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    Rosalina,
    Bowser,
    Spike
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioPartyDSCharacters\"")] // Preserves casing
pub enum MarioPartyDSCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty:IslandTourCharacters\"")] // Preserves casing
pub enum MarioPartyIslandTourCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Boo,
    #[serde(rename = "Bowser Jr.")]
    #[sqlx(rename = "Bowser Jr.")]
    BowserJr
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty:StarRushCharacters\"")] // Preserves casing
pub enum MarioPartyStarRushCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Toad,
    Toadette,
    Rosalina,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    #[serde(rename = "Diddy Kong")]
    #[sqlx(rename = "Diddy Kong")]
    DiddyKong
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioParty:TheTop100Characters\"")] // Preserves casing
pub enum MarioPartyTop100Characters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Rosalina
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"SuperMarioPartyCharacters\"")] // Preserves casing
pub enum SuperMarioPartyCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Rosalina,
    Bowser,
    Goomba,
    #[serde(rename = "Shy Guy")]
    #[sqlx(rename = "Shy Guy")]
    ShyGuy,
    Koopa,
    #[serde(rename = "Monty Mole")]
    #[sqlx(rename = "Monty Mole")]
    MontyMole,
    #[serde(rename = "Bowser Jr.")]
    #[sqlx(rename = "Bowser Jr.")]
    BowserJr,
    Boo,
    #[serde(rename = "Hammer Bro")]
    #[sqlx(rename = "Hammer Bro")]
    HammerBro,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    #[serde(rename = "Diddy Kong")]
    #[sqlx(rename = "Diddy Kong")]
    DiddyKong,
    #[serde(rename = "Dry Bones")]
    #[sqlx(rename = "Dry Bones")]
    DryBones,
    #[serde(rename = "Pom Pom")]
    #[sqlx(rename = "Pom Pom")]
    PomPom
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioPartySuperstarsCharacters\"")] // Preserves casing
pub enum MarioPartySuperstarsCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Rosalina,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    Birdo
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(type_name = "\"MarioPartyJamboreeCharacters\"")] // Preserves casing
pub enum MarioPartyJamboreeCharacters {
    Mario,
    Luigi,
    Peach,
    Yoshi,
    Wario,
    Daisy,
    Waluigi,
    Rosalina,
    Bowser,
    Goomba,
    #[serde(rename = "Shy Guy")]
    #[sqlx(rename = "Shy Guy")]
    ShyGuy,
    Koopa,
    #[serde(rename = "Monty Mole")]
    #[sqlx(rename = "Monty Mole")]
    MontyMole,
    #[serde(rename = "Bowser Jr.")]
    #[sqlx(rename = "Bowser Jr.")]
    BowserJr,
    Boo,
    Toad,
    Toadette,
    #[serde(rename = "Donkey Kong")]
    #[sqlx(rename = "Donkey Kong")]
    DonkeyKong,
    Birdo,
    Spike,
    Pauline,
    Ninji
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioParty {
    pub player_name: String,
    pub character: MarioPartyCharacters,
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
    pub character: MarioParty2Characters,
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
    pub character: MarioParty3Characters,
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
    pub character: MarioParty4Characters,
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
    pub character: MarioParty5Characters,
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
    pub character: MarioParty6Characters,
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
    pub character: MarioParty7Characters,
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
    pub character: MarioParty8Characters,
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
    pub character: MarioParty9Characters,
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
    pub character: MarioParty10Characters,
    pub mini_stars: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyDS {
    pub player_name: String,
    pub character: MarioPartyDSCharacters,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyIslandTour {
    pub player_name: String,
    pub character: MarioPartyIslandTourCharacters,
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
    pub character: MarioPartyStarRushCharacters,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct MarioPartyTop100 {
    pub player_name: String,
    pub character: MarioPartyTop100Characters,
    pub stars: i32,
    pub coins: i32
}

#[derive(Deserialize, ListFields, Clone, Debug)]
pub struct SuperMarioParty {
    pub player_name: String,
    pub character: SuperMarioPartyCharacters,
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
    pub character: MarioPartySuperstarsCharacters,
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
pub struct MarioPartyJamboree {
    pub player_name: String,
    pub character: MarioPartyJamboreeCharacters,
    pub stars: i32,
    pub coins: i32,
    pub blue_spaces: i32,
    pub red_spaces: i32,
    pub lucky_spaces: i32,
    pub unlucky_spaces: i32,
    pub item_spaces: i32,
    pub bowser_spaces: i32,
    pub event_spaces: i32,
    pub chance_time_spaces: i32,
    pub vs_spaces: i32,
    pub coins_received: i32,
    pub minigames_won: i32,
    pub hidden_blocks_found: i32,
    pub showdown_minigames_won: i32,
    pub items_bought: i32,
    pub items_used: i32,
    pub spaces_traveled: i32,
    pub reactions_used: i32,
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
    MarioPartySuperstars(Vec<MarioPartySuperstars>),
    #[serde(rename = "Mario Party Jamboree")]
    MarioPartyJamboree(Vec<MarioPartyJamboree>)
}
