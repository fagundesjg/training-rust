use crate::dtos::users::{CreateUser, UpdateUser};
use crate::models::user::User;
use chrono::{Timelike, Utc};
use cuid2;
use sqlx::{Result, SqlitePool};

pub struct UserRepository;

pub enum UserField {
    Id,
    Email,
}

impl UserField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Email => "email",
        }
    }
}

impl UserRepository {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, email, birth_date, gender, password, created_at, updated_at
                FROM users
                ORDER BY created_at DESC
                "#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by(pool: &SqlitePool, field: UserField, value: &str) -> Result<Option<User>> {
        let query = format!(
            r#"
        SELECT id, name, email, birth_date, gender, password, created_at, updated_at
        FROM users
        WHERE {} = ?
        "#,
            field.as_str()
        );

        sqlx::query_as::<_, User>(&query)
            .bind(value)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &SqlitePool, data: CreateUser) -> Result<User> {
        let cuid = cuid2::create_id();
        let now = Utc::now().naive_utc().with_nanosecond(0).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, email, birth_date, gender, password, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            cuid,
            data.name,
            data.email,
            data.birth_date,
            data.gender,
            data.password,
            now,
        )
        .execute(pool)
        .await?;

        Self::find_by(pool, UserField::Id, &cuid)
            .await
            .map(|u| u.unwrap())
    }

    pub async fn update(pool: &SqlitePool, id: &str, payload: UpdateUser) -> Result<Option<User>> {
        let result = sqlx::query(
            r#"
        UPDATE users
        SET
          name        = COALESCE(?, name),
          birth_date  = COALESCE(?, birth_date),
          gender      = COALESCE(?, gender),
          password    = COALESCE(?, password)
        WHERE id = ?
        "#,
        )
        .bind(payload.name) // Option<String>
        .bind(payload.birth_date) // Option<NaiveDateTime> ou Option<String>
        .bind(payload.gender.map(|g| g.to_string())) // Option<String>
        .bind(payload.password) // Option<String>
        .bind(id) // &str
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        let updated = UserRepository::find_by(pool, UserField::Id, id).await?;
        Ok(updated)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}
