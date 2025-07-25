use axum::{Json, extract::State, http::StatusCode};
use axum_cookie::{CookieManager, cookie::Cookie};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    db::repositories::user::{UserField, UserRepository},
    dtos::auth::CreateAuth,
    extractors::session::SessionExtractor,
    models::user::User,
    state::app::AppState,
    utils::password::verify_password,
};

pub async fn login(
    State(state): State<AppState>,
    cookie: CookieManager,
    Json(payload): Json<CreateAuth>,
) -> Result<StatusCode, StatusCode> {
    let Some(user) = UserRepository::find_by(&state.db, UserField::Email, &payload.email)
        .await
        .unwrap()
    else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let matched = verify_password(&payload.password, &user.password);

    if !matched {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_id = Uuid::new_v4().to_string();
    cookie.add(Cookie::new("session_id", session_id.clone()));

    let user_json = serde_json::to_string(&user).unwrap();

    let key = format!("session:{}", session_id.clone());
    state
        .redis
        .clone()
        .set_ex::<String, String, ()>(key, user_json, 3600)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get_session(
    SessionExtractor(user): SessionExtractor,
) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    Ok((StatusCode::OK, Json(Some(user))))
}
