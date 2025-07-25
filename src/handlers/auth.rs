use axum::{Json, extract::State, http::StatusCode};

use crate::{
    db::repositories::user::{UserField, UserRepository},
    dtos::auth::{CreateAuth, CreatedAuth},
    state::app::AppState,
    utils::password::verify_password,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<CreateAuth>,
) -> Result<(StatusCode, Json<CreatedAuth>), StatusCode> {
    let user = UserRepository::find_by(&state.db, UserField::Email, &payload.email)
        .await
        .unwrap();

    if user.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let matched = verify_password(&payload.password, &user.unwrap().password);

    if !matched {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok((
        StatusCode::CREATED,
        Json(CreatedAuth {
            token: "TOKEN-SECRETO".to_string(),
        }),
    ))
}
