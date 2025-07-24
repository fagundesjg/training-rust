use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

mod db;
mod dtos;
mod handlers;
mod models;
mod routes;
mod state;

use state::app::AppState;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar no banco");

    let state = AppState { db };

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::users::user_routes())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to create listener");

    axum::serve(listener, app).await.unwrap();
}
