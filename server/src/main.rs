use tokio;
use clap::Parser;
use std::env;
use sqlx::postgres::PgPoolOptions;
use axum::{
    Extension,
    routing::{get, post, get_service},
    Router,
    http::StatusCode
};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir, services::ServeFile};
use crate::routes::Backend;
use listfields_derive::ListFields;
use crate::requests::MarioPartyData;
use crate::listfields::{ObjectData, EnumData, Variant, ListFields, Field};

pub mod routes;
pub mod requests;
pub mod responses;
pub mod listfields;


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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let opts = CliOptions::parse();

    // Defaults values correspond to development postgres, not production
    let pg_user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    // TODO: Make this not hardcoded to password haha
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
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnSessionEnd);

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
        .route("/api/login", post(routes::login))
        .route("/api/signup", post(routes::signup))
        .route("/api/games", post(routes::games))
        .route("/api/input/schema", get(routes::input_schema))
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
