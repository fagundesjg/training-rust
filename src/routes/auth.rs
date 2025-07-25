use crate::{handlers::auth::login, state::app::AppState};
use axum::{Router, routing::post};

pub fn routes() -> Router<AppState> {
    Router::new().route("/auth", post(login))
}
