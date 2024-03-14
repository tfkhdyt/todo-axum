use anyhow::Result;
use axum::{
    routing::{delete, get},
    Router,
};
use std::env;
use todo::{handler, repo::TodoRepo};

mod db;
mod error;
mod todo;

#[derive(Clone)]
struct AppState {
    todo_repo: TodoRepo,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    let port = env::var("PORT")?;

    let pool = db::get_sqlite_connection().await?;

    let todo_repo = TodoRepo::new(pool);
    let shared_state = AppState { todo_repo };

    let app = Router::new()
        .route(
            "/todos",
            get(handler::find_all_todos).post(handler::add_todo),
        )
        .route(
            "/todos/:id",
            delete(handler::delete_todo).put(handler::update_todo),
        )
        .with_state(shared_state);

    println!("Running on http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
