use regex::Regex;
use async_trait::async_trait;
use tokio::task;
use std::collections::HashMap;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use password_auth::{generate_hash, verify_password};
use axum::{
    Extension,
    http::StatusCode,
    response::IntoResponse,
    Form,
    Json
};
use axum_login::{
    AuthUser,
    AuthnBackend,
    UserId};
use crate::requests::{GameData, MarioPartyData};
use crate::responses::MessageResponse;
use crate::listfields::{ListFields, Field, EnumData, ObjectData};

type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    pub username: String,
    password_hash: String,
}

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash.as_bytes()
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(creds.password, &user.password_hash).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

#[axum::debug_handler]
pub async fn games(
    Extension(pool): Extension<PgPool>,
    mut auth_session: AuthSession,
    Json(mp_data): Json<GameData>
) -> impl IntoResponse {
    println!("data: {mp_data:?}");
    match mp_data.player_data {
        MarioPartyData::MarioParty(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartyEntries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.peak_coins)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.question_spaces)
                    .bind(player_data.minigame_spaces)
                    .bind(player_data.exclaimation_spaces)
                    .bind(player_data.mushroom_spaces)
                    .bind(player_data.bowser_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty2(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty2Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.peak_coins)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.question_spaces)
                    .bind(player_data.exclaimation_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.battle_spaces)
                    .bind(player_data.item_spaces)
                    .bind(player_data.bank_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty3(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty3Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.peak_coins)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.question_spaces)
                    .bind(player_data.exclaimation_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.battle_spaces)
                    .bind(player_data.item_spaces)
                    .bind(player_data.bank_spaces)
                    .bind(player_data.game_guy_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty4(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty4Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.peak_coins)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.happening_spaces)
                    .bind(player_data.fortune_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.battle_spaces)
                    .bind(player_data.mushroom_spaces)
                    .bind(player_data.warp_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty5(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty5Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.peak_coins)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.capsule_spaces)
                    .bind(player_data.question_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.dk_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty6(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty6Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.orbs_used)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.character_spaces)
                    .bind(player_data.question_spaces)
                    .bind(player_data.duel_spaces)
                    .bind(player_data.miracle_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.dk_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty7(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty7Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.coins_spent_on_orbs)
                    .bind(player_data.orbs_used)
                    .bind(player_data.spaces_moved)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.character_spaces)
                    .bind(player_data.green_spaces)
                    .bind(player_data.duel_spaces)
                    .bind(player_data.mic_spaces)
                    .bind(player_data.dk_spaces)
                    .bind(player_data.bowser_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty8(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty8Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.coins_spent_in_shop)
                    .bind(player_data.candy_eaten)
                    .bind(player_data.spaces_moved)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.green_spaces)
                    .bind(player_data.lucky_spaces)
                    .bind(player_data.dk_spaces)
                    .bind(player_data.bowser_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty9(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty9Entries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.mini_stars)
                    .bind(player_data.mini_stars_lost)
                    .bind(player_data.minigame_stars)
                    .bind(player_data.minigame_stars_lost)
                    .bind(player_data.event_mini_stars)
                    .bind(player_data.event_mini_stars_lost)
                    .bind(player_data.bonus_star_mini_stars)
                    .bind(player_data.dice_block_spaces)
                    .bind(player_data.lucky_spaces)
                    .bind(player_data.unlucky_spaces)
                    .bind(player_data.spin_spaces)
                    .bind(player_data.event_spaces)
                    .bind(player_data.shuffle_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.captain_event_spaces)
                    .bind(player_data.boss_battle_spaces)
                    .bind(player_data.dash_spaces)
                    .bind(player_data.back_spaces)
                    .bind(player_data.free_for_all_spaces)
                    .bind(player_data.one_v_three_spaces)
                    .bind(player_data.battle_spaces)
                    .bind(player_data.bowser_jr_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioParty10(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioParty10Entries VALUES (DEFAULT, $1, $2, $3)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.mini_stars)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioPartyDS(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartyDSEntries VALUES (DEFAULT, $1, $2, $3, $4)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioPartyIslandTour(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartyIslandTourEntries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.green_spaces)
                    .bind(player_data.item_spaces)
                    .bind(player_data.dash_spaces)
                    .bind(player_data.blue_event_spaces)
                    .bind(player_data.piranha_plant_spaces)
                    .bind(player_data.red_event_spaces)
                    .bind(player_data.free_for_all_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.spaces_moved)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioPartyStarRush(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartyStarRushEntries VALUES (DEFAULT, $1, $2, $3, $4)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioPartyTop100(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartyTop100Entries VALUES (DEFAULT, $1, $2, $3, $4)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::SuperMarioParty(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO SuperMarioPartyEntries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.hidden_blocks)
                    .bind(player_data.items_used)
                    .bind(player_data.allies)
                    .bind(player_data.spaces_moved)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.lucky_spaces)
                    .bind(player_data.vs_spaces)
                    .bind(player_data.event_spaces)
                    .bind(player_data.item_spaces)
                    .bind(player_data.ally_spaces)
                    .bind(player_data.bad_luck_spaces)
                    .bind(player_data.extra_bad_luck_spaces)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        },
        MarioPartyData::MarioPartySuperstars(players_data) => {
            for player_data in players_data {
                let err = sqlx::query("INSERT INTO MarioPartySuperstarsEntries VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)")
                    .bind(player_data.player_name)
                    .bind(player_data.character)
                    .bind(player_data.stars)
                    .bind(player_data.coins)
                    .bind(player_data.minigame_coins)
                    .bind(player_data.hidden_blocks)
                    .bind(player_data.items_bought)
                    .bind(player_data.items_used)
                    .bind(player_data.spaces_moved)
                    .bind(player_data.blue_spaces)
                    .bind(player_data.red_spaces)
                    .bind(player_data.item_spaces)
                    .bind(player_data.event_spaces)
                    .bind(player_data.chance_time_spaces)
                    .bind(player_data.lucky_spaces)
                    .bind(player_data.bowser_spaces)
                    .bind(player_data.vs_spaces)
                    .bind(player_data.koopa_bank_spaces)
                    .bind(player_data.stickers_used)
                    .execute(&pool)
                    .await;

                err.unwrap();
            }
        }
    }
}

#[axum::debug_handler]
pub async fn signup(
    Extension(pool): Extension<PgPool>,
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user_exists = sqlx::query("SELECT 1 FROM users WHERE username = $1")
        .bind(creds.username.clone())
        .fetch_optional(&pool)
        .await
        .unwrap();

    match user_exists {
        Some(_record) => {
            println!("Already exists");
            return (
                StatusCode::FORBIDDEN,
                Json(
                    MessageResponse {
                        message: String::from("That username already exists"),
                        success: false
                    }
                )
            ).into_response();
        },
        None => {
            println!("Creating user");
            sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
                .bind(creds.username)
                .bind(generate_hash(creds.password))
                .fetch_optional(&pool)
                .await
                .unwrap();
            return (
                StatusCode::OK,
                Json(
                    MessageResponse {
                        message: String::from("User created"),
                        success: true
                    }
                )
            ).into_response();
        }
    }
}

#[axum::debug_handler]
pub async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    println!("login");
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("Unauthorized!");
            return (StatusCode::UNAUTHORIZED, Json(
                MessageResponse {
                    message: String::from("Authorization failed"),
                    success: false
                }
            )).into_response();
        },
        Err(error) => {
            println!("error: {}", error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    return (StatusCode::OK, Json(
    MessageResponse {
                    message: String::from("Success!"),
                    success: true
                }
            )).into_response();
}

#[axum::debug_handler]
pub async fn input_schema() -> impl IntoResponse {
    let mut h: HashMap<String, Vec<Field>> = HashMap::new();

    if let ObjectData::EnumData(EnumData { name, variants }) = MarioPartyData::list_fields() {
        for variant in variants {
            match variant.type_data {
                ObjectData::Fields(fields) => {
                    h.insert(variant.name, fields);
                },
                _ => {}
            }
        }
    }

    return Json(h);
}
