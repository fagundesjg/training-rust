use axum::extract::State;
use axum::{Json, extract::Path, http::StatusCode};

use crate::models::user::User;
use crate::{
    dtos::users::{CreateUser, UpdateUser},
    state::app::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut counter = state.user.counter.lock().unwrap();
    let id = *counter;
    *counter += 1;

    let user = User {
        id,
        name: payload.name,
        birth_date: payload.birth_date,
        gender: payload.gender,
    };

    state.user.users.lock().unwrap().push(user.clone());

    (StatusCode::CREATED, Json(user))
}

pub async fn get_all(State(state): State<AppState>) -> (StatusCode, Json<Vec<User>>) {
    let users = state.user.users.lock().unwrap();
    (StatusCode::OK, Json(users.clone()))
}

pub async fn get_one(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<User>>) {
    let users = state.user.users.lock().unwrap();

    if let Some(user) = users.clone().into_iter().find(|u| u.id == id) {
        return (StatusCode::OK, Json(Some(user.clone())));
    }

    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn remove(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<User>>) {
    let mut users = state.user.users.lock().unwrap();

    if let Some(index) = users.iter().position(|u| u.id == id) {
        let removed = users.remove(index);
        return (StatusCode::OK, Json(Some(removed)));
    }

    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUser>,
) -> (StatusCode, Json<Option<User>>) {
    let mut users = state.user.users.lock().unwrap();

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.update_with(payload);
        return (StatusCode::OK, Json(Some(user.clone())));
    }

    (StatusCode::NOT_FOUND, Json(None))
}
