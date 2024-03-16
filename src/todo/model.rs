use crate::error::HttpResult;

use super::dto::{AddTodoRequest, UpdateTodoRequest};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub desc: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Todo {
    pub fn new(todo: AddTodoRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: todo.title.trim().to_owned(),
            desc: todo.desc,
            status: todo.status,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }

    pub fn update(&mut self, new_todo: UpdateTodoRequest) -> HttpResult<()> {
        new_todo.validate()?;
        if let Some(title) = new_todo.title {
            if title != self.title {
                self.title = title
            }
        }
        if let Some(status) = new_todo.status {
            if status != self.status {
                self.status = status
            }
        }
        if new_todo.desc.is_some() && new_todo.desc != self.desc {
            self.desc = new_todo.desc
        }

        Ok(())
    }
}
