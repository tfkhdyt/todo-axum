use super::handler::{add_todo, delete_todo, find_all_todos, update_todo};
use crate::AppState;
use axum::{
    routing::{delete, get},
    Router,
};

pub fn todo_route() -> Router<AppState> {
    Router::new()
        .route("/", get(find_all_todos).post(add_todo))
        .route("/:id", delete(delete_todo).put(update_todo))
}
