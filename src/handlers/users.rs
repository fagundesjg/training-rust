use crate::{
    db::repositories::user::UserRepository,
    dtos::users::{CreateUser, UpdateUser},
    models::user::User,
    state::app::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    match UserRepository::create(&state.db, payload).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    match UserRepository::find_all(&state.db).await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_one(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    match UserRepository::find_by_id(&state.db, &id).await {
        Ok(Some(user)) => Ok((StatusCode::OK, Json(Some(user)))),
        Ok(None) => Ok((StatusCode::NOT_FOUND, Json(None))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    match UserRepository::delete(&state.db, &id).await {
        Ok(affected) if affected > 0 => Ok((StatusCode::OK, Json(None))),
        Ok(_) => Ok((StatusCode::NOT_FOUND, Json(None))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUser>,
) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    match UserRepository::update(&state.db, &id, payload).await {
        Ok(Some(user)) => Ok((StatusCode::OK, Json(Some(user)))),
        Ok(None) => Ok((StatusCode::NOT_FOUND, Json(None))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
