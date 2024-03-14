use super::model::Todo;
use crate::error::AppError;
use axum::http::StatusCode;
use chrono::Utc;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct TodoRepo {
    pool: Pool<Sqlite>,
}

impl TodoRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, todo: Todo) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO todos (id, title, desc, status) 
            VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(todo.id)
        .bind(todo.title)
        .bind(todo.desc)
        .bind(todo.status)
        .execute(&self.pool)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to add new todos")
        })?;

        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Todo>, AppError> {
        let todos: Vec<Todo> = sqlx::query_as("SELECT * FROM todos")
            .fetch_all(&self.pool)
            .await
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to find all todos",
                )
            })?;

        Ok(todos)
    }

    pub async fn find_one(&self, id: &str) -> Result<Todo, AppError> {
        let todo: Todo = sqlx::query_as("SELECT * FROM todos WHERE id = ?1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(
                    StatusCode::NOT_FOUND,
                    format!("todo with id {} is not found", id),
                )
            })?;

        Ok(todo)
    }

    pub async fn delete_one(&self, id: &str) -> Result<(), AppError> {
        self.find_one(id).await?;

        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| {
                println!("Error: {}", err);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to delete todo with id {}", id),
                )
            })?;

        Ok(())
    }

    pub async fn update_one(&self, id: &str, new_todo: Todo) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE todos SET title = ?2, desc = ?3, status = ?4, updated_at = ?5
            WHERE id = ?1",
        )
        .bind(id)
        .bind(new_todo.title)
        .bind(new_todo.desc)
        .bind(new_todo.status)
        .bind(Utc::now().to_string())
        .execute(&self.pool)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to update todo with id {}", id),
            )
        })?;

        Ok(())
    }
}
