use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub game: String,
    pub board: String,
    pub turns: i32,
    pub player_data: Vec<MarioPartyData>
}

macro_rules! expose_field_data {
    ($(#[$macro:ident $tokens:tt])* pub enum $name:ident { $($variantname:ident { $($fname:ident : $ftype:ty),* $(,)? }),* $(,)?}) => {
        $(#[$macro $tokens])*
        pub enum $name {
            $($variantname { $($fname : $ftype),* }),*
        }

        impl $name {
            pub fn field_data() -> HashMap<&'static str, Vec<Field>> {
                let mut map = HashMap::new();
                $(
                    let mut v = Vec::new();
                    $(
                        v.push(
                            Field {
                                name: stringify!($fname).to_string(),
                                thetype: stringify!($ftype).to_string()
                            }
                        );
                    )*
                    map.insert(stringify!($variantname), v);
                )*
                map
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Field {
    name: String,
    thetype: String
}

expose_field_data! {
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MarioPartyData {
    MarioParty {
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
    },
    MarioParty2 {
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
    },
    MarioParty3 {
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
}
}
