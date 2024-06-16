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
        let user = sqlx::query_as("SELECT * FROM users WHERE id = ?")
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
    //for data in mp_data.player_data {
    //    match data {
    //        MarioPartyData::MarioParty { .. } => {
    //            println!("Mario party one");
    //        },
    //        MarioPartyData::MarioParty2 { .. } => {
    //            println!("Mario party two");
    //        },
    //        MarioPartyData::MarioParty3 { .. } => {
    //            println!("Mario party three");
    //        },
    //    }
    //}
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
