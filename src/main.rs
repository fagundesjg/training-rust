use axum::{Router, middleware::from_fn, routing::get};
use axum_cookie::CookieLayer;
use dotenvy::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;

mod db;
mod dtos;
mod extractors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod state;
mod utils;

use crate::{middlewares::logging::logging, state::app::init_app_state};

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

    let state = init_app_state().await;

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::users::routes())
        .merge(routes::auth::routes())
        .with_state(state.clone())
        .layer(from_fn(logging))
        .layer(CookieLayer::default());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to create listener");

    axum::serve(listener, app).await.unwrap();
}
