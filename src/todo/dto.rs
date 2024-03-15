use super::model::Todo;
use crate::error::{AppError, HttpResult};
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddTodoRequest {
    pub title: String,
    pub desc: Option<String>,
    pub status: String,
}

impl AddTodoRequest {
    fn validate(&self) -> HttpResult<()> {
        match self.status.as_str() {
            "todo" | "ongoing" | "done" => (),
            _ => {
                return Err(AppError::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "status is invalid (status at least should be: todo | ongoing | done)",
                ))
            }
        }

        Ok(())
    }

    pub fn into_todo(self) -> HttpResult<Todo> {
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
    pub fn validate(&self) -> HttpResult<()> {
        if let Some(status) = &self.status {
            match status.as_str() {
                "todo" | "ongoing" | "done" => (),
                _ => return Err(AppError::new(StatusCode::BAD_REQUEST, "status is invalid")),
            }
        }

        Ok(())
    }
}
