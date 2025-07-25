use crate::{
    handlers::auth::{get_session, login},
    state::app::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth", post(login))
        .route("/auth", get(get_session))
}
