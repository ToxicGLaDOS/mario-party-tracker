use tokio;
use clap::Parser;
use std::env;
use std::collections::HashMap;
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;
use axum::{
    Extension,
    routing::{get, post, get_service},
    Router,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form
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
    #[clap(long = "static-dir", default_value = "../client/dist/index.html")]
    static_dir: String,
}

#[derive(Clone, Default)]
struct Backend {
    users: HashMap<i64, User>,
}

#[derive(Clone, Deserialize)]
struct Credentials {
    user_id: i64,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        println!("{}", user_id);
        Ok(self.users.get(&user_id).cloned())
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
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

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Auth service.
    let mut backend = Backend::default();
    backend.users.insert(123, User{id: 123, pw_hash: "foo".to_string()});
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let fallback = get_service(ServeFile::new(opts.static_dir.clone())).handle_error(
        |_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") },
    );

    // build our application with a single route
    let app = Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
        // Routes that don't match the method, but do
        // match the route cause a 405 and don't get
        // sent to the fallback without this merge
        .route("/login", post(login).merge(fallback.clone())) 
        .layer(auth_layer)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
                .layer(CorsLayer::permissive())
        )
        .nest_service(
            "/assets",
            ServeDir::new(opts.static_dir)
        )
        .fallback_service(fallback);

    println!("Hosting on {}:{}", opts.addr, opts.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", opts.addr, opts.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    id: i64,
    pw_hash: String,
}


impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.pw_hash.as_bytes()
    }
}


type AuthSession = axum_login::AuthSession<Backend>;

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
            return Redirect::to("/login").into_response();
        },
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}
