use super::dto::AddTodoRequest;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
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
            title: todo.title,
            desc: todo.desc,
            status: todo.status,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        }
    }
}
