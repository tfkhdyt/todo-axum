use super::dto::{AddTodoRequest, UpdateTodoRequest};
use crate::{error::AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn add_todo(
    State(state): State<AppState>,
    Json(payload): Json<AddTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let new_todo = payload.into_todo()?;
    state.todo_repo.create(new_todo).await?;

    let response = (
        StatusCode::CREATED,
        Json(json!({
            "message": "new todo has been added"
        })),
    );

    Ok(response)
}

pub async fn find_all_todos(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let todos = state.todo_repo.find_all().await?;

    Ok(Json(todos))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    state.todo_repo.delete_one(id.as_str()).await?;

    let response = Json(json!({
        "message": format!("todo with id {} has been deleted", id)
    }));

    Ok(response)
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut todo = state.todo_repo.find_one(id.as_str()).await?;
    todo.update(payload)?;
    state.todo_repo.update_one(&id, todo).await?;

    let response = Json(json!({
        "message": format!("todo with id {} has been updated", id)
    }));

    Ok(response)
}
