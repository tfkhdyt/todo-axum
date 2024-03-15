use super::dto::AddUserRequest;
use crate::error::{AppError, HttpResult};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::http::StatusCode;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn new(user: AddUserRequest) -> HttpResult<Self> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to hash password")
            })?
            .to_string();

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name: user.name,
            username: user.username,
            password: hashed_password,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        })
    }

    pub fn check_password(&self, password: &str) -> HttpResult<()> {
        let parsed_hash = PasswordHash::new(&self.password).map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to parse hashed password",
            )
        })?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|err| {
                println!("Warn: {}", err);
                AppError::new(StatusCode::UNAUTHORIZED, "password is invalid")
            })?;

        Ok(())
    }
}
