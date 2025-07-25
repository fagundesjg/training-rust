use crate::{models::user::User, state::app::AppState};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_cookie::CookieManager;
use redis::{AsyncCommands, RedisResult};
use serde_json;
use std::future::Future;

pub struct SessionExtractor(pub User);

impl FromRequestParts<AppState> for SessionExtractor {
    type Rejection = StatusCode;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let manager = CookieManager::from_request_parts(parts, state)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

            let session_id = manager
                .get("session_id")
                .ok_or(StatusCode::UNAUTHORIZED)?
                .value()
                .to_string();

            let key = format!("session:{}", session_id);

            let session_res: RedisResult<Option<String>> =
                state.redis.clone().get::<_, Option<String>>(key).await;

            let session_opt = session_res.map_err(|_| StatusCode::UNAUTHORIZED)?;

            let session_json = match session_opt {
                Some(json) => json,
                None => return Err(StatusCode::UNAUTHORIZED),
            };

            let user: User = serde_json::from_str(&session_json)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(SessionExtractor(user))
        }
    }
}
