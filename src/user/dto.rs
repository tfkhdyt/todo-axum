use super::model::User;
use crate::error::{AppError, HttpResult};
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddUserRequest {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl AddUserRequest {
    fn validate(&self) -> HttpResult<()> {
        if self.username.len() < 4 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "username cannot be less than 4 characters",
            ));
        }
        if self.username.len() > 16 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "username cannot be more than 16 characters",
            ));
        }

        if self.password.len() < 8 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "password cannot be less than 8 characters",
            ));
        }

        Ok(())
    }

    pub fn into_user(self) -> HttpResult<User> {
        self.validate()?;
        let new_user = User::new(self)?;

        Ok(new_user)
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl LoginRequest {
    pub fn validate(&self) -> HttpResult<()> {
        if self.username.len() < 4 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "username cannot be less than 4 characters",
            ));
        }
        if self.username.len() > 16 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "username cannot be more than 16 characters",
            ));
        }

        if self.password.len() < 8 {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "password cannot be less than 8 characters",
            ));
        }

        Ok(())
    }
}
