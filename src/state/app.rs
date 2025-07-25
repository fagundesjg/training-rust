use redis::aio::MultiplexedConnection;
use sqlx::SqlitePool;

use crate::db::{db::init_db, redis::init_redis};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub redis: MultiplexedConnection,
}

pub async fn init_app_state() -> AppState {
    AppState {
        db: init_db().await,
        redis: init_redis().await,
    }
}
