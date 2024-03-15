use super::model::User;
use crate::error::{AppError, HttpResult};
use axum::http::StatusCode;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct UserRepo {}

impl UserRepo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn verify_username_availability(
        &self,
        pool: &SqlitePool,
        username: &str,
    ) -> HttpResult<()> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?1")
            .bind(username)
            .fetch_one(pool)
            .await;

        if let Ok(_) = user {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "username has been used",
            ));
        }

        Ok(())
    }

    pub async fn create(&self, pool: &SqlitePool, user: User) -> HttpResult<()> {
        sqlx::query("INSERT INTO users (id, name, username, password) VALUES (?1, ?2, ?3, ?4)")
            .bind(user.id)
            .bind(user.name)
            .bind(user.username)
            .bind(user.password)
            .execute(pool)
            .await
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to add new user")
            })?;

        Ok(())
    }
}