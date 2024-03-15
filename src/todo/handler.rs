use super::dto::{AddTodoRequest, UpdateTodoRequest};
use crate::{error::HttpResult, AppState};
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
) -> HttpResult<impl IntoResponse> {
    let new_todo = payload.into_todo()?;
    state.todo_repo.create(&state.pool, new_todo).await?;

    let response = (
        StatusCode::CREATED,
        Json(json!({
            "message": "new todo has been added"
        })),
    );

    Ok(response)
}

pub async fn find_all_todos(State(state): State<AppState>) -> HttpResult<impl IntoResponse> {
    let todos = state.todo_repo.find_all(&state.pool).await?;

    Ok(Json(todos))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> HttpResult<impl IntoResponse> {
    state.todo_repo.find_one(&state.pool, &id).await?;
    state.todo_repo.delete_one(&state.pool, &id).await?;

    let response = Json(json!({
        "message": format!("todo with id {} has been deleted", id)
    }));

    Ok(response)
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodoRequest>,
) -> HttpResult<impl IntoResponse> {
    let mut todo = state.todo_repo.find_one(&state.pool, &id).await?;
    todo.update(payload)?;
    state.todo_repo.update_one(&state.pool, &id, todo).await?;

    let response = Json(json!({
        "message": format!("todo with id {} has been updated", id)
    }));

    Ok(response)
}
