use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::env;

pub async fn init_db() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar no banco")
}
