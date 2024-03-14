use super::model::Todo;
use crate::error::AppError;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddTodoRequest {
    pub title: String,
    pub desc: Option<String>,
    pub status: String,
}

impl AddTodoRequest {
    fn validate(&self) -> Result<(), AppError> {
        match self.status.as_str() {
            "todo" | "ongoing" | "done" => (),
            _ => return Err(AppError::new(StatusCode::BAD_REQUEST, "status is invalid")),
        }

        Ok(())
    }

    pub fn into_todo(self) -> Result<Todo, AppError> {
        self.validate()?;
        Ok(Todo::new(self))
    }
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub status: Option<String>,
}

impl UpdateTodoRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if let Some(status) = &self.status {
            match status.as_str() {
                "todo" | "ongoing" | "done" => (),
                _ => return Err(AppError::new(StatusCode::BAD_REQUEST, "status is invalid")),
            }
        }

        Ok(())
    }
}
