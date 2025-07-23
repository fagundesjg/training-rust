use axum::{Router, routing::get};
use std::sync::{Arc, Mutex};

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
    let state = AppState {
        user: state::app::UserState {
            users: Arc::new(Mutex::new(vec![])),
            counter: Arc::new(Mutex::new(1)),
        },
    };

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::users::user_routes())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
