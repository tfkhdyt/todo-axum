use super::dto::AddTodoRequest;
use crate::{error::AppError, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AddTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let new_todo = payload.into_todo()?;
    state.todo_repo.create(new_todo).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "new todo has been added"
        })),
    ))
}

pub async fn find_all_todo(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let todos = state.todo_repo.find_all().await?;

    Ok(Json(todos))
}
