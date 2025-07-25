use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub birth_date: String,
    pub gender: String,
    #[serde(skip_serializing, default)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
