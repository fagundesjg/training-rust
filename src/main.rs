use axum::{Router, middleware::from_fn, routing::get};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;

mod db;
mod dtos;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod state;
mod utils;

use state::app::AppState;

use crate::middlewares::logging::logging;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar no banco");

    let state = AppState { db };

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::users::routes())
        .merge(routes::auth::routes())
        .with_state(state.clone())
        .layer(from_fn(logging));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to create listener");

    axum::serve(listener, app).await.unwrap();
}
