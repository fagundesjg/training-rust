use crate::{
    handlers::users::{create, get_all, get_one, remove, update},
    state::app::AppState,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_all))
        .route("/users/{id}", get(get_one))
        .route("/users", post(create))
        .route("/users/{id}", put(update))
        .route("/users/{id}", delete(remove))
}
