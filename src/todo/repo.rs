use super::model::Todo;
use crate::error::{AppError, HttpResult};
use axum::http::StatusCode;
use chrono::Utc;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct TodoRepo;

impl TodoRepo {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, pool: &SqlitePool, todo: Todo) -> HttpResult<()> {
        sqlx::query(
            "INSERT INTO todos (id, title, desc, status) 
            VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(todo.id)
        .bind(todo.title)
        .bind(todo.desc)
        .bind(todo.status)
        .execute(pool)
        .await
        .map_err(|err| {
            println!("Error: {}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to add new todos")
        })?;

        Ok(())
    }

    pub async fn find_all(&self, pool: &SqlitePool) -> HttpResult<Vec<Todo>> {
        let todos: Vec<Todo> = sqlx::query_as("SELECT * FROM todos")
            .fetch_all(pool)
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

    pub async fn find_one(&self, pool: &SqlitePool, id: &str) -> HttpResult<Todo> {
        let todo: Todo = sqlx::query_as("SELECT * FROM todos WHERE id = ?1")
            .bind(id)
            .fetch_one(pool)
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

    pub async fn delete_one(&self, pool: &SqlitePool, id: &str) -> HttpResult<()> {
        self.find_one(pool, id).await?;

        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(pool)
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

    pub async fn update_one(&self, pool: &SqlitePool, id: &str, new_todo: Todo) -> HttpResult<()> {
        sqlx::query(
            "UPDATE todos SET title = ?2, desc = ?3, status = ?4, updated_at = ?5
            WHERE id = ?1",
        )
        .bind(id)
        .bind(new_todo.title)
        .bind(new_todo.desc)
        .bind(new_todo.status)
        .bind(Utc::now().to_string())
        .execute(pool)
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
