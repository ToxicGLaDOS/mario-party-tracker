use tokio;
use tokio::task;
use clap::Parser;
use regex::Regex;
use std::env;
use std::collections::HashMap;
use sqlx::FromRow;
use sqlx::postgres::{PgPool, PgPoolOptions};
use serde::{Serialize, Deserialize, de::Error as SerdeError, Deserializer};
use password_auth::{generate_hash, verify_password};
use axum::{
    Extension,
    routing::{get, post, get_service},
    Router,
    http::StatusCode,
    response::IntoResponse,
    Form,
    Json
};
use async_trait::async_trait;
use axum_login::{
    AuthUser,
    AuthnBackend,
    UserId,
    login_required,
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, services::ServeFile};

#[derive(Parser, Debug)]
struct CliOptions {
    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8081")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../client/dist")]
    static_dir: String,

    /// set whether this is a dev server
    #[clap(long = "dev", default_value = "false")]
    dev: bool
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let opts = CliOptions::parse();

    // Defaults values correspond to development postgres, not production
    let pg_user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    let pg_password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password"));
    let pg_host = env::var("POSTGRES_HOST").unwrap_or(String::from("localhost"));
    let pg_port = env::var("POSTGRES_PORT").unwrap_or(String::from("55432"));
    let pg_database = env::var("POSTGRES_DB").unwrap_or(String::from("mario-party-tracker"));

    let connection_string = format!("postgres://{}:{}@{}:{}/{}", pg_user, pg_password, pg_host, pg_port, pg_database);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string.as_str()).await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let backend = Backend::new(pool.clone());
    //backend.users.insert(123, User{id: 123, pw_hash: "foo".to_string()});
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let fallback = get_service(ServeFile::new(format!("{}/index.html", opts.static_dir))).handle_error(
        |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") },
    );

    let mut app = Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route("/api/games", post(games))
        .route("/api/input/schema", get(input_schema))
        .layer(auth_layer)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
                .layer(CorsLayer::permissive())
        );

    if !opts.dev {
        println!("Running production server");
        app = app.nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", opts.static_dir))
        )
        .fallback_service(fallback);
    }
    else {
        println!("Running devlopment server");
    }

    println!("Hosting on {}:{}", opts.addr, opts.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", opts.addr, opts.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i32,
    pub username: String,
    password_hash: String,
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

#[derive(Serialize)]
struct MessageResponse {
    message: String,
    success: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct GameData {
    game: String,
    board: String,
    turns: i32,
    player_data: Vec<MarioPartyData>
}


#[derive(Serialize, Debug)]
struct Field {
    name: String,
    thetype: String
}

macro_rules! expose_field_data {
    ($(#[$macro:ident $tokens:tt])* enum $name:ident { $($variantname:ident { $($fname:ident : $ftype:ty),* $(,)? }),* $(,)?}) => {
        $(#[$macro $tokens])*
        enum $name {
            $($variantname { $($fname : $ftype),* }),*
        }

        impl $name {
            fn field_data() -> HashMap<&'static str, Vec<Field>> {
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


expose_field_data! {
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum MarioPartyData {
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

type AuthSession = axum_login::AuthSession<Backend>;

#[axum::debug_handler]
async fn input_schema() -> impl IntoResponse {
    let re = Regex::new(r"([A-Z]|[0-9]+)").unwrap();
    let mut field_data = MarioPartyData::field_data();
    let mut new_data: HashMap<String, Vec<Field>> = HashMap::new();
    for (key, value) in field_data.drain() {
        let after = re.replace_all(key, " $1");
        new_data.insert(after.clone().trim().to_string(), value);
    }

    return Json(new_data);
}

#[axum::debug_handler]
async fn games(
    Extension(pool): Extension<PgPool>,
    mut auth_session: AuthSession,
    Json(mp_data): Json<GameData>
) -> impl IntoResponse {
    println!("data: {mp_data:?}");
    for data in mp_data.player_data {
        match data {
            MarioPartyData::MarioParty { .. } => {
                println!("Mario party one");
            },
            MarioPartyData::MarioParty2 { .. } => {
                println!("Mario party two");
            },
            MarioPartyData::MarioParty3 { .. } => {
                println!("Mario party three");
            }
        }
    }
}

#[axum::debug_handler]
async fn signup(
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
async fn login(
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
